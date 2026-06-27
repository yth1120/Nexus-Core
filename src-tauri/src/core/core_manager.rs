use std::sync::Arc;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::event::AppEvent;
use crate::network::{EngineState, NetworkContext, NetworkEngine};
use crate::node::NodeManager;
use crate::profile::ProfileManager;
use crate::rule::RuleManager;
use crate::runtime::RuntimeContext;
use crate::session::{Session, SessionManager};
use crate::utils::AppResult;

/// Proxy routing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CoreMode {
    Rule,
    Global,
    Direct,
}

/// Traffic capture mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TrafficMode {
    SystemProxy,
    Tun,
    Hybrid,
}

/// Serializable snapshot of the core's current state.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreStatus {
    pub engine: EngineState,
    pub session: Option<Session>,
    pub active_profile: Option<String>,
    pub current_node: Option<String>,
    pub mode: CoreMode,
}

/// Top-level orchestrator for all network subsystems (spec §3).
///
/// Owns the manager hierarchy and drives the high-level lifecycle:
/// `Frontend → IPC → CoreManager → { Session, Profile, Node, Rule } → NetworkEngine`.
/// Every operation is mock in Phase 3 — no real connection is established and no
/// traffic flows.
///
/// ## Design note (v1.0)
///
/// CoreManager currently serves as both lifecycle orchestrator and service
/// locator — all IPC commands route through it, and it exposes accessors for
/// every subsystem (engine, proxy, DNS, geo, etc.). This creates a "God Object"
/// with 16+ responsibility layers. It is functional and correct for v1.0, but
/// future iterations should split it into domain-specific facades with IPC
/// handlers calling managers directly rather than through this single entry
/// point (see §A2).
pub struct CoreManager {
    context: Arc<RuntimeContext>,
    session_manager: Arc<SessionManager>,
    profile_manager: Arc<ProfileManager>,
    node_manager: Arc<NodeManager>,
    rule_manager: Arc<RuleManager>,
    network_engine: Arc<NetworkEngine>,
    mode: RwLock<CoreMode>,
}

impl CoreManager {
    /// Build the entire manager tree from the shared runtime context.
    /// Construction is synchronous and infallible.
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        let session_manager = Arc::new(SessionManager::new(context.clone()));
        let profile_manager = Arc::new(ProfileManager::new(context.clone()));
        let node_manager = Arc::new(NodeManager::new(context.clone()));
        let rule_manager = Arc::new(RuleManager::new(context.clone()));

        let network_context = Arc::new(NetworkContext::new(context.clone()));
        let network_engine = Arc::new(NetworkEngine::new(network_context));

        Self {
            context,
            session_manager,
            profile_manager,
            node_manager,
            rule_manager,
            network_engine,
            mode: RwLock::new(CoreMode::Rule),
        }
    }

    // ----- top-level lifecycle -----

    /// Start the core: load + compile rules, load profiles, then start the
    /// engine. Publishes `CoreStarted`.
    pub async fn start(&self) -> AppResult<()> {
        tracing::info!("CoreManager starting...");
        self.rule_manager.load().await?;
        self.rule_manager.compile()?;
        let _ = self.profile_manager.load().await?;
        self.network_engine.start().await?;
        self.context.publish(AppEvent::CoreStarted);
        tracing::info!("CoreManager started");
        Ok(())
    }

    /// Stop the core: stop the engine and destroy the session.
    /// Publishes `CoreStopped`.
    pub async fn stop(&self) -> AppResult<()> {
        tracing::info!("CoreManager stopping...");
        self.network_engine.stop().await?;
        self.session_manager.destroy()?;
        self.context.publish(AppEvent::CoreStopped);
        tracing::info!("CoreManager stopped");
        Ok(())
    }

    /// Graceful shutdown of all subsystems.
    ///
    /// Order: engine → session → protocol/transport → TUN → DNS →
    /// proxy → cancel shutdown token → stop all background tasks.
    /// After this call, only process exit can release remaining resources
    /// (periodic geo/update loops and the telemetry thread will stop at OS
    /// cleanup — documented limitation for v1.0).
    pub async fn shutdown(&self) -> AppResult<()> {
        tracing::info!("CoreManager shutdown initiated...");

        // 1. Stop network engine + destroy session
        let _ = self.network_engine.stop().await;
        let _ = self.session_manager.destroy();

        // 2. Stop protocol + transport layers
        let _ = self.stop_protocol_layer().await;

        // 3. Stop TUN + DNS
        let _ = self.stop_tun().await;
        let _ = self.stop_dns().await;

        // 4. Stop proxy adapters (HTTP + SOCKS5)
        if let Some(ref pm) = self.context.get_proxy_manager() {
            let _ = pm.stop_http().await;
            let _ = pm.stop_socks5().await;
        }

        // 5. Cancel shutdown token — propagates to all child CancellationTokens
        self.context.shutdown_token.cancel();
        tracing::info!("ShutdownToken cancelled");

        // 6. Stop all registered background tasks
        self.context.task_manager().shutdown_all();
        tracing::info!("All background tasks stopped");

        // 7. Publish final event
        self.context.publish(AppEvent::CoreStopped);
        tracing::info!("CoreManager shutdown complete");
        Ok(())
    }

    /// Restart the core (stop then start).
    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    // ----- profile connection flow -----

    /// Connect a profile (and optional node): activate profile → select node →
    /// create session → start engine. Returns the created [`Session`].
    pub async fn connect_profile(
        &self,
        profile_id: &str,
        node_id: Option<String>,
    ) -> AppResult<Session> {
        tracing::info!("Connecting profile: {}", profile_id);
        self.profile_manager.activate(profile_id).await?;
        if let Some(ref nid) = node_id {
            self.node_manager.set_current(nid)?;
        }
        let session = self
            .session_manager
            .create(profile_id.to_string(), node_id)?;
        self.network_engine.start().await?;
        Ok(session)
    }

    /// Disconnect the active profile: stop engine → destroy session →
    /// deactivate profile.
    pub async fn disconnect_profile(&self) -> AppResult<()> {
        tracing::info!("Disconnecting profile");
        self.network_engine.stop().await?;
        self.session_manager.destroy()?;
        self.profile_manager.deactivate().await?;
        Ok(())
    }

    // ----- configuration -----

    /// Set the proxy routing mode (mock — stored only).
    pub fn set_mode(&self, mode: CoreMode) -> AppResult<()> {
        *self.mode.write() = mode;
        tracing::info!("Core mode set: {:?}", mode);
        Ok(())
    }

    /// The current routing mode.
    pub fn mode(&self) -> CoreMode {
        *self.mode.read()
    }

    /// Reload configuration: reload rules and profiles (mock hot-reload).
    pub async fn reload_config(&self) -> AppResult<()> {
        tracing::info!("CoreManager reloading config...");
        self.rule_manager.reload().await?;
        self.profile_manager.reload().await?;
        Ok(())
    }

    // ----- status -----

    /// A serializable snapshot of the current core state.
    pub fn status(&self) -> CoreStatus {
        CoreStatus {
            engine: self.network_engine.status(),
            session: self.session_manager.current(),
            active_profile: self.profile_manager.active(),
            current_node: self.node_manager.current(),
            mode: self.mode(),
        }
    }

    /// The current session, if one is active (used by `get_current_session`).
    pub fn current_session(&self) -> Option<Session> {
        self.session_manager.current()
    }

    // ----- Phase 4: protocol + transport layer lifecycle -----

    /// Start the protocol and transport layers.
    pub async fn start_protocol_layer(&self) -> AppResult<()> {
        if let Some(ref pm) = self.context.get_protocol_manager() {
            pm.start().await?;
        }
        if let Some(ref tm) = self.context.get_transport_manager() {
            tm.start().await?;
        }
        tracing::info!("Protocol + transport layers started");
        Ok(())
    }

    /// Stop the protocol and transport layers (reverse order).
    pub async fn stop_protocol_layer(&self) -> AppResult<()> {
        if let Some(ref tm) = self.context.get_transport_manager() {
            tm.stop().await?;
        }
        if let Some(ref pm) = self.context.get_protocol_manager() {
            pm.stop().await?;
        }
        tracing::info!("Protocol + transport layers stopped");
        Ok(())
    }

    // ----- Phase 5: engine layer accessors -----

    /// Get the engine manager from the runtime context.
    pub fn engine_manager(&self) -> Option<Arc<crate::engine::EngineManager>> {
        self.context.get_engine_manager()
    }

    /// Get the pipeline manager from the runtime context.
    pub fn pipeline_manager(&self) -> Option<Arc<crate::pipeline::PipelineManager>> {
        self.context.get_pipeline_manager()
    }

    /// Get the proxy manager from the runtime context.
    pub fn proxy_manager(&self) -> Option<Arc<crate::proxy::ProxyManager>> {
        self.context.get_proxy_manager()
    }

    pub fn tun_manager(&self) -> Option<Arc<crate::tun::TunManager>> {
        self.context.get_tun_manager()
    }

    pub fn route_manager(&self) -> Option<Arc<crate::tun::RouteManager>> {
        self.context.get_route_manager()
    }

    pub async fn start_tun(&self) -> AppResult<()> {
        if let Some(ref tm) = self.context.get_tun_manager() {
            tm.start().await?;
        }
        Ok(())
    }

    pub async fn stop_tun(&self) -> AppResult<()> {
        if let Some(ref tm) = self.context.get_tun_manager() {
            tm.stop().await?;
        }
        Ok(())
    }

    pub async fn start_dns(&self) -> AppResult<()> {
        if let Some(ref dm) = self.context.get_dns_manager() {
            dm.start().await?;
        }
        Ok(())
    }
    pub async fn stop_dns(&self) -> AppResult<()> {
        if let Some(ref dm) = self.context.get_dns_manager() {
            dm.stop().await?;
        }
        Ok(())
    }
    pub async fn reload_rules(&self) -> AppResult<()> {
        if let Some(ref re) = self.context.get_rule_engine() {
            re.reload().await?;
        }
        Ok(())
    }
    pub fn dns_manager(&self) -> Option<Arc<crate::dns::DnsManager>> {
        self.context.get_dns_manager()
    }
    pub fn rule_engine(&self) -> Option<Arc<crate::rule_engine::RuleEngineManager>> {
        self.context.get_rule_engine()
    }
    pub fn subscription_manager(&self) -> Option<Arc<crate::subscription::SubscriptionManager>> {
        self.context.get_subscription_manager()
    }
    pub fn ruleset_manager(&self) -> Option<Arc<crate::ruleset::RuleSetManager>> {
        self.context.get_ruleset_manager()
    }

    // ----- Phase 13: core installer accessors -----

    pub fn installer_manager(&self) -> Option<Arc<crate::core_installer::InstallerManager>> {
        self.context.get_installer_manager()
    }
    pub fn version_manager(&self) -> Option<Arc<crate::core_installer::VersionManager>> {
        self.context.get_version_manager()
    }
    pub fn update_manager(&self) -> Option<Arc<crate::core_installer::UpdateManager>> {
        self.context.get_update_manager()
    }

    // ----- Phase 14: geo accessor -----

    pub fn geo_manager(&self) -> Option<Arc<crate::geo::GeoManager>> {
        self.context.get_geo_manager()
    }

    // ----- Phase 15: telemetry & update accessors -----

    pub fn telemetry_recorder(&self) -> Option<Arc<crate::telemetry::TelemetryRecorder>> {
        self.context.get_telemetry_recorder()
    }

    pub fn app_updater(&self) -> Option<Arc<crate::release::AppUpdater>> {
        self.context.get_app_updater()
    }

    pub async fn update_subscriptions(&self) -> AppResult<()> {
        if let Some(ref sm) = self.context.get_subscription_manager() {
            sm.update_all().await?;
        }
        Ok(())
    }
    pub async fn reload_rulesets(&self) -> AppResult<()> {
        if let Some(ref rm) = self.context.get_ruleset_manager() {
            rm.reload_all().await?;
        }
        Ok(())
    }

    pub fn set_traffic_mode(&self, mode: TrafficMode) -> AppResult<()> {
        self.context.publish(AppEvent::TrafficMode {
            mode: format!("{:?}", mode),
        });
        Ok(())
    }

    /// Access the shared runtime context (for IPC handlers).
    pub fn context(&self) -> &Arc<RuntimeContext> {
        &self.context
    }
}
