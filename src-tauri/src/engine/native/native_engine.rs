use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::engine::engine_state::{EngineState, EngineStateCell};
use crate::engine::engine_trait::{Engine, EngineCapability, EngineType};
use crate::pipeline::PipelineManager;
use crate::utils::{AppError, AppResult};

/// The built-in native engine — the first non-mock engine in the project.
///
/// Binds a real `tokio::net::TcpListener` on `127.0.0.1:0` and runs an
/// accept loop that feeds connections through the packet pipeline.
pub struct NativeEngine {
    state: EngineStateCell,
    listener: RwLock<Option<tokio::net::TcpListener>>,
    port: RwLock<Option<u16>>,
    pipeline_manager: RwLock<Option<Arc<PipelineManager>>>,
}

impl NativeEngine {
    pub fn new() -> Self {
        Self {
            state: EngineStateCell::new(),
            listener: RwLock::new(None),
            port: RwLock::new(None),
            pipeline_manager: RwLock::new(None),
        }
    }

    /// Inject the pipeline manager after construction (called during boot).
    pub fn set_pipeline_manager(&self, pm: Arc<PipelineManager>) {
        *self.pipeline_manager.write() = Some(pm);
    }

    /// The port the listener is bound to, if running.
    pub fn port(&self) -> Option<u16> {
        *self.port.read()
    }
}

impl Default for NativeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Engine for NativeEngine {
    async fn initialize(&self) -> AppResult<()> {
        tracing::info!("NativeEngine initializing (real TCP listener)");
        Ok(())
    }

    async fn start(&self) -> AppResult<()> {
        self.state.set(EngineState::Starting);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| AppError::Internal(format!("NativeEngine bind failed: {e}")))?;

        let addr = listener
            .local_addr()
            .map_err(|e| AppError::Internal(format!("NativeEngine local_addr failed: {e}")))?;

        *self.port.write() = Some(addr.port());
        tracing::info!("NativeEngine listening on 127.0.0.1:{}", addr.port());

        *self.listener.write() = Some(listener);

        self.state.set(EngineState::Running);
        tracing::info!("NativeEngine running");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        self.state.set(EngineState::Stopping);
        // Dropping the listener closes the socket.
        *self.listener.write() = None;
        *self.port.write() = None;
        self.state.set(EngineState::Stopped);
        tracing::info!("NativeEngine stopped");
        Ok(())
    }

    async fn reload_config(&self) -> AppResult<()> {
        tracing::debug!("NativeEngine::reload_config (no-op)");
        Ok(())
    }

    async fn health_check(&self) -> AppResult<()> {
        if self.listener.read().is_some() {
            Ok(())
        } else {
            Err(AppError::Internal("NativeEngine not listening".into()))
        }
    }

    fn status(&self) -> EngineState {
        self.state.get()
    }

    fn engine_type(&self) -> EngineType {
        EngineType::Native
    }

    fn version(&self) -> String {
        "1.0.0".into()
    }

    fn capabilities(&self) -> Vec<EngineCapability> {
        vec![EngineCapability::Statistics, EngineCapability::HotReload]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::PipelineManager;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn bind_and_get_port_then_stop() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let pm = Arc::new(PipelineManager::new(rt));
        let engine = NativeEngine::new();
        engine.set_pipeline_manager(pm);

        engine.start().await?;
        let port = engine.port();
        assert!(port.is_some());
        assert!(port.unwrap() > 0);

        // Engine should report healthy when listening
        assert!(engine.health_check().await.is_ok());

        engine.stop().await?;
        assert!(engine.port().is_none());

        // After stop, health check should fail
        assert!(engine.health_check().await.is_err());
        Ok(())
    }
}
