use std::sync::Arc;
use std::time::Duration;

use tokio::time::interval;

use crate::event::{AppEvent, EventBus};
use crate::models::Connection;
use crate::platform::{PlatformManager, SystemConnection};

/// Polls real system connections via PlatformManager and publishes events.
///
/// Runs as a background task with a configurable interval.
/// Gracefully degrades on platforms that don't support connection enumeration.
pub struct ConnectionMonitor {
    platform: Arc<dyn PlatformManager>,
    event_bus: EventBus,
}

impl ConnectionMonitor {
    pub fn new(platform: Arc<dyn PlatformManager>, event_bus: EventBus) -> Self {
        Self {
            platform,
            event_bus,
        }
    }

    /// Poll real connections and publish `ConnectionUpdate` event.
    pub fn tick(&self) {
        match self.platform.get_active_connections() {
            Ok(system_connections) => {
                let connections: Vec<Connection> = system_connections
                    .into_iter()
                    .map(map_to_connection)
                    .collect();

                self.event_bus
                    .publish(AppEvent::ConnectionUpdate { connections });
            }
            Err(crate::utils::AppError::Unsupported(_)) => {
                // Graceful degradation: platform doesn't support connection enumeration
                tracing::debug!("Connection monitoring not supported on this platform");
            }
            Err(e) => {
                tracing::warn!("Connection monitoring error: {}", e);
            }
        }
    }

    /// Run the monitor as a background task with 1-second interval.
    /// Returns immediately; the actual work runs in a tokio task.
    pub fn run(self, shutdown: Arc<std::sync::atomic::AtomicBool>) {
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(1));
            loop {
                if shutdown.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
                ticker.tick().await;
                self.tick();
            }
            tracing::debug!("Connection monitor stopped");
        });
    }
}

/// Map a platform SystemConnection to the domain Connection model.
fn map_to_connection(sc: SystemConnection) -> Connection {
    Connection {
        id: format!("sys-{}-{}", sc.pid, sc.source.replace(['.', ':'], "-")),
        process: sc.process_name,
        source: sc.source,
        destination: sc.destination,
        rule: "DIRECT".to_string(),
        network: match sc.protocol.as_str() {
            "TCP" => crate::models::NetworkProtocol::TCP,
            _ => crate::models::NetworkProtocol::UDP,
        },
        upload: sc.upload_bytes,
        download: sc.download_bytes,
        duration: sc.duration_secs as f64,
        created_at: chrono::Utc::now().timestamp_millis(),
    }
}
