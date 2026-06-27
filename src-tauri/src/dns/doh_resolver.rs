use super::dns_trait::{DnsKind, DnsResolver};
use crate::utils::{AppError, AppResult};
use async_trait::async_trait;
use std::net::IpAddr;

pub struct DoHResolver;
#[async_trait]
impl DnsResolver for DoHResolver {
    async fn resolve(&self, host: &str) -> AppResult<Vec<IpAddr>> {
        Err(AppError::Unsupported(format!(
            "DoH resolver not implemented (host: {host})"
        )))
    }
    fn kind(&self) -> DnsKind {
        DnsKind::Doh
    }
}
