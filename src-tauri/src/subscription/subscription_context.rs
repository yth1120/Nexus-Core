use std::sync::Arc;

use crate::runtime::RuntimeContext;

use super::subscription_downloader::SubscriptionDownloader;

pub struct SubscriptionContext {
    pub runtime: Arc<RuntimeContext>,
    pub downloader: SubscriptionDownloader,
}

impl SubscriptionContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        Self {
            runtime,
            downloader: SubscriptionDownloader,
        }
    }
}
