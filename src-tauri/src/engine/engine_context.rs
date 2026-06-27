use std::sync::Arc;

use crate::config::ConfigManager;
use crate::core::AppState;
use crate::event::EventBus;
use crate::runtime::RuntimeContext;

/// Shared context passed to every engine and engine-layer component.
///
/// All fields are derived from [`RuntimeContext`] at construction time.
/// No `lazy_static!`, global singleton, or `unsafe`.
pub struct EngineContext {
    pub runtime_context: Arc<RuntimeContext>,
    pub app_state: Arc<AppState>,
    pub event_bus: Arc<EventBus>,
    pub config_manager: Arc<ConfigManager>,
}

impl EngineContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        let app_state = runtime.app_state().clone();
        let event_bus = runtime.event_bus().clone();
        let config_manager = runtime.resource_manager().config_manager.clone();
        Self {
            runtime_context: runtime,
            app_state,
            event_bus,
            config_manager,
        }
    }
}

#[cfg(test)]
impl EngineContext {
    pub(crate) fn new_for_test(runtime: Arc<RuntimeContext>) -> crate::utils::AppResult<Arc<Self>> {
        Ok(Arc::new(Self::new(runtime)))
    }
}
