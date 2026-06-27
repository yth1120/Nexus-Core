use parking_lot::RwLock;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DnsState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

impl DnsState {
    pub fn can_transition_to(self, next: DnsState) -> bool {
        use DnsState::*;
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
pub struct DnsStateCell {
    state: Arc<RwLock<DnsState>>,
}
impl DnsStateCell {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(DnsState::Stopped)),
        }
    }
    pub fn get(&self) -> DnsState {
        *self.state.read()
    }
    pub fn set(&self, n: DnsState) {
        *self.state.write() = n;
    }
    pub fn is_running(&self) -> bool {
        matches!(self.get(), DnsState::Running)
    }
}
impl Default for DnsStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legal() {
        assert!(DnsState::Stopped.can_transition_to(DnsState::Starting));
    }
    #[test]
    fn illegal() {
        assert!(!DnsState::Stopped.can_transition_to(DnsState::Running));
    }
}
