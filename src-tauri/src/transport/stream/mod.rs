#![allow(clippy::module_inception)]

use async_trait::async_trait;

use crate::utils::AppResult;

/// Abstraction over a transport-layer stream (TCP / UDP socket).
#[async_trait]
pub trait TransportStream: Send + Sync {
    async fn read(&self, _buf: &mut [u8]) -> AppResult<usize>;
    async fn write(&self, _buf: &[u8]) -> AppResult<usize>;
    async fn close(&self) -> AppResult<()>;
}

// ----- no-op implementations -----

#[derive(Debug, Default)]
pub struct TcpStream;

#[async_trait]
impl TransportStream for TcpStream {
    async fn read(&self, _buf: &mut [u8]) -> AppResult<usize> {
        Ok(0)
    }
    async fn write(&self, _buf: &[u8]) -> AppResult<usize> {
        Ok(0)
    }
    async fn close(&self) -> AppResult<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct UdpStream;

#[async_trait]
impl TransportStream for UdpStream {
    async fn read(&self, _buf: &mut [u8]) -> AppResult<usize> {
        Ok(0)
    }
    async fn write(&self, _buf: &[u8]) -> AppResult<usize> {
        Ok(0)
    }
    async fn close(&self) -> AppResult<()> {
        Ok(())
    }
}
