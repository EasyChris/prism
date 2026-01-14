// 日志相关的数据库操作

use crate::logger::RequestLog;
use super::schema::get_db_path;

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
                request_id, timestamp, profile_id, profile_name, provider,
                original_model, model_mode, forwarded_model,
                input_tokens, output_tokens, cache_creation_input_tokens, cache_read_input_tokens,
                duration_ms, upstream_duration_ms,
                status_code, error_message, is_stream,
                request_size_bytes, response_size_bytes
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
            "#,
            rusqlite::params![
                &log.request_id,
                log.timestamp,
                &log.profile_id,
                &log.profile_name,
                &log.provider,
                &log.original_model,
                &log.model_mode,
                &log.forwarded_model,
                log.input_tokens,
                log.output_tokens,
                log.cache_creation_input_tokens,
                log.cache_read_input_tokens,
                log.duration_ms,
                log.upstream_duration_ms,
                log.status_code,
                &log.error_message,
                if log.is_stream { 1 } else { 0 },
                log.request_size_bytes,
                log.response_size_bytes,
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
                SELECT request_id, timestamp, profile_id, profile_name, provider,
                       original_model, model_mode, forwarded_model,
                       input_tokens, output_tokens, cache_creation_input_tokens, cache_read_input_tokens,
                       duration_ms, upstream_duration_ms,
                       status_code, error_message, is_stream,
                       request_size_bytes, response_size_bytes
                FROM request_logs
                ORDER BY timestamp DESC
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let logs = stmt
            .query_map([limit as i64, offset as i64], |row| {
                Ok(RequestLog {
                    request_id: row.get(0)?,
                    timestamp: row.get(1)?,
                    profile_id: row.get(2)?,
                    profile_name: row.get(3)?,
                    provider: row.get(4)?,
                    original_model: row.get(5)?,
                    model_mode: row.get(6)?,
                    forwarded_model: row.get(7)?,
                    input_tokens: row.get(8)?,
                    output_tokens: row.get(9)?,
                    cache_creation_input_tokens: row.get(10)?,
                    cache_read_input_tokens: row.get(11)?,
                    duration_ms: row.get(12)?,
                    upstream_duration_ms: row.get(13)?,
                    status_code: row.get(14)?,
                    error_message: row.get(15)?,
                    is_stream: row.get::<_, i32>(16)? != 0,
                    request_size_bytes: row.get(17)?,
                    response_size_bytes: row.get(18)?,
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
