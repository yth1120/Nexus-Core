use std::fs;
use std::path::{Path, PathBuf};

use notify::event::ModifyKind;
use notify::{Event, EventKind, PollWatcher, RecursiveMode, Watcher};
use parking_lot::Mutex as ParkingMutex;
use std::sync::Arc;

use crate::config::models::{AppConfig, BackupMeta, SettingsConfig};
use crate::event::{AppEvent, EventBus};
use crate::utils::{AppError, AppResult};

/// Manages reading, writing, watching, and backing up application configuration.
pub struct ConfigManager {
    app_dir: PathBuf,
    config_dir: PathBuf,
    config_path: PathBuf,
    settings_path: PathBuf,
    profiles_path: PathBuf,
    rules_path: PathBuf,
    _watcher: Arc<ParkingMutex<Option<PollWatcher>>>,
}

impl ConfigManager {
    /// Create a new ConfigManager with the given app data directory.
    pub fn new(app_dir: &Path) -> Self {
        let config_dir = app_dir.join("config");
        let config_path = config_dir.join("app.toml");
        let settings_path = config_dir.join("settings.json");
        let profiles_path = config_dir.join("profiles.json");
        let rules_path = config_dir.join("rules.json");

        Self {
            app_dir: app_dir.to_path_buf(),
            config_dir,
            config_path,
            settings_path,
            profiles_path,
            rules_path,
            _watcher: Arc::new(ParkingMutex::new(None)),
        }
    }

    /// Ensure the config directory and sub-directories exist.
    pub fn ensure_directories(&self) -> AppResult<()> {
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(self.app_dir.join("logs"))?;
        fs::create_dir_all(self.app_dir.join("backups"))?;
        Ok(())
    }

    /// Load all config files. Missing files are created with defaults.
    pub fn load_all(&self) -> AppResult<(AppConfig, SettingsConfig)> {
        self.ensure_directories()?;

        let app_config = self.load_app_config()?;
        let settings = self.load_settings()?;

        tracing::info!("Configuration loaded from {:?}", self.config_dir);
        Ok((app_config, settings))
    }

    /// Load app.toml. Creates with defaults if missing.
    fn load_app_config(&self) -> AppResult<AppConfig> {
        if !self.config_path.exists() {
            let defaults = AppConfig::default();
            self.save_app_config_inner(&defaults)?;
            return Ok(defaults);
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: AppConfig = toml::from_str(&content).unwrap_or_else(|e| {
            tracing::warn!("Failed to parse app.toml, using defaults: {}", e);
            AppConfig::default()
        });
        Ok(config)
    }

    /// Load settings.json. Creates with defaults if missing.
    fn load_settings(&self) -> AppResult<SettingsConfig> {
        if !self.settings_path.exists() {
            let defaults = SettingsConfig::default();
            self.save_settings_inner(&defaults)?;
            return Ok(defaults);
        }

        let content =
            fs::read_to_string(&self.settings_path).unwrap_or_else(|_| String::from("{}"));
        let settings: SettingsConfig = serde_json::from_str(&content).unwrap_or_else(|e| {
            tracing::warn!("Failed to parse settings.json, using defaults: {}", e);
            SettingsConfig::default()
        });
        Ok(settings)
    }

    /// Save app config to config/app.toml (atomic write).
    pub fn save_app_config(&self, config: &AppConfig) -> AppResult<()> {
        self.ensure_directories()?;
        self.save_app_config_inner(config)
    }

    fn save_app_config_inner(&self, config: &AppConfig) -> AppResult<()> {
        let content = toml::to_string_pretty(config)?;
        self.atomic_write(&self.config_path, &content)?;
        Ok(())
    }

    /// Save settings to config/settings.json (atomic write).
    pub fn save_settings(&self, settings: &SettingsConfig) -> AppResult<()> {
        self.ensure_directories()?;
        self.save_settings_inner(settings)
    }

    fn save_settings_inner(&self, settings: &SettingsConfig) -> AppResult<()> {
        let content = serde_json::to_string_pretty(settings)?;
        self.atomic_write(&self.settings_path, &content)?;
        Ok(())
    }

    /// Atomic write: write to .tmp file, then rename.
    fn atomic_write(&self, target: &Path, content: &str) -> AppResult<()> {
        let temp = target.with_extension("tmp");
        fs::write(&temp, content)?;
        fs::rename(&temp, target)?;
        tracing::debug!("Atomic write to {:?}", target);
        Ok(())
    }

    /// Start file-system watching on the config directory.
    /// Emits `AppEvent::ConfigChanged` when files change.
    pub fn start_watching(&mut self, event_bus: EventBus) -> AppResult<()> {
        let config_dir = self.config_dir.clone();
        let eb = event_bus;

        let mut watcher = PollWatcher::new(
            move |res: notify::Result<Event>| {
                if let Ok(event) = res {
                    // Only react to data changes (content modifications)
                    let is_data_change = event.kind
                        == EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Any));
                    if is_data_change {
                        for path in event.paths {
                            let path_str = path.display().to_string();
                            tracing::info!("Config file changed: {}", path_str);
                            eb.publish(AppEvent::ConfigChanged { path: path_str });
                        }
                    }
                }
            },
            notify::Config::default().with_poll_interval(std::time::Duration::from_secs(2)),
        )?;

        watcher.watch(&config_dir, RecursiveMode::NonRecursive)?;
        *self._watcher.lock() = Some(watcher);

        tracing::info!("File watcher started on {:?}", config_dir);
        Ok(())
    }

    /// Create a timestamped backup of all config files.
    pub fn create_backup(&self) -> AppResult<PathBuf> {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        let backup_dir = self.app_dir.join("backups").join(&timestamp);
        fs::create_dir_all(&backup_dir)?;

        for (src, name) in [
            (&self.config_path, "app.toml"),
            (&self.settings_path, "settings.json"),
            (&self.profiles_path, "profiles.json"),
            (&self.rules_path, "rules.json"),
        ] {
            if src.exists() {
                fs::copy(src, backup_dir.join(name))?;
            }
        }

        // Write backup metadata
        let meta = BackupMeta {
            timestamp: chrono::Utc::now().timestamp_millis(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            files: vec![
                "app.toml".into(),
                "settings.json".into(),
                "profiles.json".into(),
                "rules.json".into(),
            ],
        };
        let meta_json = serde_json::to_string_pretty(&meta)?;
        fs::write(backup_dir.join("backup.json"), meta_json)?;

        tracing::info!("Backup created at {:?}", backup_dir);
        Ok(backup_dir)
    }

    /// Restore config files from a backup directory.
    /// The backup path is canonicalized to prevent symlink attacks.
    pub fn restore_from_backup(&self, backup_path: &Path) -> AppResult<()> {
        let backup_path = backup_path
            .canonicalize()
            .map_err(|e| AppError::Validation(format!("invalid backup path: {e}")))?;

        if !backup_path.exists() {
            return Err(AppError::NotFound(format!(
                "Backup {} not found",
                backup_path.display()
            )));
        }

        for file_name in &["app.toml", "settings.json", "profiles.json", "rules.json"] {
            let src = backup_path.join(file_name);
            if src.exists() {
                let dest = self.config_dir.join(file_name);
                fs::copy(&src, &dest)?;
            }
        }

        tracing::info!("Config restored from {:?}", backup_path);
        Ok(())
    }

    /// List available backups sorted by date (newest first).
    pub fn list_backups(&self) -> AppResult<Vec<PathBuf>> {
        let backups_dir = self.app_dir.join("backups");
        if !backups_dir.exists() {
            return Ok(Vec::new());
        }

        let mut dirs: Vec<PathBuf> = fs::read_dir(&backups_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.path())
            .collect();

        dirs.sort_by(|a, b| b.cmp(a)); // newest first
        Ok(dirs)
    }

    /// Reload configuration from disk.
    pub fn reload(&self) -> AppResult<(AppConfig, SettingsConfig)> {
        tracing::info!("Reloading configuration from disk");
        self.load_all()
    }

    /// Placeholder for reload on watch. Calls load_all internally.
    pub fn watch(&self) -> AppResult<()> {
        tracing::debug!("Config file watching: use start_watching() for real monitoring");
        Ok(())
    }

    // Getters for paths
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }
    pub fn config_path(&self) -> &Path {
        &self.config_path
    }
    pub fn settings_path(&self) -> &Path {
        &self.settings_path
    }
    pub fn profiles_path(&self) -> &Path {
        &self.profiles_path
    }
    pub fn rules_path(&self) -> &Path {
        &self.rules_path
    }
    pub fn app_dir(&self) -> &Path {
        &self.app_dir
    }
}
