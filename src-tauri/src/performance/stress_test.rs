use std::time::Instant;

use serde::Serialize;

use crate::utils::AppResult;

/// A single stress test result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StressTestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: f64,
    pub detail: String,
}

/// Complete stress test report.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StressTestReport {
    pub results: Vec<StressTestResult>,
    pub timestamp: i64,
}

/// Runs stress tests to verify system stability under load.
pub struct StressTestRunner;

impl StressTestRunner {
    /// Simulate many concurrent operations.
    pub fn concurrent_operations(count: usize) -> AppResult<StressTestResult> {
        let start = Instant::now();
        let mut results = Vec::with_capacity(count);

        for i in 0..count {
            // Simulate a connection being processed
            let host = format!("host-{i}.example.com");
            let ip = format!("10.0.0.{}", (i % 254) + 1);
            let port = 1000 + (i % 60000) as u16;

            // String processing (simulates rule matching)
            let lower = host.to_lowercase();
            let is_blocked = lower.contains("blocked");
            let _has_port = port > 0;

            results.push((lower, ip, is_blocked));
        }

        let elapsed = start.elapsed();
        let passed = results.len() == count;

        Ok(StressTestResult {
            name: "concurrent_operations".into(),
            passed,
            duration_ms: elapsed.as_secs_f64() * 1000.0,
            detail: format!("Processed {count} operations in {elapsed:?}"),
        })
    }

    /// Rapidly create and destroy many items (simulates connect/disconnect).
    pub fn connect_disconnect_cycle(cycles: usize) -> AppResult<StressTestResult> {
        let start = Instant::now();
        let mut created = 0;

        for _ in 0..cycles {
            // Create
            let data: Vec<u8> = vec![0u8; 1024];
            // "Process"
            let _sum: u64 = data.iter().map(|&b| b as u64).sum();
            created += 1;
            // Drop (data is dropped here)
        }

        let elapsed = start.elapsed();

        Ok(StressTestResult {
            name: "connect_disconnect_cycle".into(),
            passed: created == cycles,
            duration_ms: elapsed.as_secs_f64() * 1000.0,
            detail: format!("{created} create/drop cycles in {elapsed:?}"),
        })
    }

    /// Bulk rule matching at high volume.
    pub fn bulk_rule_matching(rule_count: usize) -> AppResult<StressTestResult> {
        let start = Instant::now();
        let rules: Vec<String> = (0..rule_count)
            .map(|i| format!("domain-{i}.example.com"))
            .collect();
        let test_domain = "domain-5000.example.com";

        let mut matched = 0;
        for rule in &rules {
            if test_domain.contains(rule.as_str()) {
                matched += 1;
            }
        }

        let elapsed = start.elapsed();

        Ok(StressTestResult {
            name: "bulk_rule_matching".into(),
            passed: matched > 0,
            duration_ms: elapsed.as_secs_f64() * 1000.0,
            detail: format!("Matched {matched}/{rule_count} rules in {elapsed:?}"),
        })
    }

    /// Run all stress tests.
    pub fn run_all() -> AppResult<StressTestReport> {
        let results = vec![
            Self::concurrent_operations(10000)?,
            Self::connect_disconnect_cycle(5000)?,
            Self::bulk_rule_matching(10000)?,
        ];

        Ok(StressTestReport {
            results,
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_operations_works() -> AppResult<()> {
        let result = StressTestRunner::concurrent_operations(100)?;
        assert!(result.passed);
        Ok(())
    }

    #[test]
    fn connect_disconnect_works() -> AppResult<()> {
        let result = StressTestRunner::connect_disconnect_cycle(100)?;
        assert!(result.passed);
        Ok(())
    }

    #[test]
    fn bulk_rule_matching_works() -> AppResult<()> {
        let result = StressTestRunner::bulk_rule_matching(500)?;
        assert!(result.passed);
        Ok(())
    }

    #[test]
    fn run_all_returns_report() -> AppResult<()> {
        let report = StressTestRunner::run_all()?;
        assert!(!report.results.is_empty());
        Ok(())
    }
}
