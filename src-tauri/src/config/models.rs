use serde::{Deserialize, Serialize};

/// Core management configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreConfig {
    /// Whether to auto-download missing core binaries.
    #[serde(default = "default_bool_true")]
    pub auto_download: bool,

    /// Whether to auto-update core binaries.
    #[serde(default = "default_bool_true")]
    pub auto_update: bool,

    /// Number of download retry attempts.
    #[serde(default = "default_download_retry")]
    pub download_retry: u32,

    /// Update check interval in seconds (default 86400 = 24 hours).
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            auto_download: true,
            auto_update: true,
            download_retry: 3,
            check_interval: 86400,
        }
    }
}

/// Local telemetry configuration (no network upload).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryConfig {
    /// Whether local telemetry collection is enabled.
    #[serde(default = "default_bool_true")]
    pub enabled: bool,

    /// Memory sample interval in seconds.
    #[serde(default = "default_telemetry_interval")]
    pub sample_interval: u64,

    /// Data retention in days.
    #[serde(default = "default_telemetry_retention")]
    pub retention_days: u64,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sample_interval: 60,
            retention_days: 30,
        }
    }
}

/// Application update configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfig {
    /// Update channel: "stable" or "beta".
    #[serde(default = "default_update_channel")]
    pub channel: String,

    /// Update check interval in seconds (default 86400 = 24h).
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,

    /// Whether to auto-download updates (user must still approve install).
    #[serde(default = "default_bool_false")]
    pub auto_download: bool,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            channel: "stable".into(),
            check_interval: 86400,
            auto_download: false,
        }
    }
}

/// GeoIP / GeoSite database configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoConfig {
    /// Whether to auto-update geo databases.
    #[serde(default = "default_bool_true")]
    pub auto_update: bool,

    /// Update check interval in seconds (default 86400 = 24 hours).
    #[serde(default = "default_geo_update_interval")]
    pub update_interval: u64,

    /// Path to the GeoIP MMDB database file (relative to data dir).
    #[serde(default = "default_geoip_path")]
    pub geoip_path: String,

    /// Path to the GeoSite protobuf database file (relative to data dir).
    #[serde(default = "default_geosite_path")]
    pub geosite_path: String,
}

impl Default for GeoConfig {
    fn default() -> Self {
        Self {
            auto_update: true,
            update_interval: 86400,
            geoip_path: "data/geo/geoip.mmdb".into(),
            geosite_path: "data/geo/geosite.dat".into(),
        }
    }
}

/// Main application configuration stored in config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "default_http_port")]
    pub http_port: u16,

    #[serde(default = "default_socks_port")]
    pub socks_port: u16,

    #[serde(default = "default_mixed_port")]
    pub mixed_port: u16,

    #[serde(default = "default_allow_lan")]
    pub allow_lan: bool,

    #[serde(default = "default_tun_mode")]
    pub tun_mode: bool,

    #[serde(default = "default_tun_stack")]
    pub tun_stack: String,

    #[serde(default = "default_traffic_mode")]
    pub traffic_mode: String,

    #[serde(default = "default_dns_enabled")]
    pub dns_enabled: bool,

    #[serde(default = "default_dns_resolver")]
    pub dns_resolver: String,

    #[serde(default = "default_dns_cache_enabled")]
    pub dns_cache_enabled: bool,

    #[serde(default = "default_dns_cache_size")]
    pub dns_cache_size: u32,

    #[serde(default = "default_rules_enabled")]
    pub rules_enabled: bool,

    #[serde(default = "default_dns_server")]
    pub dns_server: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_launch_on_startup")]
    pub launch_on_startup: bool,

    #[serde(default = "default_auto_check_updates")]
    pub auto_check_updates: bool,

    /// Core binary management settings.
    #[serde(default)]
    pub core: CoreConfig,

    /// GeoIP / GeoSite database settings.
    #[serde(default)]
    pub geo: GeoConfig,

    /// Local telemetry settings.
    #[serde(default)]
    pub telemetry: TelemetryConfig,

    /// Application update settings.
    #[serde(default)]
    pub update: UpdateConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            language: default_language(),
            http_port: default_http_port(),
            socks_port: default_socks_port(),
            mixed_port: default_mixed_port(),
            allow_lan: default_allow_lan(),
            tun_mode: default_tun_mode(),
            tun_stack: default_tun_stack(),
            traffic_mode: default_traffic_mode(),
            dns_enabled: default_dns_enabled(),
            dns_resolver: default_dns_resolver(),
            dns_cache_enabled: default_dns_cache_enabled(),
            dns_cache_size: default_dns_cache_size(),
            rules_enabled: default_rules_enabled(),
            dns_server: default_dns_server(),
            log_level: default_log_level(),
            launch_on_startup: default_launch_on_startup(),
            auto_check_updates: default_auto_check_updates(),
            core: CoreConfig::default(),
            geo: GeoConfig::default(),
            telemetry: TelemetryConfig::default(),
            update: UpdateConfig::default(),
        }
    }
}

/// Separate settings file (config/settings.json) for user-facing preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsConfig {
    #[serde(default = "default_theme")]
    pub theme: String,

    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default = "default_launch_on_startup")]
    pub launch_on_startup: bool,

    #[serde(default = "default_auto_check_updates")]
    pub auto_check_updates: bool,

    #[serde(default)]
    pub silent_mode: bool,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            language: default_language(),
            launch_on_startup: default_launch_on_startup(),
            auto_check_updates: default_auto_check_updates(),
            silent_mode: false,
        }
    }
}

/// Metadata for a config backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMeta {
    pub timestamp: i64,
    pub version: String,
    pub files: Vec<String>,
}

// Default value functions
fn default_theme() -> String {
    "dark".to_string()
}
fn default_language() -> String {
    "en".to_string()
}
fn default_mixed_port() -> u16 {
    7890
}
fn default_http_port() -> u16 {
    7890
}
fn default_socks_port() -> u16 {
    7891
}
fn default_allow_lan() -> bool {
    true
}
fn default_tun_mode() -> bool {
    false
}
fn default_tun_stack() -> String {
    "system".into()
}
fn default_traffic_mode() -> String {
    "system_proxy".into()
}
fn default_dns_enabled() -> bool {
    true
}
fn default_dns_resolver() -> String {
    "system".into()
}
fn default_dns_cache_enabled() -> bool {
    true
}
fn default_dns_cache_size() -> u32 {
    4096
}
fn default_rules_enabled() -> bool {
    true
}
fn default_dns_server() -> String {
    "1.1.1.1".to_string()
}
fn default_log_level() -> String {
    "INFO".to_string()
}
fn default_launch_on_startup() -> bool {
    false
}
fn default_auto_check_updates() -> bool {
    true
}
fn default_bool_true() -> bool {
    true
}
fn default_download_retry() -> u32 {
    3
}
fn default_check_interval() -> u64 {
    86400
}
fn default_geo_update_interval() -> u64 {
    86400
}
fn default_geoip_path() -> String {
    "data/geo/geoip.mmdb".into()
}
fn default_geosite_path() -> String {
    "data/geo/geosite.dat".into()
}
fn default_telemetry_interval() -> u64 {
    60
}
fn default_telemetry_retention() -> u64 {
    30
}
fn default_update_channel() -> String {
    "stable".into()
}
fn default_bool_false() -> bool {
    false
}
