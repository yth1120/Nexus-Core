use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

/// Lifecycle state of the [`NetworkEngine`](super::network_engine::NetworkEngine).
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
    /// Whether a transition from `self` to `next` is permitted by the engine
    /// state machine. `Error` is reachable from any active phase, and the engine
    /// may recover from `Error` back to `Starting` or settle to `Stopped`.
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

/// Thread-safe holder for the engine's current [`EngineState`].
///
/// Uses a non-poisoning `parking_lot::RwLock` (matching `AppState`). Locks are
/// only ever held for the duration of a single read/write — never across an
/// `.await`.
#[derive(Clone)]
pub struct NetworkState {
    state: Arc<RwLock<EngineState>>,
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(EngineState::Stopped)),
        }
    }

    /// Current state snapshot.
    pub fn get(&self) -> EngineState {
        *self.state.read()
    }

    /// Force the state to `next` regardless of the transition table. Used for
    /// terminal `Error` transitions where the normal table does not apply.
    pub fn set(&self, next: EngineState) {
        *self.state.write() = next;
    }

    /// Attempt a validated transition. Returns `true` if it was applied,
    /// `false` if the transition is not allowed from the current state.
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

impl Default for NetworkState {
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
        assert!(!EngineState::Stopped.can_transition_to(EngineState::Stopping));
    }

    #[test]
    fn state_cell_applies_only_valid_transitions() {
        let state = NetworkState::new();
        assert_eq!(state.get(), EngineState::Stopped);

        assert!(state.transition_to(EngineState::Starting));
        assert_eq!(state.get(), EngineState::Starting);

        // Illegal jump is rejected and leaves state unchanged.
        assert!(!state.transition_to(EngineState::Stopped));
        assert_eq!(state.get(), EngineState::Starting);

        assert!(state.transition_to(EngineState::Running));
        assert!(state.is_running());
    }
}
