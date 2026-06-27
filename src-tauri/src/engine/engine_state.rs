use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

/// Lifecycle state of an [`Engine`](super::engine_trait::Engine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EngineState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl EngineState {
    pub fn can_transition_to(self, next: EngineState) -> bool {
        use EngineState::*;
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
pub struct EngineStateCell {
    state: Arc<RwLock<EngineState>>,
}

impl EngineStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(EngineState::Stopped)),
        }
    }

    pub fn get(&self) -> EngineState {
        *self.state.read()
    }

    pub fn set(&self, next: EngineState) {
        *self.state.write() = next;
    }

    pub fn transition_to(&self, next: EngineState) -> bool {
        let mut guard = self.state.write();
        if guard.can_transition_to(next) {
            *guard = next;
            true
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.get(), EngineState::Running)
    }
}

impl Default for EngineStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_transitions_are_allowed() {
        assert!(EngineState::Stopped.can_transition_to(EngineState::Starting));
        assert!(EngineState::Starting.can_transition_to(EngineState::Running));
        assert!(EngineState::Running.can_transition_to(EngineState::Stopping));
        assert!(EngineState::Stopping.can_transition_to(EngineState::Stopped));
        assert!(EngineState::Error.can_transition_to(EngineState::Starting));
    }

    #[test]
    fn illegal_transitions_are_rejected() {
        assert!(!EngineState::Stopped.can_transition_to(EngineState::Running));
        assert!(!EngineState::Running.can_transition_to(EngineState::Starting));
    }

    #[test]
    fn state_cell_applies_valid_transitions() {
        let s = EngineStateCell::new();
        assert_eq!(s.get(), EngineState::Stopped);
        assert!(s.transition_to(EngineState::Starting));
        assert!(!s.transition_to(EngineState::Stopped));
        assert_eq!(s.get(), EngineState::Starting);
    }
}
