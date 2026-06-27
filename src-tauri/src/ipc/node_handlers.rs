use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::Node;
use crate::service::node_service::{self, NodeDelayResult};

#[tauri::command]
pub async fn get_nodes(state: State<'_, Arc<AppState>>) -> Result<Vec<Node>, String> {
    Ok(node_service::get_all(&state))
}

#[tauri::command]
pub async fn toggle_node_favorite(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<Node, String> {
    node_service::toggle_favorite(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_node_delay(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<NodeDelayResult, String> {
    node_service::test_delay(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_all_node_delay(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    node_service::test_all_delay(&state);
    Ok(())
}

#[tauri::command]
pub async fn connect_node(state: State<'_, Arc<AppState>>, id: String) -> Result<Node, String> {
    node_service::connect(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn disconnect_node(state: State<'_, Arc<AppState>>, id: String) -> Result<Node, String> {
    node_service::disconnect(&state, &id).map_err(|e| e.to_string())
}
