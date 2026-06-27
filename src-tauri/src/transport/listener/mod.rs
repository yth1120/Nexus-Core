#![allow(clippy::module_inception)]

use async_trait::async_trait;
use serde::Serialize;

use crate::utils::AppResult;

/// The kind of transport listener.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ListenerKind {
    Tcp,
    Udp,
}

/// Abstraction over a transport-layer listener (TCP / UDP socket).
#[async_trait]
pub trait Listener: Send + Sync {
    async fn start(&self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;
    fn kind(&self) -> ListenerKind;
}

// ----- no-op implementations -----

#[derive(Debug, Default)]
pub struct TcpListener;

#[async_trait]
impl Listener for TcpListener {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("TcpListener::start (no-op)");
        Ok(())
    }
    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("TcpListener::stop (no-op)");
        Ok(())
    }
    fn kind(&self) -> ListenerKind {
        ListenerKind::Tcp
    }
}

#[derive(Debug, Default)]
pub struct UdpListener;

#[async_trait]
impl Listener for UdpListener {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("UdpListener::start (no-op)");
        Ok(())
    }
    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("UdpListener::stop (no-op)");
        Ok(())
    }
    fn kind(&self) -> ListenerKind {
        ListenerKind::Udp
    }
}
