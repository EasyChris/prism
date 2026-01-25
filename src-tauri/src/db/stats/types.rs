// 统计相关的数据结构定义

/// 统计数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub today_requests: i32,
    pub today_tokens: i32,
    pub total_requests: i32,
    pub total_tokens: i32,
}

/// Token 使用量数据点
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenDataPoint {
    pub label: String,
    pub tokens: i32,
    pub cache_read_tokens: i32,  // 缓存命中的 token 数
}

/// 配置消耗排名数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileConsumption {
    pub profile_id: String,
    pub profile_name: String,
    pub total_tokens: i32,
    pub percentage: f32,
    pub rank: i32,
}
