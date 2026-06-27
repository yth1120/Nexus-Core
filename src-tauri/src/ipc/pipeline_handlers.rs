use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::core::CoreManager;
use crate::utils::AppError;

/// Serializable pipeline status for the `get_pipeline_status` IPC command.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PipelineStatusDto {
    pub state: String,
    pub processor_count: usize,
    pub packet_count: u64,
    pub bytes_in: u64,
    pub bytes_out: u64,
}

/// Serializable packet statistics.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PacketStatsDto {
    pub packet_count: u64,
    pub bytes_in: u64,
    pub bytes_out: u64,
}

#[tauri::command]
pub async fn get_pipeline_status(
    core: State<'_, Arc<CoreManager>>,
) -> Result<PipelineStatusDto, String> {
    let pm = core
        .pipeline_manager()
        .ok_or_else(|| AppError::Internal("pipeline manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    let (packet_count, bytes_in, bytes_out) = pm.statistics_snapshot();
    Ok(PipelineStatusDto {
        state: format!("{:?}", pm.status()),
        processor_count: pm.pipeline().len(),
        packet_count,
        bytes_in,
        bytes_out,
    })
}

#[tauri::command]
pub async fn get_packet_statistics(
    core: State<'_, Arc<CoreManager>>,
) -> Result<PacketStatsDto, String> {
    let pm = core
        .pipeline_manager()
        .ok_or_else(|| AppError::Internal("pipeline manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    let (packet_count, bytes_in, bytes_out) = pm.statistics_snapshot();
    Ok(PacketStatsDto {
        packet_count,
        bytes_in,
        bytes_out,
    })
}
