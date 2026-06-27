pub mod linux;
pub mod macos;
pub mod windows;

use serde::Serialize;

use crate::utils::AppResult;

/// Represents a real system network connection.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConnection {
    pub pid: u32,
    pub process_name: String,
    pub source: String,
    pub destination: String,
    pub protocol: String, // "TCP" or "UDP"
    pub state: String,    // "ESTABLISHED", "LISTEN", etc.
    pub duration_secs: u64,
    pub upload_bytes: i64,
    pub download_bytes: i64,
}

/// Platform-specific system operations.
///
/// Each platform provides concrete implementations. Methods that are
/// not available on a given platform return `AppError::Unsupported`.
pub trait PlatformManager: Send + Sync {
    // ===== Existing Phase 1 methods =====

    /// Enable or disable the system-wide HTTP(S) proxy (legacy signature).
    fn set_system_proxy(&self, enabled: bool);

    /// Get the current system proxy setting, if any.
    fn get_system_proxy(&self) -> Option<String>;

    /// Get the default network interface name.
    fn get_default_interface(&self) -> String;

    /// Get a list of available network interface names.
    fn get_network_interfaces(&self) -> Vec<String>;

    // ===== New Phase 2 methods =====

    /// Enable launching the application at user login.
    fn enable_auto_start(&self) -> AppResult<()>;

    /// Disable launching at user login.
    fn disable_auto_start(&self) -> AppResult<()>;

    /// Enable system-wide HTTP(S) proxy with the given host and port.
    fn enable_system_proxy(&self, host: &str, port: u16) -> AppResult<()>;

    /// Disable system-wide HTTP(S) proxy.
    fn disable_system_proxy(&self) -> AppResult<()>;

    /// Open the logs directory in the system file manager.
    fn open_logs_dir(&self) -> AppResult<()>;

    /// Open the config directory in the system file manager.
    fn open_config_dir(&self) -> AppResult<()>;

    /// Show a desktop notification.
    fn show_notification(&self, title: &str, body: &str) -> AppResult<()>;

    /// Get active system network connections (process, PID, addresses, protocol).
    fn get_active_connections(&self) -> AppResult<Vec<SystemConnection>>;
}

/// Create the platform manager for the current OS.
pub fn create_platform_manager() -> Box<dyn PlatformManager> {
    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsPlatformManager)
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOSPlatformManager)
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxPlatformManager)
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Box::new(windows::WindowsPlatformManager)
    }
}
