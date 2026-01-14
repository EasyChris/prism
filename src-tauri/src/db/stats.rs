// 统计相关的数据库操作

use super::schema::get_db_path;
use chrono::{Datelike, Local, TimeZone};

/// 统计数据结构
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

/// 获取仪表盘统计数据
pub async fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let db_path = get_db_path();

    let stats = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // 获取本地时区今天的开始时间戳（毫秒）
        let now = Local::now();
        let today_start = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .single()
            .unwrap()
            .timestamp_millis();

        // 查询今日请求数和 Token 使用量
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    COUNT(*) as request_count,
                    COALESCE(SUM(input_tokens + output_tokens), 0) as token_count
                FROM request_logs
                WHERE timestamp >= ?1
                "#,
            )
            .map_err(|e| format!("Failed to prepare today stats statement: {}", e))?;

        let (today_requests, today_tokens): (i32, i32) = stmt
            .query_row([today_start], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| format!("Failed to query today stats: {}", e))?;

        // 查询总请求数和总 Token 使用量
        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    COUNT(*) as request_count,
                    COALESCE(SUM(input_tokens + output_tokens), 0) as token_count
                FROM request_logs
                "#,
            )
            .map_err(|e| format!("Failed to prepare total stats statement: {}", e))?;

        let (total_requests, total_tokens): (i32, i32) = stmt
            .query_row([], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| format!("Failed to query total stats: {}", e))?;

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

/// 获取按小时统计的数据（今天24小时）
fn get_hourly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    // 获取本地时区今天的开始时间戳（毫秒）
    let now = Local::now();
    let today_start = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .unwrap()
        .timestamp_millis();

    // 初始化24小时的数据点
    let mut data_points: Vec<TokenDataPoint> = (0..24)
        .map(|hour| TokenDataPoint {
            label: format!("{:02}:00", hour),
            tokens: 0,
            cache_read_tokens: 0,
        })
        .collect();

    // 查询今天每小时的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 3600000 AS INTEGER) as hour,
                SUM(input_tokens + output_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp < ?1 + 86400000
            GROUP BY hour
            "#,
        )
        .map_err(|e| format!("Failed to prepare hourly stats: {}", e))?;

    let rows = stmt
        .query_map([today_start], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query hourly stats: {}", e))?;

    for row in rows {
        let (hour, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        if hour >= 0 && hour < 24 {
            data_points[hour as usize].tokens = tokens;
            data_points[hour as usize].cache_read_tokens = cache_read_tokens;
        }
    }

    Ok(data_points)
}

/// 获取按天统计的数据（本周7天）
fn get_daily_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let now = Local::now();

    // 计算本周一的开始时间戳（本地时区）
    let today_start = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .unwrap()
        .timestamp_millis();

    // 计算距离周一的天数（0=周一, 6=周日）
    let days_since_monday = now.weekday().num_days_from_monday() as i64;
    let monday_start = today_start - (days_since_monday * 86400000);

    // 初始化7天的数据点
    let weekdays = ["周一", "周二", "周三", "周四", "周五", "周六", "周日"];
    let mut data_points: Vec<TokenDataPoint> = weekdays
        .iter()
        .map(|&day| TokenDataPoint {
            label: day.to_string(),
            tokens: 0,
            cache_read_tokens: 0,
        })
        .collect();

    // 查询本周每天的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 86400000 AS INTEGER) as day,
                SUM(input_tokens + output_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp < ?1 + 604800000
            GROUP BY day
            "#,
        )
        .map_err(|e| format!("Failed to prepare daily stats: {}", e))?;

    let rows = stmt
        .query_map([monday_start], |row| {
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

/// 获取按周统计的数据（本月4周）
fn get_weekly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    let now = Local::now();

    // 计算过去28天的开始时间戳（4周，本地时区）
    let today_start = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_local_timezone(Local)
        .single()
        .unwrap()
        .timestamp_millis();
    let four_weeks_ago = today_start - (28 * 86400000);

    // 初始化4周的数据点
    let mut data_points: Vec<TokenDataPoint> = (1..=4)
        .map(|week| TokenDataPoint {
            label: format!("第{}周", week),
            tokens: 0,
            cache_read_tokens: 0,
        })
        .collect();

    // 查询过去4周每周的 Token 使用量
    let mut stmt = conn
        .prepare(
            r#"
            SELECT
                CAST((timestamp - ?1) / 604800000 AS INTEGER) as week,
                SUM(input_tokens + output_tokens) as tokens,
                SUM(cache_read_input_tokens) as cache_read_tokens
            FROM request_logs
            WHERE timestamp >= ?1 AND timestamp < ?1 + 2419200000
            GROUP BY week
            "#,
        )
        .map_err(|e| format!("Failed to prepare weekly stats: {}", e))?;

    let rows = stmt
        .query_map([four_weeks_ago], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, i32>(2)?))
        })
        .map_err(|e| format!("Failed to query weekly stats: {}", e))?;

    for row in rows {
        let (week, tokens, cache_read_tokens) = row.map_err(|e| format!("Failed to read row: {}", e))?;
        if week >= 0 && week < 4 {
            data_points[week as usize].tokens = tokens;
            data_points[week as usize].cache_read_tokens = cache_read_tokens;
        }
    }

    Ok(data_points)
}

/// 获取按月统计的数据（今年12个月）
fn get_monthly_stats(conn: &rusqlite::Connection) -> Result<Vec<TokenDataPoint>, String> {
    // 计算今年1月1日的开始时间戳（本地时区）
    let current_year = Local::now().year();
    let year_start = Local
        .with_ymd_and_hms(current_year, 1, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| "Failed to create year start timestamp".to_string())?
        .timestamp_millis();

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
                SUM(input_tokens + output_tokens) as tokens,
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
