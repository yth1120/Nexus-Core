use async_trait::async_trait;

use crate::engine::engine_state::{EngineState, EngineStateCell};
use crate::engine::engine_trait::{Engine, EngineCapability, EngineType};
use crate::utils::AppResult;

/// Xray-core mock engine — 4 capabilities: HttpProxy, Socks5Proxy, Dns, Statistics.
#[derive(Default)]
pub struct XrayEngine {
    state: EngineStateCell,
}

#[async_trait]
impl Engine for XrayEngine {
    async fn initialize(&self) -> AppResult<()> {
        tracing::debug!("XrayEngine::initialize (mock)");
        Ok(())
    }

    async fn start(&self) -> AppResult<()> {
        self.state.set(EngineState::Starting);
        tracing::debug!("XrayEngine::start (mock)");
        self.state.set(EngineState::Running);
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        self.state.set(EngineState::Stopping);
        tracing::debug!("XrayEngine::stop (mock)");
        self.state.set(EngineState::Stopped);
        Ok(())
    }

    async fn reload_config(&self) -> AppResult<()> {
        tracing::debug!("XrayEngine::reload_config (mock)");
        Ok(())
    }

    async fn health_check(&self) -> AppResult<()> {
        Ok(())
    }

    fn status(&self) -> EngineState {
        self.state.get()
    }

    fn engine_type(&self) -> EngineType {
        EngineType::Xray
    }

    fn version(&self) -> String {
        "25.1.30-mock".into()
    }

    fn capabilities(&self) -> Vec<EngineCapability> {
        vec![
            EngineCapability::HttpProxy,
            EngineCapability::Socks5Proxy,
            EngineCapability::Dns,
            EngineCapability::Statistics,
        ]
    }
}
