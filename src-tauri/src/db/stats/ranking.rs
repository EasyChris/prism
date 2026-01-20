// 配置消耗排名模块

use crate::db::schema::get_db_path;
use super::time_range::get_timestamp_for_range;
use super::types::ProfileConsumption;

/// 获取配置消耗排名（按总Token消耗）
pub async fn get_profile_consumption_ranking(
    time_range: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<ProfileConsumption>, String> {
    let db_path = get_db_path();
    let time_range = time_range.map(|s| s.to_string());
    let limit = limit.unwrap_or(10).max(1).min(100);

    let rankings = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 计算时间范围的起始时间戳（如果提供）
        let timestamp_filter = if let Some(ref tr) = time_range {
            get_timestamp_for_range(tr)?
        } else {
            None
        };

        // 查询排名数据
        let results = query_profile_rankings(&conn, timestamp_filter, limit)?;

        // 计算总 token 数和百分比
        let total_tokens: i32 = results.iter().map(|(_, _, tokens)| tokens).sum();

        let mut rankings = Vec::new();
        for (index, (profile_id, profile_name, tokens)) in results.into_iter().enumerate() {
            let percentage = if total_tokens > 0 {
                (tokens as f32 / total_tokens as f32) * 100.0
            } else {
                0.0
            };

            rankings.push(ProfileConsumption {
                profile_id,
                profile_name,
                total_tokens: tokens,
                percentage,
                rank: (index + 1) as i32,
            });
        }

        Ok::<Vec<ProfileConsumption>, String>(rankings)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(rankings)
}

/// 查询配置排名数据
fn query_profile_rankings(
    conn: &rusqlite::Connection,
    timestamp_filter: Option<i64>,
    limit: i32,
) -> Result<Vec<(String, String, i32)>, String> {
    // 构建 SQL 查询
    // 只按 profile_id 分组，避免同一配置因名称变化而重复
    // 使用子查询获取每个 profile_id 的最新 profile_name
    let sql = if timestamp_filter.is_some() {
        r#"
        SELECT
            profile_id,
            (SELECT profile_name FROM request_logs WHERE profile_id = rl.profile_id ORDER BY timestamp DESC LIMIT 1) as profile_name,
            SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as total_tokens
        FROM request_logs rl
        WHERE timestamp >= ?1
        GROUP BY profile_id
        ORDER BY total_tokens DESC
        LIMIT ?2
        "#
    } else {
        r#"
        SELECT
            profile_id,
            (SELECT profile_name FROM request_logs WHERE profile_id = rl.profile_id ORDER BY timestamp DESC LIMIT 1) as profile_name,
            SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as total_tokens
        FROM request_logs rl
        GROUP BY profile_id
        ORDER BY total_tokens DESC
        LIMIT ?1
        "#
    };

    let mut stmt = conn
        .prepare(sql)
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    // 执行查询
    let results: Vec<(String, String, i32)> = if let Some(ts) = timestamp_filter {
        stmt.query_map([ts, limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| format!("Failed to query rankings: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect rankings: {}", e))?
    } else {
        stmt.query_map([limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })
        .map_err(|e| format!("Failed to query rankings: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect rankings: {}", e))?
    };

    Ok(results)
}
