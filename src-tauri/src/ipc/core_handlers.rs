use std::sync::Arc;

use tauri::State;

use crate::core::{CoreManager, CoreStatus};
use crate::session::Session;

/// Start the network core (load rules/profiles, start the engine). Mock.
#[tauri::command]
pub async fn core_start(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.start().await.map_err(|e| e.to_string())
}

/// Stop the network core (stop the engine, destroy the session). Mock.
#[tauri::command]
pub async fn core_stop(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.stop().await.map_err(|e| e.to_string())
}

/// Restart the network core. Mock.
#[tauri::command]
pub async fn core_restart(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.restart().await.map_err(|e| e.to_string())
}

/// Get a snapshot of the current core status (engine state, session, mode).
#[tauri::command]
pub async fn get_core_status(core: State<'_, Arc<CoreManager>>) -> Result<CoreStatus, String> {
    Ok(core.status())
}

/// Connect a profile (and optional node), returning the created session. Mock.
#[tauri::command]
pub async fn connect_profile(
    core: State<'_, Arc<CoreManager>>,
    profile_id: String,
    node_id: Option<String>,
) -> Result<Session, String> {
    core.connect_profile(&profile_id, node_id)
        .await
        .map_err(|e| e.to_string())
}

/// Disconnect the active profile. Mock.
#[tauri::command]
pub async fn disconnect_profile(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.disconnect_profile().await.map_err(|e| e.to_string())
}

/// Get the current session, if any.
#[tauri::command]
pub async fn get_current_session(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Option<Session>, String> {
    Ok(core.current_session())
}
