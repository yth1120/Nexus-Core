use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// Monitors real system resources: CPU, memory, processes, uptime.
///
/// Uses the `sysinfo` crate for cross-platform system information.
/// CPU usage requires two samples spaced apart, handled internally.
pub struct SystemMonitor {
    sys: System,
    prev_cpu: f64,
    initialized: bool,
}

impl SystemMonitor {
    pub fn new() -> Self {
        // Create system with CPU refresh for accurate usage calculation
        let sys =
            System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
        Self {
            sys,
            prev_cpu: 0.0,
            initialized: false,
        }
    }

    /// Refresh all system information. Call before reading values.
    pub fn refresh(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.sys.refresh_processes();
    }

    /// CPU usage as percentage (0.0 - 100.0), averaged across all cores.
    ///
    /// On first call returns 0.0; subsequent calls return the delta
    /// since the previous refresh.
    pub fn cpu_usage(&mut self) -> f64 {
        if !self.initialized {
            // First sample: store baseline and return 0
            self.sys.refresh_cpu();
            self.prev_cpu = self
                .sys
                .cpus()
                .iter()
                .map(|cpu| cpu.cpu_usage() as f64)
                .sum::<f64>()
                / self.sys.cpus().len().max(1) as f64;
            self.initialized = true;
            return 0.0;
        }

        self.sys.refresh_cpu();
        let current = self
            .sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage() as f64)
            .sum::<f64>()
            / self.sys.cpus().len().max(1) as f64;

        // Smooth between previous and current
        let usage = (self.prev_cpu + current) / 2.0;
        self.prev_cpu = current;
        usage.clamp(0.0, 100.0)
    }

    /// Memory usage in MB.
    pub fn memory_usage(&self) -> f64 {
        let used = self.sys.used_memory(); // bytes
        used as f64 / (1024.0 * 1024.0)
    }

    /// Total system memory in MB.
    pub fn total_memory(&self) -> f64 {
        let total = self.sys.total_memory();
        total as f64 / (1024.0 * 1024.0)
    }

    /// Number of running processes.
    pub fn process_count(&self) -> usize {
        self.sys.processes().len()
    }

    /// System uptime in seconds.
    pub fn uptime(&self) -> u64 {
        System::uptime()
    }

    /// Refresh CPU specifically (more expensive, for accurate usage).
    pub fn refresh_cpu(&mut self) {
        self.sys.refresh_cpu();
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
