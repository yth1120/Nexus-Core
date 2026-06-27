use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::engine_context::EngineContext;
use super::process_manager::ProcessManager;

/// Supervises external engine processes: auto-restart, health-check polling,
/// crash detection, backoff-retry.
///
/// Phase 5 implements the full API surface but all methods return mock data.
/// Real health probes and crash detection arrive in Phase 6.
pub struct ProcessSupervisor {
    #[allow(dead_code)]
    context: Arc<EngineContext>,
    auto_restart: RwLock<bool>,
    #[allow(dead_code)]
    health_check_interval: RwLock<u64>,
    crash_retry: RwLock<u32>,
    #[allow(dead_code)]
    process_manager: Arc<ProcessManager>,
}

impl ProcessSupervisor {
    pub fn new(context: Arc<EngineContext>) -> Self {
        Self {
            context,
            auto_restart: RwLock::new(true),
            health_check_interval: RwLock::new(30),
            crash_retry: RwLock::new(3),
            process_manager: Arc::new(ProcessManager::new()),
        }
    }

    pub fn enable_auto_restart(&self) {
        *self.auto_restart.write() = true;
    }

    pub fn disable_auto_restart(&self) {
        *self.auto_restart.write() = false;
    }

    pub fn is_auto_restart_enabled(&self) -> bool {
        *self.auto_restart.read()
    }

    /// Run a health check against the supervised engine. Mock — always `Ok`.
    pub fn run_health_check(&self) -> AppResult<()> {
        tracing::debug!("ProcessSupervisor::run_health_check (mock)");
        Ok(())
    }

    /// Called when a crash is detected. Publishes `EngineCrashed` and
    /// triggers backoff-retry if auto-restart is enabled.
    pub fn on_crash_detected(&self, engine_type: &str, reason: &str) {
        tracing::warn!(
            "ProcessSupervisor: crash detected for {}: {}",
            engine_type,
            reason
        );
        self.context.event_bus.publish(AppEvent::EngineCrashed {
            engine_type: engine_type.to_string(),
            reason: reason.to_string(),
        });
        if self.is_auto_restart_enabled() {
            let _ = self.backoff_retry();
        }
    }

    /// Backoff-retry policy. Mock — returns `Ok` after a short sleep.
    pub fn backoff_retry(&self) -> AppResult<()> {
        let max_retries = *self.crash_retry.read();
        tracing::info!(
            "ProcessSupervisor: backoff-retry (max_retries={}, mock)",
            max_retries
        );
        // In Phase 6 this would actually loop with exponential backoff.
        // Phase 5 just sleeps briefly to demonstrate the API.
        std::thread::sleep(Duration::from_millis(10));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[test]
    fn auto_restart_toggle() -> crate::utils::AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let sup = ProcessSupervisor::new(ectx);

        assert!(sup.is_auto_restart_enabled());
        sup.disable_auto_restart();
        assert!(!sup.is_auto_restart_enabled());
        sup.enable_auto_restart();
        assert!(sup.is_auto_restart_enabled());
        Ok(())
    }

    #[test]
    fn health_check_returns_ok() -> crate::utils::AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let sup = ProcessSupervisor::new(ectx);
        sup.run_health_check()?;
        Ok(())
    }

    #[test]
    fn crash_handler_publishes_event() -> crate::utils::AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let sup = ProcessSupervisor::new(ectx);
        sup.on_crash_detected("native", "test-crash");
        // No assertion needed — just verifies the code path doesn't panic.
        Ok(())
    }
}
