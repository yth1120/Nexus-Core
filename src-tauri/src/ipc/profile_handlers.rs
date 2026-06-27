use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::Profile;
use crate::service::profile_service::{self, CreateProfileRequest};

#[tauri::command]
pub async fn get_profiles(state: State<'_, Arc<AppState>>) -> Result<Vec<Profile>, String> {
    Ok(profile_service::get_all(&state))
}

#[tauri::command]
pub async fn get_profile(state: State<'_, Arc<AppState>>, id: String) -> Result<Profile, String> {
    profile_service::get_by_id(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_profile(
    state: State<'_, Arc<AppState>>,
    data: CreateProfileRequest,
) -> Result<Profile, String> {
    Ok(profile_service::create(&state, data))
}

#[tauri::command]
pub async fn update_profile(
    state: State<'_, Arc<AppState>>,
    id: String,
    data: serde_json::Value,
) -> Result<Profile, String> {
    profile_service::update(&state, &id, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_profile(state: State<'_, Arc<AppState>>, id: String) -> Result<(), String> {
    profile_service::delete(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_profile(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> Result<Profile, String> {
    profile_service::toggle_active(&state, &id).map_err(|e| e.to_string())
}
