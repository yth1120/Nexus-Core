use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::{DashboardStatus, TrafficDataPoint};
use crate::service::dashboard_service;

#[tauri::command]
pub async fn get_dashboard_status(
    state: State<'_, Arc<AppState>>,
) -> Result<DashboardStatus, String> {
    Ok(dashboard_service::get_dashboard_status(&state))
}

#[tauri::command]
pub async fn get_traffic_history(
    state: State<'_, Arc<AppState>>,
) -> Result<Vec<TrafficDataPoint>, String> {
    Ok(dashboard_service::get_traffic_history(&state))
}
