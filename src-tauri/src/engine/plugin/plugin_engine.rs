use async_trait::async_trait;

use crate::engine::engine_state::{EngineState, EngineStateCell};
use crate::engine::engine_trait::{Engine, EngineCapability, EngineType};
use crate::utils::AppResult;

/// A third-party / community plugin engine identified by name.
///
/// Capabilities: Statistics (extensible in Phase 6). Constructor takes the
/// plugin name; `engine_type()` returns `Plugin(name)`.
pub struct PluginEngine {
    name: String,
    state: EngineStateCell,
}

impl PluginEngine {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: EngineStateCell::new(),
        }
    }
}

#[async_trait]
impl Engine for PluginEngine {
    async fn initialize(&self) -> AppResult<()> {
        tracing::debug!("PluginEngine[{}]::initialize (mock)", self.name);
        Ok(())
    }

    async fn start(&self) -> AppResult<()> {
        self.state.set(EngineState::Starting);
        tracing::debug!("PluginEngine[{}]::start (mock)", self.name);
        self.state.set(EngineState::Running);
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        self.state.set(EngineState::Stopping);
        tracing::debug!("PluginEngine[{}]::stop (mock)", self.name);
        self.state.set(EngineState::Stopped);
        Ok(())
    }

    async fn reload_config(&self) -> AppResult<()> {
        tracing::debug!("PluginEngine[{}]::reload_config (mock)", self.name);
        Ok(())
    }

    async fn health_check(&self) -> AppResult<()> {
        Ok(())
    }

    fn status(&self) -> EngineState {
        self.state.get()
    }

    fn engine_type(&self) -> EngineType {
        EngineType::Plugin(self.name.clone())
    }

    fn version(&self) -> String {
        "0.1.0-mock".into()
    }

    fn capabilities(&self) -> Vec<EngineCapability> {
        vec![EngineCapability::Statistics]
    }
}
