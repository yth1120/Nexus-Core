use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::http_proxy::HttpProxyAdapter;
use super::proxy_trait::{NullProxyAdapter, ProxyAdapter, ProxyKind};
use super::socks5_proxy::Socks5ProxyAdapter;

/// Serializable proxy status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyStatusDto {
    pub http_running: bool,
    pub socks5_running: bool,
    pub http_port: u16,
    pub socks5_port: u16,
}

/// Owns the active inbound proxy adapters and drives their lifecycle.
pub struct ProxyManager {
    context: Arc<RuntimeContext>,
    adapter: RwLock<Arc<dyn ProxyAdapter>>,
    http_adapter: RwLock<Option<Arc<HttpProxyAdapter>>>,
    socks_adapter: RwLock<Option<Arc<Socks5ProxyAdapter>>>,
    http_port: u16,
    socks_port: u16,
}

impl ProxyManager {
    pub fn new(context: Arc<RuntimeContext>, http_port: u16, socks_port: u16) -> Self {
        Self {
            context,
            adapter: RwLock::new(Arc::new(NullProxyAdapter)),
            http_adapter: RwLock::new(None),
            socks_adapter: RwLock::new(None),
            http_port,
            socks_port,
        }
    }

    // ----- generic adapter (Phase 3 compat) -----

    pub fn set_adapter(&self, adapter: Arc<dyn ProxyAdapter>) {
        *self.adapter.write() = adapter;
    }

    pub fn kind(&self) -> ProxyKind {
        self.adapter.read().kind()
    }

    pub async fn start(&self) -> AppResult<()> {
        let adapter = self.adapter.read().clone();
        adapter.start().await
    }

    pub async fn stop(&self) -> AppResult<()> {
        let adapter = self.adapter.read().clone();
        adapter.stop().await
    }

    // ----- Phase 7: individual proxy lifecycle -----

    pub async fn start_http(&self) -> AppResult<()> {
        let adapter = Arc::new(HttpProxyAdapter::new(self.http_port, self.context.clone()));
        adapter.start().await?;
        *self.http_adapter.write() = Some(adapter);
        Ok(())
    }

    pub async fn stop_http(&self) -> AppResult<()> {
        let adapter = self.http_adapter.read().clone();
        if let Some(ref a) = adapter {
            a.stop().await?;
        }
        *self.http_adapter.write() = None;
        Ok(())
    }

    pub async fn start_socks5(&self) -> AppResult<()> {
        let adapter = Arc::new(Socks5ProxyAdapter::new(
            self.socks_port,
            self.context.clone(),
        ));
        adapter.start().await?;
        *self.socks_adapter.write() = Some(adapter);
        Ok(())
    }

    pub async fn stop_socks5(&self) -> AppResult<()> {
        let adapter = self.socks_adapter.read().clone();
        if let Some(ref a) = adapter {
            a.stop().await?;
        }
        *self.socks_adapter.write() = None;
        Ok(())
    }

    pub fn proxy_status(&self) -> ProxyStatusDto {
        ProxyStatusDto {
            http_running: self
                .http_adapter
                .read()
                .as_ref()
                .map(|a| a.is_running())
                .unwrap_or(false),
            socks5_running: self
                .socks_adapter
                .read()
                .as_ref()
                .map(|a| a.is_running())
                .unwrap_or(false),
            http_port: self.http_port,
            socks5_port: self.socks_port,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn start_stop_http_proxy() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let mgr = ProxyManager::new(rt, 7890, 7891);

        mgr.start_http().await?;
        let status = mgr.proxy_status();
        assert!(status.http_running);

        mgr.stop_http().await?;
        let status = mgr.proxy_status();
        assert!(!status.http_running);
        Ok(())
    }
}
