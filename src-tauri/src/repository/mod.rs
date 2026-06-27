pub mod sqlite_profile_repo;
pub mod sqlite_rule_repo;
pub mod sqlite_settings_repo;
pub mod sqlite_statistics_repo;

pub use sqlite_profile_repo::SqliteProfileRepository;
pub use sqlite_rule_repo::SqliteRuleRepository;
pub use sqlite_settings_repo::SqliteSettingsRepository;
pub use sqlite_statistics_repo::SqliteStatisticsRepository;

use crate::models::{Profile, Rule, StatisticsData, TrafficDataPoint};
use crate::utils::AppResult;

/// Repository trait for Profile CRUD operations.
pub trait ProfileRepository: Send + Sync {
    fn find_all(&self) -> AppResult<Vec<Profile>>;
    fn find_by_id(&self, id: &str) -> AppResult<Option<Profile>>;
    fn insert(&self, profile: &Profile) -> AppResult<()>;
    fn update(&self, profile: &Profile) -> AppResult<()>;
    fn delete(&self, id: &str) -> AppResult<()>;
    fn set_active(&self, id: &str) -> AppResult<()>;
    fn deactivate_all(&self) -> AppResult<()>;
    fn count(&self) -> AppResult<usize>;
}

/// Repository trait for Rule CRUD operations.
pub trait RuleRepository: Send + Sync {
    fn find_all(&self) -> AppResult<Vec<Rule>>;
    fn find_by_id(&self, id: &str) -> AppResult<Option<Rule>>;
    fn insert(&self, rule: &Rule) -> AppResult<()>;
    fn update(&self, rule: &Rule) -> AppResult<()>;
    fn delete(&self, id: &str) -> AppResult<()>;
    fn toggle_enabled(&self, id: &str) -> AppResult<Rule>;
    fn count(&self) -> AppResult<usize>;
}

/// Repository trait for statistics data.
pub trait StatisticsRepository: Send + Sync {
    fn insert_data_point(&self, timestamp: i64, upload: i64, download: i64) -> AppResult<()>;
    fn get_history(&self, since: i64) -> AppResult<Vec<TrafficDataPoint>>;
    fn get_stats_summary(&self) -> AppResult<StatisticsData>;
    fn prune_older_than(&self, timestamp: i64) -> AppResult<usize>;
}

/// Repository trait for key-value settings.
pub trait SettingsRepository: Send + Sync {
    fn get(&self, key: &str) -> AppResult<Option<String>>;
    fn set(&self, key: &str, value: &str) -> AppResult<()>;
    fn get_all(&self) -> AppResult<std::collections::HashMap<String, String>>;
    fn delete(&self, key: &str) -> AppResult<()>;
}
