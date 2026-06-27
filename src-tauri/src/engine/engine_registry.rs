#![allow(clippy::default_constructed_unit_structs)]

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::engine_trait::{Engine, EngineType};

/// Registry of all registered engine backends, keyed by [`EngineType`].
pub struct EngineRegistry {
    engines: RwLock<HashMap<EngineType, Arc<dyn Engine>>>,
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, engine: Arc<dyn Engine>) {
        let key = engine.engine_type();
        self.engines.write().insert(key, engine);
    }

    pub fn unregister(&self, engine_type: &EngineType) -> bool {
        self.engines.write().remove(engine_type).is_some()
    }

    pub fn contains(&self, engine_type: &EngineType) -> bool {
        self.engines.read().contains_key(engine_type)
    }

    pub fn get(&self, engine_type: &EngineType) -> Option<Arc<dyn Engine>> {
        self.engines.read().get(engine_type).cloned()
    }

    pub fn list(&self) -> Vec<EngineType> {
        self.engines.read().keys().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.engines.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for EngineRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::native::NativeEngine;
    use crate::engine::singbox::SingBoxEngine;

    #[test]
    fn register_list_get_unregister() {
        let reg = EngineRegistry::new();
        assert_eq!(reg.len(), 0);

        let native: Arc<dyn Engine> = Arc::new(NativeEngine::default());
        let singbox: Arc<dyn Engine> = Arc::new(SingBoxEngine::default());
        reg.register(native);
        reg.register(singbox);
        assert_eq!(reg.len(), 2);
        assert!(reg.contains(&EngineType::Native));
        assert!(reg.contains(&EngineType::SingBox));
        assert!(!reg.contains(&EngineType::Mihomo));

        let list = reg.list();
        assert_eq!(list.len(), 2);

        let got = reg.get(&EngineType::Native);
        assert!(got.is_some());

        assert!(reg.unregister(&EngineType::Native));
        assert!(!reg.contains(&EngineType::Native));
        assert_eq!(reg.len(), 1);
    }
}
