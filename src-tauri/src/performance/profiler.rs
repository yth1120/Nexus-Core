use serde::Serialize;

use crate::utils::AppResult;

/// A memory snapshot captured at a point in time.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemorySnapshot {
    pub timestamp: i64,
    pub rss_mb: f64,
    pub virtual_mb: f64,
}

/// Profiler captures system-level metrics using `sysinfo`.
pub struct Profiler;

impl Profiler {
    /// Capture a snapshot of current resource usage.
    pub fn snapshot() -> MemorySnapshot {
        let sys = sysinfo::System::new_all();
        let rss_mb = sys.used_memory() as f64 / (1024.0 * 1024.0);
        let virtual_mb = sys.total_memory() as f64 / (1024.0 * 1024.0);
        MemorySnapshot {
            timestamp: chrono::Utc::now().timestamp_millis(),
            rss_mb,
            virtual_mb,
        }
    }

    /// Capture multiple snapshots at `interval_ms` apart.
    pub fn sample(count: usize, interval_ms: u64) -> AppResult<Vec<MemorySnapshot>> {
        let mut samples = Vec::with_capacity(count);
        for _ in 0..count {
            samples.push(Self::snapshot());
            if count > 1 {
                std::thread::sleep(std::time::Duration::from_millis(interval_ms));
            }
        }
        Ok(samples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_has_positive_memory() {
        let snap = Profiler::snapshot();
        assert!(snap.rss_mb >= 0.0);
        assert!(snap.virtual_mb >= 0.0);
    }

    #[test]
    fn sample_returns_correct_count() -> AppResult<()> {
        let samples = Profiler::sample(3, 10)?;
        assert_eq!(samples.len(), 3);
        Ok(())
    }
}
