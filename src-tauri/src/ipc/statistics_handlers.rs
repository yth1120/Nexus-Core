use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::{StatisticsData, TimeRange};
use crate::service::statistics_service;

#[tauri::command]
pub async fn get_statistics(
    state: State<'_, Arc<AppState>>,
    time_range: TimeRange,
) -> Result<StatisticsData, String> {
    Ok(statistics_service::get_statistics(&state, &time_range))
}
