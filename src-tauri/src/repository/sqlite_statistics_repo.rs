use std::sync::Arc;

use crate::models::{StatisticsData, TrafficDataPoint};
use crate::repository::StatisticsRepository;
use crate::storage::Database;
use crate::utils::AppResult;

pub struct SqliteStatisticsRepository {
    db: Arc<Database>,
}

impl SqliteStatisticsRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl StatisticsRepository for SqliteStatisticsRepository {
    fn insert_data_point(&self, timestamp: i64, upload: i64, download: i64) -> AppResult<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT INTO statistics_history (timestamp, upload, download)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![timestamp, upload, download],
            )?;
            Ok(())
        })
    }

    fn get_history(&self, since: i64) -> AppResult<Vec<TrafficDataPoint>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT timestamp, upload, download
                 FROM statistics_history
                 WHERE timestamp >= ?1
                 ORDER BY timestamp ASC",
            )?;
            let rows = stmt.query_map([since], |row| {
                Ok(TrafficDataPoint {
                    timestamp: row.get(0)?,
                    upload: row.get(1)?,
                    download: row.get(2)?,
                })
            })?;

            let mut points = Vec::new();
            for row in rows {
                points.push(row?);
            }
            Ok(points)
        })
    }

    fn get_stats_summary(&self) -> AppResult<StatisticsData> {
        self.db.with_connection(|conn| {
            // Today's traffic (last 24h)
            let cutoff_24h = chrono::Utc::now().timestamp_millis() - 24 * 3600 * 1000;
            let today_traffic: i64 = conn
                .query_row(
                    "SELECT COALESCE(SUM(upload + download), 0)
                     FROM statistics_history
                     WHERE timestamp >= ?1",
                    [cutoff_24h],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            // Month traffic (last 30 days)
            let cutoff_30d = chrono::Utc::now().timestamp_millis() - 30 * 24 * 3600 * 1000;
            let month_traffic: i64 = conn
                .query_row(
                    "SELECT COALESCE(SUM(upload + download), 0)
                     FROM statistics_history
                     WHERE timestamp >= ?1",
                    [cutoff_30d],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            // Max speed
            let max_speed: i64 = conn
                .query_row(
                    "SELECT COALESCE(MAX(MAX(upload, download)), 0)
                     FROM statistics_history
                     WHERE timestamp >= ?1",
                    [cutoff_30d],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            Ok(StatisticsData {
                today_traffic,
                month_traffic,
                month_quota: 322_122_547_200, // 300 GB
                max_speed,
                max_speed_date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
                history: Vec::new(), // Populated separately if needed
                daily_averages: Vec::new(),
            })
        })
    }

    fn prune_older_than(&self, timestamp: i64) -> AppResult<usize> {
        self.db.with_connection(|conn| {
            let deleted = conn.execute(
                "DELETE FROM statistics_history WHERE timestamp < ?1",
                [timestamp],
            )?;
            Ok(deleted)
        })
    }
}
