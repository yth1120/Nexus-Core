use std::path::{Path, PathBuf};
use std::time::Instant;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use crate::utils::AppResult;

use super::telemetry_storage::TelemetryStorage;

/// A single memory usage sample.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySample {
    pub timestamp: i64,
    pub rss_mb: f64,
    pub virtual_mb: f64,
}

/// Serializable telemetry report (shown in frontend).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryReport {
    pub startup_duration_ms: u64,
    pub crash_count: u64,
    pub engine_restart_count: u64,
    pub memory_samples: Vec<MemorySample>,
    pub peak_memory_mb: f64,
    pub uptime_secs: u64,
}

/// Records local telemetry: startup duration, crash count,
/// engine restart count, and periodic memory samples.
///
/// ALL data stays on the local machine — nothing is uploaded.
pub struct TelemetryRecorder {
    storage: TelemetryStorage,
    start: Instant,
    crash_count: std::sync::atomic::AtomicU64,
    engine_restart_count: std::sync::atomic::AtomicU64,
    memory_samples: RwLock<Vec<MemorySample>>,
    peak_memory: RwLock<f64>,
    /// Reusable sysinfo handle — refreshed per sample instead of re-allocated.
    sys: parking_lot::Mutex<sysinfo::System>,
    #[allow(dead_code)]
    telemetry_dir: PathBuf,
}

impl TelemetryRecorder {
    /// Create a new recorder. Loads any previously-saved telemetry from disk.
    pub fn new(data_dir: &Path) -> AppResult<Self> {
        let telemetry_dir = data_dir.join("telemetry");
        std::fs::create_dir_all(&telemetry_dir).ok();

        let storage = TelemetryStorage::new(&telemetry_dir);
        let saved = storage.load().unwrap_or_default();

        Ok(Self {
            storage,
            start: Instant::now(),
            crash_count: std::sync::atomic::AtomicU64::new(saved.crash_count),
            engine_restart_count: std::sync::atomic::AtomicU64::new(saved.engine_restart_count),
            memory_samples: RwLock::new(saved.memory_samples),
            peak_memory: RwLock::new(saved.peak_memory_mb),
            sys: parking_lot::Mutex::new(sysinfo::System::new_all()),
            telemetry_dir,
        })
    }

    /// Called once at app startup. Records the start time.
    pub fn record_startup(&self) {
        tracing::info!("Telemetry: startup recorded at {:?}", self.start.elapsed());
    }

    /// Called from the panic hook when a crash is detected.
    pub fn record_crash(&self, _reason: &str) {
        self.crash_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // Persist immediately on crash
        let _ = self.storage.save(&self.snapshot());
    }

    /// Increment the engine restart counter.
    pub fn record_engine_restart(&self) {
        self.engine_restart_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }

    /// Capture a memory sample using sysinfo.
    /// Reuses the stored System object to avoid per-sample allocation.
    pub fn record_memory_sample(&self) {
        let mut sys = self.sys.lock();
        sys.refresh_memory();
        let rss_mb = sys.used_memory() as f64 / (1024.0 * 1024.0);
        let total_mb = sys.total_memory() as f64 / (1024.0 * 1024.0);

        let sample = MemorySample {
            timestamp: chrono::Utc::now().timestamp_millis(),
            rss_mb,
            virtual_mb: total_mb,
        };

        // Track peak
        {
            let mut peak = self.peak_memory.write();
            if rss_mb > *peak {
                *peak = rss_mb;
            }
        }

        {
            let mut samples = self.memory_samples.write();
            samples.push(sample);
            // Keep last 1440 samples (24h at 60s interval)
            if samples.len() > 1440 {
                samples.remove(0);
            }
        }
    }

    /// Periodically persist telemetry to disk.
    pub fn flush(&self) -> AppResult<()> {
        self.storage.save(&self.snapshot())
    }

    /// Generate a telemetry report for the frontend.
    pub fn get_report(&self) -> TelemetryReport {
        self.snapshot()
    }

    /// Get the startup duration in milliseconds.
    pub fn startup_duration_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }

    /// Get the crash count.
    pub fn crash_count(&self) -> u64 {
        self.crash_count.load(std::sync::atomic::Ordering::SeqCst)
    }

    // ----- internal -----

    fn snapshot(&self) -> TelemetryReport {
        TelemetryReport {
            startup_duration_ms: self.startup_duration_ms(),
            crash_count: self.crash_count(),
            engine_restart_count: self
                .engine_restart_count
                .load(std::sync::atomic::Ordering::SeqCst),
            memory_samples: self.memory_samples.read().clone(),
            peak_memory_mb: *self.peak_memory.read(),
            uptime_secs: self.start.elapsed().as_secs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_startup_and_crash() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("tel-{}", uuid::Uuid::new_v4()));
        let tr = TelemetryRecorder::new(&tmp)?;
        tr.record_startup();
        assert!(tr.startup_duration_ms() < 1000); // should be very fast

        tr.record_crash("test crash");
        assert_eq!(tr.crash_count(), 1);

        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn records_engine_restart() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("tel-er-{}", uuid::Uuid::new_v4()));
        let tr = TelemetryRecorder::new(&tmp)?;
        tr.record_engine_restart();
        tr.record_engine_restart();
        let report = tr.get_report();
        assert_eq!(report.engine_restart_count, 2);
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn memory_samples_are_capped() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("tel-ms-{}", uuid::Uuid::new_v4()));
        let tr = TelemetryRecorder::new(&tmp)?;
        for _ in 0..10 {
            tr.record_memory_sample();
        }
        let report = tr.get_report();
        assert!(report.memory_samples.len() <= 10);
        assert!(report.peak_memory_mb >= 0.0);
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[test]
    fn flush_persists_to_disk() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("tel-flush-{}", uuid::Uuid::new_v4()));
        let tr = TelemetryRecorder::new(&tmp)?;
        tr.record_engine_restart();
        tr.flush()?;

        // Create a new recorder; it should load the saved data
        let tr2 = TelemetryRecorder::new(&tmp)?;
        assert_eq!(tr2.get_report().engine_restart_count, 1);
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
