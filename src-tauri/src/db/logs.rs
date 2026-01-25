// 日志相关的数据库操作

use crate::logger::RequestLog;
use super::schema::get_db_path;

/// 保存日志到数据库
/// 返回 true 表示新插入的记录，false 表示更新了现有记录
pub async fn save_log_to_db(log: &RequestLog) -> Result<bool, String> {
    let db_path = get_db_path();
    let log = log.clone(); // 克隆 log 以避免生命周期问题

    let is_new = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 先检查记录是否已存在
        let exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM request_logs WHERE request_id = ?1",
                [&log.request_id],
                |row| row.get::<_, i32>(0).map(|count| count > 0),
            )
            .unwrap_or(false);

        conn.execute(
            r#"
            INSERT OR IGNORE INTO request_logs (
                request_id, timestamp, profile_id, profile_name, provider,
                original_model, model_mode, forwarded_model,
                input_tokens, output_tokens, cache_creation_input_tokens, cache_read_input_tokens,
                duration_ms, upstream_duration_ms,
                status_code, error_message, is_stream,
                request_size_bytes, response_size_bytes, response_body
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
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
                &log.response_body,
            ],
        )
        .map_err(|e| format!("Failed to insert log: {}", e))?;

        // 返回是否是新记录（!exists 表示之前不存在，现在是新插入的）
        Ok::<bool, String>(!exists)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(is_new)
}

/// 更新日志到数据库（用于流式响应的 Token 统计更新）
pub async fn update_log_to_db(log: &RequestLog) -> Result<(), String> {
    let db_path = get_db_path();
    let log = log.clone(); // 克隆 log 以避免生命周期问题

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        conn.execute(
            r#"
            UPDATE request_logs SET
                input_tokens = ?1,
                output_tokens = ?2,
                cache_creation_input_tokens = ?3,
                cache_read_input_tokens = ?4,
                duration_ms = ?5,
                response_body = ?6
            WHERE request_id = ?7
            "#,
            rusqlite::params![
                log.input_tokens,
                log.output_tokens,
                log.cache_creation_input_tokens,
                log.cache_read_input_tokens,
                log.duration_ms,
                &log.response_body,
                &log.request_id,
            ],
        )
        .map_err(|e| format!("Failed to update log: {}", e))?;

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(())
}

/// 从数据库查询日志
pub async fn get_logs_from_db(limit: usize, offset: usize) -> Result<Vec<RequestLog>, String> {
    let db_path = get_db_path();

    log::debug!("Querying logs from database: limit={}, offset={}", limit, offset);

    let logs = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    rl.request_id, rl.timestamp, rl.profile_id,
                    COALESCE(p.name, '已删除的配置 (' || rl.profile_id || ')') as profile_name,
                    rl.provider,
                    rl.original_model, rl.model_mode, rl.forwarded_model,
                    rl.input_tokens, rl.output_tokens, rl.cache_creation_input_tokens, rl.cache_read_input_tokens,
                    rl.duration_ms, rl.upstream_duration_ms,
                    rl.status_code, rl.error_message, rl.is_stream,
                    rl.request_size_bytes, rl.response_size_bytes, rl.response_body
                FROM request_logs rl
                LEFT JOIN profiles p ON rl.profile_id = p.id
                ORDER BY rl.timestamp DESC
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let logs = stmt
            .query_map([limit as i64, offset as i64], |row| {
                // 使用 unwrap_or 提供默认值，防止 NULL 值导致的错误
                let input_tokens: i32 = row.get(8).unwrap_or(0);
                let output_tokens: i32 = row.get(9).unwrap_or(0);
                let cache_creation: i32 = row.get(10).unwrap_or(0);
                let cache_read: i32 = row.get(11).unwrap_or(0);
                let duration: i64 = row.get(12).unwrap_or(0);
                let status: i32 = row.get(14).unwrap_or(0);

                log::trace!(
                    "Parsed log: tokens=({}/{}), cache=({}/{}), duration={}, status={}",
                    input_tokens, output_tokens, cache_creation, cache_read, duration, status
                );

                Ok(RequestLog {
                    request_id: row.get(0)?,
                    timestamp: row.get(1)?,
                    profile_id: row.get(2)?,
                    profile_name: row.get(3)?,
                    provider: row.get(4)?,
                    original_model: row.get(5)?,
                    model_mode: row.get(6)?,
                    forwarded_model: row.get(7)?,
                    input_tokens,
                    output_tokens,
                    cache_creation_input_tokens: cache_creation,
                    cache_read_input_tokens: cache_read,
                    duration_ms: duration,
                    upstream_duration_ms: row.get(13).ok(),
                    status_code: status,
                    error_message: row.get(15).ok(),
                    is_stream: row.get::<_, i32>(16).unwrap_or(0) != 0,
                    request_size_bytes: row.get(17).ok(),
                    response_size_bytes: row.get(18).ok(),
                    response_body: row.get(19).ok(),
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

/// 清理超过指定天数的旧日志
///
/// # Arguments
/// * `retention_days` - 保留的天数，默认30天
///
/// # Returns
/// * `Ok(usize)` - 删除的日志条数
pub async fn cleanup_old_logs(retention_days: i64) -> Result<usize, String> {
    let db_path = get_db_path();

    let deleted_count = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 计算保留时间的截止时间戳（毫秒）
        let now = chrono::Local::now().timestamp_millis();
        let cutoff_timestamp = now - (retention_days * 86400000); // 86400000ms = 1天

        // 删除超过保留期的日志
        let deleted = conn.execute(
            "DELETE FROM request_logs WHERE timestamp < ?1",
            [cutoff_timestamp],
        )
        .map_err(|e| format!("Failed to delete old logs: {}", e))?;

        log::info!("Cleaned up {} old logs (retention: {} days)", deleted, retention_days);

        Ok::<usize, String>(deleted)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(deleted_count)
}

/// 去重日志记录
/// 删除重复的 request_id 记录，保留最新的记录（按 id 降序）
///
/// # Returns
/// * `Ok(usize)` - 删除的重复日志条数
pub async fn deduplicate_logs() -> Result<usize, String> {
    let db_path = get_db_path();

    let deleted_count = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 删除重复记录，保留每个 request_id 的最新记录（id 最大的）
        let deleted = conn.execute(
            r#"
            DELETE FROM request_logs
            WHERE id NOT IN (
                SELECT MAX(id)
                FROM request_logs
                GROUP BY request_id
            )
            "#,
            [],
        )
        .map_err(|e| format!("Failed to deduplicate logs: {}", e))?;

        if deleted > 0 {
            log::info!("Deduplicated {} duplicate log records", deleted);
        }

        Ok::<usize, String>(deleted)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(deleted_count)
}
