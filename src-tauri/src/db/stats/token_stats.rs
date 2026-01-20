// Token 使用量统计模块

use crate::db::schema::get_db_path;
use super::time_range::{get_today_start, get_today_end, get_year_start};
use super::types::TokenDataPoint;
use chrono::{Datelike, Local, TimeZone, Timelike};

/// 获取 Token 使用量统计数据（按时间范围）
pub async fn get_token_stats(time_range: &str) -> Result<Vec<TokenDataPoint>, String> {
    let db_path = get_db_path();
    let time_range = time_range.to_string();

    let data_points = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        match time_range.as_str() {
            "hour" => get_hourly_stats(&conn),
            "day" => get_daily_stats(&conn),
            "week" => get_weekly_stats(&conn),
            "month" => get_monthly_stats(&conn),
            _ => Err(format!("Invalid time range: {}", time_range)),
        }
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(data_points)
}

/// 获取按小时统计的数据（当前时间往前5小时 + 往后7小时，动态时间轴）
fn get_hourly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let now = Local::now();

    // 计算当前小时的时间戳（精确到小时，分钟、秒、纳秒设为0）
    let current_hour_timestamp = now
        .with_minute(0)
        .ok_or_else(|| "Failed to set minute to 0".to_string())?
        .with_second(0)
        .ok_or_else(|| "Failed to set second to 0".to_string())?
        .with_nanosecond(0)
        .ok_or_else(|| "Failed to set nanosecond to 0".to_string())?
        .timestamp_millis();

    // 往前5小时，往后7小时（共13个小时）
    let start_timestamp = current_hour_timestamp - (5 * 3600000);
    let end_timestamp = current_hour_timestamp + (8 * 3600000); // +8 因为要包含往后7小时
    let total_hours: i32 = 13;

    // 初始化数据点
    let mut data_points: Vec<TokenDataPoint> = Vec::new();
    for i in 0..total_hours {
        let hour_timestamp = start_timestamp + (i as i64 * 3600000);
        let hour_time = Local
            .timestamp_millis_opt(hour_timestamp)
            .single()
            .ok_or_else(|| "Failed to create time from timestamp".to_string())?;

        data_points.push(TokenDataPoint {
            label: format!("{:02}:00", hour_time.hour()),
            tokens: 0,
            cache_read_tokens: 0,
        });
    }

    // 查询指定时间范围内每小时的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 3600000 AS INTEGER) as hour,
                SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp < ?2
            GROUP BY hour
            "#,
        )
        .map_err(|e| format!("Failed to prepare hourly stats: {}", e))?;

    let rows = stmt
        .query_map([start_timestamp, end_timestamp], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query hourly stats: {}", e))?;

    for row in rows {
        let (hour, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        // hour 是相对于 start_timestamp 的小时数，直接作为数组索引
        if hour >= 0 && hour < total_hours {
            let index = hour as usize;
            if index < data_points.len() {
                data_points[index].tokens = tokens;
                data_points[index].cache_read_tokens = cache_read_tokens;
            }
        }
    }

    Ok(data_points)
}

/// 获取按天统计的数据（前6天+今天，共7天）
fn get_daily_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let today_start = get_today_start()?;
    let six_days_ago = today_start - (6 * 86400000);

    // 初始化7天的数据点（从6天前到今天）
    let mut data_points: Vec<TokenDataPoint> = Vec::new();
    for i in 0..7 {
        let day_timestamp = six_days_ago + (i * 86400000);
        let day_date = Local.timestamp_millis_opt(day_timestamp)
            .single()
            .ok_or_else(|| "Failed to create date from timestamp".to_string())?;

        data_points.push(TokenDataPoint {
            label: format!("{}月{}日", day_date.month(), day_date.day()),
            tokens: 0,
            cache_read_tokens: 0,
        });
    }

    // 查询最近7天每天的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 86400000 AS INTEGER) as day,
                SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp < ?1 + 604800000
            GROUP BY day
            "#,
        )
        .map_err(|e| format!("Failed to prepare daily stats: {}", e))?;

    let rows = stmt
        .query_map([six_days_ago], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query daily stats: {}", e))?;

    for row in rows {
        let (day, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        if day >= 0 && day < 7 {
            data_points[day as usize].tokens = tokens;
            data_points[day as usize].cache_read_tokens = cache_read_tokens;
        }
    }

    Ok(data_points)
}

/// 获取按周统计的数据（最近4周）
fn get_weekly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let today_end = get_today_end()?;
    let four_weeks_ago = today_end - (28 * 86400000);

    // 初始化4周的数据点（从旧到新）
    let mut data_points: Vec<TokenDataPoint> = (1..=4)
        .map(|week| TokenDataPoint {
            label: format!("第{}周", week),
            tokens: 0,
            cache_read_tokens: 0,
        })
        .collect();

    // 查询最近4周每周的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 604800000 AS INTEGER) as week,
                SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp <= ?2
            GROUP BY week
            "#,
        )
        .map_err(|e| format!("Failed to prepare weekly stats: {}", e))?;

    let rows = stmt
        .query_map([four_weeks_ago, today_end], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query weekly stats: {}", e))?;

    for row in rows {
        let (week, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        if week >= 0 && week <= 3 {
            data_points[week as usize].tokens = tokens;
            data_points[week as usize].cache_read_tokens = cache_read_tokens;
        }
    }

    Ok(data_points)
}

/// 获取按月统计的数据（今年12个月）
fn get_monthly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let year_start = get_year_start()?;

    // 初始化12个月的数据点
    let mut data_points: Vec<TokenDataPoint> = (1..=12)
        .map(|month| TokenDataPoint {
            label: format!("{}月", month),
            tokens: 0,
            cache_read_tokens: 0,
        })
        .collect();

    // 查询今年每月的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 2592000000 AS INTEGER) as month,
                SUM(input_tokens + output_tokens + cache_creation_input_tokens + cache_read_input_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1
            GROUP BY month
            "#,
        )
        .map_err(|e| format!("Failed to prepare monthly stats: {}", e))?;

    let rows = stmt
        .query_map([year_start], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query monthly stats: {}", e))?;

    for row in rows {
        let (month, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        if month >= 0 && month < 12 {
            data_points[month as usize].tokens = tokens;
            data_points[month as usize].cache_read_tokens = cache_read_tokens;
        }
    }

    Ok(data_points)
}
