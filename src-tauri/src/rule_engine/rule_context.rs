use super::rule_cache::RuleCache;
use crate::runtime::RuntimeContext;
use std::sync::Arc;

pub struct RuleContext {
    pub runtime: Arc<RuntimeContext>,
    pub cache: Arc<RuleCache>,
}
impl RuleContext {
    pub fn new(runtime: Arc<RuntimeContext>, cache: Arc<RuleCache>) -> Self {
        Self { runtime, cache }
    }
}
