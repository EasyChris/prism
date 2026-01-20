use tiktoken_rs::cl100k_base;

/// 本地 token 计数器（用于上游 API 不返回 token 统计时的兜底方案）
pub struct TokenCounter {
    bpe: tiktoken_rs::CoreBPE,
}

impl TokenCounter {
    /// 创建新的 token 计数器
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let bpe = cl100k_base()?;
        Ok(Self { bpe })
    }

    /// 计算文本的 token 数量
    pub fn count_tokens(&self, text: &str) -> usize {
        self.bpe.encode_with_special_tokens(text).len()
    }

    /// 从请求体中提取并计算 input tokens
    pub fn count_input_tokens(&self, request_body: &str) -> i32 {
        // 解析请求体 JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(request_body) {
            let mut total_tokens = 0;

            // 计算 system prompt tokens
            if let Some(system) = json.get("system").and_then(|s| s.as_str()) {
                total_tokens += self.count_tokens(system);
            }

            // 计算 messages tokens
            if let Some(messages) = json.get("messages").and_then(|m| m.as_array()) {
                for message in messages {
                    // 计算 role tokens (通常很少)
                    if let Some(role) = message.get("role").and_then(|r| r.as_str()) {
                        total_tokens += self.count_tokens(role);
                    }

                    // 计算 content tokens
                    if let Some(content) = message.get("content") {
                        if let Some(text) = content.as_str() {
                            // 简单文本内容
                            total_tokens += self.count_tokens(text);
                        } else if let Some(arr) = content.as_array() {
                            // 复杂内容（包含 text, image 等）
                            for item in arr {
                                if let Some(text) = item.get("text").and_then(|t| t.as_str()) {
                                    total_tokens += self.count_tokens(text);
                                }
                                // 图片等其他类型暂时忽略，因为计算比较复杂
                            }
                        }
                    }
                }
            }

            total_tokens as i32
        } else {
            log::warn!("Failed to parse request body for token counting");
            0
        }
    }

    /// 计算输出文本的 token 数量
    pub fn count_output_tokens(&self, output_text: &str) -> i32 {
        self.count_tokens(output_text) as i32
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new().expect("Failed to initialize token counter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tokens() {
        let counter = TokenCounter::new().unwrap();
        let text = "Hello, world!";
        let count = counter.count_tokens(text);
        assert!(count > 0);
    }

    #[test]
    fn test_count_input_tokens() {
        let counter = TokenCounter::new().unwrap();
        let request_body = r#"{
            "model": "claude-3-5-sonnet-20241022",
            "messages": [
                {"role": "user", "content": "Hello, how are you?"}
            ]
        }"#;
        let count = counter.count_input_tokens(request_body);
        assert!(count > 0);
    }
}
