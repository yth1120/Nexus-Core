use std::sync::Arc;

use parking_lot::RwLock;
use serde::Serialize;

/// Lifecycle state of the [`PipelineManager`](super::pipeline_manager::PipelineManager).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PipelineState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl PipelineState {
    pub fn can_transition_to(self, next: PipelineState) -> bool {
        use PipelineState::*;
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
pub struct PipelineStateCell {
    state: Arc<RwLock<PipelineState>>,
}

impl PipelineStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(PipelineState::Stopped)),
        }
    }

    pub fn get(&self) -> PipelineState {
        *self.state.read()
    }

    pub fn set(&self, next: PipelineState) {
        *self.state.write() = next;
    }

    pub fn transition_to(&self, next: PipelineState) -> bool {
        let mut guard = self.state.write();
        if guard.can_transition_to(next) {
            *guard = next;
            true
        } else {
            false
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.get(), PipelineState::Running)
    }
}

impl Default for PipelineStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_transitions() {
        assert!(PipelineState::Stopped.can_transition_to(PipelineState::Starting));
        assert!(PipelineState::Starting.can_transition_to(PipelineState::Running));
        assert!(PipelineState::Running.can_transition_to(PipelineState::Stopping));
    }

    #[test]
    fn illegal_transitions() {
        assert!(!PipelineState::Stopped.can_transition_to(PipelineState::Running));
        assert!(!PipelineState::Running.can_transition_to(PipelineState::Starting));
    }

    #[test]
    fn cell_applies_valid_transitions() {
        let c = PipelineStateCell::new();
        assert_eq!(c.get(), PipelineState::Stopped);
        assert!(c.transition_to(PipelineState::Starting));
        assert!(!c.transition_to(PipelineState::Stopped));
    }
}
