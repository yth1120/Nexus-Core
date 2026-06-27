use super::dns_cache::DnsCache;
use super::dns_trait::{DnsResolver, NullDnsResolver};
use crate::runtime::RuntimeContext;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct DnsContext {
    pub runtime: Arc<RuntimeContext>,
    pub resolver: RwLock<Arc<dyn DnsResolver>>,
    pub cache: Arc<DnsCache>,
}

impl DnsContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            runtime,
            resolver: RwLock::new(Arc::new(NullDnsResolver)),
            cache: Arc::new(DnsCache::default()),
        }
    }
}
