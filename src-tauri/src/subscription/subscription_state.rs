use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscriptionState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl SubscriptionState {
    pub fn can_transition_to(self, n: SubscriptionState) -> bool {
        use SubscriptionState::*;
        matches!(
            (self, n),
            (Stopped, Starting)
                | (Starting, Running)
                | (Starting, Error)
                | (Running, Stopping)
                | (Running, Error)
                | (Stopping, Stopped)
                | (Stopping, Error)
                | (Error, Starting)
                | (Error, Stopped)
        )
    }
}

#[derive(Clone)]
pub struct SubscriptionStateCell {
    state: std::sync::Arc<parking_lot::RwLock<SubscriptionState>>,
}
impl SubscriptionStateCell {
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(parking_lot::RwLock::new(SubscriptionState::Stopped)),
        }
    }
    pub fn get(&self) -> SubscriptionState {
        *self.state.read()
    }
    pub fn set(&self, n: SubscriptionState) {
        *self.state.write() = n;
    }
    pub fn is_running(&self) -> bool {
        matches!(self.get(), SubscriptionState::Running)
    }
}
impl Default for SubscriptionStateCell {
    fn default() -> Self {
        Self::new()
    }
}
