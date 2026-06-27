use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReport {
    pub os: String,
    pub cpu_count: usize,
    pub total_memory_mb: u64,
    pub disk_free_mb: u64,
}

impl SystemReport {
    pub fn collect() -> Self {
        use sysinfo::System;
        let sys = System::new_all();
        Self {
            os: std::env::consts::OS.to_string(),
            cpu_count: sys.cpus().len(),
            total_memory_mb: sys.total_memory() / (1024 * 1024),
            disk_free_mb: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collects() {
        let r = SystemReport::collect();
        assert!(!r.os.is_empty());
    }
}
