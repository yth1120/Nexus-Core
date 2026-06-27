use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::release::app_updater::UpdateInfo;
use crate::utils::AppError;

#[tauri::command]
pub async fn check_app_update(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Option<UpdateInfo>, String> {
    let au = core
        .app_updater()
        .ok_or_else(|| AppError::Internal("app updater not initialized".into()))
        .map_err(|e| e.to_string())?;
    au.check_update().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn download_app_update(
    core: State<'_, Arc<CoreManager>>,
    version: String,
) -> Result<(), String> {
    let au = core
        .app_updater()
        .ok_or_else(|| AppError::Internal("app updater not initialized".into()))
        .map_err(|e| e.to_string())?;

    // Check for update first
    let info = au
        .check_update()
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| {
            AppError::NotFound(format!("no update available for {version}")).to_string()
        })?;

    let dest = std::env::temp_dir().join(format!("nexus-core-{version}.msi"));
    au.download_update(&info, &dest)
        .await
        .map_err(|e| e.to_string())?;

    core.context()
        .publish(crate::event::AppEvent::UpdateDownloaded {
            version: info.version,
        });

    Ok(())
}

#[tauri::command]
pub async fn apply_app_update(
    core: State<'_, Arc<CoreManager>>,
    _version: String,
) -> Result<(), String> {
    // On Windows, the updater would spawn the MSI installer
    // On macOS, it would mount the DMG and copy the app
    // On Linux, it would replace the AppImage
    //
    // For now this is a placeholder — Tauri's updater plugin handles
    // the actual platform-specific installation.
    core.context()
        .publish(crate::event::AppEvent::UpdateApplied {
            version: _version.clone(),
        });
    tracing::info!("Update applied (placeholder): {_version}");
    Ok(())
}

#[tauri::command]
pub async fn rollback_update(
    core: State<'_, Arc<CoreManager>>,
    _version: String,
) -> Result<(), String> {
    // Rollback would restore the previous version from a backup
    // This is a placeholder — actual rollback requires platform-specific logic
    tracing::warn!("Rollback requested to {_version} (placeholder)");
    core.context()
        .publish(crate::event::AppEvent::UpdateFailed {
            version: _version,
            error: "Rollback not yet implemented".into(),
        });
    Err("Rollback not yet implemented".into())
}
