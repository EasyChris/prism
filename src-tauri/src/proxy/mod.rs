use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use std::net::SocketAddr;
use std::time::Instant;
use crate::config::SharedConfigManager;
use crate::logger::RequestLog;

pub async fn start_proxy_server(config: SharedConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/v1/messages", post(handle_messages))
        .with_state(config);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    log::info!("Proxy server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_messages(
    State(config): State<SharedConfigManager>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    log::info!("Received request to /v1/messages");

    // 从配置中获取激活的 Profile
    let profile = {
        let config_guard = config.read().map_err(|e| {
            log::error!("Failed to acquire config read lock: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        config_guard.get_active_profile().cloned().ok_or_else(|| {
            log::error!("No active profile found");
            StatusCode::SERVICE_UNAVAILABLE
        })?
    };

    log::info!("Using profile: {} ({})", profile.name, profile.api_base_url);

    // 解析请求体以获取模型信息并应用模型映射
    let (original_model, mapped_model, modified_body) = if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&body) {
        let original = json.get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown")
            .to_string();

        // 使用 Profile 的 resolve_model 方法进行模型映射
        let mapped = profile.resolve_model(&original);

        // 如果模型发生了映射，修改请求体中的 model 字段
        if original != mapped {
            log::info!("Model mapping: {} -> {}", original, mapped);
            json["model"] = serde_json::Value::String(mapped.clone());
        }

        let new_body = serde_json::to_string(&json).unwrap_or(body.clone());
        (original, mapped, new_body)
    } else {
        let default_model = "unknown".to_string();
        (default_model.clone(), default_model, body.clone())
    };

    log::info!("Original model: {}", original_model);
    log::info!("Mapped model: {}", mapped_model);

    // 检查是否是流式请求
    let is_stream = modified_body.contains("\"stream\":true") || modified_body.contains("\"stream\": true");
    log::info!("Request is streaming: {}", is_stream);

    // 构建上游 API URL
    let upstream_url = format!("{}/v1/messages", profile.api_base_url);
    log::info!("Forwarding to: {}", upstream_url);

    // 创建 HTTP 客户端
    let client = reqwest::Client::new();

    // 准备请求头，添加 API Key
    let mut request_headers = convert_headers(&headers);

    // 设置 Authorization 头（Bearer token）
    if !profile.api_key.is_empty() {
        let auth_value = format!("Bearer {}", profile.api_key);
        if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&auth_value) {
            request_headers.insert(reqwest::header::AUTHORIZATION, header_value);
        }
    }

    // 确保必要的头存在
    if !request_headers.contains_key(reqwest::header::CONTENT_TYPE) {
        request_headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
    }

    // 设置更真实的 User-Agent 以避免被 Cloudflare 拦截
    if !request_headers.contains_key(reqwest::header::USER_AGENT) {
        request_headers.insert(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            ),
        );
    }

    // 移除可能导致问题的头
    request_headers.remove(reqwest::header::HOST);
    request_headers.remove("connection");

    // 打印请求头用于调试
    log::info!("Request headers:");
    for (key, value) in request_headers.iter() {
        if key == reqwest::header::AUTHORIZATION {
            log::info!("  {}: Bearer ***", key);
        } else if let Ok(v) = value.to_str() {
            log::info!("  {}: {}", key, v);
        }
    }

    // 转发请求到上游 API（使用修改后的请求体）
    log::info!("Sending request to upstream...");
    let response = client
        .post(&upstream_url)
        .headers(request_headers)
        .body(modified_body)
        .send()
        .await
        .map_err(|e| {
            log::error!("Failed to forward request: {}", e);
            log::error!("Error details: {:?}", e);
            if e.is_timeout() {
                log::error!("Request timed out");
            }
            if e.is_connect() {
                log::error!("Connection error");
            }
            StatusCode::BAD_GATEWAY
        })?;

    log::info!("Received response from upstream");

    let status = response.status();

    // 获取响应头（移除压缩和传输编码相关的头）
    let mut response_headers = HeaderMap::new();
    for (key, value) in response.headers().iter() {
        // 跳过这些头，因为 reqwest 已经自动处理了，我们会发送普通的响应体
        if key == "content-encoding"
            || key == "content-length"
            || key == "transfer-encoding" {
            continue;
        }
        response_headers.insert(key.clone(), value.clone());
    }

    // 如果是流式响应，使用流式处理
    if is_stream {
        log::info!("Handling streaming response");

        // 创建日志记录（流式响应的 Token 统计会在流结束后更新）
        // 记录映射后的模型名称
        let mut request_log = RequestLog::new(
            profile.id.clone(),
            profile.name.clone(),
            mapped_model.clone(),
            profile.api_base_url.clone(),
        );
        request_log.duration_ms = start_time.elapsed().as_millis() as i64;
        request_log.status_code = status.as_u16() as i32;
        request_log.is_stream = true;

        // 先保存基础日志（Token 为 0）
        let log_clone = request_log.clone();
        tokio::spawn(async move {
            crate::logger::save_log(log_clone).await;
        });

        return handle_stream_response(response, request_log, start_time).await;
    }

    // 非流式响应，直接返回
    log::info!("Reading response body...");
    let response_body = response.text().await.map_err(|e| {
        log::error!("Failed to read response body: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    log::info!("Response body length: {} bytes", response_body.len());
    log::info!("Received response with status: {}", status);

    // 打印响应体前 500 字符用于调试
    let preview = if response_body.len() > 500 {
        &response_body[..500]
    } else {
        &response_body
    };
    log::info!("Response body preview: {}", preview);

    // 解析 Token 信息
    let (input_tokens, output_tokens) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response_body) {
        log::info!("Successfully parsed JSON response");
        let input = json.get("usage")
            .and_then(|u| u.get("input_tokens"))
            .and_then(|t| t.as_i64())
            .unwrap_or(0) as i32;
        let output = json.get("usage")
            .and_then(|u| u.get("output_tokens"))
            .and_then(|t| t.as_i64())
            .unwrap_or(0) as i32;
        log::info!("Parsed tokens - input: {}, output: {}", input, output);
        (input, output)
    } else {
        log::warn!("Failed to parse response body as JSON");
        (0, 0)
    };

    // 计算耗时
    let duration_ms = start_time.elapsed().as_millis() as i64;

    // 记录日志（使用映射后的模型名称）
    let mut request_log = RequestLog::new(
        profile.id.clone(),
        profile.name.clone(),
        mapped_model.clone(),
        profile.api_base_url.clone(),
    );
    request_log.input_tokens = input_tokens;
    request_log.output_tokens = output_tokens;
    request_log.duration_ms = duration_ms;
    request_log.status_code = status.as_u16() as i32;
    request_log.is_stream = false;

    // 异步保存日志
    tokio::spawn(async move {
        crate::logger::save_log(request_log).await;
    });

    log::info!("Sending response back to client...");

    let response = (status, response_headers, response_body).into_response();
    log::info!("Response created successfully");
    Ok(response)
}

async fn handle_stream_response(
    response: reqwest::Response,
    mut request_log: RequestLog,
    start_time: Instant,
) -> Result<Response, StatusCode> {
    use axum::body::Body;
    use futures::stream::StreamExt;

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

                // 尝试解析 SSE 事件提取 Token 信息
                if let Ok(text) = std::str::from_utf8(&chunk) {
                    for line in text.lines() {
                        if line.starts_with("data: ") {
                            let json_str = &line[6..];
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                                // 提取 usage 信息
                                if let Some(usage) = json.get("usage") {
                                    if let Some(input) = usage.get("input_tokens").and_then(|t| t.as_i64()) {
                                        input_tokens = input as i32;
                                    }
                                    if let Some(output) = usage.get("output_tokens").and_then(|t| t.as_i64()) {
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

fn convert_headers(headers: &HeaderMap) -> reqwest::header::HeaderMap {
    let mut new_headers = reqwest::header::HeaderMap::new();

    for (key, value) in headers.iter() {
        if let Ok(value) = value.to_str() {
            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(value) {
                new_headers.insert(key.clone(), header_value);
            }
        }
    }

    new_headers
}
