use std::net::IpAddr;

use async_trait::async_trait;

use super::dns_trait::{DnsKind, DnsResolver};
use crate::utils::{AppError, AppResult};

/// Real DNS resolver using the OS resolver via `tokio::net::lookup_host`.
pub struct SystemResolver;

#[async_trait]
impl DnsResolver for SystemResolver {
    async fn resolve(&self, host: &str) -> AppResult<Vec<IpAddr>> {
        let addrs: Vec<IpAddr> = tokio::net::lookup_host(format!("{host}:0"))
            .await
            .map_err(|e| AppError::Io(format!("dns lookup failed for {host}: {e}")))?
            .map(|sa| sa.ip())
            .collect();
        if addrs.is_empty() {
            return Err(AppError::NotFound(format!("no addresses found for {host}")));
        }
        Ok(addrs)
    }

    fn kind(&self) -> DnsKind {
        DnsKind::System
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn resolve_localhost() -> AppResult<()> {
        let r = SystemResolver;
        let ips = r.resolve("localhost").await?;
        assert!(!ips.is_empty());
        assert!(ips.contains(&IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)));
        Ok(())
    }
}
