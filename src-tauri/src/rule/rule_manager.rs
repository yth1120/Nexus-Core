use std::sync::Arc;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::models::Rule;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

/// Manages routing rules for the network core.
///
/// Phase 3 loads rules from the in-memory `AppState` cache into a compiled-rule
/// cache placeholder. `compile` is intentionally a stub — real rule compilation
/// (building a matcher) arrives in Phase 4.
pub struct RuleManager {
    context: Arc<RuntimeContext>,
    // Placeholder for the compiled rule set. Phase 4 replaces `Vec<Rule>` with a
    // compiled matcher structure.
    compiled: RwLock<Vec<Rule>>,
}

impl RuleManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            compiled: RwLock::new(Vec::new()),
        }
    }

    /// Load rules from the `AppState` cache into the rule cache.
    pub async fn load(&self) -> AppResult<()> {
        let rules = self.context.app_state().rules.read().clone();
        let count = rules.len();
        *self.compiled.write() = rules;
        tracing::info!("RuleManager loaded {} rules", count);
        Ok(())
    }

    /// Reload rules and publish `RuleReloaded`.
    pub async fn reload(&self) -> AppResult<()> {
        self.load().await?;
        let count = self.compiled.read().len();
        self.context.publish(AppEvent::RuleReloaded { count });
        tracing::info!("RuleManager reloaded {} rules", count);
        Ok(())
    }

    /// Compile the loaded rules into an optimized matcher.
    ///
    /// Intentionally left as a stub in Phase 3 — no compilation is performed.
    /// Phase 4 builds the real matcher here.
    pub fn compile(&self) -> AppResult<()> {
        // Stub: rule compilation is implemented in Phase 4.
        Ok(())
    }

    /// Number of currently-loaded rules in the cache.
    pub fn loaded_count(&self) -> usize {
        self.compiled.read().len()
    }
}
