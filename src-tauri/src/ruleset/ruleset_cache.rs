use parking_lot::RwLock;
use std::collections::HashMap;

use crate::rule_engine::CompiledRule;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleSetInfo {
    pub id: String,
    pub name: String,
    pub url: String,
    pub rule_count: usize,
}

pub struct RuleSetCache {
    entries: RwLock<HashMap<String, Vec<CompiledRule>>>,
}

impl RuleSetCache {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }
    pub fn insert(&self, id: &str, rules: Vec<CompiledRule>) {
        self.entries.write().insert(id.to_string(), rules);
    }
    pub fn get(&self, id: &str) -> Option<Vec<CompiledRule>> {
        self.entries.read().get(id).cloned()
    }
    pub fn remove(&self, id: &str) {
        self.entries.write().remove(id);
    }
    pub fn clear(&self) {
        self.entries.write().clear();
    }
    pub fn list(&self) -> Vec<RuleSetInfo> {
        self.entries
            .read()
            .iter()
            .map(|(k, v)| RuleSetInfo {
                id: k.clone(),
                name: k.clone(),
                url: String::new(),
                rule_count: v.len(),
            })
            .collect()
    }
}

impl Default for RuleSetCache {
    fn default() -> Self {
        Self::new()
    }
}
