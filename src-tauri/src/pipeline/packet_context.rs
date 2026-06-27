use std::net::SocketAddr;

use chrono::Utc;
use uuid::Uuid;

/// Metadata carried alongside a [`Packet`](super::packet::Packet) through the
/// processor chain.
#[derive(Debug, Clone)]
pub struct PacketContext {
    pub connection_id: Uuid,
    pub source: SocketAddr,
    pub destination: SocketAddr,
    pub protocol: String,
    pub created_at: chrono::DateTime<Utc>,
}

impl PacketContext {
    pub fn new(source: SocketAddr, dest: SocketAddr, protocol: &str) -> Self {
        Self {
            connection_id: Uuid::new_v4(),
            source,
            destination: dest,
            protocol: protocol.to_string(),
            created_at: Utc::now(),
        }
    }
}
