use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;

use crate::pipeline::packet::Packet;
use crate::pipeline::packet_context::PacketContext;
use crate::pipeline::packet_processor::PacketProcessor;
use crate::utils::AppResult;

/// Sidecar processor — records packet count and byte totals using lock-free
/// atomics. Returns the packet unchanged.
pub struct StatisticsProcessor {
    pub packet_count: AtomicU64,
    pub bytes_in: AtomicU64,
    pub bytes_out: AtomicU64,
}

impl StatisticsProcessor {
    pub fn new() -> Self {
        Self {
            packet_count: AtomicU64::new(0),
            bytes_in: AtomicU64::new(0),
            bytes_out: AtomicU64::new(0),
        }
    }

    pub fn snapshot(&self) -> (u64, u64, u64) {
        (
            self.packet_count.load(Ordering::Relaxed),
            self.bytes_in.load(Ordering::Relaxed),
            self.bytes_out.load(Ordering::Relaxed),
        )
    }
}

impl Default for StatisticsProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PacketProcessor for StatisticsProcessor {
    async fn process(&self, _ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
        let n = packet.len() as u64;
        self.packet_count.fetch_add(1, Ordering::Relaxed);
        self.bytes_in.fetch_add(n, Ordering::Relaxed);
        self.bytes_out.fetch_add(n, Ordering::Relaxed);
        Ok(packet)
    }

    fn name(&self) -> &'static str {
        "statistics"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn counts_packets_and_bytes() -> AppResult<()> {
        let proc = StatisticsProcessor::new();
        let ctx = PacketContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2),
            "TCP",
        );

        let p1 = Packet::Tcp(Bytes::from_static(b"abc"));
        let p2 = Packet::Tcp(Bytes::from_static(b"12345"));
        let p3 = Packet::Tcp(Bytes::from_static(b"xy"));

        proc.process(&ctx, p1).await?;
        proc.process(&ctx, p2).await?;
        proc.process(&ctx, p3).await?;

        let (count, bytes_in, bytes_out) = proc.snapshot();
        assert_eq!(count, 3);
        assert_eq!(bytes_in, 10);
        assert_eq!(bytes_out, 10);
        Ok(())
    }
}
