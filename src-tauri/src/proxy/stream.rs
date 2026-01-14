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
use crate::logger::RequestLog;

/// 包装流，用于在转发的同时收集 Token 统计信息
struct TokenCollectorStream {
    inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>,
    token_stats: Arc<Mutex<TokenStats>>,
}

#[derive(Default)]
struct TokenStats {
    input_tokens: i32,
    output_tokens: i32,
    cache_creation_input_tokens: i32,
    cache_read_input_tokens: i32,
}

impl Stream for TokenCollectorStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                // 尝试解析 Token 信息（不阻塞转发）
                if let Ok(text) = std::str::from_utf8(&chunk) {
                    if let Ok(mut stats) = self.token_stats.lock() {
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let json_str = &line[6..];
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                                    if let Some(usage) = json.get("usage") {
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
                                    }
                                }
                            }
                        }
                    }
                }
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => {
                log::error!("Stream error: {}", e);
                Poll::Ready(Some(Err(std::io::Error::new(std::io::ErrorKind::Other, e))))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// 处理流式响应（真正的流式转发）
pub(super) async fn handle_stream_response(
    response: reqwest::Response,
    request_log: RequestLog,
    start_time: Instant,
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

    // 创建包装流
    let stream = TokenCollectorStream {
        inner: Box::pin(response.bytes_stream()),
        token_stats: token_stats_clone,
    };

    // 在流结束后保存日志
    let request_log_clone = request_log.clone();
    tokio::spawn(async move {
        // 等待一小段时间确保流已完成
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut log = request_log_clone;
        if let Ok(stats) = token_stats.lock() {
            log.input_tokens = stats.input_tokens;
            log.output_tokens = stats.output_tokens;
            log.cache_creation_input_tokens = stats.cache_creation_input_tokens;
            log.cache_read_input_tokens = stats.cache_read_input_tokens;
            log.duration_ms = start_time.elapsed().as_millis() as i64;

            log::info!(
                "Stream completed - input_tokens: {}, output_tokens: {}, cache_creation: {}, cache_read: {}",
                stats.input_tokens, stats.output_tokens, stats.cache_creation_input_tokens, stats.cache_read_input_tokens
            );
        }

        crate::logger::save_log(log).await;
    });

    // 立即返回流式响应
    let body = Body::from_stream(stream);
    Ok((response_headers, body).into_response())
}
