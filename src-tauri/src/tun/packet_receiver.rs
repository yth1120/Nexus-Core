use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::pipeline::{PacketDispatcher, PacketPipeline};
use crate::utils::AppResult;

use super::tun_device::TunDevice;

/// Reads packets from the TUN device and dispatches them into the pipeline (TUN → Pipeline).
pub struct PacketReceiver {
    device: Arc<dyn TunDevice>,
}

impl PacketReceiver {
    pub fn new(device: Arc<dyn TunDevice>) -> Self {
        Self { device }
    }

    /// Run the receive loop: read → convert → dispatch → repeat.
    pub async fn run(
        &self,
        _pipeline: Arc<PacketPipeline>,
        _dispatcher: Arc<PacketDispatcher>,
        token: CancellationToken,
    ) -> AppResult<()> {
        loop {
            tokio::select! {
                _ = token.cancelled() => break,
                result = self.device.read_packet() => {
                    match result {
                        Ok(tp) => {
                            let _pkt = tp.into_packet();
                            // In Phase 9: dispatch through pipeline.
                            tracing::debug!("TUN packet received: {} bytes", _pkt.len());
                        }
                        Err(e) => {
                            tracing::debug!("TUN read error (expected for null device): {e}");
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tun::tun_device::NullTunDevice;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn receiver_exits_on_null_device_error() -> AppResult<()> {
        let d: Arc<dyn TunDevice> = Arc::new(NullTunDevice);
        let recv = PacketReceiver::new(d);
        let pipeline = Arc::new(PacketPipeline::new());
        let dispatcher = Arc::new(PacketDispatcher::new(
            pipeline.clone(),
            crate::runtime::RuntimeContext::new_for_test()?,
        ));
        let token = CancellationToken::new();
        // Should exit immediately because NullTunDevice returns Unsupported.
        recv.run(pipeline, dispatcher, token).await?;
        Ok(())
    }
}
