use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthReport {
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub thread_count: usize,
    pub connections: usize,
    pub engine_status: String,
    pub uptime_secs: u64,
}

impl HealthReport {
    pub fn new() -> Self {
        let sys = sysinfo::System::new_all();
        Self {
            memory_mb: sys.used_memory() as f64 / (1024.0 * 1024.0),
            cpu_percent: 0.0,
            thread_count: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(0),
            connections: 0,
            engine_status: "unknown".into(),
            uptime_secs: sysinfo::System::uptime(),
        }
    }
}

impl Default for HealthReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_report() {
        let r = HealthReport::new();
        assert!(r.thread_count > 0);
    }
}
