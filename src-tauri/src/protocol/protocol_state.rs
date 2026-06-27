use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

/// Lifecycle state of the [`ProtocolManager`](super::protocol_manager::ProtocolManager).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl ProtocolState {
    /// Whether a transition from `self` to `next` is permitted.
    pub fn can_transition_to(self, next: ProtocolState) -> bool {
        use ProtocolState::*;
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

/// Thread-safe holder for the protocol layer's current state.
#[derive(Clone)]
pub struct ProtocolStateCell {
    state: Arc<RwLock<ProtocolState>>,
}

impl ProtocolStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(ProtocolState::Stopped)),
        }
    }

    pub fn get(&self) -> ProtocolState {
        *self.state.read()
    }

    pub fn set(&self, next: ProtocolState) {
        *self.state.write() = next;
    }

    /// Attempt a validated transition. Returns `true` if applied.
    pub fn transition_to(&self, next: ProtocolState) -> bool {
        let mut guard = self.state.write();
        if guard.can_transition_to(next) {
            *guard = next;
            true
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.get(), ProtocolState::Running)
    }
}

impl Default for ProtocolStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_transitions_are_allowed() {
        assert!(ProtocolState::Stopped.can_transition_to(ProtocolState::Starting));
        assert!(ProtocolState::Starting.can_transition_to(ProtocolState::Running));
        assert!(ProtocolState::Running.can_transition_to(ProtocolState::Stopping));
        assert!(ProtocolState::Stopping.can_transition_to(ProtocolState::Stopped));
        assert!(ProtocolState::Error.can_transition_to(ProtocolState::Starting));
    }

    #[test]
    fn illegal_transitions_are_rejected() {
        assert!(!ProtocolState::Stopped.can_transition_to(ProtocolState::Running));
        assert!(!ProtocolState::Running.can_transition_to(ProtocolState::Starting));
    }

    #[test]
    fn state_cell_applies_only_valid_transitions() {
        let state = ProtocolStateCell::new();
        assert_eq!(state.get(), ProtocolState::Stopped);
        assert!(state.transition_to(ProtocolState::Starting));
        assert!(!state.transition_to(ProtocolState::Stopped)); // illegal
        assert_eq!(state.get(), ProtocolState::Starting);
        assert!(state.transition_to(ProtocolState::Running));
        assert!(state.is_running());
    }
}
