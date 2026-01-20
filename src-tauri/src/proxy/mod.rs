mod handler;
mod stream;
mod utils;
mod proxy_config;

pub use proxy_config::{ProxyConfig, ProxyServerStatus};

use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::{RwLock, mpsc};
use crate::config::SharedConfigManager;
use crate::db::{save_proxy_status, load_proxy_status};
use handler::handle_messages;

/// 代理服务器控制命令
#[derive(Debug, Clone)]
pub enum ProxyCommand {
    Restart(ProxyConfig),
    Shutdown,
}

/// 代理服务器状态管理器
#[derive(Clone)]
pub struct ProxyStatusManager {
    status: Arc<RwLock<ProxyServerStatus>>,
    command_tx: Arc<RwLock<Option<mpsc::Sender<ProxyCommand>>>>,
}

impl ProxyStatusManager {
    pub fn new() -> Self {
        Self {
            status: Arc::new(RwLock::new(ProxyServerStatus::default())),
            command_tx: Arc::new(RwLock::new(None)),
        }
    }

    /// 设置控制通道
    pub async fn set_command_channel(&self, tx: mpsc::Sender<ProxyCommand>) {
        *self.command_tx.write().await = Some(tx);
    }

    /// 发送重启命令
    pub async fn restart(&self, config: ProxyConfig) -> Result<(), String> {
        let tx = self.command_tx.read().await;
        if let Some(tx) = tx.as_ref() {
            tx.send(ProxyCommand::Restart(config))
                .await
                .map_err(|e| format!("Failed to send restart command: {}", e))?;
            Ok(())
        } else {
            Err("Proxy server not initialized".to_string())
        }
    }

    /// 发送关闭命令
    pub async fn shutdown(&self) -> Result<(), String> {
        let tx = self.command_tx.read().await;
        if let Some(tx) = tx.as_ref() {
            tx.send(ProxyCommand::Shutdown)
                .await
                .map_err(|e| format!("Failed to send shutdown command: {}", e))?;
            Ok(())
        } else {
            Err("Proxy server not initialized".to_string())
        }
    }

    /// 获取当前状态
    pub async fn get_status(&self) -> ProxyServerStatus {
        self.status.read().await.clone()
    }

    /// 更新状态
    pub async fn update_status<F>(&self, f: F)
    where
        F: FnOnce(&mut ProxyServerStatus),
    {
        let mut status = self.status.write().await;
        f(&mut status);
    }

    /// 保存状态到数据库
    pub async fn persist(&self) -> Result<(), String> {
        let status = self.get_status().await;
        save_proxy_status(&status).await
    }

    /// 从数据库加载状态
    pub async fn load(&self) -> Result<(), String> {
        let status = load_proxy_status().await?;
        *self.status.write().await = status;
        Ok(())
    }
}

impl Default for ProxyStatusManager {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn start_proxy_server(
    config: SharedConfigManager,
    initial_config: ProxyConfig,
    status_manager: ProxyStatusManager,
) -> Result<(), Box<dyn std::error::Error>> {
    // 创建控制通道
    let (command_tx, mut command_rx) = mpsc::channel::<ProxyCommand>(10);
    status_manager.set_command_channel(command_tx).await;

    let mut current_config = initial_config;

    loop {
        // 验证配置
        if let Err(e) = current_config.validate() {
            log::error!("Invalid proxy config: {}", e);
            status_manager.update_status(|status| {
                status.is_running = false;
                status.last_error = Some(e.clone());
            }).await;
            status_manager.persist().await?;

            // 等待新的配置
            match command_rx.recv().await {
                Some(ProxyCommand::Restart(new_config)) => {
                    current_config = new_config;
                    continue;
                }
                Some(ProxyCommand::Shutdown) | None => break,
            }
        }

        let addr = match current_config.to_socket_addr() {
            Ok(addr) => addr,
            Err(e) => {
                log::error!("Failed to parse address: {}", e);
                status_manager.update_status(|status| {
                    status.is_running = false;
                    status.last_error = Some(e.clone());
                }).await;
                status_manager.persist().await?;

                // 等待新的配置
                match command_rx.recv().await {
                    Some(ProxyCommand::Restart(new_config)) => {
                        current_config = new_config;
                        continue;
                    }
                    Some(ProxyCommand::Shutdown) | None => break,
                }
            }
        };

        log::info!("Proxy server starting on {}", addr);

        // 先绑定端口，确保端口可用
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                log::error!("Failed to bind to {}: {}", addr, e);
                status_manager.update_status(|status| {
                    status.is_running = false;
                    status.last_error = Some(format!("Failed to bind: {}", e));
                }).await;
                status_manager.persist().await?;

                // 等待新的配置
                match command_rx.recv().await {
                    Some(ProxyCommand::Restart(new_config)) => {
                        current_config = new_config;
                        continue;
                    }
                    Some(ProxyCommand::Shutdown) | None => break,
                }
            }
        };

        log::info!("Proxy server listening on {}", addr);

        // 端口绑定成功后，更新状态为运行中
        status_manager.update_status(|status| {
            status.is_running = true;
            status.addr = Some(addr.to_string());
            status.started_at = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
            );
            status.last_error = None;
        }).await;
        status_manager.persist().await?;

        // 创建应用
        let app = Router::new()
            .route("/v1/messages", post(handle_messages))
            .with_state(config.clone());

        // 创建关闭信号通道
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

        // 启动服务器
        let status_manager_clone = status_manager.clone();
        let server_handle = tokio::spawn(async move {
            let serve = axum::serve(listener, app).with_graceful_shutdown(async move {
                shutdown_rx.recv().await;
                log::info!("Proxy server is shutting down...");
            });

            if let Err(e) = serve.await {
                log::error!("Server error: {}", e);
            }

            // 更新状态为已停止
            status_manager_clone.update_status(|status| {
                status.is_running = false;
            }).await;
            let _ = status_manager_clone.persist().await;
        });

        // 等待控制命令
        match command_rx.recv().await {
            Some(ProxyCommand::Restart(new_config)) => {
                log::info!("Restarting proxy server with new config: {}:{}", new_config.host, new_config.port);

                // 发送关闭信号
                let _ = shutdown_tx.send(()).await;

                // 等待服务器关闭
                let _ = server_handle.await;

                // 更新配置并继续循环
                current_config = new_config;
                continue;
            }
            Some(ProxyCommand::Shutdown) | None => {
                log::info!("Shutting down proxy server");
                let _ = shutdown_tx.send(()).await;
                let _ = server_handle.await;
                break;
            }
        }
    }

    Ok(())
}
