use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::telemetry::telemetry_recorder::TelemetryReport;
use crate::utils::AppError;

#[tauri::command]
pub async fn get_telemetry_report(
    core: State<'_, Arc<CoreManager>>,
) -> Result<TelemetryReport, String> {
    let tr = core
        .telemetry_recorder()
        .ok_or_else(|| AppError::Internal("telemetry not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(tr.get_report())
}

#[tauri::command]
pub async fn get_startup_duration(core: State<'_, Arc<CoreManager>>) -> Result<u64, String> {
    let tr = core
        .telemetry_recorder()
        .ok_or_else(|| AppError::Internal("telemetry not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(tr.startup_duration_ms())
}

#[tauri::command]
pub async fn get_crash_count(core: State<'_, Arc<CoreManager>>) -> Result<u64, String> {
    let tr = core
        .telemetry_recorder()
        .ok_or_else(|| AppError::Internal("telemetry not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(tr.crash_count())
}
