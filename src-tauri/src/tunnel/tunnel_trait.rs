use async_trait::async_trait;
use serde::Serialize;

use crate::utils::AppResult;

/// The kind of tunnel/traffic-capture backend an adapter provides.
///
/// Phase 3 ships only the enum plus a no-op adapter; real System-proxy /
/// TUN / TAP integration arrives in Phase 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TunnelKind {
    System,
    Tun,
    Tap,
}

/// Abstraction over a traffic-capture tunnel.
///
/// Implementations are held behind `Arc<dyn TunnelAdapter>` so the active
/// capture strategy can be swapped at runtime. All methods are no-ops in
/// Phase 3 — no device is created and no packets are processed.
#[async_trait]
pub trait TunnelAdapter: Send + Sync {
    /// Bring up the tunnel (create device / enable capture). No-op until Phase 4.
    async fn start(&self) -> AppResult<()>;

    /// Tear down the tunnel. No-op until Phase 4.
    async fn stop(&self) -> AppResult<()>;

    /// The kind of tunnel this adapter implements.
    fn kind(&self) -> TunnelKind;
}

/// Default no-op tunnel adapter used until a real backend is wired in.
#[derive(Debug, Default)]
pub struct NullTunnelAdapter;

#[async_trait]
impl TunnelAdapter for NullTunnelAdapter {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("NullTunnelAdapter::start (no-op)");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("NullTunnelAdapter::stop (no-op)");
        Ok(())
    }

    fn kind(&self) -> TunnelKind {
        TunnelKind::System
    }
}
