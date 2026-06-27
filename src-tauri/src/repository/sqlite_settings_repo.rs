use std::collections::HashMap;
use std::sync::Arc;

use crate::repository::SettingsRepository;
use crate::storage::Database;
use crate::utils::AppResult;

pub struct SqliteSettingsRepository {
    db: Arc<Database>,
}

impl SqliteSettingsRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl SettingsRepository for SqliteSettingsRepository {
    fn get(&self, key: &str) -> AppResult<Option<String>> {
        self.db.with_connection(|conn| {
            let result: Option<String> = conn
                .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
                    row.get(0)
                })
                .ok();
            Ok(result)
        })
    }

    fn set(&self, key: &str, value: &str) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                rusqlite::params![key, value],
            )?;
            Ok(())
        })
    }

    fn get_all(&self) -> AppResult<HashMap<String, String>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare("SELECT key, value FROM settings ORDER BY key")?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?;

            let mut map = HashMap::new();
            for row in rows {
                let (k, v) = row?;
                map.insert(k, v);
            }
            Ok(map)
        })
    }

    fn delete(&self, key: &str) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute("DELETE FROM settings WHERE key = ?1", [key])?;
            Ok(())
        })
    }
}
