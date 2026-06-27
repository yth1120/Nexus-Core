use std::sync::Arc;

use parking_lot::RwLock;

use crate::runtime::RuntimeContext;

use super::adapter::{NullAdapter, ProtocolAdapter};
use super::inbound::{Inbound, MixedInbound};
use super::outbound::{DirectOutbound, Outbound};

/// Bundle of adapters the [`ProtocolManager`](super::protocol_manager::ProtocolManager)
/// orchestrates. Defaults all to no-op stubs until Phase 5 wires real backends.
pub struct ProtocolContext {
    pub runtime: Arc<RuntimeContext>,
    pub inbound_adapter: RwLock<Arc<dyn Inbound>>,
    pub outbound_adapter: RwLock<Arc<dyn Outbound>>,
    pub protocol_adapter: RwLock<Arc<dyn ProtocolAdapter>>,
}

impl ProtocolContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            runtime,
            inbound_adapter: RwLock::new(Arc::new(MixedInbound)),
            outbound_adapter: RwLock::new(Arc::new(DirectOutbound)),
            protocol_adapter: RwLock::new(Arc::new(NullAdapter)),
        }
    }
}

#[cfg(test)]
impl ProtocolContext {
    /// Build a context for unit tests backed by a test [`RuntimeContext`].
    pub(crate) fn new_for_test(runtime: Arc<RuntimeContext>) -> crate::utils::AppResult<Arc<Self>> {
        Ok(Arc::new(Self::new(runtime)))
    }
}
