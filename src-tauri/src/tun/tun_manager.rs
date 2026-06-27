use std::sync::Arc;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::tun_context::TunContext;
use super::tun_state::{TunState, TunStateCell};

pub struct TunManager {
    context: Arc<TunContext>,
    state: TunStateCell,
}

impl TunManager {
    pub fn new(context: Arc<TunContext>) -> Self {
        Self {
            context,
            state: TunStateCell::new(),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        tracing::info!("TunManager initialized");
        Ok(())
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(TunState::Starting);
        self.set_state(TunState::Running);
        self.context.runtime.publish(AppEvent::TunStarted);
        tracing::info!("TunManager running");
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), TunState::Stopped) {
            return Ok(());
        }
        self.set_state(TunState::Stopping);
        self.set_state(TunState::Stopped);
        self.context.runtime.publish(AppEvent::TunStopped);
        tracing::info!("TunManager stopped");
        Ok(())
    }

    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    pub fn status(&self) -> TunState {
        self.state.get()
    }

    pub fn device(&self) -> Arc<dyn super::tun_device::TunDevice> {
        self.context.device.read().clone()
    }

    fn set_state(&self, next: TunState) {
        self.state.set(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn lifecycle_start_stop() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let ctx = super::super::tun_context::TunContext::new_for_test(rt)?;
        let mgr = TunManager::new(ctx);
        assert_eq!(mgr.status(), TunState::Stopped);
        mgr.start().await?;
        assert_eq!(mgr.status(), TunState::Running);
        mgr.stop().await?;
        assert_eq!(mgr.status(), TunState::Stopped);
        Ok(())
    }
}
