// Tauri 命令：配置管理 API

use crate::config::{ConfigManager, ModelMappingMode, Profile};
use crate::logger::RequestLog;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::{Manager, State};

pub type SharedConfigManager = Arc<RwLock<ConfigManager>>;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDto {
    pub id: String,
    pub name: String,
    pub api_base_url: String,
    pub api_key: String,
    pub model_id: String,
    pub is_active: bool,
    pub model_mapping_mode: ModelMappingMode,
    pub override_model: Option<String>,
    pub model_mappings: HashMap<String, String>,
}

impl From<&Profile> for ProfileDto {
    fn from(profile: &Profile) -> Self {
        ProfileDto {
            id: profile.id.clone(),
            name: profile.name.clone(),
            api_base_url: profile.api_base_url.clone(),
            api_key: profile.api_key.clone(),
            model_id: profile.model_id.clone(),
            is_active: profile.is_active,
            model_mapping_mode: profile.model_mapping_mode.clone(),
            override_model: profile.override_model.clone(),
            model_mappings: profile.model_mappings.clone(),
        }
    }
}

#[tauri::command]
pub fn get_all_profiles(config: State<SharedConfigManager>) -> Result<Vec<ProfileDto>, String> {
    let manager = config.read().map_err(|e| e.to_string())?;

    let profiles: Vec<ProfileDto> = manager
        .get_profiles_with_keys()
        .iter()
        .map(|(key, profile)| {
            let mut dto = ProfileDto::from(*profile);
            dto.id = key.clone();  // 使用 HashMap key 作为 ID
            dto
        })
        .collect();

    Ok(profiles)
}

// 创建配置时使用的 DTO（不需要 id 和 isActive）
#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileDto {
    pub name: String,
    pub api_base_url: String,
    pub api_key: String,
    pub model_id: String,
    pub model_mapping_mode: ModelMappingMode,
    pub override_model: Option<String>,
    pub model_mappings: HashMap<String, String>,
}

#[tauri::command]
pub fn create_profile(
    config: State<SharedConfigManager>,
    profile: CreateProfileDto,
) -> Result<String, String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    let mut new_profile = Profile::new(
        profile.name,
        profile.api_base_url,
        profile.api_key,
        profile.model_id,
    );

    // 设置模型映射相关字段
    new_profile.model_mapping_mode = profile.model_mapping_mode;
    new_profile.override_model = profile.override_model;
    new_profile.model_mappings = profile.model_mappings;

    let profile_id = manager.create_profile(new_profile).map_err(|e| e.to_string())?;

    // 保存到文件
    let config_path = crate::config::get_config_path();
    manager.save_to_file(&config_path).map_err(|e| e.to_string())?;

    Ok(profile_id)
}

#[tauri::command]
pub fn update_profile(
    config: State<SharedConfigManager>,
    id: String,
    profile: ProfileDto,
) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    // 获取原有配置以保留 ID 和 isActive 状态
    let existing_profile = manager.get_profile(&id)
        .ok_or_else(|| "Profile not found".to_string())?;

    // 直接构造 Profile，保留原有的 ID 和 isActive
    let updated_profile = Profile {
        id: id.clone(),
        name: profile.name,
        api_base_url: profile.api_base_url,
        api_key: profile.api_key,
        model_id: profile.model_id,
        is_active: existing_profile.is_active,
        model_mapping_mode: profile.model_mapping_mode,
        override_model: profile.override_model,
        model_mappings: profile.model_mappings,
    };

    manager.update_profile(&id, updated_profile).map_err(|e| e.to_string())?;

    // 保存到文件
    let config_path = crate::config::get_config_path();
    manager.save_to_file(&config_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_profile(config: State<SharedConfigManager>, id: String) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    manager.delete_profile(&id).map_err(|e| e.to_string())?;

    // 保存到文件
    let config_path = crate::config::get_config_path();
    manager.save_to_file(&config_path).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn activate_profile(config: State<SharedConfigManager>, id: String) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    manager.activate_profile(&id).map_err(|e| e.to_string())?;

    // 保存到文件
    let config_path = crate::config::get_config_path();
    manager.save_to_file(&config_path).map_err(|e| e.to_string())?;

    log::info!("Profile activated: {}", id);

    Ok(())
}

// 日志相关命令

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestLogDto {
    pub id: Option<i64>,
    pub timestamp: i64,
    pub profile_id: String,
    pub profile_name: String,
    pub model: String,
    pub provider: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i64,
    pub status_code: i32,
    pub error_message: Option<String>,
    pub is_stream: bool,
}

impl From<RequestLog> for RequestLogDto {
    fn from(log: RequestLog) -> Self {
        RequestLogDto {
            id: None,
            timestamp: log.timestamp,
            profile_id: log.profile_id,
            profile_name: log.profile_name,
            model: log.model,
            provider: log.provider,
            input_tokens: log.input_tokens,
            output_tokens: log.output_tokens,
            duration_ms: log.duration_ms,
            status_code: log.status_code,
            error_message: log.error_message,
            is_stream: log.is_stream,
        }
    }
}

#[tauri::command]
pub async fn get_logs(
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<RequestLogDto>, String> {
    // 限制最多返回 100 条日志
    let limit = limit.unwrap_or(100).min(100) as usize;
    let offset = offset.unwrap_or(0) as usize;

    let logs = crate::logger::get_logs(limit, offset).await;

    Ok(logs.into_iter().map(RequestLogDto::from).collect())
}
