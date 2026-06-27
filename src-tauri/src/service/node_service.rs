use rand::Rng;

use crate::core::AppState;
use crate::models::{Node, NodeStatus};
use crate::utils::AppResult;

pub fn get_all(state: &AppState) -> Vec<Node> {
    state.nodes.read().clone()
}

pub fn toggle_favorite(state: &AppState, id: &str) -> AppResult<Node> {
    let mut nodes = state.nodes.write();
    let node = nodes
        .iter_mut()
        .find(|n| n.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Node {}", id)))?;
    node.is_favorite = !node.is_favorite;
    Ok(node.clone())
}

/// Result of a delay test.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDelayResult {
    pub delay: i64,
    pub loss: f64,
}

/// Test delay for a single node. Returns random latency values.
pub fn test_delay(state: &AppState, id: &str) -> AppResult<NodeDelayResult> {
    let mut rng = rand::thread_rng();
    let delay: i64 = rng.gen_range(5..350);
    let loss: f64 = rng.gen_range(0.0..5.0);

    // Update node in place
    let mut nodes = state.nodes.write();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
        node.delay = Some(delay);
        node.loss = Some(loss);
        node.status = if rng.gen::<f64>() > 0.1 {
            NodeStatus::Online
        } else {
            NodeStatus::Offline
        };
    }

    Ok(NodeDelayResult { delay, loss })
}

/// Test delay for all nodes.
pub fn test_all_delay(state: &AppState) {
    let mut rng = rand::thread_rng();
    let mut nodes = state.nodes.write();

    for node in nodes.iter_mut() {
        node.delay = Some(rng.gen_range(5..350));
        node.loss = Some(rng.gen_range(0.0..5.0));
        node.status = if rng.gen::<f64>() > 0.1 {
            NodeStatus::Online
        } else {
            NodeStatus::Offline
        };
    }
}

/// Connect to a node (mock — just sets is_connected).
pub fn connect(state: &AppState, id: &str) -> AppResult<Node> {
    let mut nodes = state.nodes.write();

    // Disconnect all others first
    for node in nodes.iter_mut() {
        node.is_connected = false;
    }

    let target = nodes
        .iter_mut()
        .find(|n| n.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Node {}", id)))?;

    target.is_connected = true;
    target.status = NodeStatus::Online;

    // Enable system proxy on connect (via ResourceManager)
    if let Some(rm) = state.get_resource_manager() {
        let port = state.get_config().mixed_port;
        let _ = rm.platform_manager.enable_system_proxy("127.0.0.1", port);
        rm.event_bus.publish(crate::event::AppEvent::StatusChange(
            crate::models::DashboardRunStatus::Running,
        ));
    }

    Ok(target.clone())
}

/// Disconnect from a node (mock).
pub fn disconnect(state: &AppState, id: &str) -> AppResult<Node> {
    let mut nodes = state.nodes.write();
    let node = nodes
        .iter_mut()
        .find(|n| n.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Node {}", id)))?;

    node.is_connected = false;

    // Disable system proxy on disconnect
    if let Some(rm) = state.get_resource_manager() {
        let _ = rm.platform_manager.disable_system_proxy();
        rm.event_bus.publish(crate::event::AppEvent::StatusChange(
            crate::models::DashboardRunStatus::Stopped,
        ));
    }

    Ok(node.clone())
}
