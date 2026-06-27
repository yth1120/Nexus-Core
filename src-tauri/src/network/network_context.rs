use std::sync::Arc;

use crate::connection::ConnectionManager;
use crate::dns::{DnsContext, DnsManager};
use crate::proxy::ProxyManager;
use crate::runtime::RuntimeContext;
use crate::tunnel::TunnelManager;

/// The bundle of network sub-systems the [`NetworkEngine`](super::network_engine::NetworkEngine)
/// drives.
///
/// Constructed by `CoreManager` and handed to the engine. Holds the shared
/// [`RuntimeContext`] plus the four network sub-managers (all no-op in Phase 3).
pub struct NetworkContext {
    pub runtime: Arc<RuntimeContext>,
    pub tunnel_manager: Arc<TunnelManager>,
    pub dns_manager: Arc<DnsManager>,
    pub proxy_manager: Arc<ProxyManager>,
    pub connection_manager: Arc<ConnectionManager>,
}

impl NetworkContext {
    /// Build the context, constructing each sub-manager from the shared runtime.
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        let tunnel_manager = Arc::new(TunnelManager::new(runtime.clone()));
        let dns_ctx = Arc::new(DnsContext::new(runtime.clone()));
        let dns_manager = Arc::new(DnsManager::new(dns_ctx));
        let proxy_manager = Arc::new(ProxyManager::new(runtime.clone(), 7890, 7891));
        let connection_manager = Arc::new(ConnectionManager::new(runtime.clone()));
        Self {
            runtime,
            tunnel_manager,
            dns_manager,
            proxy_manager,
            connection_manager,
        }
    }
}
