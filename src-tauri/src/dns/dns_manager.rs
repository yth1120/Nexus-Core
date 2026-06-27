use std::net::IpAddr;
use std::sync::Arc;

use crate::event::AppEvent;
use crate::utils::AppResult;

use super::dns_context::DnsContext;
use super::dns_state::{DnsState, DnsStateCell};
use super::dns_trait::DnsResolver;

pub struct DnsManager {
    context: Arc<DnsContext>,
    state: DnsStateCell,
}

impl DnsManager {
    pub fn new(context: Arc<DnsContext>) -> Self {
        Self {
            context,
            state: DnsStateCell::new(),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        tracing::info!("DnsManager initialized");
        Ok(())
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(DnsState::Starting);
        self.set_state(DnsState::Running);
        self.context.runtime.publish(AppEvent::DnsStarted);
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), DnsState::Stopped) {
            return Ok(());
        }
        self.set_state(DnsState::Stopping);
        self.set_state(DnsState::Stopped);
        self.context.runtime.publish(AppEvent::DnsStopped);
        Ok(())
    }

    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    pub fn status(&self) -> DnsState {
        self.state.get()
    }

    pub fn set_resolver(&self, r: Arc<dyn DnsResolver>) {
        *self.context.resolver.write() = r;
    }

    pub fn resolver_kind(&self) -> String {
        format!("{:?}", self.context.resolver.read().kind())
    }

    pub async fn resolve(&self, domain: &str) -> AppResult<Vec<IpAddr>> {
        if let Some(cached) = self.context.cache.get(domain) {
            return Ok(cached);
        }
        let r = self.context.resolver.read().clone();
        let ips = r.resolve(domain).await?;
        self.context
            .cache
            .insert(super::dns_record::DnsRecord::new(domain, ips.clone(), 300));
        self.context.runtime.publish(AppEvent::DnsResolved {
            domain: domain.to_string(),
            ips: ips.iter().map(|i| i.to_string()).collect(),
        });
        Ok(ips)
    }

    pub fn flush_cache(&self) {
        self.context.cache.clear();
        self.context.runtime.publish(AppEvent::DnsCacheFlushed);
    }

    pub fn cache_size(&self) -> usize {
        self.context.cache.len()
    }

    fn set_state(&self, n: DnsState) {
        self.state.set(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn lifecycle_start_stop() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let ctx = Arc::new(crate::dns::DnsContext::new(rt));
        let mgr = DnsManager::new(ctx);
        assert_eq!(mgr.status(), DnsState::Stopped);
        mgr.start().await?;
        assert_eq!(mgr.status(), DnsState::Running);
        mgr.stop().await?;
        assert_eq!(mgr.status(), DnsState::Stopped);
        Ok(())
    }
}
