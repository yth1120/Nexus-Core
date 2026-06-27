use std::sync::Arc;

use crate::event::AppEvent;
use crate::transport::listener::Listener;
use crate::transport::stream::TransportStream;
use crate::utils::AppResult;

use super::transport_context::TransportContext;
use super::transport_state::{TransportState, TransportStateCell};

/// Manages the transport-layer lifecycle: listeners and streams.
pub struct TransportManager {
    context: Arc<TransportContext>,
    state: TransportStateCell,
}

impl TransportManager {
    pub fn new(context: Arc<TransportContext>) -> Self {
        Self {
            context,
            state: TransportStateCell::new(),
        }
    }

    pub fn status(&self) -> TransportState {
        self.state.get()
    }

    pub fn listener_count(&self) -> usize {
        self.context.listeners.read().len()
    }

    pub fn stream_count(&self) -> usize {
        self.context.streams.read().len()
    }

    pub async fn initialize(&self) -> AppResult<()> {
        tracing::debug!("TransportManager::initialize (no-op)");
        Ok(())
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(TransportState::Starting);

        let listeners = self.context.listeners.read().clone();
        for listener in &listeners {
            listener.start().await?;
        }

        self.set_state(TransportState::Running);
        self.context.runtime.publish(AppEvent::TransportStarted);
        tracing::info!(
            "TransportManager running (mock, {} listeners)",
            listeners.len()
        );
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), TransportState::Stopped) {
            return Ok(());
        }
        self.set_state(TransportState::Stopping);

        let listeners = self.context.listeners.read().clone();
        for listener in &listeners {
            listener.stop().await?;
        }

        self.set_state(TransportState::Stopped);
        self.context.runtime.publish(AppEvent::TransportStopped);
        tracing::info!("TransportManager stopped (mock)");
        Ok(())
    }

    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    pub fn register_listener(&self, listener: Arc<dyn Listener>) {
        self.context.listeners.write().push(listener);
    }

    pub fn unregister_listener(&self, listener: Arc<dyn Listener>) {
        let kind = listener.kind();
        self.context.listeners.write().retain(|l| l.kind() != kind);
    }

    pub fn register_stream(&self, stream: Arc<dyn TransportStream>) {
        self.context.streams.write().push(stream);
    }

    pub fn unregister_stream<T: TransportStream + 'static>(&self, _stream: Arc<T>) {
        // In Phase 4, drop the last registered stream as a coarse approximation.
        // Phase 5 replaces this with identity-based removal.
        self.context.streams.write().pop();
    }

    fn set_state(&self, next: TransportState) {
        self.state.set(next);
        self.context
            .runtime
            .publish(AppEvent::TransportStateChanged(next));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    use crate::transport::listener::TcpListener;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn start_stop_drives_state_machine() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let tctx = TransportContext::new_for_test(ctx)?;
        let mgr = TransportManager::new(tctx);

        assert_eq!(mgr.status(), TransportState::Stopped);
        mgr.start().await?;
        assert_eq!(mgr.status(), TransportState::Running);
        mgr.stop().await?;
        assert_eq!(mgr.status(), TransportState::Stopped);
        Ok(())
    }

    #[test]
    fn register_unregister_listeners() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let tctx = TransportContext::new_for_test(ctx)?;
        let mgr = TransportManager::new(tctx);

        assert_eq!(mgr.listener_count(), 0);
        mgr.register_listener(Arc::new(TcpListener));
        assert_eq!(mgr.listener_count(), 1);
        mgr.unregister_listener(Arc::new(TcpListener));
        assert_eq!(mgr.listener_count(), 0);
        Ok(())
    }
}
