use async_trait::async_trait;

use crate::utils::AppResult;

use super::packet::Packet;
use super::packet_context::PacketContext;

/// A stage in the packet-processing chain.
///
/// Processors are called in registration order. Each receives the output of the
/// previous processor and may inspect, transform, or pass through the packet.
#[async_trait]
pub trait PacketProcessor: Send + Sync {
    /// Process a packet. The returned packet becomes the input to the next
    /// processor in the chain.
    async fn process(&self, ctx: &PacketContext, packet: Packet) -> AppResult<Packet>;

    /// Human-readable name for this processor (used for remove-by-name).
    fn name(&self) -> &'static str;
}
