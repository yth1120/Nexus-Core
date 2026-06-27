use std::fmt;
use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::utils::{AppError, AppResult};

use super::engine_state::EngineState;

/// Identifies a concrete engine backend.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EngineType {
    Native,
    SingBox,
    Mihomo,
    Xray,
    Plugin(String),
}

impl fmt::Display for EngineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineType::Native => write!(f, "native"),
            EngineType::SingBox => write!(f, "sing-box"),
            EngineType::Mihomo => write!(f, "mihomo"),
            EngineType::Xray => write!(f, "xray"),
            EngineType::Plugin(name) => write!(f, "plugin:{}", name),
        }
    }
}

impl FromStr for EngineType {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "native" => Ok(EngineType::Native),
            "sing-box" | "singbox" => Ok(EngineType::SingBox),
            "mihomo" => Ok(EngineType::Mihomo),
            "xray" => Ok(EngineType::Xray),
            other if other.starts_with("plugin:") => {
                let name = other.strip_prefix("plugin:").unwrap_or("unknown");
                Ok(EngineType::Plugin(name.to_string()))
            }
            _ => Err(AppError::Validation(format!("unknown engine type: {s}"))),
        }
    }
}

/// A capability that an engine may advertise.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EngineCapability {
    HttpProxy,
    Socks5Proxy,
    MixedProxy,
    Tun,
    Dns,
    Rule,
    Script,
    ClashApi,
    Statistics,
    HotReload,
}

/// Uniform interface for every pluggable network backend.
#[async_trait]
pub trait Engine: Send + Sync {
    async fn initialize(&self) -> AppResult<()>;
    async fn start(&self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;
    async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }
    async fn reload_config(&self) -> AppResult<()>;
    async fn health_check(&self) -> AppResult<()>;
    fn status(&self) -> EngineState;
    fn engine_type(&self) -> EngineType;
    fn version(&self) -> String;
    fn capabilities(&self) -> Vec<EngineCapability>;
}
