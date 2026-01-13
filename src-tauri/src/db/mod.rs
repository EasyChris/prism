// 数据库模块：SQLite 日志存储

use crate::logger::RequestLog;
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

    // 创建日志表
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS request_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            profile_id TEXT NOT NULL,
            profile_name TEXT NOT NULL,
            model TEXT NOT NULL,
            provider TEXT NOT NULL,
            input_tokens INTEGER NOT NULL,
            output_tokens INTEGER NOT NULL,
            duration_ms INTEGER NOT NULL,
            status_code INTEGER NOT NULL,
            error_message TEXT,
            is_stream INTEGER NOT NULL
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

/// 保存日志到数据库
pub async fn save_log_to_db(log: &RequestLog) -> Result<(), String> {
    let db_path = get_db_path();
    let log = log.clone(); // 克隆 log 以避免生命周期问题

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        conn.execute(
            r#"
            INSERT INTO request_logs (
                timestamp, profile_id, profile_name, model, provider,
                input_tokens, output_tokens, duration_ms, status_code,
                error_message, is_stream
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
            rusqlite::params![
                log.timestamp,
                &log.profile_id,
                &log.profile_name,
                &log.model,
                &log.provider,
                log.input_tokens,
                log.output_tokens,
                log.duration_ms,
                log.status_code,
                &log.error_message,
                if log.is_stream { 1 } else { 0 },
            ],
        )
        .map_err(|e| format!("Failed to insert log: {}", e))?;

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(())
}

/// 从数据库查询日志
pub async fn get_logs_from_db(limit: usize, offset: usize) -> Result<Vec<RequestLog>, String> {
    let db_path = get_db_path();

    let logs = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT timestamp, profile_id, profile_name, model, provider,
                       input_tokens, output_tokens, duration_ms, status_code,
                       error_message, is_stream
                FROM request_logs
                ORDER BY timestamp DESC
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let logs = stmt
            .query_map([limit as i64, offset as i64], |row| {
                Ok(RequestLog {
                    timestamp: row.get(0)?,
                    profile_id: row.get(1)?,
                    profile_name: row.get(2)?,
                    model: row.get(3)?,
                    provider: row.get(4)?,
                    input_tokens: row.get(5)?,
                    output_tokens: row.get(6)?,
                    duration_ms: row.get(7)?,
                    status_code: row.get(8)?,
                    error_message: row.get(9)?,
                    is_stream: row.get::<_, i32>(10)? != 0,
                })
            })
            .map_err(|e| format!("Failed to query logs: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect logs: {}", e))?;

        Ok::<Vec<RequestLog>, String>(logs)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(logs)
}
