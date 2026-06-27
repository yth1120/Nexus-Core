use std::time::{Duration, Instant};

use serde::Serialize;

use crate::utils::AppResult;

/// A single benchmark result.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_duration_ms: f64,
    pub avg_duration_ms: f64,
    pub ops_per_sec: f64,
}

/// Complete benchmark report.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkReport {
    pub results: Vec<BenchmarkResult>,
    pub timestamp: i64,
}

/// In-process benchmark runner using `std::time::Instant`.
pub struct BenchmarkRunner;

impl BenchmarkRunner {
    /// Measure rule matching throughput using simple string matching.
    pub fn bench_rule_matching(iterations: usize) -> BenchmarkResult {
        let rules: Vec<(&str, &str)> = vec![
            ("google.com", "DOMAIN-SUFFIX"),
            ("telegram.org", "DOMAIN-KEYWORD"),
            ("192.168.0.0/16", "IP-CIDR"),
            ("netflix.com", "DOMAIN"),
            ("github.com", "DOMAIN-SUFFIX"),
        ];
        let test_domains: Vec<&str> = vec![
            "mail.google.com",
            "web.telegram.org",
            "192.168.1.1",
            "netflix.com",
            "api.github.com",
            "unknown.example.com",
        ];

        let start = Instant::now();
        for _ in 0..iterations {
            for domain in &test_domains {
                for (rule_payload, _rule_type) in &rules {
                    let _ = domain.contains(rule_payload);
                }
            }
        }
        let elapsed = start.elapsed();

        let total_ops = iterations * test_domains.len() * rules.len();
        Self::make_result("rule_matching", total_ops, elapsed)
    }

    /// Measure startup-like initialization time.
    pub fn bench_startup_like() -> BenchmarkResult {
        let iterations = 100;
        let start = Instant::now();
        for _ in 0..iterations {
            // Simulate initialization: memory alloc, string parsing, hashmap creation
            let mut map = std::collections::HashMap::new();
            map.insert("key1", "value1");
            map.insert("key2", "value2");
            let _ = "1.2.3.4".parse::<std::net::IpAddr>();
            let _ = serde_json::json!({"name": "test", "port": 7890});
        }
        let elapsed = start.elapsed();
        Self::make_result("startup_like", iterations, elapsed)
    }

    /// Measure DNS-like domain processing throughput.
    pub fn bench_dns_throughput(query_count: usize) -> BenchmarkResult {
        let domains: Vec<&str> = vec![
            "google.com",
            "github.com",
            "telegram.org",
            "netflix.com",
            "apple.com",
        ];

        let start = Instant::now();
        for _ in 0..query_count {
            for domain in &domains {
                let lower = domain.to_lowercase();
                let _has_suffix = lower.ends_with(".com");
                let _len = lower.len();
            }
        }
        let elapsed = start.elapsed();

        let total_ops = query_count * domains.len();
        Self::make_result("dns_throughput", total_ops, elapsed)
    }

    /// Run all benchmarks.
    pub fn run_all() -> AppResult<BenchmarkReport> {
        let results = vec![
            Self::bench_rule_matching(1000),
            Self::bench_startup_like(),
            Self::bench_dns_throughput(1000),
        ];

        Ok(BenchmarkReport {
            results,
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    fn make_result(name: &str, total_ops: usize, elapsed: Duration) -> BenchmarkResult {
        let total_ms = elapsed.as_secs_f64() * 1000.0;
        let avg_ms = if total_ops > 0 {
            total_ms / total_ops as f64
        } else {
            0.0
        };
        let ops_per_sec = if elapsed.as_secs_f64() > 0.0 {
            total_ops as f64 / elapsed.as_secs_f64()
        } else {
            f64::INFINITY
        };
        BenchmarkResult {
            name: name.into(),
            iterations: total_ops,
            total_duration_ms: total_ms,
            avg_duration_ms: avg_ms,
            ops_per_sec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_rule_matching_works() {
        let result = BenchmarkRunner::bench_rule_matching(100);
        assert_eq!(result.name, "rule_matching");
        assert!(result.ops_per_sec > 0.0);
        assert!(result.total_duration_ms >= 0.0);
    }

    #[test]
    fn bench_startup_like_works() {
        let result = BenchmarkRunner::bench_startup_like();
        assert!(result.iterations > 0);
    }

    #[test]
    fn run_all_returns_report() -> AppResult<()> {
        let report = BenchmarkRunner::run_all()?;
        assert!(!report.results.is_empty());
        Ok(())
    }
}
