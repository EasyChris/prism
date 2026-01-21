use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// 模型处理模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelMode {
    /// 透传模式：不修改模型名称
    Passthrough,
    /// 覆盖模式：使用 Profile 的默认模型
    Override,
    /// 映射模式：通过映射规则转换
    Mapping,
}

impl ModelMode {
    pub fn as_str(&self) -> &str {
        match self {
            ModelMode::Passthrough => "passthrough",
            ModelMode::Override => "override",
            ModelMode::Mapping => "mapping",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "passthrough" => ModelMode::Passthrough,
            "override" => ModelMode::Override,
            "mapping" => ModelMode::Mapping,
            _ => ModelMode::Passthrough,
        }
    }

    /// 从 config 模块的 ModelMappingMode 转换
    pub fn from_mapping_mode(mode: &crate::config::ModelMappingMode) -> Self {
        match mode {
            crate::config::ModelMappingMode::Passthrough => ModelMode::Passthrough,
            crate::config::ModelMappingMode::Override => ModelMode::Override,
            crate::config::ModelMappingMode::Map => ModelMode::Mapping,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    // 请求标识
    pub request_id: String,
    pub timestamp: i64,

    // Profile 信息
    pub profile_id: String,
    pub profile_name: String,
    pub provider: String,

    // 模型信息（新增字段）
    pub original_model: String,      // 原始请求的模型名称
    pub model_mode: String,           // 模型处理模式
    pub forwarded_model: String,      // 实际转发的模型名称

    // Token 统计
    pub input_tokens: i32,
    pub output_tokens: i32,

    // 缓存相关统计
    pub cache_creation_input_tokens: i32,  // 创建缓存的 token 数
    pub cache_read_input_tokens: i32,      // 从缓存读取的 token 数（命中缓存）

    // 性能指标
    pub duration_ms: i64,
    pub upstream_duration_ms: Option<i64>,  // 上游响应时间

    // 请求/响应信息
    pub status_code: i32,
    pub error_message: Option<String>,
    pub is_stream: bool,
    pub request_size_bytes: Option<i64>,    // 请求体大小
    pub response_size_bytes: Option<i64>,   // 响应体大小
    pub response_body: Option<String>,      // 响应体内容（仅在 output_tokens=0 时记录，用于调试）
}

impl RequestLog {
    pub fn new(
        profile_id: String,
        profile_name: String,
        original_model: String,
        model_mode: ModelMode,
        forwarded_model: String,
        api_base_url: String,
        request_size: usize,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX EPOCH")
            .as_millis() as i64;

        // 生成唯一的请求 ID
        let request_id = Uuid::new_v4().to_string();

        // 从 API Base URL 提取 provider
        let provider = extract_provider(&api_base_url);

        Self {
            request_id,
            timestamp,
            profile_id,
            profile_name,
            provider,
            original_model,
            model_mode: model_mode.as_str().to_string(),
            forwarded_model,
            input_tokens: 0,
            output_tokens: 0,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
            duration_ms: 0,
            upstream_duration_ms: None,
            status_code: 0,
            error_message: None,
            is_stream: false,
            request_size_bytes: Some(request_size as i64),
            response_size_bytes: None,
            response_body: None,
        }
    }
}

fn extract_provider(api_base_url: &str) -> String {
    if api_base_url.contains("anthropic.com") {
        "Anthropic".to_string()
    } else if api_base_url.contains("openai.com") {
        "OpenAI".to_string()
    } else {
        "Custom".to_string()
    }
}

// 保存日志到数据库
pub async fn save_log(log: RequestLog) {
    if let Err(e) = crate::db::save_log_to_db(&log).await {
        log::error!("Failed to save log to database: {}", e);
    }
}

// 更新日志到数据库（用于流式响应的 Token 统计更新）
pub async fn update_log(log: RequestLog) {
    if let Err(e) = crate::db::update_log_to_db(&log).await {
        log::error!("Failed to update log to database: {}", e);
    }
}

// 查询日志（从数据库）
pub async fn get_logs(limit: usize, offset: usize) -> Vec<RequestLog> {
    match crate::db::get_logs_from_db(limit, offset).await {
        Ok(logs) => logs,
        Err(e) => {
            log::error!("Failed to get logs from database: {}", e);
            Vec::new()
        }
    }
}
