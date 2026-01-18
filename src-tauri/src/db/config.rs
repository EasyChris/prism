// 配置相关的数据库操作

use crate::config::{Profile, MappingRule, ModelMappingMode};
use super::schema::get_db_path;
use std::time::{SystemTime, UNIX_EPOCH};

/// 保存 Profile 到数据库
pub async fn save_profile_to_db(profile: &Profile) -> Result<(), String> {
    let db_path = get_db_path();
    let profile = profile.clone();

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get timestamp: {}", e))?
            .as_secs() as i64;

        // 插入或更新 profile
        conn.execute(
            r#"
            INSERT INTO profiles (
                id, name, api_base_url, api_key, is_active,
                model_mapping_mode, override_model, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                api_base_url = excluded.api_base_url,
                api_key = excluded.api_key,
                is_active = excluded.is_active,
                model_mapping_mode = excluded.model_mapping_mode,
                override_model = excluded.override_model,
                updated_at = excluded.updated_at
            "#,
            rusqlite::params![
                &profile.id,
                &profile.name,
                &profile.api_base_url,
                &profile.api_key,
                if profile.is_active { 1 } else { 0 },
                profile.model_mapping_mode.as_str(),
                &profile.override_model,
                now,
                now,
            ],
        )
        .map_err(|e| format!("Failed to save profile: {}", e))?;

        // 删除旧的映射规则
        conn.execute(
            "DELETE FROM model_mappings WHERE profile_id = ?1",
            rusqlite::params![&profile.id],
        )
        .map_err(|e| format!("Failed to delete old mappings: {}", e))?;

        // 插入新的映射规则
        for (order, rule) in profile.model_mappings.iter().enumerate() {
            conn.execute(
                r#"
                INSERT INTO model_mappings (
                    profile_id, pattern, target, use_regex, rule_order
                ) VALUES (?1, ?2, ?3, ?4, ?5)
                "#,
                rusqlite::params![
                    &profile.id,
                    &rule.pattern,
                    &rule.target,
                    if rule.use_regex { 1 } else { 0 },
                    order as i32,
                ],
            )
            .map_err(|e| format!("Failed to save mapping rule: {}", e))?;
        }

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(())
}

/// 从数据库加载所有 Profiles
pub async fn load_profiles_from_db() -> Result<Vec<Profile>, String> {
    let db_path = get_db_path();

    let profiles = tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT id, name, api_base_url, api_key, is_active,
                       model_mapping_mode, override_model
                FROM profiles
                ORDER BY created_at DESC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let profiles = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let name: String = row.get(1)?;
                let api_base_url: String = row.get(2)?;
                let api_key: String = row.get(3)?;
                let is_active: i32 = row.get(4)?;
                let model_mapping_mode: String = row.get(5)?;
                let override_model: Option<String> = row.get(6)?;

                Ok((id, name, api_base_url, api_key, is_active, model_mapping_mode, override_model))
            })
            .map_err(|e| format!("Failed to query profiles: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect profiles: {}", e))?;

        Ok::<Vec<_>, String>(profiles)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    // 为每个 profile 加载映射规则
    let mut result = Vec::new();
    for (id, name, api_base_url, api_key, is_active, model_mapping_mode, override_model) in profiles {
        let mappings = load_mappings_for_profile(&id).await?;

        result.push(Profile {
            id,
            name,
            api_base_url,
            api_key,
            is_active: is_active != 0,
            model_mapping_mode: ModelMappingMode::from_str(&model_mapping_mode),
            override_model,
            model_mappings: mappings,
        });
    }

    Ok(result)
}

/// 加载指定 Profile 的映射规则
async fn load_mappings_for_profile(profile_id: &str) -> Result<Vec<MappingRule>, String> {
    let db_path = get_db_path();
    let profile_id = profile_id.to_string();

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let mut stmt = conn
            .prepare(
                r#"
                SELECT pattern, target, use_regex
                FROM model_mappings
                WHERE profile_id = ?1
                ORDER BY rule_order ASC
                "#,
            )
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let mappings = stmt
            .query_map([&profile_id], |row| {
                Ok(MappingRule {
                    pattern: row.get(0)?,
                    target: row.get(1)?,
                    use_regex: row.get::<_, i32>(2)? != 0,
                })
            })
            .map_err(|e| format!("Failed to query mappings: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect mappings: {}", e))?;

        Ok::<Vec<MappingRule>, String>(mappings)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}

/// 删除 Profile
pub async fn delete_profile_from_db(profile_id: &str) -> Result<(), String> {
    let db_path = get_db_path();
    let profile_id = profile_id.to_string();

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        conn.execute(
            "DELETE FROM profiles WHERE id = ?1",
            rusqlite::params![&profile_id],
        )
        .map_err(|e| format!("Failed to delete profile: {}", e))?;

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(())
}

/// 保存应用配置（如 proxy_api_key, enable_auth）
pub async fn save_app_config(key: &str, value: &str) -> Result<(), String> {
    let db_path = get_db_path();
    let key = key.to_string();
    let value = value.to_string();

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Failed to get timestamp: {}", e))?
            .as_secs() as i64;

        conn.execute(
            r#"
            INSERT INTO app_config (key, value, updated_at)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at
            "#,
            rusqlite::params![&key, &value, now],
        )
        .map_err(|e| format!("Failed to save app config: {}", e))?;

        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))??;

    Ok(())
}

/// 加载应用配置
pub async fn load_app_config(key: &str) -> Result<Option<String>, String> {
    let db_path = get_db_path();
    let key = key.to_string();

    tokio::task::spawn_blocking(move || {
        let conn = rusqlite::Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        let result = conn.query_row(
            "SELECT value FROM app_config WHERE key = ?1",
            rusqlite::params![&key],
            |row| row.get::<_, String>(0),
        );

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(format!("Failed to load app config: {}", e)),
        }
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
