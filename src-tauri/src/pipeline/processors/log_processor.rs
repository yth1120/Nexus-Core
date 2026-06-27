use async_trait::async_trait;

use crate::event::AppEvent;
use crate::pipeline::Packet;
use crate::pipeline::PacketContext;
use crate::pipeline::PacketProcessor;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

/// Observability processor — logs packet metadata and publishes
/// `PacketProcessed` events. Returns the packet unchanged.
pub struct LogProcessor {
    context: RuntimeContext,
}

impl LogProcessor {
    pub fn new(context: RuntimeContext) -> Self {
        Self { context }
    }
}

#[async_trait]
impl PacketProcessor for LogProcessor {
    async fn process(&self, ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
        let size = packet.len();
        tracing::debug!(
            "pkt conn={} src={} dst={} size={}",
            ctx.connection_id,
            ctx.source,
            ctx.destination,
            size,
        );
        self.context.publish(AppEvent::PacketProcessed {
            connection_id: ctx.connection_id.to_string(),
            size: size as u64,
        });
        Ok(packet)
    }

    fn name(&self) -> &'static str {
        "log"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn passes_packet_through_unchanged() -> AppResult<()> {
        let rt = crate::runtime::RuntimeContext::new_for_test()?;
        let proc = LogProcessor::new((*rt).clone());
        let ctx = PacketContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2),
            "TCP",
        );
        let pkt = Packet::Tcp(Bytes::from_static(b"test-data"));
        let result = proc.process(&ctx, pkt).await?;
        assert_eq!(result.payload(), b"test-data");
        Ok(())
    }
}
