use std::path::Path;
use std::sync::Arc;

use parking_lot::Mutex;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::utils::{AppError, AppResult};

/// Wraps a connection pool for SQLite with migration support.
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
    /// Schema version for tracking migrations
    schema_version: Arc<Mutex<i32>>,
}

impl Database {
    /// Open or create the database, set pragmas, and run migrations.
    pub fn open(path: &Path) -> AppResult<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::builder()
            .max_size(4)
            .build(manager)
            .map_err(|e| AppError::Database(format!("Failed to create connection pool: {}", e)))?;

        // Apply pragmas
        {
            let conn = pool.get().map_err(|e| {
                AppError::Database(format!("Failed to get connection from pool: {}", e))
            })?;
            conn.execute_batch(
                "PRAGMA journal_mode=WAL;
                 PRAGMA foreign_keys=ON;
                 PRAGMA synchronous=NORMAL;",
            )?;
        }

        let schema_version = Arc::new(Mutex::new(0));
        let db = Self {
            pool,
            schema_version,
        };

        db.run_migrations()?;

        tracing::info!("Database opened at {:?}", path);
        Ok(db)
    }

    /// Run all pending schema migrations.
    pub fn run_migrations(&self) -> AppResult<()> {
        let conn = self
            .pool
            .get()
            .map_err(|e| AppError::Database(format!("Pool error: {}", e)))?;

        let mut version = *self.schema_version.lock();

        // Migration V1: Initial schema
        if version < 1 {
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS schema_version (
                    version INTEGER PRIMARY KEY
                );

                CREATE TABLE IF NOT EXISTS profiles (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    profile_type TEXT NOT NULL DEFAULT 'Custom',
                    status TEXT NOT NULL DEFAULT 'Inactive',
                    latency INTEGER DEFAULT 0,
                    updated TEXT NOT NULL,
                    config_url TEXT,
                    traffic_used INTEGER,
                    traffic_total INTEGER
                );

                CREATE TABLE IF NOT EXISTS rules (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    rule_type TEXT NOT NULL,
                    payload TEXT NOT NULL,
                    proxy TEXT NOT NULL,
                    enabled INTEGER NOT NULL DEFAULT 1,
                    tags TEXT NOT NULL DEFAULT '[]',
                    created_at INTEGER NOT NULL
                );

                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );

                CREATE TABLE IF NOT EXISTS statistics_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    timestamp INTEGER NOT NULL,
                    upload INTEGER NOT NULL DEFAULT 0,
                    download INTEGER NOT NULL DEFAULT 0
                );

                CREATE INDEX IF NOT EXISTS idx_stats_timestamp
                    ON statistics_history(timestamp);

                INSERT INTO schema_version (version) VALUES (1);",
            )?;
            version = 1;
        }

        *self.schema_version.lock() = version;
        tracing::info!("Database migrations complete (version {})", version);
        Ok(())
    }

    /// Get a connection from the pool (blocking).
    pub fn connection(&self) -> AppResult<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool
            .get()
            .map_err(|e| AppError::Database(format!("Pool error: {}", e)))
    }

    /// Execute a closure with a pooled connection.
    pub fn with_connection<F, T>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&rusqlite::Connection) -> AppResult<T>,
    {
        let conn = self.connection()?;
        f(&conn)
    }
}
