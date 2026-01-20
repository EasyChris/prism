// Tauri 命令：配置管理 API

use crate::config::{ConfigManager, MappingRule, ModelMappingMode, Profile};
use crate::logger::RequestLog;
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
    pub is_active: bool,
    pub model_mapping_mode: ModelMappingMode,
    pub override_model: Option<String>,
    pub model_mappings: Vec<MappingRule>,
}

impl From<&Profile> for ProfileDto {
    fn from(profile: &Profile) -> Self {
        ProfileDto {
            id: profile.id.clone(),
            name: profile.name.clone(),
            api_base_url: profile.api_base_url.clone(),
            api_key: profile.api_key.clone(),
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
    pub model_mapping_mode: ModelMappingMode,
    pub override_model: Option<String>,
    pub model_mappings: Vec<MappingRule>,
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
    );

    // 设置模型映射相关字段
    new_profile.model_mapping_mode = profile.model_mapping_mode;
    new_profile.override_model = profile.override_model;
    new_profile.model_mappings = profile.model_mappings;

    let profile_id = manager.create_profile(new_profile.clone()).map_err(|e| e.to_string())?;

    // 异步保存到数据库
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::db::save_profile_to_db(&new_profile).await {
            log::error!("Failed to save profile to database: {}", e);
        }
    });

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
        is_active: existing_profile.is_active,
        model_mapping_mode: profile.model_mapping_mode,
        override_model: profile.override_model,
        model_mappings: profile.model_mappings,
    };

    manager.update_profile(&id, updated_profile.clone()).map_err(|e| e.to_string())?;

    // 异步保存到数据库
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::db::save_profile_to_db(&updated_profile).await {
            log::error!("Failed to save profile to database: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn delete_profile(config: State<SharedConfigManager>, id: String) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    manager.delete_profile(&id).map_err(|e| e.to_string())?;

    // 异步从数据库删除
    let id_clone = id.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::db::delete_profile_from_db(&id_clone).await {
            log::error!("Failed to delete profile from database: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn activate_profile(config: State<SharedConfigManager>, id: String) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;

    manager.activate_profile(&id).map_err(|e| e.to_string())?;

    // 异步保存所有 profiles 到数据库（因为需要更新所有的 is_active 状态）
    let profiles: Vec<_> = manager.list_profiles().iter().map(|p| (*p).clone()).collect();
    tauri::async_runtime::spawn(async move {
        for profile in profiles {
            if let Err(e) = crate::db::save_profile_to_db(&profile).await {
                log::error!("Failed to save profile to database: {}", e);
            }
        }
    });

    log::info!("Profile activated: {}", id);

    Ok(())
}

// 日志相关命令

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestLogDto {
    pub id: Option<i64>,
    pub request_id: String,
    pub timestamp: i64,
    pub profile_id: String,
    pub profile_name: String,
    pub provider: String,
    pub original_model: String,
    pub model_mode: String,
    pub forwarded_model: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub duration_ms: i64,
    pub upstream_duration_ms: Option<i64>,
    pub status_code: i32,
    pub error_message: Option<String>,
    pub is_stream: bool,
    pub request_size_bytes: Option<i64>,
    pub response_size_bytes: Option<i64>,
}

impl From<RequestLog> for RequestLogDto {
    fn from(log: RequestLog) -> Self {
        RequestLogDto {
            id: None,
            request_id: log.request_id,
            timestamp: log.timestamp,
            profile_id: log.profile_id,
            profile_name: log.profile_name,
            provider: log.provider,
            original_model: log.original_model,
            model_mode: log.model_mode,
            forwarded_model: log.forwarded_model,
            input_tokens: log.input_tokens,
            output_tokens: log.output_tokens,
            duration_ms: log.duration_ms,
            upstream_duration_ms: log.upstream_duration_ms,
            status_code: log.status_code,
            error_message: log.error_message,
            is_stream: log.is_stream,
            request_size_bytes: log.request_size_bytes,
            response_size_bytes: log.response_size_bytes,
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

// 统计数据相关命令

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStatsDto {
    pub today_requests: i32,
    pub today_tokens: i32,
    pub total_requests: i32,
    pub total_tokens: i32,
}

#[tauri::command]
pub async fn get_dashboard_stats() -> Result<DashboardStatsDto, String> {
    let stats = crate::db::get_dashboard_stats().await?;

    Ok(DashboardStatsDto {
        today_requests: stats.today_requests,
        today_tokens: stats.today_tokens,
        total_requests: stats.total_requests,
        total_tokens: stats.total_tokens,
    })
}

// Token 统计数据相关命令

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenDataPointDto {
    pub label: String,
    pub tokens: i32,
}

#[tauri::command]
pub async fn get_token_stats(time_range: String) -> Result<Vec<TokenDataPointDto>, String> {
    let data_points = crate::db::get_token_stats(&time_range).await?;

    Ok(data_points
        .into_iter()
        .map(|dp| TokenDataPointDto {
            label: dp.label,
            tokens: dp.tokens,
        })
        .collect())
}

// API Key 管理相关命令

#[tauri::command]
pub fn get_proxy_api_key(config: State<SharedConfigManager>) -> Result<Option<String>, String> {
    let manager = config.read().map_err(|e| e.to_string())?;
    Ok(manager.get_api_key().cloned())
}

#[tauri::command]
pub fn refresh_proxy_api_key(config: State<SharedConfigManager>) -> Result<String, String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;
    let new_key = manager.refresh_api_key();

    // 异步保存到数据库
    let key_clone = new_key.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::db::save_app_config("proxy_api_key", &key_clone).await {
            log::error!("Failed to save proxy API key to database: {}", e);
        }
    });

    log::info!("Proxy API key refreshed");
    Ok(new_key)
}

#[tauri::command]
pub fn get_auth_enabled(config: State<SharedConfigManager>) -> Result<bool, String> {
    let manager = config.read().map_err(|e| e.to_string())?;
    Ok(manager.is_auth_enabled())
}

#[tauri::command]
pub fn set_auth_enabled(config: State<SharedConfigManager>, enabled: bool) -> Result<(), String> {
    let mut manager = config.write().map_err(|e| e.to_string())?;
    manager.set_auth_enabled(enabled);

    // 异步保存到数据库
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::db::save_app_config("enable_auth", &enabled.to_string()).await {
            log::error!("Failed to save auth enabled to database: {}", e);
        }
    });

    log::info!("Auth enabled set to: {}", enabled);
    Ok(())
}

#[tauri::command]
pub async fn get_proxy_server_url() -> Result<String, String> {
    // 从数据库加载代理配置
    let config = crate::db::load_proxy_config().await?;
    Ok(format!("http://{}:{}", config.host, config.port))
}

// ==================== 窗口控制命令 ====================

#[tauri::command]
pub fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e: tauri::Error| e.to_string())?;
        window.set_focus().map_err(|e: tauri::Error| e.to_string())?;
        log::info!("Main window shown");
        Ok(())
    } else {
        Err("Main window not found".to_string())
    }
}

#[tauri::command]
pub fn update_tray_menu(app: tauri::AppHandle, config: State<SharedConfigManager>) -> Result<(), String> {
    crate::tray::rebuild_tray_menu(&app, &config)
        .map_err(|e| format!("Failed to update tray menu: {}", e))
}

// ==================== 代理服务器配置命令 ====================

#[tauri::command]
pub async fn get_proxy_config() -> Result<crate::proxy::ProxyConfig, String> {
    crate::db::load_proxy_config().await
}

#[tauri::command]
pub async fn set_proxy_config(
    status_manager: State<'_, crate::proxy::ProxyStatusManager>,
    config: crate::proxy::ProxyConfig,
) -> Result<(), String> {
    // 验证配置
    config.validate()?;

    // 保存到数据库
    crate::db::save_proxy_config(&config).await?;

    log::info!("Proxy config updated: {}:{}", config.host, config.port);

    // 自动触发重启
    status_manager.restart(config).await?;

    log::info!("Proxy server restart triggered");
    Ok(())
}

#[tauri::command]
pub async fn get_proxy_status() -> Result<crate::proxy::ProxyServerStatus, String> {
    crate::db::load_proxy_status().await
}

#[tauri::command]
pub async fn restart_proxy_server(
    _config: State<'_, SharedConfigManager>,
    proxy_config: crate::proxy::ProxyConfig,
) -> Result<String, String> {
    // 验证配置
    proxy_config.validate()?;

    // 保存配置
    crate::db::save_proxy_config(&proxy_config).await?;

    // 注意：这里需要实现重启逻辑
    // 由于当前的架构限制，我们暂时只保存配置
    // 实际的重启需要应用重启后才能生效

    log::info!("Proxy server will restart on next app launch: {}:{}", proxy_config.host, proxy_config.port);

    Ok(format!("Proxy server configured to listen on {}:{}. Restart the app to apply changes.", proxy_config.host, proxy_config.port))
}
