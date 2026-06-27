use async_trait::async_trait;

use crate::utils::{AppError, AppResult};

use super::tun_packet::TunPacket;

#[async_trait]
pub trait TunDevice: Send + Sync {
    async fn create(&self, _name: &str, _mtu: u16) -> AppResult<()>;
    async fn close(&self) -> AppResult<()>;
    async fn read_packet(&self) -> AppResult<TunPacket>;
    async fn write_packet(&self, _packet: &TunPacket) -> AppResult<()>;
}

#[derive(Debug, Default)]
pub struct NullTunDevice;

#[async_trait]
impl TunDevice for NullTunDevice {
    async fn create(&self, _name: &str, _mtu: u16) -> AppResult<()> {
        Err(AppError::Unsupported("tun device not available".into()))
    }
    async fn close(&self) -> AppResult<()> {
        Err(AppError::Unsupported("tun device not available".into()))
    }
    async fn read_packet(&self) -> AppResult<TunPacket> {
        Err(AppError::Unsupported("tun device not available".into()))
    }
    async fn write_packet(&self, _packet: &TunPacket) -> AppResult<()> {
        Err(AppError::Unsupported("tun device not available".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn null_device_returns_unsupported() {
        let d = NullTunDevice;
        assert!(d.create("utun", 1500).await.is_err());
        assert!(d.read_packet().await.is_err());
    }
}
