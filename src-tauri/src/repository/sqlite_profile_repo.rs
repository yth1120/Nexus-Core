use std::sync::Arc;

use crate::models::{Profile, ProfileStatus, ProfileType};
use crate::repository::ProfileRepository;
use crate::storage::Database;
use crate::utils::{AppError, AppResult};

pub struct SqliteProfileRepository {
    db: Arc<Database>,
}

impl SqliteProfileRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl ProfileRepository for SqliteProfileRepository {
    fn find_all(&self) -> AppResult<Vec<Profile>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, profile_type, status, latency, updated, config_url, traffic_used, traffic_total
                 FROM profiles ORDER BY updated DESC",
            )?;
            let rows = stmt.query_map([], |row| {
                Ok(Profile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    profile_type: parse_profile_type(&row.get::<_, String>(2)?),
                    status: parse_profile_status(&row.get::<_, String>(3)?),
                    latency: row.get(4)?,
                    updated: row.get(5)?,
                    config_url: row.get(6)?,
                    traffic_used: row.get(7)?,
                    traffic_total: row.get(8)?,
                })
            })?;

            let mut profiles = Vec::new();
            for row in rows {
                profiles.push(row?);
            }
            Ok(profiles)
        })
    }

    fn find_by_id(&self, id: &str) -> AppResult<Option<Profile>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, profile_type, status, latency, updated, config_url, traffic_used, traffic_total
                 FROM profiles WHERE id = ?1",
            )?;
            let mut rows = stmt.query_map([id], |row| {
                Ok(Profile {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    profile_type: parse_profile_type(&row.get::<_, String>(2)?),
                    status: parse_profile_status(&row.get::<_, String>(3)?),
                    latency: row.get(4)?,
                    updated: row.get(5)?,
                    config_url: row.get(6)?,
                    traffic_used: row.get(7)?,
                    traffic_total: row.get(8)?,
                })
            })?;

            Ok(rows.next().transpose()?)
        })
    }

    fn insert(&self, profile: &Profile) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO profiles (id, name, profile_type, status, latency, updated, config_url, traffic_used, traffic_total)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    profile.id,
                    profile.name,
                    profile_type_str(&profile.profile_type),
                    profile_status_str(&profile.status),
                    profile.latency,
                    profile.updated,
                    profile.config_url,
                    profile.traffic_used,
                    profile.traffic_total,
                ],
            )?;
            Ok(())
        })
    }

    fn update(&self, profile: &Profile) -> AppResult<()> {
        self.insert(profile)
    }

    fn delete(&self, id: &str) -> AppResult<()> {
        self.db.with_connection(|conn| {
            let deleted = conn.execute("DELETE FROM profiles WHERE id = ?1", [id])?;
            if deleted == 0 {
                return Err(AppError::NotFound(format!("Profile {}", id)));
            }
            Ok(())
        })
    }

    fn set_active(&self, id: &str) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "UPDATE profiles SET status = 'Inactive' WHERE status = 'Active'",
                [],
            )?;
            let updated =
                conn.execute("UPDATE profiles SET status = 'Active' WHERE id = ?1", [id])?;
            if updated == 0 {
                return Err(AppError::NotFound(format!("Profile {}", id)));
            }
            Ok(())
        })
    }

    fn deactivate_all(&self) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute("UPDATE profiles SET status = 'Inactive'", [])?;
            Ok(())
        })
    }

    fn count(&self) -> AppResult<usize> {
        self.db.with_connection(|conn| {
            let count: i64 =
                conn.query_row("SELECT COUNT(*) FROM profiles", [], |row| row.get(0))?;
            Ok(count as usize)
        })
    }
}

// Helper: parse profile type from DB string
fn parse_profile_type(s: &str) -> ProfileType {
    match s {
        "Subscription" => ProfileType::Subscription,
        "WireGuard" => ProfileType::WireGuard,
        "VLESS" => ProfileType::Vless,
        "Clash Meta" => ProfileType::ClashMeta,
        _ => ProfileType::Custom,
    }
}

fn profile_type_str(t: &ProfileType) -> &'static str {
    match t {
        ProfileType::Subscription => "Subscription",
        ProfileType::WireGuard => "WireGuard",
        ProfileType::Vless => "VLESS",
        ProfileType::ClashMeta => "Clash Meta",
        ProfileType::Custom => "Custom",
    }
}

fn parse_profile_status(s: &str) -> ProfileStatus {
    match s {
        "Active" => ProfileStatus::Active,
        "Inactive" => ProfileStatus::Inactive,
        "Error" => ProfileStatus::Error,
        _ => ProfileStatus::Inactive,
    }
}

fn profile_status_str(s: &ProfileStatus) -> &'static str {
    match s {
        ProfileStatus::Active => "Active",
        ProfileStatus::Inactive => "Inactive",
        ProfileStatus::Error => "Error",
    }
}
