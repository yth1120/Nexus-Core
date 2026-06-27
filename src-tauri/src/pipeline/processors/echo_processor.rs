use async_trait::async_trait;

use crate::pipeline::packet::Packet;
use crate::pipeline::packet_context::PacketContext;
use crate::pipeline::packet_processor::PacketProcessor;
use crate::utils::AppResult;

/// Identity processor — returns the packet unchanged.
///
/// The TCP accept loop handles the actual socket I/O; this processor is a
/// pure-transform pass-through that proves the pipeline works end-to-end.
pub struct EchoProcessor;

#[async_trait]
impl PacketProcessor for EchoProcessor {
    async fn process(&self, _ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
        Ok(packet)
    }

    fn name(&self) -> &'static str {
        "echo"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn echo_returns_identical_packet() -> AppResult<()> {
        let proc = EchoProcessor;
        let ctx = PacketContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 12345),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 54321),
            "TCP",
        );
        let pkt = Packet::Tcp(Bytes::from_static(b"hello world"));
        let result = proc.process(&ctx, pkt).await?;
        assert_eq!(result.payload(), b"hello world");
        Ok(())
    }
}
