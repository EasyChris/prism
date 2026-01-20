// 仪表盘统计模块

use crate::db::schema::get_db_path;
use super::time_range::get_today_start;
use super::types::DashboardStats;

/// 获取仪表盘统计数据
pub async fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let db_path = get_db_path();

    let stats = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 获取今天的开始时间戳（毫秒）
        let today_start = get_today_start()?;

        // 查询今日请求数和 Token 使用量
        let (today_requests, today_tokens) = query_stats_by_time(&conn, Some(today_start))?;

        // 查询总请求数和总 Token 使用量
        let (total_requests, total_tokens) = query_stats_by_time(&conn, None)?;

        Ok::<DashboardStats, String>(DashboardStats {
            today_requests,
            today_tokens,
            total_requests,
            total_tokens,
        })
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(stats)
}

/// 查询统计数据（按时间过滤）
fn query_stats_by_time(
    conn: &rusqlite::Connection,
    timestamp_filter: Option<i64>,
) -> Result<(i32, i32), String> {
    let (sql, params): (&str, Vec<i64>) = if let Some(ts) = timestamp_filter {
        (
            r#"
            SELECT
                COUNT(*) as request_count,
                COALESCE(SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens), 0) as token_count
            FROM request_logs
            WHERE timestamp >= ?1
            "#,
            vec![ts],
        )
    } else {
        (
            r#"
            SELECT
                COUNT(*) as request_count,
                COALESCE(SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens), 0) as token_count
            FROM request_logs
            "#,
            vec![],
        )
    };

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let result = if params.is_empty() {
        stmt.query_row([], |row| Ok((row.get(0)?, row.get(1)?)))
    } else {
        stmt.query_row([params[0]], |row| Ok((row.get(0)?, row.get(1)?)))
    }
    .map_err(|e| format!("Failed to query stats: {}", e))?;

    Ok(result)
}
