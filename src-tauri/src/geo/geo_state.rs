use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GeoState {
    Idle,
    Loading,
    Ready,
    Updating,
    Error,
}

#[derive(Clone)]
pub struct GeoStateCell {
    state: std::sync::Arc<parking_lot::RwLock<GeoState>>,
    message: std::sync::Arc<parking_lot::RwLock<String>>,
}

impl GeoStateCell {
    pub fn new() -> Self {
        Self {
            state: std::sync::Arc::new(parking_lot::RwLock::new(GeoState::Idle)),
            message: std::sync::Arc::new(parking_lot::RwLock::new(String::new())),
        }
    }

    pub fn get(&self) -> GeoState {
        *self.state.read()
    }

    pub fn set(&self, s: GeoState) {
        *self.state.write() = s;
    }

    pub fn set_with_message(&self, s: GeoState, msg: impl Into<String>) {
        *self.state.write() = s;
        *self.message.write() = msg.into();
    }

    pub fn message(&self) -> String {
        self.message.read().clone()
    }
}

impl Default for GeoStateCell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_idle() {
        assert_eq!(GeoStateCell::new().get(), GeoState::Idle);
    }

    #[test]
    fn set_and_get() {
        let cell = GeoStateCell::new();
        cell.set(GeoState::Loading);
        assert_eq!(cell.get(), GeoState::Loading);
    }

    #[test]
    fn message_persistence() {
        let cell = GeoStateCell::new();
        cell.set_with_message(GeoState::Ready, "databases loaded");
        assert_eq!(cell.get(), GeoState::Ready);
        assert_eq!(cell.message(), "databases loaded");
    }
}
