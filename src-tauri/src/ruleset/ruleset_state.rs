use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleSetState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl RuleSetState {
    pub fn can_transition_to(self, n: RuleSetState) -> bool {
        use RuleSetState::*;
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
pub struct RuleSetStateCell {
    state: std::sync::Arc<parking_lot::RwLock<RuleSetState>>,
}
impl RuleSetStateCell {
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(parking_lot::RwLock::new(RuleSetState::Stopped)),
        }
    }
    pub fn get(&self) -> RuleSetState {
        *self.state.read()
    }
    pub fn set(&self, n: RuleSetState) {
        *self.state.write() = n;
    }
}
impl Default for RuleSetStateCell {
    fn default() -> Self {
        Self::new()
    }
}
