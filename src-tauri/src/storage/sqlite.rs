use rusqlite::Connection;
use std::path::Path;

use crate::utils::AppResult;

/// Initialize a raw SQLite connection (reference implementation).
///
/// **Superseded by [`super::database::DatabaseManager`]** which uses an r2d2
/// connection pool with migration support. This function remains as a fallback
/// for simple single-connection use cases.
#[allow(dead_code)]
pub fn init_database(app_dir: &Path) -> AppResult<Connection> {
    let db_path = app_dir.join("nexus_core.db");

    tracing::info!("Initializing database at {:?}", db_path);

    let conn = Connection::open(&db_path)?;

    // Performance and safety pragmas
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;
         PRAGMA synchronous=NORMAL;",
    )?;

    tracing::info!("Database initialized successfully");
    Ok(conn)
}
