use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TunState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl TunState {
    pub fn can_transition_to(self, next: TunState) -> bool {
        use TunState::*;
        matches!(
            (self, next),
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
pub struct TunStateCell {
    state: Arc<RwLock<TunState>>,
}

impl TunStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TunState::Stopped)),
        }
    }
    pub fn get(&self) -> TunState {
        *self.state.read()
    }
    pub fn set(&self, next: TunState) {
        *self.state.write() = next;
    }
    pub fn transition_to(&self, next: TunState) -> bool {
        let mut g = self.state.write();
        if g.can_transition_to(next) {
            *g = next;
            true
        } else {
            false
        }
    }
    pub fn is_running(&self) -> bool {
        matches!(self.get(), TunState::Running)
    }
}

impl Default for TunStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legal_transitions() {
        assert!(TunState::Stopped.can_transition_to(TunState::Starting));
        assert!(TunState::Starting.can_transition_to(TunState::Running));
        assert!(TunState::Running.can_transition_to(TunState::Stopping));
    }
    #[test]
    fn illegal_transitions() {
        assert!(!TunState::Stopped.can_transition_to(TunState::Running));
    }
}
