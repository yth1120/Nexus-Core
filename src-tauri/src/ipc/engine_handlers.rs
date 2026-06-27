use std::str::FromStr;
use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::core::CoreManager;
use crate::engine::engine_trait::EngineType;
use crate::utils::AppError;

/// Serializable engine status snapshot for the `get_engine_status` IPC command.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineStatusDto {
    pub engine_type: String,
    pub state: String,
    pub capabilities: Vec<String>,
    pub version: String,
    pub healthy: bool,
}

#[tauri::command]
pub async fn get_engine_list(core: State<'_, Arc<CoreManager>>) -> Result<Vec<String>, String> {
    let em = core
        .engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(em.engine_list().iter().map(|e| e.to_string()).collect())
}

#[tauri::command]
pub async fn get_current_engine(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Option<String>, String> {
    let em = core
        .engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(em.current_engine().map(|e| e.engine_type().to_string()))
}

#[tauri::command]
pub async fn switch_engine(
    core: State<'_, Arc<CoreManager>>,
    engine_type: String,
) -> Result<(), String> {
    let et = EngineType::from_str(&engine_type).map_err(|e| e.to_string())?;
    core.engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?
        .switch_engine(&et)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_engine(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?
        .start_current()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_engine(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?
        .stop_current()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_engine(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?
        .restart_current()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reload_engine(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?
        .reload_current()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_engine_capabilities(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Vec<String>, String> {
    let em = core
        .engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(em
        .capabilities()
        .iter()
        .map(|c| format!("{:?}", c))
        .collect())
}

#[tauri::command]
pub async fn get_engine_status(
    core: State<'_, Arc<CoreManager>>,
) -> Result<EngineStatusDto, String> {
    let em = core
        .engine_manager()
        .ok_or_else(|| AppError::Internal("engine manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    let cur = em.current_engine();
    let engine_type_str = cur
        .as_ref()
        .map(|e| e.engine_type().to_string())
        .unwrap_or_else(|| "none".to_string());
    let version_str = cur.as_ref().map(|e| e.version()).unwrap_or_default();
    let healthy = cur.is_some();
    Ok(EngineStatusDto {
        engine_type: engine_type_str,
        state: format!("{:?}", em.current_status()),
        capabilities: em
            .capabilities()
            .iter()
            .map(|c| format!("{:?}", c))
            .collect(),
        version: version_str,
        healthy,
    })
}
