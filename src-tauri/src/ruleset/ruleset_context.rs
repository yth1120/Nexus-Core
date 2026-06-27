use std::sync::Arc;

use crate::runtime::RuntimeContext;

use super::ruleset_cache::RuleSetCache;

pub struct RuleSetContext {
    pub runtime: Arc<RuntimeContext>,
    pub cache: Arc<RuleSetCache>,
}

impl RuleSetContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            runtime,
            cache: Arc::new(RuleSetCache::new()),
        }
    }
}
