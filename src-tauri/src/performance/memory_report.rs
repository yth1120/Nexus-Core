use serde::Serialize;

use super::profiler::MemorySnapshot;

/// A generated memory analysis report.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryReport {
    pub timestamp: i64,
    pub sample_count: usize,
    pub current_rss_mb: f64,
    pub peak_rss_mb: f64,
    pub average_rss_mb: f64,
    pub virtual_mb: f64,
    pub trend: String, // "stable", "increasing", "decreasing"
}

/// Generates memory reports from snapshot data.
pub struct MemoryProfiler;

impl MemoryProfiler {
    /// Generate a memory report from a set of snapshots.
    pub fn generate_report(samples: &[MemorySnapshot]) -> MemoryReport {
        let sample_count = samples.len();
        let current = samples.last();
        let current_rss_mb = current.map(|s| s.rss_mb).unwrap_or(0.0);
        let peak_rss_mb = samples.iter().map(|s| s.rss_mb).fold(0.0_f64, f64::max);
        let average_rss_mb = if sample_count > 0 {
            samples.iter().map(|s| s.rss_mb).sum::<f64>() / sample_count as f64
        } else {
            0.0
        };
        let virtual_mb = current.map(|s| s.virtual_mb).unwrap_or(0.0);

        // Detect trend
        let trend = if sample_count >= 3 {
            let first_avg: f64 = samples[..3].iter().map(|s| s.rss_mb).sum::<f64>() / 3.0;
            let last_avg: f64 = samples[sample_count - 3..]
                .iter()
                .map(|s| s.rss_mb)
                .sum::<f64>()
                / 3.0;
            let delta = last_avg - first_avg;
            if delta > 5.0 {
                "increasing"
            } else if delta < -5.0 {
                "decreasing"
            } else {
                "stable"
            }
        } else {
            "stable"
        };

        MemoryReport {
            timestamp: chrono::Utc::now().timestamp_millis(),
            sample_count,
            current_rss_mb,
            peak_rss_mb,
            average_rss_mb,
            virtual_mb,
            trend: trend.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_report_from_samples() {
        let samples = vec![
            MemorySnapshot {
                timestamp: 1,
                rss_mb: 50.0,
                virtual_mb: 100.0,
            },
            MemorySnapshot {
                timestamp: 2,
                rss_mb: 55.0,
                virtual_mb: 100.0,
            },
            MemorySnapshot {
                timestamp: 3,
                rss_mb: 60.0,
                virtual_mb: 100.0,
            },
            MemorySnapshot {
                timestamp: 4,
                rss_mb: 58.0,
                virtual_mb: 100.0,
            },
        ];

        let report = MemoryProfiler::generate_report(&samples);
        assert_eq!(report.sample_count, 4);
        assert_eq!(report.peak_rss_mb, 60.0);
        assert!(report.current_rss_mb > 0.0);
        assert!(!report.trend.is_empty());
    }

    #[test]
    fn empty_samples_generates_safe_report() {
        let report = MemoryProfiler::generate_report(&[]);
        assert_eq!(report.sample_count, 0);
    }
}
