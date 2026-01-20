// 时间范围计算工具模块

use chrono::{Datelike, Local, TimeZone};

/// 获取今天开始时间戳（毫秒）
pub fn get_today_start() -> Result<i64, String> {
    let now = Local::now();
    Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
        .single()
        .ok_or_else(|| "Failed to create today start timestamp".to_string())
        .map(|dt| dt.timestamp_millis())
}

/// 获取今天结束时间戳（毫秒）
pub fn get_today_end() -> Result<i64, String> {
    let now = Local::now();
    Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 23, 59, 59)
        .single()
        .ok_or_else(|| "Failed to create today end timestamp".to_string())
        .map(|dt| dt.timestamp_millis())
}

/// 获取今年开始时间戳（毫秒）
pub fn get_year_start() -> Result<i64, String> {
    let current_year = Local::now().year();
    Local
        .with_ymd_and_hms(current_year, 1, 1, 0, 0, 0)
        .single()
        .ok_or_else(|| "Failed to create year start timestamp".to_string())
        .map(|dt| dt.timestamp_millis())
}

/// 根据时间范围获取起始时间戳
pub fn get_timestamp_for_range(time_range: &str) -> Result<Option<i64>, String> {
    match time_range {
        "hour" => get_today_start().map(Some),
        "day" => {
            let today_start = get_today_start()?;
            Ok(Some(today_start - (6 * 86400000)))
        }
        "week" => {
            let today_end = get_today_end()?;
            Ok(Some(today_end - (28 * 86400000)))
        }
        "month" => get_year_start().map(Some),
        _ => Ok(None),
    }
}
