use axum::{
    body::Body,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::stream::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Instant;
use bytes::Bytes;
use tokio::sync::oneshot;
use crate::logger::RequestLog;
use super::token_counter::TokenCounter;

/// 包装流，用于在转发的同时收集 Token 统计信息
struct TokenCollectorStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    token_stats: Arc<Mutex<TokenStats>>,
    completion_tx: Option<oneshot::Sender<()>>,
}

#[derive(Default, Clone)]
struct TokenStats {
    input_tokens: i32,
    output_tokens: i32,
    cache_creation_input_tokens: i32,
    cache_read_input_tokens: i32,
    has_usage: bool,  // 标记是否已经收集到 usage 信息
    output_text: String,  // 收集输出文本用于本地计数
}

impl Stream for TokenCollectorStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                // 尝试解析 Token 信息（不阻塞转发）
                if let Ok(text) = std::str::from_utf8(&chunk) {
                    // 添加调试日志：输出收到的 chunk 内容（截断显示）
                    let preview = if text.len() > 500 {
                        format!("{}...", &text[..500])
                    } else {
                        text.to_string()
                    };
                    log::debug!("📦 Received chunk ({} bytes): {}", chunk.len(), preview);

                    if let Ok(mut stats) = self.token_stats.lock() {
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let json_str = &line[6..];

                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                                    // 记录事件类型
                                    let event_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("unknown");
                                    log::debug!("🔍 SSE event type: {}", event_type);

                                    // 收集输出文本（用于本地 token 计数）
                                    if event_type == "content_block_delta" {
                                        if let Some(delta) = json.get("delta") {
                                            if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                                stats.output_text.push_str(text);
                                            }
                                        }
                                    }

                                    // 尝试从顶层 usage 字段提取（message_delta 事件）
                                    if let Some(usage) = json.get("usage") {
                                        stats.has_usage = true;
                                        log::debug!("✅ Found usage in top-level: {:?}", usage);

                                        // 使用最新值更新（SSE 流中的 usage 是累积的，每次都是完整值）
                                        // 只在字段存在时更新，避免用 0 覆盖已有的非零值
                                        if let Some(input) = usage.get("input_tokens")
                                            .and_then(|t| t.as_i64())
                                            .or_else(|| usage.get("prompt_tokens").and_then(|t| t.as_i64())) {
                                            if input > 0 || stats.input_tokens == 0 {
                                                stats.input_tokens = input as i32;
                                            }
                                        }
                                        if let Some(output) = usage.get("output_tokens")
                                            .and_then(|t| t.as_i64())
                                            .or_else(|| usage.get("completion_tokens").and_then(|t| t.as_i64())) {
                                            if output > 0 || stats.output_tokens == 0 {
                                                stats.output_tokens = output as i32;
                                            }
                                        }
                                        if let Some(cache_creation) = usage.get("cache_creation_input_tokens")
                                            .and_then(|t| t.as_i64()) {
                                            if cache_creation > 0 || stats.cache_creation_input_tokens == 0 {
                                                stats.cache_creation_input_tokens = cache_creation as i32;
                                            }
                                        }
                                        if let Some(cache_read) = usage.get("cache_read_input_tokens")
                                            .and_then(|t| t.as_i64()) {
                                            if cache_read > 0 || stats.cache_read_input_tokens == 0 {
                                                stats.cache_read_input_tokens = cache_read as i32;
                                            }
                                        }
                                        log::debug!("📊 Updated token stats: in={}, out={}, cache_creation={}, cache_read={}",
                                            stats.input_tokens, stats.output_tokens,
                                            stats.cache_creation_input_tokens, stats.cache_read_input_tokens);
                                    }

                                    // 尝试从 message.usage 字段提取（message_start 事件）
                                    if let Some(message) = json.get("message") {
                                        if let Some(usage) = message.get("usage") {
                                            stats.has_usage = true;
                                            log::debug!("✅ Found usage in message: {:?}", usage);

                                            if let Some(input) = usage.get("input_tokens")
                                                .and_then(|t| t.as_i64())
                                                .or_else(|| usage.get("prompt_tokens").and_then(|t| t.as_i64())) {
                                                stats.input_tokens = input as i32;
                                            }
                                            if let Some(output) = usage.get("output_tokens")
                                                .and_then(|t| t.as_i64())
                                                .or_else(|| usage.get("completion_tokens").and_then(|t| t.as_i64())) {
                                                stats.output_tokens = output as i32;
                                            }
                                            if let Some(cache_creation) = usage.get("cache_creation_input_tokens")
                                                .and_then(|t| t.as_i64()) {
                                                stats.cache_creation_input_tokens = cache_creation as i32;
                                            }
                                            if let Some(cache_read) = usage.get("cache_read_input_tokens")
                                                .and_then(|t| t.as_i64()) {
                                                stats.cache_read_input_tokens = cache_read as i32;
                                            }
                                            log::debug!("📊 Updated token stats: in={}, out={}, cache_creation={}, cache_read={}",
                                                stats.input_tokens, stats.output_tokens,
                                                stats.cache_creation_input_tokens, stats.cache_read_input_tokens);
                                        }
                                    }
                                } else {
                                    log::debug!("⚠️  Failed to parse JSON from SSE line");
                                }
                            }
                        }
                    }
                }
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => {
                log::error!("Stream error: {}", e);
                // 流出错时也要发送完成信号
                if let Some(tx) = self.completion_tx.take() {
                    let _ = tx.send(());
                }
                Poll::Ready(Some(Err(std::io::Error::new(std::io::ErrorKind::Other, e))))
            }
            Poll::Ready(None) => {
                // 流结束时发送完成信号
                if let Some(tx) = self.completion_tx.take() {
                    log::debug!("Stream completed, sending completion signal");
                    let _ = tx.send(());
                }
                Poll::Ready(None)
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

/// 处理流式响应（真正的流式转发）
pub(super) async fn handle_stream_response(
    response: reqwest::Response,
    request_log: RequestLog,
    start_time: Instant,
    request_body: String,  // 添加请求体参数用于计算 input tokens
) -> Result<Response, StatusCode> {
    // 获取响应头
    let mut response_headers = HeaderMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
            response_headers.insert(key.clone(), value);
        }
    }

    // 创建共享的 Token 统计
    let token_stats = Arc::new(Mutex::new(TokenStats::default()));
    let token_stats_clone = Arc::clone(&token_stats);

    // 创建 channel 用于流完成通知
    let (completion_tx, completion_rx) = oneshot::channel();

    // 创建包装流
    let stream = TokenCollectorStream {
        inner: Box::pin(response.bytes_stream()),
        token_stats: token_stats_clone,
        completion_tx: Some(completion_tx),
    };

    // 在流结束后更新日志（等待流真正完成的信号）
    let request_log_clone = request_log.clone();
    let request_body_clone = request_body.clone();
    tokio::spawn(async move {
        // 等待流完成信号，最多等待 120 秒（超长响应的兜底）
        let timeout_duration = tokio::time::Duration::from_secs(120);
        match tokio::time::timeout(timeout_duration, completion_rx).await {
            Ok(Ok(())) => {
                log::debug!("Received stream completion signal");
            }
            Ok(Err(_)) => {
                log::warn!("Stream completion channel closed unexpectedly");
            }
            Err(_) => {
                log::warn!("Stream completion timeout after 120s");
            }
        }

        // 额外等待 100ms 确保最后的 token 统计已经处理完
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut log = request_log_clone;
        if let Ok(stats) = token_stats.lock() {
            log.input_tokens = stats.input_tokens;
            log.output_tokens = stats.output_tokens;
            log.cache_creation_input_tokens = stats.cache_creation_input_tokens;
            log.cache_read_input_tokens = stats.cache_read_input_tokens;
            log.duration_ms = start_time.elapsed().as_millis() as i64;

            // 如果上游 API 没有返回 token 统计（或返回 0），使用本地计数作为兜底
            if !stats.has_usage || (stats.input_tokens == 0 && stats.output_tokens == 0) {
                log::warn!("⚠️  No valid usage info from upstream API, using local token counting");

                // 尝试使用本地 token 计数
                if let Ok(counter) = TokenCounter::new() {
                    // 计算 input tokens（从原始请求体）
                    if !request_body_clone.is_empty() {
                        let local_input_tokens = counter.count_input_tokens(&request_body_clone);
                        log.input_tokens = local_input_tokens;
                        log::info!("🔢 Local count - input tokens: {}", local_input_tokens);
                    }

                    // 计算 output tokens（从收集的输出文本）
                    if !stats.output_text.is_empty() {
                        let local_output_tokens = counter.count_output_tokens(&stats.output_text);
                        log.output_tokens = local_output_tokens;
                        log::info!("🔢 Local count - output tokens: {}", local_output_tokens);
                    }
                } else {
                    log::error!("Failed to initialize token counter");
                }
            }

            // 输出流式响应的统计信息
            let total_tokens = log.input_tokens + log.output_tokens;

            if stats.has_usage && (stats.input_tokens > 0 || stats.output_tokens > 0) {
                log::info!("✅ Stream completed");
                log::info!("📊 Stats: {} tokens (in: {}, out: {}) | {}ms",
                    total_tokens, log.input_tokens, log.output_tokens, log.duration_ms);

                if stats.cache_creation_input_tokens > 0 || stats.cache_read_input_tokens > 0 {
                    log::info!("💾 Cache: creation: {}, read: {}",
                        stats.cache_creation_input_tokens, stats.cache_read_input_tokens);
                }
            } else {
                log::info!("✅ Stream completed (local counting)");
                log::info!("📊 Stats: {} tokens (in: {}, out: {}) | {}ms",
                    total_tokens, log.input_tokens, log.output_tokens, log.duration_ms);
            }
            log::info!("{}\n", "=".repeat(60));
        }

        // 使用 UPDATE 更新已存在的日志记录
        crate::logger::update_log(log).await;
    });

    // 立即返回流式响应
    let body = Body::from_stream(stream);
    Ok((response_headers, body).into_response())
}
