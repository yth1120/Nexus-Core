use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::RwLock;
use tokio::time::interval;

use crate::event::AppEvent;
use crate::monitoring::ConnectionMonitor;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::connection_state::ActiveConnection;

/// Name of the background task on the shared [`TaskManager`].
const TASK_ID: &str = "connection-monitor";

/// Maximum number of concurrent active connections.
/// Rejects new connections above this threshold to prevent memory exhaustion.
const MAX_CONNECTIONS: usize = 10_000;

/// Manages active-connection tracking.
pub struct ConnectionManager {
    context: Arc<RuntimeContext>,
    running: RwLock<bool>,
    connections: RwLock<HashMap<String, Arc<ActiveConnection>>>,
}

impl ConnectionManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            running: RwLock::new(false),
            connections: RwLock::new(HashMap::new()),
        }
    }

    // ----- lifecycle (Phase 3 compat) -----

    pub async fn start(&self) -> AppResult<()> {
        if *self.running.read() {
            return Ok(());
        }
        let platform = self.context.resource_manager().platform_manager.clone();
        let event_bus = self.context.resource_manager().event_bus.clone();
        self.context
            .task_manager()
            .spawn(TASK_ID, move |flag: Arc<AtomicBool>| async move {
                let monitor = ConnectionMonitor::new(platform, event_bus);
                let mut ticker = interval(Duration::from_secs(1));
                while !flag.load(Ordering::SeqCst) {
                    ticker.tick().await;
                    monitor.tick();
                }
            });
        *self.running.write() = true;
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        let _ = self.context.task_manager().stop(TASK_ID);
        *self.running.write() = false;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        *self.running.read()
    }

    pub fn active_count(&self) -> usize {
        self.context.app_state().connections.read().len()
    }

    pub fn active_packet_count(&self) -> u64 {
        self.context
            .get_pipeline_manager()
            .map(|pm| pm.statistics_snapshot().0)
            .unwrap_or(0)
    }

    // ----- Phase 7: proxy connection tracking -----

    pub fn add_connection(
        &self,
        protocol: &str,
        source: &str,
        destination: &str,
    ) -> Option<Arc<ActiveConnection>> {
        let current = self.connections.read().len();
        if current >= MAX_CONNECTIONS {
            tracing::warn!(
                "Connection limit reached ({}/{}), rejecting new connection",
                current,
                MAX_CONNECTIONS
            );
            return None;
        }
        let id = uuid::Uuid::new_v4().to_string();
        let conn = Arc::new(ActiveConnection::new(&id, protocol, source, destination));
        self.connections.write().insert(id.clone(), conn.clone());
        self.context.publish(AppEvent::ConnectionCreated {
            id: id.clone(),
            protocol: protocol.to_string(),
        });
        Some(conn)
    }

    pub fn remove_connection(&self, id: &str) -> Option<Arc<ActiveConnection>> {
        let conn = self.connections.write().remove(id);
        if let Some(ref c) = conn {
            c.close();
            self.context.publish(AppEvent::ConnectionClosed {
                id: c.id.clone(),
                duration: c.duration_ms.load(Ordering::Relaxed),
                bytes_in: c.bytes_in.load(Ordering::Relaxed),
                bytes_out: c.bytes_out.load(Ordering::Relaxed),
            });
        }
        conn
    }

    pub fn find_connection(&self, id: &str) -> Option<Arc<ActiveConnection>> {
        self.connections.read().get(id).cloned()
    }

    pub fn active_connections(&self) -> Vec<Arc<ActiveConnection>> {
        self.connections
            .read()
            .values()
            .filter(|c| *c.state.read() != super::connection_state::ConnectionState::Closed)
            .cloned()
            .collect()
    }

    pub fn connection_count(&self) -> usize {
        self.active_connections().len()
    }
}
