use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

/// Lifecycle state of the [`TransportManager`](super::transport_manager::TransportManager).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransportState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl TransportState {
    pub fn can_transition_to(self, next: TransportState) -> bool {
        use TransportState::*;
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

/// Thread-safe holder for the transport layer's current state.
#[derive(Clone)]
pub struct TransportStateCell {
    state: Arc<RwLock<TransportState>>,
}

impl TransportStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TransportState::Stopped)),
        }
    }

    pub fn get(&self) -> TransportState {
        *self.state.read()
    }

    pub fn set(&self, next: TransportState) {
        *self.state.write() = next;
    }

    pub fn transition_to(&self, next: TransportState) -> bool {
        let mut guard = self.state.write();
        if guard.can_transition_to(next) {
            *guard = next;
            true
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.get(), TransportState::Running)
    }
}

impl Default for TransportStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_transitions_are_allowed() {
        assert!(TransportState::Stopped.can_transition_to(TransportState::Starting));
        assert!(TransportState::Starting.can_transition_to(TransportState::Running));
        assert!(TransportState::Running.can_transition_to(TransportState::Stopping));
        assert!(TransportState::Stopping.can_transition_to(TransportState::Stopped));
    }

    #[test]
    fn illegal_transitions_are_rejected() {
        assert!(!TransportState::Stopped.can_transition_to(TransportState::Running));
        assert!(!TransportState::Running.can_transition_to(TransportState::Starting));
    }

    #[test]
    fn state_cell_applies_valid_transitions() {
        let state = TransportStateCell::new();
        assert_eq!(state.get(), TransportState::Stopped);
        assert!(state.transition_to(TransportState::Starting));
        assert!(!state.transition_to(TransportState::Stopped));
        assert_eq!(state.get(), TransportState::Starting);
    }
}
