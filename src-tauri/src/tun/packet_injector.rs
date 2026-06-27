use std::sync::Arc;

use crate::pipeline::Packet;
use crate::utils::AppResult;

use super::tun_device::TunDevice;
use super::tun_packet::TunPacket;

/// Writes pipeline packets into the TUN device (Pipeline → TUN).
pub struct PacketInjector {
    device: Arc<dyn TunDevice>,
}

impl PacketInjector {
    pub fn new(device: Arc<dyn TunDevice>) -> Self {
        Self { device }
    }

    pub async fn inject(&self, packet: TunPacket) -> AppResult<()> {
        self.device.write_packet(&packet).await
    }

    /// Convert a pipeline Packet to TunPacket and inject.
    pub async fn inject_pipeline(&self, packet: &Packet) -> AppResult<()> {
        let tp = TunPacket::from_packet(packet)?;
        self.inject(tp).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tun::tun_device::NullTunDevice;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn inject_returns_unsupported_for_null_device() -> AppResult<()> {
        let d: Arc<dyn TunDevice> = Arc::new(NullTunDevice);
        let inj = PacketInjector::new(d);
        let tp = TunPacket::IPv4(bytes::Bytes::from_static(
            b"\x45\x00\x00\x14\x00\x00\x00\x00\x40\x00\x00\x00\x7f\x00\x00\x01\x7f\x00\x00\x02",
        ));
        assert!(inj.inject(tp).await.is_err());
        Ok(())
    }
}
