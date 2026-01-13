use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub timestamp: i64,
    pub profile_id: String,
    pub profile_name: String,
    pub model: String,
    pub provider: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i64,
    pub status_code: i32,
    pub error_message: Option<String>,
    pub is_stream: bool,
}

impl RequestLog {
    pub fn new(
        profile_id: String,
        profile_name: String,
        model: String,
        api_base_url: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        // 从 API Base URL 提取 provider
        let provider = extract_provider(&api_base_url);

        Self {
            timestamp,
            profile_id,
            profile_name,
            model,
            provider,
            input_tokens: 0,
            output_tokens: 0,
            duration_ms: 0,
            status_code: 0,
            error_message: None,
            is_stream: false,
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
