use std::sync::Arc;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::packet::Packet;
use super::packet_context::PacketContext;
use super::packet_pipeline::PacketPipeline;

/// Bridges connection ingress → pipeline → outbound.
///
/// Publishes `packet:received` before pipeline execution and `packet:sent`
/// after. The pipeline itself produces `packet:processed` events via
/// the log processor.
pub struct PacketDispatcher {
    pipeline: Arc<PacketPipeline>,
    context: Arc<RuntimeContext>,
}

impl PacketDispatcher {
    pub fn new(pipeline: Arc<PacketPipeline>, context: Arc<RuntimeContext>) -> Self {
        Self { pipeline, context }
    }

    /// Dispatch a packet through the pipeline, publishing lifecycle events.
    pub async fn dispatch(&self, ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
        let size = packet.len() as u64;
        self.context.publish(AppEvent::PacketReceived {
            connection_id: ctx.connection_id.to_string(),
            size,
        });

        let result = self.pipeline.execute(ctx, packet).await?;

        self.context.publish(AppEvent::PacketSent {
            connection_id: ctx.connection_id.to_string(),
            size: result.len() as u64,
        });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[tokio::test]
    async fn dispatch_runs_pipeline_and_publishes_events() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let pipeline = Arc::new(PacketPipeline::new());
        let dispatcher = PacketDispatcher::new(pipeline, rt);

        let ctx = PacketContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2),
            "TCP",
        );
        let pkt = Packet::Tcp(Bytes::from_static(b"dispatch-test"));
        let result = dispatcher.dispatch(&ctx, pkt).await?;
        assert_eq!(result.payload(), b"dispatch-test");
        Ok(())
    }
}
