use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::security::audit::{SecurityAuditor, SecurityReport};
use crate::security::download_validator::{DownloadValidation, DownloadValidator};
use crate::security::path_validator::PathValidator;

#[tauri::command]
pub async fn run_security_audit(
    core: State<'_, Arc<CoreManager>>,
) -> Result<SecurityReport, String> {
    let app_dir = {
        let rm = core
            .context()
            .resource_manager()
            .config_manager
            .config_dir()
            .to_path_buf();
        rm.parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| rm.clone())
    };

    let report = SecurityAuditor::run_audit(&app_dir).map_err(|e| e.to_string())?;

    core.context()
        .publish(crate::event::AppEvent::SecurityAuditCompleted);

    Ok(report)
}

#[tauri::command]
pub async fn validate_path(
    _core: State<'_, Arc<CoreManager>>,
    path: String,
) -> Result<bool, String> {
    let base = std::env::temp_dir();
    let result = PathValidator::validate(&path, &base);
    Ok(result.is_ok())
}

#[tauri::command]
pub async fn validate_download(
    _core: State<'_, Arc<CoreManager>>,
    file_path: String,
    expected_hash: Option<String>,
) -> Result<DownloadValidation, String> {
    use std::path::Path;
    let path = Path::new(&file_path);
    let hash = expected_hash.as_deref();
    DownloadValidator::validate(path, hash, None).map_err(|e| e.to_string())
}
