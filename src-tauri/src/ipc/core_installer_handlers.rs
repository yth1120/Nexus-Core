use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::core_installer::{CoreState, VersionManifest};
use crate::utils::AppError;

/// Serializable download progress DTO.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadProgress {
    pub state: CoreState,
    pub message: String,
}

// ----- install / uninstall -----

#[tauri::command]
pub async fn install_core(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
    version: String,
) -> Result<(), String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.install(&core_name, &version)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn uninstall_core(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
    version: String,
) -> Result<(), String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.uninstall(&core_name, &version)
        .await
        .map_err(|e| e.to_string())
}

// ----- version queries -----

#[tauri::command]
pub async fn get_core_versions(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
) -> Result<Vec<VersionManifest>, String> {
    let vm = core
        .version_manager()
        .ok_or_else(|| AppError::Internal("version manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(vm.get_installed_versions(&core_name))
}

#[tauri::command]
pub async fn get_current_core_version(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
) -> Result<Option<VersionManifest>, String> {
    let vm = core
        .version_manager()
        .ok_or_else(|| AppError::Internal("version manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    let installed = vm.get_installed_versions(&core_name);
    Ok(installed.into_iter().find(|v| v.is_current))
}

// ----- update -----

#[tauri::command]
pub async fn check_core_update(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
) -> Result<Option<String>, String> {
    let vm = core
        .version_manager()
        .ok_or_else(|| AppError::Internal("version manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    vm.check_update(&core_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_core(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
) -> Result<(), String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.update(&core_name)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// ----- version switching / rollback -----

#[tauri::command]
pub async fn switch_core_version(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
    version: String,
) -> Result<(), String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.switch_version(&core_name, &version)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rollback_core(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
) -> Result<String, String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.rollback(&core_name).map_err(|e| e.to_string())
}

// ----- download only -----

#[tauri::command]
pub async fn download_core(
    core: State<'_, Arc<CoreManager>>,
    core_name: String,
    version: String,
) -> Result<(), String> {
    // Download-only: use the installer but skip extract + register.
    // For now, this just triggers a full install (download + verify + extract).
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    im.install(&core_name, &version)
        .await
        .map_err(|e| e.to_string())
}

// ----- progress -----

#[tauri::command]
pub async fn get_download_progress(
    core: State<'_, Arc<CoreManager>>,
) -> Result<DownloadProgress, String> {
    let im = core
        .installer_manager()
        .ok_or_else(|| AppError::Internal("installer manager not initialized".into()))
        .map_err(|e| e.to_string())?;
    let state_cell = im.state();
    Ok(DownloadProgress {
        state: state_cell.get(),
        message: state_cell.message(),
    })
}
