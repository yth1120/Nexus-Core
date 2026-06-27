use std::sync::Arc;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::utils::{AppError, AppResult};

use super::engine_context::EngineContext;
use super::engine_event::{
    publish_engine_lifecycle, publish_engine_registered, publish_engine_switched,
    publish_engine_unregistered,
};
use super::engine_factory::EngineFactory;
use super::engine_registry::EngineRegistry;
use super::engine_state::{EngineState, EngineStateCell};
use super::engine_trait::{Engine, EngineCapability, EngineType};

/// Orchestrates the pluggable engine layer: registry, current-engine selection,
/// hot-switch, and lifecycle delegation.
pub struct EngineManager {
    context: Arc<EngineContext>,
    registry: Arc<EngineRegistry>,
    current: RwLock<Option<Arc<dyn Engine>>>,
    state: EngineStateCell,
}

impl EngineManager {
    /// Build the manager and pre-register all four built-in engines via
    /// [`EngineFactory`]. Does NOT auto-start — lifecycle is driven via IPC.
    pub fn new(context: Arc<EngineContext>) -> Self {
        let registry = Arc::new(EngineRegistry::new());
        let builtins: &[EngineType] = &[
            EngineType::Native,
            EngineType::SingBox,
            EngineType::Mihomo,
            EngineType::Xray,
        ];
        for et in builtins {
            if let Ok(engine) = EngineFactory::create(et) {
                registry.register(engine);
            }
        }
        Self {
            context,
            registry,
            current: RwLock::new(None),
            state: EngineStateCell::new(),
        }
    }

    pub fn initialize(&self) -> AppResult<()> {
        tracing::info!(
            "EngineManager initialized ({} engines)",
            self.registry.len()
        );
        Ok(())
    }

    // ----- registry management -----

    pub fn register_engine(&self, engine: Arc<dyn Engine>) {
        let name = engine.engine_type().to_string();
        self.registry.register(engine);
        publish_engine_registered(&self.context.event_bus, &name);
    }

    pub fn unregister_engine(&self, engine_type: &EngineType) -> bool {
        let removed = self.registry.unregister(engine_type);
        if removed {
            publish_engine_unregistered(&self.context.event_bus, &engine_type.to_string());
        }
        removed
    }

    pub fn engine_list(&self) -> Vec<EngineType> {
        self.registry.list()
    }

    // ----- hot-switch -----

    /// Switch the currently-active engine: stop old → start new.
    pub async fn switch_engine(&self, engine_type: &EngineType) -> AppResult<()> {
        let new_engine = self
            .registry
            .get(engine_type)
            .ok_or_else(|| AppError::NotFound(format!("engine {engine_type} not registered")))?;

        // Clone the Arc out of the lock before any .await.
        let old_engine = self.current.write().take();
        if let Some(ref old) = old_engine {
            old.stop().await?;
        }
        new_engine.initialize().await?;
        new_engine.start().await?;
        *self.current.write() = Some(new_engine.clone());

        let from_str = old_engine
            .as_ref()
            .map(|e| e.engine_type().to_string())
            .unwrap_or_else(|| "none".to_string());
        publish_engine_switched(&self.context.event_bus, &from_str, &engine_type.to_string());
        self.state.set(EngineState::Running);
        tracing::info!("Engine switched: {} -> {}", from_str, engine_type);
        Ok(())
    }

    // ----- lifecycle delegation -----

    pub async fn start_current(&self) -> AppResult<()> {
        // Clone out of the lock before await.
        let engine = self.current.read().clone();
        let engine = engine.ok_or_else(|| AppError::Internal("no engine selected".into()))?;
        let name = engine.engine_type().to_string();
        publish_engine_lifecycle(
            &self.context.event_bus,
            AppEvent::EngineStarting {
                engine_type: name.clone(),
            },
        );
        engine.start().await?;
        publish_engine_lifecycle(
            &self.context.event_bus,
            AppEvent::EngineStarted { engine_type: name },
        );
        self.state.set(EngineState::Running);
        Ok(())
    }

    pub async fn stop_current(&self) -> AppResult<()> {
        let engine = self.current.read().clone();
        let engine = engine.ok_or_else(|| AppError::Internal("no engine selected".into()))?;
        let name = engine.engine_type().to_string();
        publish_engine_lifecycle(
            &self.context.event_bus,
            AppEvent::EngineStopping {
                engine_type: name.clone(),
            },
        );
        engine.stop().await?;
        publish_engine_lifecycle(
            &self.context.event_bus,
            AppEvent::EngineStopped { engine_type: name },
        );
        self.state.set(EngineState::Stopped);
        Ok(())
    }

    pub async fn restart_current(&self) -> AppResult<()> {
        let engine = self.current.read().clone();
        let engine = engine.ok_or_else(|| AppError::Internal("no engine selected".into()))?;
        let name = engine.engine_type().to_string();
        publish_engine_lifecycle(
            &self.context.event_bus,
            AppEvent::EngineRestarting { engine_type: name },
        );
        engine.restart().await?;
        Ok(())
    }

    pub async fn reload_current(&self) -> AppResult<()> {
        let engine = self.current.read().clone();
        let engine = engine.ok_or_else(|| AppError::Internal("no engine selected".into()))?;
        engine.reload_config().await
    }

    pub fn current_engine(&self) -> Option<Arc<dyn Engine>> {
        self.current.read().clone()
    }

    pub fn current_status(&self) -> EngineState {
        self.state.get()
    }

    pub fn capabilities(&self) -> Vec<EngineCapability> {
        self.current
            .read()
            .as_ref()
            .map(|e| e.capabilities())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn register_and_list_engines() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let mgr = EngineManager::new(ectx);

        let list = mgr.engine_list();
        assert_eq!(list.len(), 4);
        assert!(list.contains(&EngineType::Native));
        assert!(list.contains(&EngineType::SingBox));
        Ok(())
    }

    #[tokio::test]
    async fn switch_engine_and_lifecycle() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let mgr = EngineManager::new(ectx);

        mgr.switch_engine(&EngineType::Native).await?;
        assert!(mgr.current_engine().is_some());
        assert_eq!(
            mgr.current_engine().unwrap().engine_type(),
            EngineType::Native
        );

        mgr.switch_engine(&EngineType::SingBox).await?;
        assert_eq!(
            mgr.current_engine().unwrap().engine_type(),
            EngineType::SingBox
        );

        mgr.stop_current().await?;
        assert_eq!(mgr.current_status(), EngineState::Stopped);
        Ok(())
    }

    #[tokio::test]
    async fn capabilities_from_current_engine() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let ectx = EngineContext::new_for_test(ctx)?;
        let mgr = EngineManager::new(ectx);

        mgr.switch_engine(&EngineType::Native).await?;
        let caps = mgr.capabilities();
        assert!(caps.contains(&EngineCapability::Statistics));
        assert!(caps.contains(&EngineCapability::HotReload));
        Ok(())
    }
}
