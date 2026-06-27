use std::sync::Arc;

use crate::utils::AppResult;

use super::subscription_context::SubscriptionContext;
use super::subscription_state::{SubscriptionState, SubscriptionStateCell};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub auto_update: bool,
    pub update_interval: u64,
    pub last_updated: Option<i64>,
}

pub struct SubscriptionManager {
    _context: Arc<SubscriptionContext>,
    state: SubscriptionStateCell,
    subscriptions: parking_lot::RwLock<Vec<Subscription>>,
}

impl SubscriptionManager {
    pub fn new(context: Arc<SubscriptionContext>) -> Self {
        Self {
            _context: context,
            state: SubscriptionStateCell::new(),
            subscriptions: parking_lot::RwLock::new(Vec::new()),
        }
    }

    pub fn status(&self) -> SubscriptionState {
        self.state.get()
    }
    pub async fn start(&self) -> AppResult<()> {
        self.state.set(SubscriptionState::Starting);
        self.state.set(SubscriptionState::Running);
        Ok(())
    }
    pub async fn stop(&self) -> AppResult<()> {
        self.state.set(SubscriptionState::Stopping);
        self.state.set(SubscriptionState::Stopped);
        Ok(())
    }

    pub fn add_subscription(&self, name: &str, url: &str) -> AppResult<Subscription> {
        let sub = Subscription {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            url: url.into(),
            enabled: true,
            auto_update: true,
            update_interval: 21600,
            last_updated: None,
        };
        {
            self.subscriptions.write().push(sub.clone());
        }
        Ok(sub)
    }

    pub fn remove_subscription(&self, id: &str) -> bool {
        let mut guard = self.subscriptions.write();
        let pos = guard.iter().position(|s| s.id == id);
        if let Some(idx) = pos {
            guard.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn list(&self) -> Vec<Subscription> {
        self.subscriptions.read().clone()
    }

    pub async fn update_all(&self) -> AppResult<()> {
        let subs = { self.subscriptions.read().clone() };
        for s in &subs {
            if s.enabled {
                let _ = self._context.downloader.download(&s.url, None, None).await;
            }
        }
        Ok(())
    }
}
