use async_trait::async_trait;
use serde::Serialize;

use crate::utils::AppResult;

/// The kind of inbound proxy listener an adapter provides.
///
/// Phase 3 ships only the enum plus a no-op adapter; the real listeners
/// (HTTP / SOCKS5 / Mixed / Transparent) arrive in Phase 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProxyKind {
    Http,
    Socks5,
    Mixed,
    Transparent,
}

/// Abstraction over an inbound proxy listener.
///
/// Implementations are held behind `Arc<dyn ProxyAdapter>` so the active proxy
/// strategy can be swapped at runtime. All methods are no-ops in Phase 3 — no
/// socket is opened and no data is forwarded.
#[async_trait]
pub trait ProxyAdapter: Send + Sync {
    /// Start accepting inbound connections. No-op until Phase 4.
    async fn start(&self) -> AppResult<()>;

    /// Stop the listener and release its port. No-op until Phase 4.
    async fn stop(&self) -> AppResult<()>;

    /// The kind of proxy this adapter implements.
    fn kind(&self) -> ProxyKind;
}

/// Default no-op proxy adapter used until a real listener is wired in.
///
/// Implements [`ProxyAdapter`] with success-returning stubs.
#[derive(Debug, Default)]
pub struct NullProxyAdapter;

#[async_trait]
impl ProxyAdapter for NullProxyAdapter {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("NullProxyAdapter::start (no-op)");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("NullProxyAdapter::stop (no-op)");
        Ok(())
    }

    fn kind(&self) -> ProxyKind {
        ProxyKind::Mixed
    }
}
