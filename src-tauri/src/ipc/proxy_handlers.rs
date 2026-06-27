use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::proxy::ProxyStatusDto;
use crate::utils::AppError;

#[tauri::command]
pub async fn start_http_proxy(core: State<'_, Arc<CoreManager>>) -> Result<ProxyStatusDto, String> {
    let pm = core
        .proxy_manager()
        .ok_or_else(|| AppError::Internal("proxy manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    pm.start_http().await.map_err(|e| e.to_string())?;
    Ok(pm.proxy_status())
}

#[tauri::command]
pub async fn stop_http_proxy(core: State<'_, Arc<CoreManager>>) -> Result<ProxyStatusDto, String> {
    let pm = core
        .proxy_manager()
        .ok_or_else(|| AppError::Internal("proxy manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    pm.stop_http().await.map_err(|e| e.to_string())?;
    Ok(pm.proxy_status())
}

#[tauri::command]
pub async fn start_socks5_proxy(
    core: State<'_, Arc<CoreManager>>,
) -> Result<ProxyStatusDto, String> {
    let pm = core
        .proxy_manager()
        .ok_or_else(|| AppError::Internal("proxy manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    pm.start_socks5().await.map_err(|e| e.to_string())?;
    Ok(pm.proxy_status())
}

#[tauri::command]
pub async fn stop_socks5_proxy(
    core: State<'_, Arc<CoreManager>>,
) -> Result<ProxyStatusDto, String> {
    let pm = core
        .proxy_manager()
        .ok_or_else(|| AppError::Internal("proxy manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    pm.stop_socks5().await.map_err(|e| e.to_string())?;
    Ok(pm.proxy_status())
}

#[tauri::command]
pub async fn get_proxy_status(core: State<'_, Arc<CoreManager>>) -> Result<ProxyStatusDto, String> {
    let pm = core
        .proxy_manager()
        .ok_or_else(|| AppError::Internal("proxy manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(pm.proxy_status())
}
