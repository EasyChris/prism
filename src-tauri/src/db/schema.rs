// 数据库表结构和初始化

use std::path::PathBuf;

/// 获取数据库文件路径
pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("com.prism.app");
    std::fs::create_dir_all(&path).ok();
    path.push("logs.db");
    path
}

/// 初始化数据库
pub async fn init_database() -> Result<(), String> {
    let db_path = get_db_path();
    log::info!("Initializing database at: {:?}", db_path);

    // 使用 rusqlite 直接操作数据库
    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // 创建日志表（如果不存在）
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS request_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id TEXT,
            timestamp INTEGER NOT NULL,

            -- Profile 信息
            profile_id TEXT NOT NULL,
            profile_name TEXT NOT NULL,
            provider TEXT NOT NULL,

            -- 模型信息
            original_model TEXT NOT NULL,
            model_mode TEXT NOT NULL,
            forwarded_model TEXT NOT NULL,

            -- Token 统计
            input_tokens INTEGER NOT NULL,
            output_tokens INTEGER NOT NULL,

            -- 缓存相关统计
            cache_creation_input_tokens INTEGER NOT NULL DEFAULT 0,
            cache_read_input_tokens INTEGER NOT NULL DEFAULT 0,

            -- 性能指标
            duration_ms INTEGER NOT NULL,
            upstream_duration_ms INTEGER,

            -- 请求/响应信息
            status_code INTEGER NOT NULL,
            error_message TEXT,
            is_stream INTEGER NOT NULL,
            request_size_bytes INTEGER,
            response_size_bytes INTEGER
        )
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON request_logs(timestamp DESC)",
        [],
    )
    .map_err(|e| format!("Failed to create index: {}", e))?;

    log::info!("Database initialized successfully");
    Ok(())
}
