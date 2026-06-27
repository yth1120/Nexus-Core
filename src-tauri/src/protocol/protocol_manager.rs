use std::sync::Arc;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::protocol_context::ProtocolContext;
use super::protocol_state::{ProtocolState, ProtocolStateCell};

/// Manages the protocol-layer lifecycle: inbound adapters, outbound adapters,
/// and the protocol adapter.
pub struct ProtocolManager {
    context: Arc<ProtocolContext>,
    state: ProtocolStateCell,
}

impl ProtocolManager {
    pub fn new(context: Arc<ProtocolContext>) -> Self {
        Self {
            context,
            state: ProtocolStateCell::new(),
        }
    }

    pub fn status(&self) -> ProtocolState {
        self.state.get()
    }

    /// Count of known inbound protocol variants (3: Http, Socks5, Mixed).
    pub fn inbound_count(&self) -> usize {
        3
    }

    /// Count of known outbound protocol variants (3: Direct, Proxy, Reject).
    pub fn outbound_count(&self) -> usize {
        3
    }

    pub async fn initialize(&self) -> AppResult<()> {
        let adapter = self.context.protocol_adapter.read().clone();
        adapter.initialize().await
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(ProtocolState::Starting);

        let adapter = self.context.protocol_adapter.read().clone();
        if let Err(e) = adapter.initialize().await {
            self.set_state(ProtocolState::Error);
            return Err(e);
        }

        let inbound = self.context.inbound_adapter.read().clone();
        if let Err(e) = inbound.start().await {
            self.set_state(ProtocolState::Error);
            return Err(e);
        }

        self.context.runtime.publish(AppEvent::InboundStarted {
            kind: "mixed".into(),
        });
        self.set_state(ProtocolState::Running);
        self.context.runtime.publish(AppEvent::ProtocolStarted);
        tracing::info!("ProtocolManager running (mock)");
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), ProtocolState::Stopped) {
            return Ok(());
        }
        self.set_state(ProtocolState::Stopping);

        let inbound = self.context.inbound_adapter.read().clone();
        inbound.stop().await?;

        self.context.runtime.publish(AppEvent::InboundStopped {
            kind: "mixed".into(),
        });
        self.set_state(ProtocolState::Stopped);
        self.context.runtime.publish(AppEvent::ProtocolStopped);
        tracing::info!("ProtocolManager stopped (mock)");
        Ok(())
    }

    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    fn set_state(&self, next: ProtocolState) {
        self.state.set(next);
        self.context
            .runtime
            .publish(AppEvent::ProtocolStateChanged(next));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn start_then_stop_drives_state_machine() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let pctx = ProtocolContext::new_for_test(ctx)?;
        let mgr = ProtocolManager::new(pctx);

        assert_eq!(mgr.status(), ProtocolState::Stopped);
        mgr.start().await?;
        assert_eq!(mgr.status(), ProtocolState::Running);
        mgr.stop().await?;
        assert_eq!(mgr.status(), ProtocolState::Stopped);
        Ok(())
    }
}
