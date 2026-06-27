pub mod tray_manager;

pub use tray_manager::TrayManager;

/// Legacy build_tray function for Phase 1 compatibility.
pub fn build_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let _tray = TrayManager::new(app)?;
    Ok(())
}
