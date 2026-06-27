use std::sync::Arc;

use crate::engine::engine_trait::Engine;
use crate::utils::{AppError, AppResult};

use super::super::engine_context::EngineContext;

/// Framework for future dynamic-library plugin loading.
///
/// Phase 5 is architecture only — `load()` returns `Unsupported` and
/// `list_available()` is empty. Real `.dll`/`.so`/`.dylib` loading arrives
/// in a future phase.
pub struct PluginLoader {
    #[allow(dead_code)]
    engine_context: Arc<EngineContext>,
}

impl PluginLoader {
    pub fn new(engine_context: Arc<EngineContext>) -> Self {
        Self { engine_context }
    }

    /// Load a plugin by name. Currently unsupported — returns an error.
    pub fn load(&self, name: &str) -> AppResult<Arc<dyn Engine>> {
        Err(AppError::Unsupported(format!(
            "plugin loading not implemented (name: {name})"
        )))
    }

    /// List potentially-available plugins (directories / files to try).
    pub fn list_available(&self) -> Vec<String> {
        Vec::new()
    }
}
