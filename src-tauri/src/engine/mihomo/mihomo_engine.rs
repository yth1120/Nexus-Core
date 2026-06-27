use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::engine::engine_state::{EngineState, EngineStateCell};
use crate::engine::engine_trait::{Engine, EngineCapability, EngineType};
use crate::engine::external::{ProcessConfig, ProcessHandle};
use crate::engine::process_manager::ProcessManager;
use crate::utils::{AppError, AppResult};

fn read_str(lock: &RwLock<String>) -> AppResult<String> {
    lock.read()
        .map(|g| g.clone())
        .map_err(|e| AppError::Internal(format!("lock: {e}")))
}
fn read_opt_handle(
    lock: &RwLock<Option<Arc<ProcessHandle>>>,
) -> AppResult<Option<Arc<ProcessHandle>>> {
    lock.read()
        .map(|g| g.clone())
        .map_err(|e| AppError::Internal(format!("lock: {e}")))
}
fn write_handle(
    lock: &RwLock<Option<Arc<ProcessHandle>>>,
    val: Option<Arc<ProcessHandle>>,
) -> AppResult<()> {
    lock.write()
        .map(|mut g| *g = val)
        .map_err(|e| AppError::Internal(format!("lock: {e}")))
}

use super::config_generator;

pub struct MihomoEngine {
    state: EngineStateCell,
    handle: RwLock<Option<Arc<ProcessHandle>>>,
    process_manager: ProcessManager,
    config_path: RwLock<String>,
}

impl MihomoEngine {
    pub fn new() -> Self {
        Self {
            state: EngineStateCell::new(),
            handle: RwLock::new(None),
            process_manager: ProcessManager::new(),
            config_path: RwLock::new(String::new()),
        }
    }

    pub fn set_config_path(&self, path: &str) {
        let _ = self.config_path.write().map(|mut g| *g = path.to_string());
    }
}

impl Default for MihomoEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Engine for MihomoEngine {
    async fn initialize(&self) -> AppResult<()> {
        Ok(())
    }

    async fn start(&self) -> AppResult<()> {
        self.state.set(EngineState::Starting);
        let config_path = read_str(&self.config_path)?;
        if config_path.is_empty() {
            return Err(AppError::Internal("mihomo config path not set".into()));
        }
        let config = config_generator::generate(&Default::default(), &[], &[])?;
        std::fs::write(&config_path, &config)
            .map_err(|e| AppError::Io(format!("write config: {e}")))?;
        let pc = ProcessConfig::new("mihomo", &config_path);
        let handle = self.process_manager.spawn(&pc).await?;
        write_handle(&self.handle, Some(handle))?;
        self.state.set(EngineState::Running);
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        self.state.set(EngineState::Stopping);
        let existing = read_opt_handle(&self.handle)?;
        if let Some(ref h) = existing {
            self.process_manager.kill(h).await?;
        }
        write_handle(&self.handle, None)?;
        self.state.set(EngineState::Stopped);
        Ok(())
    }

    async fn reload_config(&self) -> AppResult<()> {
        let config = config_generator::generate(&Default::default(), &[], &[])?;
        let config_path = read_str(&self.config_path)?;
        std::fs::write(&config_path, &config)
            .map_err(|e| AppError::Io(format!("reload config: {e}")))?;
        Ok(())
    }

    async fn health_check(&self) -> AppResult<()> {
        if read_opt_handle(&self.handle)?
            .as_ref()
            .is_some_and(|h| h.is_running())
        {
            Ok(())
        } else {
            Err(AppError::Internal("mihomo not running".into()))
        }
    }

    fn status(&self) -> EngineState {
        self.state.get()
    }
    fn engine_type(&self) -> EngineType {
        EngineType::Mihomo
    }
    fn version(&self) -> String {
        "1.19.0".into()
    }
    fn capabilities(&self) -> Vec<EngineCapability> {
        vec![
            EngineCapability::HttpProxy,
            EngineCapability::Socks5Proxy,
            EngineCapability::MixedProxy,
            EngineCapability::Tun,
            EngineCapability::Dns,
            EngineCapability::Rule,
            EngineCapability::ClashApi,
            EngineCapability::Statistics,
        ]
    }
}
