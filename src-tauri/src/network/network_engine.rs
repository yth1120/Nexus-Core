use std::sync::Arc;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::network_context::NetworkContext;
use super::network_state::{EngineState, NetworkState};

/// Orchestrates the network sub-systems (tunnel, DNS, proxy, connection) behind
/// a single lifecycle state machine.
///
/// Phase 3 is mock: `start`/`stop` drive the [`EngineState`] machine and the
/// no-op sub-managers, publishing `EngineStateChanged` on every transition.
/// No protocol, proxy, TUN, or DNS work is performed.
pub struct NetworkEngine {
    context: Arc<NetworkContext>,
    state: NetworkState,
}

impl NetworkEngine {
    pub fn new(context: Arc<NetworkContext>) -> Self {
        Self {
            context,
            state: NetworkState::new(),
        }
    }

    /// Current engine state.
    pub fn status(&self) -> EngineState {
        self.state.get()
    }

    /// Prepare sub-systems without starting traffic capture. No-op in Phase 3.
    pub async fn initialize(&self) -> AppResult<()> {
        tracing::info!("NetworkEngine initialize (mock)");
        Ok(())
    }

    /// Start the engine: `Stopped → Starting → Running`.
    ///
    /// Brings up the sub-managers in order (all no-op) and publishes
    /// `EngineStateChanged` for each transition. On failure the engine moves to
    /// `Error` and the error is returned. Idempotent while already `Running`.
    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(EngineState::Starting);

        if let Err(e) = self.start_subsystems().await {
            self.set_state(EngineState::Error);
            tracing::error!("NetworkEngine start failed: {}", e);
            return Err(e);
        }

        self.set_state(EngineState::Running);
        tracing::info!("NetworkEngine running (mock)");
        Ok(())
    }

    /// Stop the engine: `Running → Stopping → Stopped`. Idempotent while
    /// already `Stopped`.
    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), EngineState::Stopped) {
            return Ok(());
        }
        self.set_state(EngineState::Stopping);

        if let Err(e) = self.stop_subsystems().await {
            self.set_state(EngineState::Error);
            tracing::error!("NetworkEngine stop failed: {}", e);
            return Err(e);
        }

        self.set_state(EngineState::Stopped);
        tracing::info!("NetworkEngine stopped (mock)");
        Ok(())
    }

    /// Restart: stop then start.
    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    // ----- internals -----

    async fn start_subsystems(&self) -> AppResult<()> {
        self.context.tunnel_manager.start().await?;
        self.context.dns_manager.start().await?;
        self.context.proxy_manager.start().await?;
        self.context.connection_manager.start().await?;
        Ok(())
    }

    async fn stop_subsystems(&self) -> AppResult<()> {
        // Reverse order of start.
        self.context.connection_manager.stop().await?;
        self.context.proxy_manager.stop().await?;
        self.context.dns_manager.stop().await?;
        self.context.tunnel_manager.stop().await?;
        Ok(())
    }

    /// Update state and publish the change to backend subscribers + frontend.
    fn set_state(&self, next: EngineState) {
        self.state.set(next);
        self.context
            .runtime
            .publish(AppEvent::EngineStateChanged(next));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::NetworkContext;
    use crate::runtime::RuntimeContext;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn start_then_stop_drives_the_state_machine() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let net = Arc::new(NetworkContext::new(ctx));
        let engine = NetworkEngine::new(net);

        assert_eq!(engine.status(), EngineState::Stopped);

        engine.start().await?;
        assert_eq!(engine.status(), EngineState::Running);

        // Idempotent: starting again while running is a no-op.
        engine.start().await?;
        assert_eq!(engine.status(), EngineState::Running);

        engine.stop().await?;
        assert_eq!(engine.status(), EngineState::Stopped);
        Ok(())
    }
}
