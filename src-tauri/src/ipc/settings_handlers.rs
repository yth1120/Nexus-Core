use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::service::settings_service;

#[tauri::command]
pub async fn get_settings_defaults(
    state: State<'_, Arc<AppState>>,
) -> Result<serde_json::Value, String> {
    Ok(settings_service::get_defaults(&state))
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, Arc<AppState>>,
    values: serde_json::Value,
) -> Result<serde_json::Value, String> {
    Ok(settings_service::save_settings(&state, values))
}

#[tauri::command]
pub async fn validate_setting(
    _state: State<'_, Arc<AppState>>,
    key: String,
    value: serde_json::Value,
) -> Result<bool, String> {
    Ok(settings_service::validate(&key, &value))
}
