use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CoreState {
    Idle,
    Downloading,
    Verifying,
    Extracting,
    Installing,
    Error,
}

#[derive(Clone)]
pub struct CoreStateCell {
    state: std::sync::Arc<parking_lot::RwLock<CoreState>>,
    message: std::sync::Arc<parking_lot::RwLock<String>>,
}

impl CoreStateCell {
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(parking_lot::RwLock::new(CoreState::Idle)),
            message: std::sync::Arc::new(parking_lot::RwLock::new(String::new())),
        }
    }

    pub fn get(&self) -> CoreState {
        *self.state.read()
    }

    pub fn set(&self, s: CoreState) {
        *self.state.write() = s;
    }

    /// Set state with an attached human-readable message.
    pub fn set_with_message(&self, s: CoreState, msg: impl Into<String>) {
        *self.state.write() = s;
        *self.message.write() = msg.into();
    }

    /// Get the current human-readable status message, if any.
    pub fn message(&self) -> String {
        self.message.read().clone()
    }
}

impl Default for CoreStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_idle() {
        assert_eq!(CoreStateCell::new().get(), CoreState::Idle);
    }

    #[test]
    fn set_and_get() {
        let cell = CoreStateCell::new();
        cell.set(CoreState::Downloading);
        assert_eq!(cell.get(), CoreState::Downloading);
    }

    #[test]
    fn message_persistence() {
        let cell = CoreStateCell::new();
        cell.set_with_message(CoreState::Verifying, "checking SHA-256...");
        assert_eq!(cell.get(), CoreState::Verifying);
        assert_eq!(cell.message(), "checking SHA-256...");
    }

    #[test]
    fn verifying_is_distinct_from_error() {
        assert_ne!(CoreState::Verifying, CoreState::Error);
    }
}
