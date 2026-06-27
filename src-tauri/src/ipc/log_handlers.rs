use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::LogEntry;
use crate::service::log_service;

#[tauri::command]
pub async fn get_logs(
    state: State<'_, Arc<AppState>>,
    level_filter: Option<String>,
) -> Result<Vec<LogEntry>, String> {
    Ok(log_service::get_all(&state, level_filter))
}

#[tauri::command]
pub async fn get_recent_logs(
    state: State<'_, Arc<AppState>>,
    limit: usize,
) -> Result<Vec<LogEntry>, String> {
    Ok(log_service::get_recent(&state, limit))
}
