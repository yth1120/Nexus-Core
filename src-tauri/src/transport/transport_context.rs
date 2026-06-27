use std::sync::Arc;

use parking_lot::RwLock;

use crate::runtime::RuntimeContext;

use super::listener::Listener;
use super::stream::TransportStream;

/// Bundle of listeners and streams the [`TransportManager`](super::transport_manager::TransportManager)
/// orchestrates. Starts empty until listeners/streams are registered.
pub struct TransportContext {
    pub runtime: Arc<RuntimeContext>,
    pub listeners: RwLock<Vec<Arc<dyn Listener>>>,
    pub streams: RwLock<Vec<Arc<dyn TransportStream>>>,
}

impl TransportContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            runtime,
            listeners: RwLock::new(Vec::new()),
            streams: RwLock::new(Vec::new()),
        }
    }
}

#[cfg(test)]
impl TransportContext {
    pub(crate) fn new_for_test(runtime: Arc<RuntimeContext>) -> crate::utils::AppResult<Arc<Self>> {
        Ok(Arc::new(Self::new(runtime)))
    }
}
