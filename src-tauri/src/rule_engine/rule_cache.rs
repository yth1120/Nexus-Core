use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use super::rule_compiler::CompiledRule;

pub struct RuleCache {
    rules: RwLock<HashMap<String, Arc<Vec<CompiledRule>>>>,
}

impl RuleCache {
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(HashMap::new()),
        }
    }

    pub fn load(&self, compiled: Vec<CompiledRule>) {
        let mut grouped: HashMap<String, Vec<CompiledRule>> = HashMap::new();
        for r in compiled {
            grouped.entry(r.rule_type.clone()).or_default().push(r);
        }
        let map: HashMap<String, Arc<Vec<CompiledRule>>> =
            grouped.into_iter().map(|(k, v)| (k, Arc::new(v))).collect();
        *self.rules.write() = map;
    }

    /// Return rules for `rule_type` as a cheap `Arc` clone — no per-call
    /// vector duplication.
    pub fn get(&self, rule_type: &str) -> Arc<Vec<CompiledRule>> {
        self.rules
            .read()
            .get(rule_type)
            .cloned()
            .unwrap_or_default()
    }

    pub fn all_types(&self) -> Vec<String> {
        self.rules.read().keys().cloned().collect()
    }

    pub fn clear(&self) {
        self.rules.write().clear();
    }
    pub fn len(&self) -> usize {
        self.rules.read().values().map(|v| v.len()).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for RuleCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_and_get() {
        let c = RuleCache::new();
        c.load(vec![CompiledRule::new(
            "DomainSuffix",
            ".com",
            super::super::rule_result::RuleResult::Proxy,
        )]);
        let rules = c.get("DomainSuffix");
        assert_eq!(rules.len(), 1);
        assert_eq!(c.len(), 1);
    }
}
