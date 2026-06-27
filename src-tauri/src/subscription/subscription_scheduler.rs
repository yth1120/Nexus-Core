use std::sync::Arc;

use parking_lot::RwLock;
use tokio_util::sync::CancellationToken;

use super::subscription_context::SubscriptionContext;

pub struct SubscriptionScheduler {
    #[allow(dead_code)]
    context: Arc<SubscriptionContext>,
    interval: RwLock<u64>,
}

impl SubscriptionScheduler {
    pub fn new(context: Arc<SubscriptionContext>, interval_secs: u64) -> Self {
        Self {
            context,
            interval: RwLock::new(interval_secs),
        }
    }

    pub fn set_interval(&self, secs: u64) {
        *self.interval.write() = secs;
    }

    pub async fn run(&self, token: CancellationToken) {
        let mut tick = tokio::time::interval(std::time::Duration::from_secs(*self.interval.read()));
        loop {
            tokio::select! {
                _ = token.cancelled() => break,
                _ = tick.tick() => {
                    tracing::debug!("Subscription scheduler tick");
                }
            }
        }
    }
}
