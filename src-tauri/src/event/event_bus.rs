use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

use crate::models::{Connection, DashboardRunStatus, DashboardStatus, LogEntry};
use crate::network::network_state::EngineState;
use crate::protocol::protocol_state::ProtocolState;
use crate::transport::transport_state::TransportState;

/// Application events emitted to the frontend and backend subscribers.
/// Serialized as `{ "event": "traffic:update", "data": { ... } }`
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum AppEvent {
    // --- Existing Phase 1 events (unchanged) ---
    #[serde(rename = "traffic:update")]
    TrafficUpdate {
        upload: f64,
        download: f64,
        timestamp: i64,
    },

    #[serde(rename = "log:new")]
    LogNew(LogEntry),

    #[serde(rename = "connection:new")]
    ConnectionNew(Connection),

    #[serde(rename = "connection:close")]
    ConnectionClose { id: String },

    #[serde(rename = "status:change")]
    StatusChange(DashboardRunStatus),

    #[serde(rename = "profile:activate")]
    ProfileActivate {
        #[serde(rename = "profileId")]
        profile_id: String,
    },

    #[serde(rename = "theme:change")]
    ThemeChange(String),

    // --- New Phase 2 events ---
    #[serde(rename = "config:changed")]
    ConfigChanged { path: String },

    #[serde(rename = "dashboard:update")]
    DashboardUpdate(DashboardStatus),

    #[serde(rename = "statistics:update")]
    StatisticsUpdate {
        cpu_usage: f64,
        memory_usage: f64,
        uptime: i64,
        active_connections: i64,
    },

    #[serde(rename = "connection:update")]
    ConnectionUpdate { connections: Vec<Connection> },

    // --- New Phase 3 events (network core lifecycle) ---
    #[serde(rename = "core:started")]
    CoreStarted,

    #[serde(rename = "core:stopped")]
    CoreStopped,

    #[serde(rename = "session:created")]
    SessionCreated {
        id: String,
        #[serde(rename = "profileId")]
        profile_id: String,
        #[serde(rename = "nodeId")]
        node_id: Option<String>,
    },

    #[serde(rename = "session:destroyed")]
    SessionDestroyed { id: String },

    #[serde(rename = "profile:activated")]
    ProfileActivated {
        #[serde(rename = "profileId")]
        profile_id: String,
    },

    #[serde(rename = "profile:deactivated")]
    ProfileDeactivated {
        #[serde(rename = "profileId")]
        profile_id: String,
    },

    #[serde(rename = "node:changed")]
    NodeChanged {
        #[serde(rename = "nodeId")]
        node_id: String,
    },

    #[serde(rename = "rule:reloaded")]
    RuleReloaded { count: usize },

    #[serde(rename = "engine:state")]
    EngineStateChanged(EngineState),

    // --- New Phase 4 events (protocol + transport layer lifecycle) ---
    #[serde(rename = "protocol:started")]
    ProtocolStarted,

    #[serde(rename = "protocol:stopped")]
    ProtocolStopped,

    #[serde(rename = "transport:started")]
    TransportStarted,

    #[serde(rename = "transport:stopped")]
    TransportStopped,

    #[serde(rename = "inbound:started")]
    InboundStarted { kind: String },

    #[serde(rename = "inbound:stopped")]
    InboundStopped { kind: String },

    #[serde(rename = "outbound:connected")]
    OutboundConnected { route: String },

    #[serde(rename = "outbound:disconnected")]
    OutboundDisconnected { route: String },

    #[serde(rename = "connection:dispatched")]
    ConnectionDispatched {
        #[serde(rename = "connectionId")]
        connection_id: String,
        route: String,
    },

    #[serde(rename = "protocol:state")]
    ProtocolStateChanged(ProtocolState),

    #[serde(rename = "transport:state")]
    TransportStateChanged(TransportState),

    // --- New Phase 5 events (pluggable engine lifecycle) ---
    #[serde(rename = "engine:registered")]
    EngineRegistered {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:unregistered")]
    EngineUnregistered {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:switched")]
    EngineSwitched { from: String, to: String },

    #[serde(rename = "engine:starting")]
    EngineStarting {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:started")]
    EngineStarted {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:stopping")]
    EngineStopping {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:stopped")]
    EngineStopped {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:restarting")]
    EngineRestarting {
        #[serde(rename = "engineType")]
        engine_type: String,
    },

    #[serde(rename = "engine:health")]
    EngineHealth {
        #[serde(rename = "engineType")]
        engine_type: String,
        healthy: bool,
    },

    #[serde(rename = "engine:crashed")]
    EngineCrashed {
        #[serde(rename = "engineType")]
        engine_type: String,
        reason: String,
    },

    // --- New Phase 6 events (packet pipeline) ---
    #[serde(rename = "packet:received")]
    PacketReceived {
        #[serde(rename = "connectionId")]
        connection_id: String,
        size: u64,
    },

    #[serde(rename = "packet:processed")]
    PacketProcessed {
        #[serde(rename = "connectionId")]
        connection_id: String,
        size: u64,
    },

    #[serde(rename = "packet:sent")]
    PacketSent {
        #[serde(rename = "connectionId")]
        connection_id: String,
        size: u64,
    },

    #[serde(rename = "pipeline:started")]
    PipelineStarted,

    #[serde(rename = "pipeline:stopped")]
    PipelineStopped,

    // --- New Phase 7 events (proxy layer) ---
    #[serde(rename = "proxy:started")]
    ProxyStarted { kind: String },

    #[serde(rename = "proxy:stopped")]
    ProxyStopped { kind: String },

    #[serde(rename = "connection:created")]
    ConnectionCreated { id: String, protocol: String },

    #[serde(rename = "connection:closed")]
    ConnectionClosed {
        id: String,
        duration: u64,
        #[serde(rename = "bytesIn")]
        bytes_in: u64,
        #[serde(rename = "bytesOut")]
        bytes_out: u64,
    },

    // --- New Phase 8 events ---
    #[serde(rename = "tun:started")]
    TunStarted,

    #[serde(rename = "tun:stopped")]
    TunStopped,

    #[serde(rename = "tun:error")]
    TunError { reason: String },

    #[serde(rename = "route:created")]
    RouteCreated { dest: String },

    #[serde(rename = "route:deleted")]
    RouteDeleted { dest: String },

    #[serde(rename = "traffic:mode")]
    TrafficMode { mode: String },

    // --- New Phase 9 events ---
    #[serde(rename = "dns:started")]
    DnsStarted,
    #[serde(rename = "dns:stopped")]
    DnsStopped,
    #[serde(rename = "dns:resolved")]
    DnsResolved { domain: String, ips: Vec<String> },
    #[serde(rename = "dns:cache")]
    DnsCacheFlushed,
    #[serde(rename = "rule:compiled")]
    RuleCompiled { count: usize },
    #[serde(rename = "rule:matched")]
    RuleMatched { domain: String, result: String },

    // Phase 11
    #[serde(rename = "subscription:added")]
    SubscriptionAdded { id: String, url: String },
    #[serde(rename = "subscription:removed")]
    SubscriptionRemoved { id: String },
    #[serde(rename = "subscription:updated")]
    SubscriptionUpdated { id: String },
    #[serde(rename = "subscription:failed")]
    SubscriptionFailed { id: String, error: String },
    #[serde(rename = "ruleset:downloaded")]
    RuleSetDownloaded { id: String },
    #[serde(rename = "ruleset:compiled")]
    RuleSetCompiled { id: String, count: usize },
    #[serde(rename = "ruleset:reloaded")]
    RuleSetReloaded { count: usize },
    #[serde(rename = "profile:synced")]
    ProfileSynced {
        #[serde(rename = "profileId")]
        profile_id: String,
    },
    #[serde(rename = "backup:created")]
    BackupCreated { path: String },
    #[serde(rename = "backup:restored")]
    BackupRestored,
    #[serde(rename = "diagnostics:generated")]
    DiagnosticsGenerated { path: String },
    #[serde(rename = "session:recovered")]
    SessionRecovered { profile: String },
    #[serde(rename = "crash:detected")]
    CrashDetected { reason: String },

    // Phase 13
    #[serde(rename = "core:download_started")]
    CoreDownloadStarted { core: String, version: String },
    #[serde(rename = "core:download_progress")]
    CoreDownloadProgress { core: String, percent: u32 },
    #[serde(rename = "core:download_finished")]
    CoreDownloadFinished { core: String },
    #[serde(rename = "core:download_failed")]
    CoreDownloadFailed { core: String, error: String },
    #[serde(rename = "core:install_started")]
    CoreInstallStarted { core: String, version: String },
    #[serde(rename = "core:install_finished")]
    CoreInstallFinished { core: String },
    #[serde(rename = "core:update_available")]
    CoreUpdateAvailable { core: String, version: String },
    #[serde(rename = "core:updated")]
    CoreUpdated {
        core: String,
        from: String,
        to: String,
    },
    #[serde(rename = "core:rollback")]
    CoreRollback {
        core: String,
        from: String,
        to: String,
    },

    // Phase 14: GeoIP / GeoSite
    #[serde(rename = "geo:loaded")]
    GeoLoaded {
        geoip_version: String,
        geosite_version: String,
    },
    #[serde(rename = "geo:updated")]
    GeoUpdated { database: String, version: String },
    #[serde(rename = "geo:failed")]
    GeoFailed { database: String, error: String },
    #[serde(rename = "geo:matched")]
    GeoMatched {
        rule_type: String,
        payload: String,
        host: String,
    },

    // Phase 15: Release Engineering & Performance
    #[serde(rename = "update:available")]
    UpdateAvailable { version: String, notes: String },
    #[serde(rename = "update:downloaded")]
    UpdateDownloaded { version: String },
    #[serde(rename = "update:applied")]
    UpdateApplied { version: String },
    #[serde(rename = "update:failed")]
    UpdateFailed { version: String, error: String },
    #[serde(rename = "benchmark:completed")]
    BenchmarkCompleted,
    #[serde(rename = "stress:completed")]
    StressTestCompleted,
    #[serde(rename = "security:audit")]
    SecurityAuditCompleted,
}

/// Central event bus with broadcast channel and sticky event cache.
///
/// - Frontend receives events via Tauri `app_handle.emit()`
/// - Backend components subscribe via `subscribe()` / `subscribe_sticky()`
/// - Sticky events cache the last value for late subscribers
#[derive(Debug, Clone)]
pub struct EventBus {
    tx: broadcast::Sender<AppEvent>,
    sticky_cache: Arc<RwLock<HashMap<&'static str, AppEvent>>>,
    app_handle: Arc<RwLock<Option<AppHandle>>>,
}

impl EventBus {
    /// Create a new EventBus with the given channel capacity.
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self {
            tx,
            sticky_cache: Arc::new(RwLock::new(HashMap::new())),
            app_handle: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the Tauri AppHandle for frontend event emission.
    pub fn set_app_handle(&self, handle: AppHandle) {
        *self.app_handle.write() = Some(handle);
    }

    /// Publish an event to all backend subscribers and the frontend.
    ///
    /// Sticky events are cached so new subscribers receive the last value.
    /// Which events are sticky is determined by their variant — currently
    /// `DashboardUpdate`, `StatisticsUpdate`, and `StatusChange` are sticky.
    pub fn publish(&self, event: AppEvent) {
        // Cache sticky events
        if is_sticky(&event) {
            let key = event_name(&event);
            self.sticky_cache.write().insert(key, event.clone());
        }

        // Broadcast to backend subscribers (ignore lagged subscribers)
        let _ = self.tx.send(event.clone());

        // Emit to frontend via Tauri
        if let Some(ref handle) = *self.app_handle.read() {
            emit_to_frontend(handle, &event);
        }
    }

    /// Subscribe to all events. Returns a broadcast Receiver.
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.tx.subscribe()
    }

    /// Subscribe and also receive the last sticky value for each event type.
    pub fn subscribe_sticky(&self) -> (broadcast::Receiver<AppEvent>, Vec<AppEvent>) {
        let rx = self.tx.subscribe();
        let sticky: Vec<AppEvent> = self.sticky_cache.read().values().cloned().collect();
        (rx, sticky)
    }

    /// Get the last published value for a sticky event, if any.
    pub fn last(&self, event_name: &str) -> Option<AppEvent> {
        self.sticky_cache.read().get(event_name).cloned()
    }
}

/// Determine if an event is sticky (new subscribers should get the last value).
fn is_sticky(event: &AppEvent) -> bool {
    matches!(
        event,
        AppEvent::DashboardUpdate(_)
            | AppEvent::StatisticsUpdate { .. }
            | AppEvent::StatusChange(_)
            | AppEvent::ConfigChanged { .. }
            | AppEvent::EngineStateChanged(_)
            | AppEvent::ProtocolStateChanged(_)
            | AppEvent::TransportStateChanged(_)
    )
}

/// Extract the event name string from an AppEvent variant.
fn event_name(event: &AppEvent) -> &'static str {
    match event {
        AppEvent::TrafficUpdate { .. } => "traffic:update",
        AppEvent::LogNew(_) => "log:new",
        AppEvent::ConnectionNew(_) => "connection:new",
        AppEvent::ConnectionClose { .. } => "connection:close",
        AppEvent::StatusChange(_) => "status:change",
        AppEvent::ProfileActivate { .. } => "profile:activate",
        AppEvent::ThemeChange(_) => "theme:change",
        AppEvent::ConfigChanged { .. } => "config:changed",
        AppEvent::DashboardUpdate(_) => "dashboard:update",
        AppEvent::StatisticsUpdate { .. } => "statistics:update",
        AppEvent::ConnectionUpdate { .. } => "connection:update",
        AppEvent::CoreStarted => "core:started",
        AppEvent::CoreStopped => "core:stopped",
        AppEvent::SessionCreated { .. } => "session:created",
        AppEvent::SessionDestroyed { .. } => "session:destroyed",
        AppEvent::ProfileActivated { .. } => "profile:activated",
        AppEvent::ProfileDeactivated { .. } => "profile:deactivated",
        AppEvent::NodeChanged { .. } => "node:changed",
        AppEvent::RuleReloaded { .. } => "rule:reloaded",
        AppEvent::EngineStateChanged(_) => "engine:state",
        AppEvent::ProtocolStarted => "protocol:started",
        AppEvent::ProtocolStopped => "protocol:stopped",
        AppEvent::TransportStarted => "transport:started",
        AppEvent::TransportStopped => "transport:stopped",
        AppEvent::InboundStarted { .. } => "inbound:started",
        AppEvent::InboundStopped { .. } => "inbound:stopped",
        AppEvent::OutboundConnected { .. } => "outbound:connected",
        AppEvent::OutboundDisconnected { .. } => "outbound:disconnected",
        AppEvent::ConnectionDispatched { .. } => "connection:dispatched",
        AppEvent::ProtocolStateChanged(_) => "protocol:state",
        AppEvent::TransportStateChanged(_) => "transport:state",
        AppEvent::EngineRegistered { .. } => "engine:registered",
        AppEvent::EngineUnregistered { .. } => "engine:unregistered",
        AppEvent::EngineSwitched { .. } => "engine:switched",
        AppEvent::EngineStarting { .. } => "engine:starting",
        AppEvent::EngineStarted { .. } => "engine:started",
        AppEvent::EngineStopping { .. } => "engine:stopping",
        AppEvent::EngineStopped { .. } => "engine:stopped",
        AppEvent::EngineRestarting { .. } => "engine:restarting",
        AppEvent::EngineHealth { .. } => "engine:health",
        AppEvent::EngineCrashed { .. } => "engine:crashed",
        AppEvent::PacketReceived { .. } => "packet:received",
        AppEvent::PacketProcessed { .. } => "packet:processed",
        AppEvent::PacketSent { .. } => "packet:sent",
        AppEvent::PipelineStarted => "pipeline:started",
        AppEvent::PipelineStopped => "pipeline:stopped",
        AppEvent::ProxyStarted { .. } => "proxy:started",
        AppEvent::ProxyStopped { .. } => "proxy:stopped",
        AppEvent::ConnectionCreated { .. } => "connection:created",
        AppEvent::ConnectionClosed { .. } => "connection:closed",
        AppEvent::TunStarted => "tun:started",
        AppEvent::TunStopped => "tun:stopped",
        AppEvent::TunError { .. } => "tun:error",
        AppEvent::RouteCreated { .. } => "route:created",
        AppEvent::RouteDeleted { .. } => "route:deleted",
        AppEvent::TrafficMode { .. } => "traffic:mode",
        AppEvent::DnsStarted => "dns:started",
        AppEvent::DnsStopped => "dns:stopped",
        AppEvent::DnsResolved { .. } => "dns:resolved",
        AppEvent::DnsCacheFlushed => "dns:cache",
        AppEvent::RuleCompiled { .. } => "rule:compiled",
        AppEvent::RuleMatched { .. } => "rule:matched",
        AppEvent::SubscriptionAdded { .. } => "subscription:added",
        AppEvent::SubscriptionRemoved { .. } => "subscription:removed",
        AppEvent::SubscriptionUpdated { .. } => "subscription:updated",
        AppEvent::SubscriptionFailed { .. } => "subscription:failed",
        AppEvent::RuleSetDownloaded { .. } => "ruleset:downloaded",
        AppEvent::RuleSetCompiled { .. } => "ruleset:compiled",
        AppEvent::RuleSetReloaded { .. } => "ruleset:reloaded",
        AppEvent::ProfileSynced { .. } => "profile:synced",
        AppEvent::BackupCreated { .. } => "backup:created",
        AppEvent::BackupRestored => "backup:restored",
        AppEvent::DiagnosticsGenerated { .. } => "diagnostics:generated",
        AppEvent::SessionRecovered { .. } => "session:recovered",
        AppEvent::CrashDetected { .. } => "crash:detected",
        AppEvent::CoreDownloadStarted { .. } => "core:download_started",
        AppEvent::CoreDownloadProgress { .. } => "core:download_progress",
        AppEvent::CoreDownloadFinished { .. } => "core:download_finished",
        AppEvent::CoreDownloadFailed { .. } => "core:download_failed",
        AppEvent::CoreInstallStarted { .. } => "core:install_started",
        AppEvent::CoreInstallFinished { .. } => "core:install_finished",
        AppEvent::CoreUpdateAvailable { .. } => "core:update_available",
        AppEvent::CoreUpdated { .. } => "core:updated",
        AppEvent::CoreRollback { .. } => "core:rollback",
        AppEvent::GeoLoaded { .. } => "geo:loaded",
        AppEvent::GeoUpdated { .. } => "geo:updated",
        AppEvent::GeoFailed { .. } => "geo:failed",
        AppEvent::GeoMatched { .. } => "geo:matched",
        AppEvent::UpdateAvailable { .. } => "update:available",
        AppEvent::UpdateDownloaded { .. } => "update:downloaded",
        AppEvent::UpdateApplied { .. } => "update:applied",
        AppEvent::UpdateFailed { .. } => "update:failed",
        AppEvent::BenchmarkCompleted => "benchmark:completed",
        AppEvent::StressTestCompleted => "stress:completed",
        AppEvent::SecurityAuditCompleted => "security:audit",
    }
}

/// Emit an event to the frontend via Tauri's event system.
pub(crate) fn emit_to_frontend(app_handle: &AppHandle, event: &AppEvent) {
    let (event_name, payload_json) = match event {
        AppEvent::TrafficUpdate {
            upload,
            download,
            timestamp,
        } => (
            "traffic:update",
            serde_json::json!({ "upload": upload, "download": download, "timestamp": timestamp }),
        ),
        AppEvent::LogNew(entry) => ("log:new", serde_json::to_value(entry).unwrap_or_default()),
        AppEvent::ConnectionNew(conn) => (
            "connection:new",
            serde_json::to_value(conn).unwrap_or_default(),
        ),
        AppEvent::ConnectionClose { id } => ("connection:close", serde_json::json!({ "id": id })),
        AppEvent::StatusChange(status) => (
            "status:change",
            serde_json::to_value(status).unwrap_or_default(),
        ),
        AppEvent::ProfileActivate { profile_id } => (
            "profile:activate",
            serde_json::json!({ "profileId": profile_id }),
        ),
        AppEvent::ThemeChange(theme) => ("theme:change", serde_json::json!(theme)),
        AppEvent::ConfigChanged { path } => ("config:changed", serde_json::json!({ "path": path })),
        AppEvent::DashboardUpdate(status) => (
            "dashboard:update",
            serde_json::to_value(status).unwrap_or_default(),
        ),
        AppEvent::StatisticsUpdate {
            cpu_usage,
            memory_usage,
            uptime,
            active_connections,
        } => (
            "statistics:update",
            serde_json::json!({
                "cpuUsage": cpu_usage,
                "memoryUsage": memory_usage,
                "uptime": uptime,
                "activeConnections": active_connections,
            }),
        ),
        AppEvent::ConnectionUpdate { connections } => (
            "connection:update",
            serde_json::json!({ "connections": connections }),
        ),
        AppEvent::CoreStarted => ("core:started", serde_json::Value::Null),
        AppEvent::CoreStopped => ("core:stopped", serde_json::Value::Null),
        AppEvent::SessionCreated {
            id,
            profile_id,
            node_id,
        } => (
            "session:created",
            serde_json::json!({ "id": id, "profileId": profile_id, "nodeId": node_id }),
        ),
        AppEvent::SessionDestroyed { id } => ("session:destroyed", serde_json::json!({ "id": id })),
        AppEvent::ProfileActivated { profile_id } => (
            "profile:activated",
            serde_json::json!({ "profileId": profile_id }),
        ),
        AppEvent::ProfileDeactivated { profile_id } => (
            "profile:deactivated",
            serde_json::json!({ "profileId": profile_id }),
        ),
        AppEvent::NodeChanged { node_id } => {
            ("node:changed", serde_json::json!({ "nodeId": node_id }))
        }
        AppEvent::RuleReloaded { count } => {
            ("rule:reloaded", serde_json::json!({ "count": count }))
        }
        AppEvent::EngineStateChanged(state) => (
            "engine:state",
            serde_json::to_value(state).unwrap_or_default(),
        ),
        AppEvent::ProtocolStarted => ("protocol:started", serde_json::Value::Null),
        AppEvent::ProtocolStopped => ("protocol:stopped", serde_json::Value::Null),
        AppEvent::TransportStarted => ("transport:started", serde_json::Value::Null),
        AppEvent::TransportStopped => ("transport:stopped", serde_json::Value::Null),
        AppEvent::InboundStarted { kind } => {
            ("inbound:started", serde_json::json!({ "kind": kind }))
        }
        AppEvent::InboundStopped { kind } => {
            ("inbound:stopped", serde_json::json!({ "kind": kind }))
        }
        AppEvent::OutboundConnected { route } => {
            ("outbound:connected", serde_json::json!({ "route": route }))
        }
        AppEvent::OutboundDisconnected { route } => (
            "outbound:disconnected",
            serde_json::json!({ "route": route }),
        ),
        AppEvent::ConnectionDispatched {
            connection_id,
            route,
        } => (
            "connection:dispatched",
            serde_json::json!({ "connectionId": connection_id, "route": route }),
        ),
        AppEvent::ProtocolStateChanged(state) => (
            "protocol:state",
            serde_json::to_value(state).unwrap_or_default(),
        ),
        AppEvent::TransportStateChanged(state) => (
            "transport:state",
            serde_json::to_value(state).unwrap_or_default(),
        ),
        AppEvent::EngineRegistered { engine_type } => (
            "engine:registered",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineUnregistered { engine_type } => (
            "engine:unregistered",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineSwitched { from, to } => (
            "engine:switched",
            serde_json::json!({ "from": from, "to": to }),
        ),
        AppEvent::EngineStarting { engine_type } => (
            "engine:starting",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineStarted { engine_type } => (
            "engine:started",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineStopping { engine_type } => (
            "engine:stopping",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineStopped { engine_type } => (
            "engine:stopped",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineRestarting { engine_type } => (
            "engine:restarting",
            serde_json::json!({ "engineType": engine_type }),
        ),
        AppEvent::EngineHealth {
            engine_type,
            healthy,
        } => (
            "engine:health",
            serde_json::json!({ "engineType": engine_type, "healthy": healthy }),
        ),
        AppEvent::EngineCrashed {
            engine_type,
            reason,
        } => (
            "engine:crashed",
            serde_json::json!({ "engineType": engine_type, "reason": reason }),
        ),
        AppEvent::PacketReceived {
            connection_id,
            size,
        } => (
            "packet:received",
            serde_json::json!({ "connectionId": connection_id, "size": size }),
        ),
        AppEvent::PacketProcessed {
            connection_id,
            size,
        } => (
            "packet:processed",
            serde_json::json!({ "connectionId": connection_id, "size": size }),
        ),
        AppEvent::PacketSent {
            connection_id,
            size,
        } => (
            "packet:sent",
            serde_json::json!({ "connectionId": connection_id, "size": size }),
        ),
        AppEvent::PipelineStarted => ("pipeline:started", serde_json::Value::Null),
        AppEvent::PipelineStopped => ("pipeline:stopped", serde_json::Value::Null),
        AppEvent::ProxyStarted { kind } => ("proxy:started", serde_json::json!({ "kind": kind })),
        AppEvent::ProxyStopped { kind } => ("proxy:stopped", serde_json::json!({ "kind": kind })),
        AppEvent::ConnectionCreated { id, protocol } => (
            "connection:created",
            serde_json::json!({ "id": id, "protocol": protocol }),
        ),
        AppEvent::ConnectionClosed {
            id,
            duration,
            bytes_in,
            bytes_out,
        } => (
            "connection:closed",
            serde_json::json!({ "id": id, "duration": duration, "bytesIn": bytes_in, "bytesOut": bytes_out }),
        ),
        AppEvent::TunStarted => ("tun:started", serde_json::Value::Null),
        AppEvent::TunStopped => ("tun:stopped", serde_json::Value::Null),
        AppEvent::TunError { reason } => ("tun:error", serde_json::json!({ "reason": reason })),
        AppEvent::RouteCreated { dest } => ("route:created", serde_json::json!({ "dest": dest })),
        AppEvent::RouteDeleted { dest } => ("route:deleted", serde_json::json!({ "dest": dest })),
        AppEvent::TrafficMode { mode } => ("traffic:mode", serde_json::json!({ "mode": mode })),
        AppEvent::DnsStarted => ("dns:started", serde_json::Value::Null),
        AppEvent::DnsStopped => ("dns:stopped", serde_json::Value::Null),
        AppEvent::DnsResolved { domain, ips } => (
            "dns:resolved",
            serde_json::json!({ "domain": domain, "ips": ips }),
        ),
        AppEvent::DnsCacheFlushed => ("dns:cache", serde_json::Value::Null),
        AppEvent::RuleCompiled { count } => {
            ("rule:compiled", serde_json::json!({ "count": count }))
        }
        AppEvent::RuleMatched { domain, result } => (
            "rule:matched",
            serde_json::json!({ "domain": domain, "result": result }),
        ),
        AppEvent::SubscriptionAdded { id, url } => (
            "subscription:added",
            serde_json::json!({ "id": id, "url": url }),
        ),
        AppEvent::SubscriptionRemoved { id } => {
            ("subscription:removed", serde_json::json!({ "id": id }))
        }
        AppEvent::SubscriptionUpdated { id } => {
            ("subscription:updated", serde_json::json!({ "id": id }))
        }
        AppEvent::SubscriptionFailed { id, error } => (
            "subscription:failed",
            serde_json::json!({ "id": id, "error": error }),
        ),
        AppEvent::RuleSetDownloaded { id } => {
            ("ruleset:downloaded", serde_json::json!({ "id": id }))
        }
        AppEvent::RuleSetCompiled { id, count } => (
            "ruleset:compiled",
            serde_json::json!({ "id": id, "count": count }),
        ),
        AppEvent::RuleSetReloaded { count } => {
            ("ruleset:reloaded", serde_json::json!({ "count": count }))
        }
        AppEvent::ProfileSynced { profile_id } => (
            "profile:synced",
            serde_json::json!({ "profileId": profile_id }),
        ),
        AppEvent::BackupCreated { path } => ("backup:created", serde_json::json!({ "path": path })),
        AppEvent::BackupRestored => ("backup:restored", serde_json::Value::Null),
        AppEvent::DiagnosticsGenerated { path } => {
            ("diagnostics:generated", serde_json::json!({ "path": path }))
        }
        AppEvent::SessionRecovered { profile } => (
            "session:recovered",
            serde_json::json!({ "profile": profile }),
        ),
        AppEvent::CrashDetected { reason } => {
            ("crash:detected", serde_json::json!({ "reason": reason }))
        }
        AppEvent::CoreDownloadStarted { core, version } => (
            "core:download_started",
            serde_json::json!({ "core": core, "version": version }),
        ),
        AppEvent::CoreDownloadProgress { core, percent } => (
            "core:download_progress",
            serde_json::json!({ "core": core, "percent": percent }),
        ),
        AppEvent::CoreDownloadFinished { core } => (
            "core:download_finished",
            serde_json::json!({ "core": core }),
        ),
        AppEvent::CoreDownloadFailed { core, error } => (
            "core:download_failed",
            serde_json::json!({ "core": core, "error": error }),
        ),
        AppEvent::CoreInstallStarted { core, version } => (
            "core:install_started",
            serde_json::json!({ "core": core, "version": version }),
        ),
        AppEvent::CoreInstallFinished { core } => {
            ("core:install_finished", serde_json::json!({ "core": core }))
        }
        AppEvent::CoreUpdateAvailable { core, version } => (
            "core:update_available",
            serde_json::json!({ "core": core, "version": version }),
        ),
        AppEvent::CoreUpdated { core, from, to } => (
            "core:updated",
            serde_json::json!({ "core": core, "from": from, "to": to }),
        ),
        AppEvent::CoreRollback { core, from, to } => (
            "core:rollback",
            serde_json::json!({ "core": core, "from": from, "to": to }),
        ),
        AppEvent::GeoLoaded {
            geoip_version,
            geosite_version,
        } => (
            "geo:loaded",
            serde_json::json!({ "geoipVersion": geoip_version, "geositeVersion": geosite_version }),
        ),
        AppEvent::GeoUpdated { database, version } => (
            "geo:updated",
            serde_json::json!({ "database": database, "version": version }),
        ),
        AppEvent::GeoFailed { database, error } => (
            "geo:failed",
            serde_json::json!({ "database": database, "error": error }),
        ),
        AppEvent::GeoMatched {
            rule_type,
            payload,
            host,
        } => (
            "geo:matched",
            serde_json::json!({ "ruleType": rule_type, "payload": payload, "host": host }),
        ),
        AppEvent::UpdateAvailable { version, notes } => (
            "update:available",
            serde_json::json!({ "version": version, "notes": notes }),
        ),
        AppEvent::UpdateDownloaded { version } => (
            "update:downloaded",
            serde_json::json!({ "version": version }),
        ),
        AppEvent::UpdateApplied { version } => {
            ("update:applied", serde_json::json!({ "version": version }))
        }
        AppEvent::UpdateFailed { version, error } => (
            "update:failed",
            serde_json::json!({ "version": version, "error": error }),
        ),
        AppEvent::BenchmarkCompleted => ("benchmark:completed", serde_json::json!({})),
        AppEvent::StressTestCompleted => ("stress:completed", serde_json::json!({})),
        AppEvent::SecurityAuditCompleted => ("security:audit", serde_json::json!({})),
    };

    if let Err(e) = app_handle.emit(event_name, payload_json) {
        tracing::warn!("Failed to emit event '{}': {}", event_name, e);
    }
}

// ===== Backward-compatible emit_event function =====

/// Legacy emit_event function for Phase 1 compatibility.
/// New code should use `EventBus::publish()` instead.
pub fn emit_event(app_handle: &AppHandle, event: AppEvent) {
    emit_to_frontend(app_handle, &event);
}
