use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::RwLock;
use serde::Serialize;

/// The lifecycle stage of a proxied connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    Created,
    Connected,
    Closing,
    Closed,
    Error,
}

/// A tracked active proxy connection with byte counters and timing.
pub struct ActiveConnection {
    pub id: String,
    pub state: RwLock<ConnectionState>,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub bytes_in: AtomicU64,
    pub bytes_out: AtomicU64,
    pub created_at: i64,
    pub duration_ms: AtomicU64,
}

impl ActiveConnection {
    pub fn new(id: &str, protocol: &str, source: &str, destination: &str) -> Self {
        Self {
            id: id.to_string(),
            state: RwLock::new(ConnectionState::Created),
            protocol: protocol.to_string(),
            source: source.to_string(),
            destination: destination.to_string(),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
            created_at: chrono::Utc::now().timestamp_millis(),
            duration_ms: AtomicU64::new(0),
        }
    }

    pub fn add_bytes(&self, sent: u64, received: u64) {
        self.bytes_out.fetch_add(sent, Ordering::Relaxed);
        self.bytes_in.fetch_add(received, Ordering::Relaxed);
    }

    pub fn close(&self) {
        *self.state.write() = ConnectionState::Closed;
        let now = chrono::Utc::now().timestamp_millis();
        self.duration_ms
            .store((now - self.created_at) as u64, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle() {
        let conn = ActiveConnection::new("c1", "HTTP", "127.0.0.1:1", "example.com:443");
        assert_eq!(*conn.state.read(), ConnectionState::Created);
        conn.add_bytes(100, 200);
        assert_eq!(conn.bytes_out.load(Ordering::Relaxed), 100);
        assert_eq!(conn.bytes_in.load(Ordering::Relaxed), 200);
        conn.close();
        assert_eq!(*conn.state.read(), ConnectionState::Closed);
        assert!(conn.created_at > 0);
    }
}
