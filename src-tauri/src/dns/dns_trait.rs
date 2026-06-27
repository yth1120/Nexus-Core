use std::net::IpAddr;

use async_trait::async_trait;
use serde::Serialize;

use crate::utils::{AppError, AppResult};

/// The kind of DNS resolution backend an adapter provides.
///
/// Phase 3 ships only the enum plus a no-op resolver; real System DNS / DoH /
/// DoT / Fake-IP resolution arrives in Phase 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DnsKind {
    System,
    Doh,
    Dot,
    FakeIp,
}

/// Abstraction over a DNS resolver.
///
/// Implementations are held behind `Arc<dyn DnsResolver>` so the active resolver
/// can be swapped at runtime. No real resolution happens in Phase 3.
#[async_trait]
pub trait DnsResolver: Send + Sync {
    /// Resolve a host to a set of IP addresses. Stubbed in Phase 3.
    async fn resolve(&self, host: &str) -> AppResult<Vec<IpAddr>>;

    /// The kind of resolver this adapter implements.
    fn kind(&self) -> DnsKind;
}

/// Default resolver that performs no resolution.
///
/// Returns [`AppError::Unsupported`] so callers can detect the stub rather than
/// silently receiving an empty answer. No real DNS query is ever issued.
#[derive(Debug, Default)]
pub struct NullDnsResolver;

#[async_trait]
impl DnsResolver for NullDnsResolver {
    async fn resolve(&self, host: &str) -> AppResult<Vec<IpAddr>> {
        Err(AppError::Unsupported(format!(
            "dns resolution not implemented (host: {host})"
        )))
    }

    fn kind(&self) -> DnsKind {
        DnsKind::System
    }
}
