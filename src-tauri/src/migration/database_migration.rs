use crate::storage::Database;
use crate::utils::AppResult;

pub struct DatabaseMigrator;

impl DatabaseMigrator {
    pub fn run_v2(db: &Database) -> AppResult<()> {
        db.with_connection(|conn| {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS subscriptions (
                    id TEXT PRIMARY KEY, name TEXT NOT NULL, url TEXT NOT NULL,
                    enabled INTEGER DEFAULT 1, auto_update INTEGER DEFAULT 1,
                    last_updated INTEGER
                );",
            )?;
            Ok(())
        })
    }

    pub fn run_v3(db: &Database) -> AppResult<()> {
        db.with_connection(|conn| {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS rulesets (
                    id TEXT PRIMARY KEY, url TEXT NOT NULL, etag TEXT,
                    rule_count INTEGER DEFAULT 0, last_downloaded INTEGER
                );",
            )?;
            Ok(())
        })
    }
}
