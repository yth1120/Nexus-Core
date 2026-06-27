// Traffic monitor is currently implemented in core/runtime.rs.
// This module is reserved for future dedicated traffic monitoring logic,
// including real network interface polling, TUN statistics collection,
// and per-connection bandwidth tracking.

/// Placeholder for future traffic monitor implementation.
/// When real networking is added, this module will contain:
/// - Network interface stats polling
/// - TUN device traffic counting
/// - Per-connection bandwidth aggregation
/// - Traffic quota enforcement
pub struct TrafficMonitor;

impl TrafficMonitor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TrafficMonitor {
    fn default() -> Self {
        Self::new()
    }
}
