use serde::Serialize;
use sysinfo::Networks;

/// Per-interface network speed measurement.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSpeed {
    pub interface: String,
    pub upload_speed: f64,   // bytes/sec
    pub download_speed: f64, // bytes/sec
}

/// Monitors network traffic via sysinfo's network interface counters.
///
/// Calculates speeds by comparing cumulative byte counters between ticks.
/// Gracefully degrades if platform doesn't support network stats.
pub struct NetworkMonitor {
    prev_rx: u64,
    prev_tx: u64,
    prev_time: Option<std::time::Instant>,
    interfaces: Vec<String>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let interfaces: Vec<String> = networks.keys().cloned().collect();
        let (rx, tx) = total_bytes(&networks);

        Self {
            prev_rx: rx,
            prev_tx: tx,
            prev_time: Some(std::time::Instant::now()),
            interfaces,
        }
    }

    /// Refresh and calculate current upload/download speeds.
    /// Returns per-interface speeds, or an empty vec if unsupported.
    pub fn get_speeds(&mut self) -> Vec<NetworkSpeed> {
        let networks = Networks::new_with_refreshed_list();
        let (rx, tx) = total_bytes(&networks);
        let now = std::time::Instant::now();

        let mut speeds = Vec::new();

        if let Some(prev_time) = self.prev_time {
            let elapsed = now.duration_since(prev_time).as_secs_f64();
            if elapsed > 0.0 {
                let download_speed = (rx.saturating_sub(self.prev_rx)) as f64 / elapsed;
                let upload_speed = (tx.saturating_sub(self.prev_tx)) as f64 / elapsed;

                // If we have total but no per-interface breakdown, report as default
                if !self.interfaces.is_empty() {
                    speeds.push(NetworkSpeed {
                        interface: self
                            .interfaces
                            .first()
                            .cloned()
                            .unwrap_or_else(|| "default".into()),
                        upload_speed,
                        download_speed,
                    });
                }
            }
        }

        self.prev_rx = rx;
        self.prev_tx = tx;
        self.prev_time = Some(now);
        self.interfaces = networks.keys().cloned().collect();

        speeds
    }

    /// Get the list of available network interface names.
    pub fn interfaces(&self) -> &[String] {
        &self.interfaces
    }
}

impl Default for NetworkMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Sum total received/transmitted bytes across all network interfaces.
fn total_bytes(networks: &Networks) -> (u64, u64) {
    let mut rx: u64 = 0;
    let mut tx: u64 = 0;
    for (_name, data) in networks.iter() {
        rx = rx.saturating_add(data.total_received());
        tx = tx.saturating_add(data.total_transmitted());
    }
    (rx, tx)
}
