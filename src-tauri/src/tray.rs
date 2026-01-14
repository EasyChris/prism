use tauri::{
    menu::{Menu, MenuBuilder, MenuItemBuilder, PredefinedMenuItem, SubmenuBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    image::Image,
    AppHandle, Manager, Runtime,
};
use std::sync::{Arc, RwLock};
use crate::config::{ConfigManager, get_config_path};

pub type SharedConfigManager = Arc<RwLock<ConfigManager>>;

/// 初始化系统托盘
pub fn init_tray<R: Runtime>(app: &AppHandle<R>, config: SharedConfigManager) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing system tray...");

    // 构建托盘菜单
    let menu = build_tray_menu(app, &config)?;

    // 加载托盘图标
    let icon_bytes = include_bytes!("../icons/tray_icon_v2.png");
    let img = image::load_from_memory(icon_bytes)?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let icon = Image::new_owned(rgba.into_raw(), width, height);

    // 创建托盘图标
    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .tooltip("Prism Hub")
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| {
            handle_tray_menu_event(app, event, config.clone());
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                log::debug!("Tray icon clicked");
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    log::info!("System tray initialized successfully");
    Ok(())
}

/// 构建托盘菜单
fn build_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
    config: &SharedConfigManager,
) -> Result<Menu<R>, Box<dyn std::error::Error>> {
    let config_guard = config.read().map_err(|e| format!("Failed to read config: {}", e))?;

    // 获取当前服务状态
    let active_profile = config_guard.get_active_profile();
    let status_text = if active_profile.is_some() {
        "🟢 代理服务运行中"
    } else {
        "⚪ 代理服务未激活"
    };

    // 构建配置档案子菜单
    let profiles = config_guard.list_profiles();
    let mut profile_submenu = SubmenuBuilder::new(app, "配置档案");

    if profiles.is_empty() {
        let empty_item = MenuItemBuilder::new("(无配置)")
            .enabled(false)
            .build(app)?;
        profile_submenu = profile_submenu.item(&empty_item);
    } else {
        for profile in profiles {
            let is_active = active_profile
                .as_ref()
                .map_or(false, |p| p.id == profile.id);

            // 如果是激活的配置，在名称前添加勾选标记
            let display_name = if is_active {
                format!("✓ {}", profile.name)
            } else {
                profile.name.clone()
            };

            let item = MenuItemBuilder::new(&display_name)
                .id(&profile.id)
                .enabled(true)
                .build(app)?;

            profile_submenu = profile_submenu.item(&item);
        }
    }

    // 构建主菜单
    let status_item = MenuItemBuilder::new(status_text)
        .id("status")
        .enabled(false)
        .build(app)?;

    let show_window_item = MenuItemBuilder::new("显示主窗口")
        .id("show_window")
        .build(app)?;

    let quit_item = PredefinedMenuItem::quit(app, Some("退出"))?;

    let menu = MenuBuilder::new(app)
        .item(&status_item)
        .separator()
        .item(&profile_submenu.build()?)
        .separator()
        .item(&show_window_item)
        .item(&quit_item)
        .build()?;

    Ok(menu)
}

/// 处理托盘菜单点击事件
fn handle_tray_menu_event<R: Runtime>(
    app: &AppHandle<R>,
    event: tauri::menu::MenuEvent,
    config: SharedConfigManager,
) {
    let menu_id = event.id().as_ref();
    log::info!("Tray menu event: {}", menu_id);

    match menu_id {
        "show_window" => {
            // 显示主窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "status" => {
            // 状态项不可点击，忽略
        }
        profile_id => {
            // 切换配置
            log::info!("Switching to profile: {}", profile_id);

            let result = {
                let mut config_guard = match config.write() {
                    Ok(guard) => guard,
                    Err(e) => {
                        log::error!("Failed to acquire config write lock: {}", e);
                        return;
                    }
                };

                // 激活配置
                if let Err(e) = config_guard.activate_profile(profile_id) {
                    log::error!("Failed to activate profile: {}", e);
                    return;
                }

                // 保存配置
                config_guard.save_to_file(&get_config_path())
            };

            if let Err(e) = result {
                log::error!("Failed to save config: {}", e);
                return;
            }

            // 重建托盘菜单
            if let Err(e) = rebuild_tray_menu(app, &config) {
                log::error!("Failed to rebuild tray menu: {}", e);
            }
        }
    }
}

/// 重建托盘菜单（配置变更时调用）
pub fn rebuild_tray_menu<R: Runtime>(
    app: &AppHandle<R>,
    config: &SharedConfigManager,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Rebuilding tray menu...");

    // 构建新菜单
    let new_menu = build_tray_menu(app, config)?;

    // 获取托盘图标并更新菜单
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(new_menu))?;
        log::info!("Tray menu rebuilt successfully");
    } else {
        log::warn!("Tray icon not found");
    }

    Ok(())
}
