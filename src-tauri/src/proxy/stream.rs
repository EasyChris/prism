use axum::{
    body::Body,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use futures::stream::StreamExt;
use std::time::Instant;
use crate::logger::RequestLog;

/// 处理流式响应
pub(super) async fn handle_stream_response(
    response: reqwest::Response,
    mut request_log: RequestLog,
    start_time: Instant,
) -> Result<Response, StatusCode> {
    // 获取响应头
    let mut response_headers = HeaderMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
            response_headers.insert(key.clone(), value);
        }
    }

    // 收集所有流数据并解析 Token 信息
    let mut stream = response.bytes_stream();
    let mut buffer = Vec::new();
    let mut input_tokens = 0;
    let mut output_tokens = 0;

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                buffer.extend_from_slice(&chunk);

                // 尝试解析 SSE 事件提取 Token 信息（兼容不同供应商格式）
                if let Ok(text) = std::str::from_utf8(&chunk) {
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let json_str = &line[6..];
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                                // 提取 usage 信息（兼容多种字段名）
                                if let Some(usage) = json.get("usage") {
                                    // 尝试 input_tokens 或 prompt_tokens
                                    if let Some(input) = usage.get("input_tokens")
                                        .and_then(|t| t.as_i64())
                                        .or_else(|| usage.get("prompt_tokens").and_then(|t| t.as_i64())) {
                                        input_tokens = input as i32;
                                    }
                                    // 尝试 output_tokens 或 completion_tokens
                                    if let Some(output) = usage.get("output_tokens")
                                        .and_then(|t| t.as_i64())
                                        .or_else(|| usage.get("completion_tokens").and_then(|t| t.as_i64())) {
                                        output_tokens = output as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Stream error: {}", e);
                break;
            }
        }
    }

    // 更新日志的 Token 信息
    request_log.input_tokens = input_tokens;
    request_log.output_tokens = output_tokens;
    request_log.duration_ms = start_time.elapsed().as_millis() as i64;

    log::info!("Stream completed - input_tokens: {}, output_tokens: {}", input_tokens, output_tokens);

    // 异步保存更新后的日志
    tokio::spawn(async move {
        crate::logger::save_log(request_log).await;
    });

    // 将缓存的数据作为响应体返回
    let body = Body::from(buffer);

    Ok((response_headers, body).into_response())
}
