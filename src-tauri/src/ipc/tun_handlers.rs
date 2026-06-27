use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::core::CoreManager;
use crate::tun::tun_state::TunState;
use crate::utils::AppError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TunStatusDto {
    pub state: String,
    pub device_available: bool,
}

#[tauri::command]
pub async fn start_tun(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.start_tun().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_tun(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.stop_tun().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restart_tun(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.stop_tun().await.map_err(|e| e.to_string())?;
    core.start_tun().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_tun_status(core: State<'_, Arc<CoreManager>>) -> Result<TunStatusDto, String> {
    let state = core
        .tun_manager()
        .map(|tm| tm.status())
        .unwrap_or(TunState::Stopped);
    Ok(TunStatusDto {
        state: format!("{:?}", state),
        device_available: false,
    })
}

#[tauri::command]
pub async fn set_traffic_mode(
    core: State<'_, Arc<CoreManager>>,
    mode: String,
) -> Result<(), String> {
    let m = match mode.as_str() {
        "system_proxy" => crate::core::TrafficMode::SystemProxy,
        "tun" => crate::core::TrafficMode::Tun,
        "hybrid" => crate::core::TrafficMode::Hybrid,
        _ => return Err(AppError::Validation(format!("unknown traffic mode: {mode}")).to_string()),
    };
    core.set_traffic_mode(m).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_traffic_mode(_core: State<'_, Arc<CoreManager>>) -> Result<String, String> {
    Ok("system_proxy".into())
}
