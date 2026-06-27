use std::sync::Arc;

use parking_lot::RwLock;
use rand::Rng;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};

/// Tracks the current node selection for the network core.
///
/// Phase 3 stores the selected node id and returns mock latency. No real
/// connection to any node is established and no latency probe is sent.
pub struct NodeManager {
    context: Arc<RuntimeContext>,
    current_node_id: RwLock<Option<String>>,
}

impl NodeManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            current_node_id: RwLock::new(None),
        }
    }

    /// Set the current node by id. Publishes `NodeChanged`.
    pub fn set_current(&self, id: &str) -> AppResult<()> {
        if !self.node_exists(id) {
            return Err(AppError::NotFound(format!("Node {id}")));
        }

        *self.current_node_id.write() = Some(id.to_string());
        self.context.publish(AppEvent::NodeChanged {
            node_id: id.to_string(),
        });
        tracing::info!("Current node set: {}", id);
        Ok(())
    }

    /// The currently-selected node id, if any.
    pub fn current(&self) -> Option<String> {
        self.current_node_id.read().clone()
    }

    /// Mock latency test — returns a random value without any real connection.
    pub async fn test_latency(&self, id: &str) -> AppResult<u32> {
        if !self.node_exists(id) {
            return Err(AppError::NotFound(format!("Node {id}")));
        }
        let latency = rand::thread_rng().gen_range(5..350);
        tracing::debug!("Mock latency for node {}: {}ms", id, latency);
        Ok(latency)
    }

    fn node_exists(&self, id: &str) -> bool {
        self.context
            .app_state()
            .nodes
            .read()
            .iter()
            .any(|n| n.id == id)
    }
}
