use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::geo::GeoStatus;
use crate::utils::AppError;

#[tauri::command]
pub async fn get_geo_status(core: State<'_, Arc<CoreManager>>) -> Result<GeoStatus, String> {
    let gm = core
        .geo_manager()
        .ok_or_else(|| AppError::Internal("geo manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(gm.status())
}

#[tauri::command]
pub async fn reload_geo_database(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    let gm = core
        .geo_manager()
        .ok_or_else(|| AppError::Internal("geo manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    gm.reload().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_geo_database(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    let gm = core
        .geo_manager()
        .ok_or_else(|| AppError::Internal("geo manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    gm.update().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn match_geoip(
    core: State<'_, Arc<CoreManager>>,
    ip: String,
) -> Result<Option<String>, String> {
    let gm = core
        .geo_manager()
        .ok_or_else(|| AppError::Internal("geo manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    gm.match_country(&ip).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn match_geosite(
    core: State<'_, Arc<CoreManager>>,
    domain: String,
    category: String,
) -> Result<bool, String> {
    let gm = core
        .geo_manager()
        .ok_or_else(|| AppError::Internal("geo manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(gm.match_domain_category(&domain, &category))
}
