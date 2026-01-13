use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use std::time::Instant;
use crate::config::SharedConfigManager;
use crate::logger::RequestLog;
use super::stream::handle_stream_response;
use super::utils::convert_headers;

/// 处理 /v1/messages 请求
pub(super) async fn handle_messages(
    State(config): State<SharedConfigManager>,
    headers: HeaderMap,
    body: String,
) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    log::info!("Received request to /v1/messages");

    // API Key 鉴权检查
    {
        let config_guard = config.read().map_err(|e| {
            log::error!("Failed to acquire config read lock: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        // 如果启用了访问授权，则验证 API Key
        if config_guard.is_auth_enabled() {
            let auth_header = headers.get(axum::http::header::AUTHORIZATION);

            let api_key = match auth_header {
                Some(value) => {
                    let auth_str = value.to_str().map_err(|_| {
                        log::warn!("Invalid Authorization header format");
                        StatusCode::UNAUTHORIZED
                    })?;

                    // 支持 "Bearer sk-xxx" 格式
                    if let Some(key) = auth_str.strip_prefix("Bearer ") {
                        key
                    } else {
                        log::warn!("Authorization header missing 'Bearer ' prefix");
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }
                None => {
                    log::warn!("Missing Authorization header");
                    return Err(StatusCode::UNAUTHORIZED);
                }
            };

            // 验证 API Key
            if !config_guard.verify_api_key(api_key) {
                log::warn!("Invalid API key: {}", api_key);
                return Err(StatusCode::UNAUTHORIZED);
            }

            log::info!("API key verified successfully");
        }
    }

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

    // 计算请求体大小（在移动之前）
    let request_size = modified_body.len();

    // 检查是否是流式请求
    let is_stream = modified_body.contains("\"stream\":true") || modified_body.contains("\"stream\": true");
    log::info!("Request is streaming: {}", is_stream);

    // 构建上游 API URL
    let upstream_url = format!("{}/v1/messages", profile.api_base_url);
    log::info!("Forwarding to: {}", upstream_url);

    // 创建 HTTP 客户端（设置 60 秒超时）
    // reqwest 默认启用所有解压功能（gzip, deflate, br, zstd）
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .connect_timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| {
            log::error!("Failed to create HTTP client: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

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
    request_headers.remove("x-api-key");  // 移除测试占位符
    request_headers.remove("content-length");  // reqwest 会自动计算

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
        let mut request_log = RequestLog::new(
            profile.id.clone(),
            profile.name.clone(),
            original_model.clone(),
            crate::logger::ModelMode::from_mapping_mode(&profile.model_mapping_mode),
            mapped_model.clone(),
            profile.api_base_url.clone(),
            request_size,
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

    // 先读取为字节，以便处理可能的编码问题
    let response_bytes = response.bytes().await.map_err(|e| {
        log::error!("Failed to read response bytes: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    log::info!("Response body length: {} bytes", response_bytes.len());
    log::info!("Received response with status: {}", status);

    // 尝试将字节转换为 UTF-8 字符串
    let response_body = match String::from_utf8(response_bytes.to_vec()) {
        Ok(text) => {
            log::info!("Successfully decoded response as UTF-8");
            text
        }
        Err(e) => {
            log::warn!("Response is not valid UTF-8, attempting lossy conversion: {}", e);
            // 使用 lossy 转换，替换无效字符
            String::from_utf8_lossy(&response_bytes).to_string()
        }
    };

    // 打印响应体前 500 字符用于调试（安全地处理 UTF-8 边界）
    let preview = if response_body.len() > 500 {
        // 使用 char_indices 找到安全的切割点
        response_body.char_indices()
            .take(500)
            .last()
            .map(|(idx, _)| &response_body[..=idx])
            .unwrap_or(&response_body)
    } else {
        &response_body
    };
    log::info!("Response body preview: {}", preview);

    // 解析 Token 信息（兼容不同供应商的响应格式）
    let (input_tokens, output_tokens) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response_body) {
        log::info!("Successfully parsed JSON response");

        // 尝试多种可能的 token 字段路径（兼容不同供应商）
        let input = json.get("usage")
            .and_then(|u| u.get("input_tokens"))
            .and_then(|t| t.as_i64())
            .or_else(|| {
                // 尝试其他可能的字段名
                json.get("usage")
                    .and_then(|u| u.get("prompt_tokens"))
                    .and_then(|t| t.as_i64())
            })
            .unwrap_or(0) as i32;

        let output = json.get("usage")
            .and_then(|u| u.get("output_tokens"))
            .and_then(|t| t.as_i64())
            .or_else(|| {
                // 尝试其他可能的字段名
                json.get("usage")
                    .and_then(|u| u.get("completion_tokens"))
                    .and_then(|t| t.as_i64())
            })
            .unwrap_or(0) as i32;

        log::info!("Parsed tokens - input: {}, output: {}", input, output);
        (input, output)
    } else {
        log::warn!("Failed to parse response body as JSON, skipping token extraction");
        log::warn!("This may be due to non-JSON response or unsupported format from provider");
        (0, 0)
    };

    // 计算耗时
    let duration_ms = start_time.elapsed().as_millis() as i64;

    // 记录日志（使用新的字段）
    let response_size = response_body.len();
    let mut request_log = RequestLog::new(
        profile.id.clone(),
        profile.name.clone(),
        original_model.clone(),
        crate::logger::ModelMode::from_mapping_mode(&profile.model_mapping_mode),
        mapped_model.clone(),
        profile.api_base_url.clone(),
        request_size,
    );
    request_log.input_tokens = input_tokens;
    request_log.output_tokens = output_tokens;
    request_log.duration_ms = duration_ms;
    request_log.status_code = status.as_u16() as i32;
    request_log.is_stream = false;
    request_log.response_size_bytes = Some(response_size as i64);

    // 异步保存日志
    tokio::spawn(async move {
        crate::logger::save_log(request_log).await;
    });

    log::info!("Sending response back to client...");

    let response = (status, response_headers, response_body).into_response();
    log::info!("Response created successfully");
    Ok(response)
}
