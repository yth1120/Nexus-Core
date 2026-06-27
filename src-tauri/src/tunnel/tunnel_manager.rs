use std::sync::Arc;

use parking_lot::RwLock;

use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::tunnel_trait::{NullTunnelAdapter, TunnelAdapter, TunnelKind};

/// Owns the active tunnel adapter and drives its lifecycle.
///
/// Defaults to [`NullTunnelAdapter`]; a future phase calls
/// [`TunnelManager::set_adapter`] to install a real System / TUN / TAP backend.
pub struct TunnelManager {
    // Retained for dependency access in later phases (routing, config).
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
    adapter: RwLock<Arc<dyn TunnelAdapter>>,
}

impl TunnelManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            adapter: RwLock::new(Arc::new(NullTunnelAdapter)),
        }
    }

    /// Swap in a different tunnel adapter (e.g. a real TUN backend in Phase 4).
    pub fn set_adapter(&self, adapter: Arc<dyn TunnelAdapter>) {
        *self.adapter.write() = adapter;
    }

    /// The kind of the currently-installed adapter.
    pub fn kind(&self) -> TunnelKind {
        self.adapter.read().kind()
    }

    /// Bring up the active tunnel. No-op in Phase 3.
    pub async fn start(&self) -> AppResult<()> {
        let adapter = self.adapter.read().clone();
        adapter.start().await
    }

    /// Tear down the active tunnel. No-op in Phase 3.
    pub async fn stop(&self) -> AppResult<()> {
        let adapter = self.adapter.read().clone();
        adapter.stop().await
    }
}
