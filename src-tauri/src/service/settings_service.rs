use crate::core::AppState;

/// Get default settings values derived from the current config.
pub fn get_defaults(state: &AppState) -> serde_json::Value {
    let config = state.get_config();
    serde_json::json!({
        "launchOnStartup": config.launch_on_startup,
        "silentMode": false,
        "mixedPort": config.mixed_port.to_string(),
        "allowLan": config.allow_lan,
        "tunMode": config.tun_mode,
        "dnsServer": config.dns_server,
        "themeMode": config.theme,
        "logLevel": config.log_level,
        "language": config.language,
    })
}

/// Known setting keys — any key not in this list is rejected.
const VALID_SETTING_KEYS: &[&str] = &[
    "launchOnStartup",
    "mixedPort",
    "allowLan",
    "tunMode",
    "dnsServer",
    "themeMode",
    "logLevel",
    "language",
];

/// Save settings values. Updates config in memory and writes to disk.
/// Returns the updated config.
pub fn save_settings(state: &AppState, values: serde_json::Value) -> serde_json::Value {
    // Reject unknown keys — prevents arbitrary data from being persisted
    if let Some(obj) = values.as_object() {
        for key in obj.keys() {
            if !VALID_SETTING_KEYS.contains(&key.as_str()) {
                tracing::warn!("Rejected unknown setting key: {key}");
                return get_defaults(state);
            }
        }
    }

    let mut config = state.get_config();

    if let Some(v) = values.get("launchOnStartup").and_then(|v| v.as_bool()) {
        config.launch_on_startup = v;
    }
    if let Some(v) = values.get("mixedPort").and_then(|v| v.as_str()) {
        if let Ok(port) = v.parse::<u16>() {
            config.mixed_port = port;
        }
    }
    if let Some(v) = values.get("allowLan").and_then(|v| v.as_bool()) {
        config.allow_lan = v;
    }
    if let Some(v) = values.get("tunMode").and_then(|v| v.as_bool()) {
        config.tun_mode = v;
    }
    if let Some(v) = values.get("dnsServer").and_then(|v| v.as_str()) {
        config.dns_server = v.to_string();
    }
    if let Some(v) = values.get("themeMode").and_then(|v| v.as_str()) {
        config.theme = v.to_string();
    }
    if let Some(v) = values.get("logLevel").and_then(|v| v.as_str()) {
        config.log_level = v.to_string();
    }
    if let Some(v) = values.get("language").and_then(|v| v.as_str()) {
        config.language = v.to_string();
    }

    state.update_config(config.clone());

    // Persist to disk via ResourceManager if available
    if let Some(rm) = state.get_resource_manager() {
        // Save to config file
        let _ = rm.config_manager.save_app_config(&config);
        // Save individual settings to SettingsRepository
        let _ = rm.settings_repo.set("theme", &config.theme);
        let _ = rm.settings_repo.set("language", &config.language);
        let _ = rm.settings_repo.set("log_level", &config.log_level);
        let _ = rm
            .settings_repo
            .set("launch_on_startup", &config.launch_on_startup.to_string());

        // Emit theme change event if theme was updated
        if let Some(v) = values.get("themeMode").and_then(|v| v.as_str()) {
            rm.event_bus
                .publish(crate::event::AppEvent::ThemeChange(v.to_string()));
        }
    }

    // Return the updated defaults
    get_defaults(state)
}

/// Validate a setting key/value pair.
pub fn validate(_key: &str, value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => !s.is_empty(),
        serde_json::Value::Number(_) => true,
        serde_json::Value::Bool(_) => true,
        _ => false,
    }
}
