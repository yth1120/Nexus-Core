use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::Connection;
use crate::service::connection_service;

#[tauri::command]
pub async fn get_connections(state: State<'_, Arc<AppState>>) -> Result<Vec<Connection>, String> {
    Ok(connection_service::get_all(&state))
}

#[tauri::command]
pub async fn close_connection(state: State<'_, Arc<AppState>>, id: String) -> Result<(), String> {
    connection_service::close_by_id(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn close_all_connections(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    connection_service::close_all(&state);
    Ok(())
}
