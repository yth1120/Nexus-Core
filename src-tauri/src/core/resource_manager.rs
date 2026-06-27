use std::path::PathBuf;
use std::sync::Arc;

use tauri::AppHandle;

use crate::config::ConfigManager;
use crate::core::task_manager::TaskManager;
use crate::event::EventBus;
use crate::platform::PlatformManager;
use crate::repository::{
    ProfileRepository, RuleRepository, SettingsRepository, SqliteProfileRepository,
    SqliteRuleRepository, SqliteSettingsRepository, SqliteStatisticsRepository,
    StatisticsRepository,
};
use crate::storage::Database;
use crate::utils::AppResult;

/// Central orchestrator for all backend subsystems.
///
/// Owns the lifecycle of ConfigManager, Database, EventBus, TaskManager,
/// PlatformManager, and all Repositories. Created once during app setup
/// and stored in AppState.
pub struct ResourceManager {
    pub config_manager: Arc<ConfigManager>,
    pub database: Arc<Database>,
    pub event_bus: EventBus,
    pub task_manager: Arc<TaskManager>,
    pub platform_manager: Arc<dyn PlatformManager>,

    // Repositories
    pub profile_repo: Arc<dyn ProfileRepository>,
    pub rule_repo: Arc<dyn RuleRepository>,
    pub statistics_repo: Arc<dyn StatisticsRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
}

/// Configuration passed to ResourceManager during initialization.
pub struct InitConfig {
    pub app_dir: PathBuf,
    pub app_handle: AppHandle,
}

impl ResourceManager {
    /// Initialize all components in dependency order.
    ///
    /// Order matters: Config → DB → EventBus → TaskManager → Platform → Repos
    pub async fn initialize(cfg: InitConfig) -> AppResult<Self> {
        tracing::info!("Initializing ResourceManager...");

        // 1. ConfigManager (needed first for paths and settings)
        let config_manager = Arc::new(ConfigManager::new(&cfg.app_dir));
        let (_app_config, _settings) = config_manager.load_all()?;
        tracing::info!("Config loaded");

        // 2. Database (needed for repositories)
        let db_path = cfg.app_dir.join("nexus_core.db");
        let database = Arc::new(Database::open(&db_path)?);
        tracing::info!("Database initialized");

        // 3. EventBus (needed for system communication)
        let event_bus = EventBus::new(256);
        event_bus.set_app_handle(cfg.app_handle);
        tracing::info!("EventBus initialized");

        // 4. TaskManager (needed for background tasks)
        let task_manager = Arc::new(TaskManager::new());
        tracing::info!("TaskManager initialized");

        // 5. PlatformManager (depends only on the OS)
        let platform_manager: Arc<dyn PlatformManager> =
            crate::platform::create_platform_manager().into();
        tracing::info!("PlatformManager initialized");

        // 6. Repositories (depend on Database)
        let profile_repo: Arc<dyn ProfileRepository> =
            Arc::new(SqliteProfileRepository::new(database.clone()));
        let rule_repo: Arc<dyn RuleRepository> =
            Arc::new(SqliteRuleRepository::new(database.clone()));
        let statistics_repo: Arc<dyn StatisticsRepository> =
            Arc::new(SqliteStatisticsRepository::new(database.clone()));
        let settings_repo: Arc<dyn SettingsRepository> =
            Arc::new(SqliteSettingsRepository::new(database.clone()));
        tracing::info!("Repositories initialized");

        // 7. Seed seed data if database is empty
        seed_if_empty(&profile_repo, &rule_repo).await?;

        // Apply config values for theme and language from saved settings
        if let Ok(Some(theme)) = settings_repo.get("theme") {
            let mut config = config_manager.load_all()?.0;
            config.theme = theme;
            let _ = config_manager.save_app_config(&config);
        }

        tracing::info!("ResourceManager initialization complete");

        Ok(Self {
            config_manager,
            database,
            event_bus,
            task_manager,
            platform_manager,
            profile_repo,
            rule_repo,
            statistics_repo,
            settings_repo,
        })
    }

    /// Graceful shutdown: stop tasks, flush state, close connections.
    pub async fn shutdown(self) -> AppResult<()> {
        tracing::info!("Shutting down ResourceManager...");

        // 1. Stop all background tasks
        self.task_manager.shutdown_all();
        tracing::info!("Tasks stopped");

        // 2. Flush is implicit — all repos use write-through
        // 3. EventBus dropped, broadcast channel closes
        drop(self.event_bus);
        tracing::info!("EventBus closed");

        // 4. Database pool closed when Database is dropped
        drop(self.database);
        tracing::info!("Database closed");

        tracing::info!("ResourceManager shutdown complete");
        Ok(())
    }
}

/// Seed initial data if the database is empty.
async fn seed_if_empty(
    profile_repo: &Arc<dyn ProfileRepository>,
    rule_repo: &Arc<dyn RuleRepository>,
) -> AppResult<()> {
    // If profiles table is empty, seed from the in-memory seed data
    if profile_repo.count()? == 0 {
        tracing::info!("Seeding initial profiles...");
        for profile in crate::core::app_state::seed_profiles() {
            profile_repo.insert(&profile)?;
        }
        tracing::info!("Profiles seeded");
    }

    if rule_repo.count()? == 0 {
        tracing::info!("Seeding initial rules...");
        for rule in crate::core::app_state::seed_rules() {
            rule_repo.insert(&rule)?;
        }
        tracing::info!("Rules seeded");
    }

    Ok(())
}

#[cfg(test)]
impl ResourceManager {
    /// Build a ResourceManager for unit tests.
    ///
    /// Uses an in-memory SQLite database (no temp files), a no-op platform
    /// manager (never shells out), and no Tauri `AppHandle` (frontend emit is
    /// skipped). The config manager is constructed without touching the disk.
    pub(crate) fn new_for_test() -> AppResult<Self> {
        use std::path::Path;

        let config_manager = Arc::new(ConfigManager::new(Path::new(".")));
        let database = Arc::new(Database::open(Path::new(":memory:"))?);
        let event_bus = EventBus::new(64);
        let task_manager = Arc::new(TaskManager::new());
        let platform_manager: Arc<dyn PlatformManager> =
            Arc::new(test_support::NoopPlatformManager);

        let profile_repo: Arc<dyn ProfileRepository> =
            Arc::new(SqliteProfileRepository::new(database.clone()));
        let rule_repo: Arc<dyn RuleRepository> =
            Arc::new(SqliteRuleRepository::new(database.clone()));
        let statistics_repo: Arc<dyn StatisticsRepository> =
            Arc::new(SqliteStatisticsRepository::new(database.clone()));
        let settings_repo: Arc<dyn SettingsRepository> =
            Arc::new(SqliteSettingsRepository::new(database.clone()));

        Ok(Self {
            config_manager,
            database,
            event_bus,
            task_manager,
            platform_manager,
            profile_repo,
            rule_repo,
            statistics_repo,
            settings_repo,
        })
    }
}

// ===== Test support =====

#[cfg(test)]
pub(crate) mod test_support {
    use crate::platform::{PlatformManager, SystemConnection};
    use crate::utils::{AppError, AppResult};

    /// No-op platform manager for unit tests — never shells out to `netstat`,
    /// `lsof`, the registry, or any system command.
    #[derive(Debug, Default)]
    pub struct NoopPlatformManager;

    impl PlatformManager for NoopPlatformManager {
        fn set_system_proxy(&self, _enabled: bool) {}
        fn get_system_proxy(&self) -> Option<String> {
            None
        }
        fn get_default_interface(&self) -> String {
            String::new()
        }
        fn get_network_interfaces(&self) -> Vec<String> {
            Vec::new()
        }
        fn enable_auto_start(&self) -> AppResult<()> {
            Ok(())
        }
        fn disable_auto_start(&self) -> AppResult<()> {
            Ok(())
        }
        fn enable_system_proxy(&self, _host: &str, _port: u16) -> AppResult<()> {
            Ok(())
        }
        fn disable_system_proxy(&self) -> AppResult<()> {
            Ok(())
        }
        fn open_logs_dir(&self) -> AppResult<()> {
            Ok(())
        }
        fn open_config_dir(&self) -> AppResult<()> {
            Ok(())
        }
        fn show_notification(&self, _title: &str, _body: &str) -> AppResult<()> {
            Ok(())
        }
        fn get_active_connections(&self) -> AppResult<Vec<SystemConnection>> {
            Err(AppError::Unsupported("noop platform (test)".into()))
        }
    }
}
