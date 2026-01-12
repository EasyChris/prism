use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use tokio::sync::Mutex;

// 全局日志存储（临时方案，后续会用数据库）
lazy_static::lazy_static! {
    static ref LOG_STORAGE: Arc<Mutex<Vec<RequestLog>>> = Arc::new(Mutex::new(Vec::new()));
}

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

// 保存日志到内存
pub async fn save_log(log: RequestLog) {
    let mut storage = LOG_STORAGE.lock().await;
    storage.push(log);

    // 限制内存中最多保存 1000 条日志
    if storage.len() > 1000 {
        storage.remove(0);
    }
}

// 查询日志
pub async fn get_logs(limit: usize, offset: usize) -> Vec<RequestLog> {
    let storage = LOG_STORAGE.lock().await;
    storage.iter()
        .rev()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect()
}
