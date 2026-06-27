#![allow(clippy::default_constructed_unit_structs)]

use std::sync::Arc;

use crate::utils::AppResult;

use super::engine_trait::{Engine, EngineType};
use super::mihomo::MihomoEngine;
use super::native::NativeEngine;
use super::plugin::PluginEngine;
use super::singbox::SingBoxEngine;
use super::xray::XrayEngine;

/// Central constructor for all engine backends.
///
/// `create` returns a mock engine for every variant. `destroy` is a no-op
/// until Phase 6 wires real external-process management.
pub struct EngineFactory;

impl EngineFactory {
    pub fn create(engine_type: &EngineType) -> AppResult<Arc<dyn Engine>> {
        match engine_type {
            EngineType::Native => Ok(Arc::new(NativeEngine::default())),
            EngineType::SingBox => Ok(Arc::new(SingBoxEngine::default())),
            EngineType::Mihomo => Ok(Arc::new(MihomoEngine::default())),
            EngineType::Xray => Ok(Arc::new(XrayEngine::default())),
            EngineType::Plugin(name) => Ok(Arc::new(PluginEngine::new(name.clone()))),
        }
    }

    pub fn destroy(_engine: Arc<dyn Engine>) -> AppResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_each_variant_returns_correct_type() -> AppResult<()> {
        for et in &[
            EngineType::Native,
            EngineType::SingBox,
            EngineType::Mihomo,
            EngineType::Xray,
            EngineType::Plugin("test".into()),
        ] {
            let e = EngineFactory::create(et)?;
            assert_eq!(e.engine_type(), *et);
            assert!(!e.capabilities().is_empty());
            assert!(!e.version().is_empty());
        }
        Ok(())
    }
}
