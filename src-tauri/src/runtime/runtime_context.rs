use std::sync::Arc;

use crate::core::app_state::AppState;
use crate::core::resource_manager::ResourceManager;
use crate::core::task_manager::TaskManager;
use crate::core_installer::{InstallerManager, UpdateManager, VersionManager};
use crate::dispatcher::Dispatcher;
use crate::dns::DnsManager;
use crate::engine::EngineManager;
use crate::event::{create_emitter, create_noop_emitter, AppEvent, BackendEmitter, EventBus};
use crate::geo::GeoManager;
use crate::pipeline::PipelineManager;
use crate::protocol::ProtocolManager;
use crate::proxy::ProxyManager;
use crate::release::AppUpdater;
use crate::rule_engine::RuleEngineManager;
use crate::ruleset::RuleSetManager;
use crate::subscription::SubscriptionManager;
use crate::telemetry::TelemetryRecorder;
use crate::transport::TransportManager;
use crate::tun::{RouteManager, TunManager};

use super::shutdown_token::ShutdownToken;

/// Unified dependency-injection container for all network-core managers.
///
/// Every manager is constructed with an `Arc<RuntimeContext>` and pulls
/// its shared dependencies from here — no `lazy_static!`, global singleton, or
/// `unsafe` (spec §13). The context wraps the already-initialized
/// [`ResourceManager`] and derives the `event_bus` / `task_manager` handles from
/// it (both are cheap, all-`Arc`-inside clones that share the underlying
/// channels).
///
/// `Clone` is explicit and cheap — all fields are `Arc`.
///
/// ## Known limitation (v1.0)
///
/// Each manager registered via `set_*_manager()` creates a reference cycle:
/// `RuntimeContext → RwLock<Option<Arc<Manager>>> → Manager → XxxContext → Arc<RuntimeContext>`.
/// This prevents `RuntimeContext` from being deallocated until process exit.
/// In a desktop app this is a benign leak (OS reclaims all memory on exit),
/// but it means no subsystem-level cleanup occurs on shutdown beyond what
/// `CoreManager::shutdown()` explicitly orchestrates. Future refactors
/// should convert `XxxContext` fields to `Weak<RuntimeContext>` (see §B5).
#[derive(Clone)]
pub struct RuntimeContext {
    app_state: Arc<AppState>,
    event_bus: Arc<EventBus>,
    resource_manager: Arc<ResourceManager>,
    task_manager: Arc<TaskManager>,
    pub protocol_manager: Arc<parking_lot::RwLock<Option<Arc<ProtocolManager>>>>,
    pub transport_manager: Arc<parking_lot::RwLock<Option<Arc<TransportManager>>>>,
    pub dispatcher: Arc<parking_lot::RwLock<Option<Arc<Dispatcher>>>>,
    pub engine_manager: Arc<parking_lot::RwLock<Option<Arc<EngineManager>>>>,
    pub pipeline_manager: Arc<parking_lot::RwLock<Option<Arc<PipelineManager>>>>,
    pub proxy_manager: Arc<parking_lot::RwLock<Option<Arc<ProxyManager>>>>,
    pub tun_manager: Arc<parking_lot::RwLock<Option<Arc<TunManager>>>>,
    pub route_manager: Arc<parking_lot::RwLock<Option<Arc<RouteManager>>>>,
    pub dns_manager: Arc<parking_lot::RwLock<Option<Arc<DnsManager>>>>,
    pub rule_engine: Arc<parking_lot::RwLock<Option<Arc<RuleEngineManager>>>>,
    pub subscription_manager: Arc<parking_lot::RwLock<Option<Arc<SubscriptionManager>>>>,
    pub ruleset_manager: Arc<parking_lot::RwLock<Option<Arc<RuleSetManager>>>>,
    pub installer_manager: Arc<parking_lot::RwLock<Option<Arc<InstallerManager>>>>,
    pub version_manager: Arc<parking_lot::RwLock<Option<Arc<VersionManager>>>>,
    pub update_manager: Arc<parking_lot::RwLock<Option<Arc<UpdateManager>>>>,
    pub geo_manager: Arc<parking_lot::RwLock<Option<Arc<GeoManager>>>>,
    pub telemetry_recorder: Arc<parking_lot::RwLock<Option<Arc<TelemetryRecorder>>>>,
    pub app_updater: Arc<parking_lot::RwLock<Option<Arc<AppUpdater>>>>,
    pub emitter: Arc<dyn BackendEmitter>,
    pub shutdown_token: Arc<ShutdownToken>,
}

impl RuntimeContext {
    /// Build a context from the central [`AppState`] and the initialized
    /// [`ResourceManager`]. `event_bus` and `task_manager` are derived from the
    /// resource manager so every subsystem shares the same channels.
    /// If `app_handle` is provided, production `TauriEmitter` is used;
    /// otherwise a no-op emitter for tests.
    pub fn new(
        app_state: Arc<AppState>,
        resource_manager: Arc<ResourceManager>,
        app_handle: Option<tauri::AppHandle>,
    ) -> Self {
        let event_bus = Arc::new(resource_manager.event_bus.clone());
        let task_manager = resource_manager.task_manager.clone();
        let emitter: Arc<dyn BackendEmitter> = match app_handle {
            Some(h) => create_emitter(h),
            None => create_noop_emitter(),
        };
        Self {
            app_state,
            event_bus,
            resource_manager,
            task_manager,
            protocol_manager: Arc::new(parking_lot::RwLock::new(None)),
            transport_manager: Arc::new(parking_lot::RwLock::new(None)),
            dispatcher: Arc::new(parking_lot::RwLock::new(None)),
            engine_manager: Arc::new(parking_lot::RwLock::new(None)),
            pipeline_manager: Arc::new(parking_lot::RwLock::new(None)),
            proxy_manager: Arc::new(parking_lot::RwLock::new(None)),
            tun_manager: Arc::new(parking_lot::RwLock::new(None)),
            route_manager: Arc::new(parking_lot::RwLock::new(None)),
            dns_manager: Arc::new(parking_lot::RwLock::new(None)),
            rule_engine: Arc::new(parking_lot::RwLock::new(None)),
            subscription_manager: Arc::new(parking_lot::RwLock::new(None)),
            ruleset_manager: Arc::new(parking_lot::RwLock::new(None)),
            installer_manager: Arc::new(parking_lot::RwLock::new(None)),
            version_manager: Arc::new(parking_lot::RwLock::new(None)),
            update_manager: Arc::new(parking_lot::RwLock::new(None)),
            geo_manager: Arc::new(parking_lot::RwLock::new(None)),
            telemetry_recorder: Arc::new(parking_lot::RwLock::new(None)),
            app_updater: Arc::new(parking_lot::RwLock::new(None)),
            emitter,
            shutdown_token: Arc::new(ShutdownToken::new()),
        }
    }

    /// The central application state container.
    pub fn app_state(&self) -> &Arc<AppState> {
        &self.app_state
    }

    /// The shared event bus (backend broadcast + frontend emit).
    pub fn event_bus(&self) -> &Arc<EventBus> {
        &self.event_bus
    }

    /// The backend resource orchestrator (config / db / platform / repositories).
    pub fn resource_manager(&self) -> &Arc<ResourceManager> {
        &self.resource_manager
    }

    /// The named background-task lifecycle manager.
    pub fn task_manager(&self) -> &Arc<TaskManager> {
        &self.task_manager
    }

    /// Convenience: publish an event onto the shared bus.
    pub fn publish(&self, event: AppEvent) {
        self.event_bus.publish(event);
    }

    // ----- Phase 4 manager accessors (two-phase init) -----

    pub fn set_protocol_manager(&self, pm: Arc<ProtocolManager>) {
        *self.protocol_manager.write() = Some(pm);
    }

    pub fn get_protocol_manager(&self) -> Option<Arc<ProtocolManager>> {
        self.protocol_manager.read().clone()
    }

    pub fn set_transport_manager(&self, tm: Arc<TransportManager>) {
        *self.transport_manager.write() = Some(tm);
    }

    pub fn get_transport_manager(&self) -> Option<Arc<TransportManager>> {
        self.transport_manager.read().clone()
    }

    pub fn set_dispatcher(&self, d: Arc<Dispatcher>) {
        *self.dispatcher.write() = Some(d);
    }

    pub fn get_dispatcher(&self) -> Option<Arc<Dispatcher>> {
        self.dispatcher.read().clone()
    }

    // ----- Phase 5 engine layer accessors -----

    pub fn set_engine_manager(&self, em: Arc<EngineManager>) {
        *self.engine_manager.write() = Some(em);
    }

    pub fn get_engine_manager(&self) -> Option<Arc<EngineManager>> {
        self.engine_manager.read().clone()
    }

    // ----- Phase 6 pipeline layer accessors -----

    pub fn set_pipeline_manager(&self, pm: Arc<PipelineManager>) {
        *self.pipeline_manager.write() = Some(pm);
    }

    pub fn get_pipeline_manager(&self) -> Option<Arc<PipelineManager>> {
        self.pipeline_manager.read().clone()
    }

    // ----- Phase 7 proxy layer accessors -----

    pub fn set_proxy_manager(&self, pm: Arc<ProxyManager>) {
        *self.proxy_manager.write() = Some(pm);
    }

    pub fn get_proxy_manager(&self) -> Option<Arc<ProxyManager>> {
        self.proxy_manager.read().clone()
    }

    pub fn set_tun_manager(&self, tm: Arc<TunManager>) {
        *self.tun_manager.write() = Some(tm);
    }
    pub fn get_tun_manager(&self) -> Option<Arc<TunManager>> {
        self.tun_manager.read().clone()
    }

    pub fn set_route_manager(&self, rm: Arc<RouteManager>) {
        *self.route_manager.write() = Some(rm);
    }
    pub fn get_route_manager(&self) -> Option<Arc<RouteManager>> {
        self.route_manager.read().clone()
    }

    pub fn set_dns_manager(&self, dm: Arc<DnsManager>) {
        *self.dns_manager.write() = Some(dm);
    }
    pub fn get_dns_manager(&self) -> Option<Arc<DnsManager>> {
        self.dns_manager.read().clone()
    }

    pub fn set_rule_engine(&self, re: Arc<RuleEngineManager>) {
        *self.rule_engine.write() = Some(re);
    }
    pub fn get_rule_engine(&self) -> Option<Arc<RuleEngineManager>> {
        self.rule_engine.read().clone()
    }
    pub fn set_subscription_manager(&self, sm: Arc<SubscriptionManager>) {
        *self.subscription_manager.write() = Some(sm);
    }
    pub fn get_subscription_manager(&self) -> Option<Arc<SubscriptionManager>> {
        self.subscription_manager.read().clone()
    }
    pub fn set_ruleset_manager(&self, rm: Arc<RuleSetManager>) {
        *self.ruleset_manager.write() = Some(rm);
    }
    pub fn get_ruleset_manager(&self) -> Option<Arc<RuleSetManager>> {
        self.ruleset_manager.read().clone()
    }

    // ----- Phase 13: core installer layer accessors -----

    pub fn set_installer_manager(&self, im: Arc<InstallerManager>) {
        *self.installer_manager.write() = Some(im);
    }

    pub fn get_installer_manager(&self) -> Option<Arc<InstallerManager>> {
        self.installer_manager.read().clone()
    }

    pub fn set_version_manager(&self, vm: Arc<VersionManager>) {
        *self.version_manager.write() = Some(vm);
    }

    pub fn get_version_manager(&self) -> Option<Arc<VersionManager>> {
        self.version_manager.read().clone()
    }

    pub fn set_update_manager(&self, um: Arc<UpdateManager>) {
        *self.update_manager.write() = Some(um);
    }

    pub fn get_update_manager(&self) -> Option<Arc<UpdateManager>> {
        self.update_manager.read().clone()
    }

    // ----- Phase 14: geo layer accessors -----

    pub fn set_geo_manager(&self, gm: Arc<GeoManager>) {
        *self.geo_manager.write() = Some(gm);
    }

    pub fn get_geo_manager(&self) -> Option<Arc<GeoManager>> {
        self.geo_manager.read().clone()
    }

    // ----- Phase 15: telemetry & update accessors -----

    pub fn set_telemetry_recorder(&self, tr: Arc<TelemetryRecorder>) {
        *self.telemetry_recorder.write() = Some(tr);
    }

    pub fn get_telemetry_recorder(&self) -> Option<Arc<TelemetryRecorder>> {
        self.telemetry_recorder.read().clone()
    }

    pub fn set_app_updater(&self, au: Arc<AppUpdater>) {
        *self.app_updater.write() = Some(au);
    }

    pub fn get_app_updater(&self) -> Option<Arc<AppUpdater>> {
        self.app_updater.read().clone()
    }
}

#[cfg(test)]
impl RuntimeContext {
    /// Build a context backed by an in-memory test [`ResourceManager`] and a
    /// fresh [`AppState`], for unit tests.
    pub(crate) fn new_for_test() -> crate::utils::AppResult<Arc<Self>> {
        let app_state = Arc::new(AppState::new());
        let resource_manager = Arc::new(ResourceManager::new_for_test()?);
        Ok(Arc::new(Self::new(app_state, resource_manager, None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_derives_handles_from_resource_manager() -> crate::utils::AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;

        // The task_manager handle is the very same Arc the ResourceManager owns.
        assert!(Arc::ptr_eq(
            ctx.task_manager(),
            &ctx.resource_manager().task_manager
        ));

        // Publishing without an AppHandle is a harmless no-op (no subscribers).
        ctx.publish(AppEvent::CoreStarted);
        Ok(())
    }
}
