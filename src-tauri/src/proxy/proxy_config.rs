// 代理服务器配置管理

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::str::FromStr;

/// 代理服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    /// 监听地址（默认：127.0.0.1）
    pub host: String,
    /// 监听端口（默认：15288）
    pub port: u16,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 15288,
        }
    }
}

impl ProxyConfig {
    /// 验证配置是否有效
    pub fn validate(&self) -> Result<(), String> {
        // 验证 IP 地址
        if IpAddr::from_str(&self.host).is_err() {
            return Err(format!("Invalid host address: {}", self.host));
        }

        // 验证端口范围
        if self.port == 0 {
            return Err("Port cannot be 0".to_string());
        }

        Ok(())
    }

    /// 获取完整的 SocketAddr
    pub fn to_socket_addr(&self) -> Result<std::net::SocketAddr, String> {
        let ip = IpAddr::from_str(&self.host)
            .map_err(|e| format!("Invalid host address: {}", e))?;
        Ok(std::net::SocketAddr::new(ip, self.port))
    }

    /// 获取配置键名
    pub const fn config_key() -> &'static str {
        "proxy_server_config"
    }

    /// 获取配置键名 - 用于存储服务运行状态
    pub const fn status_key() -> &'static str {
        "proxy_server_status"
    }
}

/// 代理服务器运行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyServerStatus {
    /// 是否正在运行
    pub is_running: bool,
    /// 当前监听地址
    pub addr: Option<String>,
    /// 启动时间（Unix 时间戳）
    pub started_at: Option<i64>,
    /// 总请求数
    pub total_requests: u64,
    /// 最后错误信息
    pub last_error: Option<String>,
}

impl Default for ProxyServerStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            addr: None,
            started_at: None,
            total_requests: 0,
            last_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ProxyConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 15288);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let config = ProxyConfig {
            host: "invalid.ip.address".to_string(),
            port: 15288,
        };
        assert!(config.validate().is_err());

        let config = ProxyConfig {
            host: "0.0.0.0".to_string(),
            port: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_to_socket_addr() {
        let config = ProxyConfig {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let addr = config.to_socket_addr().unwrap();
        assert_eq!(addr.to_string(), "127.0.0.1:8080");
    }
}
