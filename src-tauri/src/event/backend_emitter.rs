use std::sync::Arc;

use tauri::AppHandle;

use crate::event::AppEvent;

/// Abstracts event emission so background `tokio::spawn` tasks never hold
/// or reference `tauri::AppHandle` directly.
pub trait BackendEmitter: Send + Sync {
    fn emit(&self, event: AppEvent);
}

/// Production emitter — wraps `tauri::AppHandle`.
pub struct TauriEmitter {
    app_handle: AppHandle,
}

impl TauriEmitter {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
}

impl BackendEmitter for TauriEmitter {
    fn emit(&self, event: AppEvent) {
        use crate::event::event_bus::emit_to_frontend;
        emit_to_frontend(&self.app_handle, &event);
    }
}

/// No-op emitter for tests and early boot.
pub struct NoopEmitter;

impl BackendEmitter for NoopEmitter {
    fn emit(&self, _event: AppEvent) {}
}

/// Convenience: create an `Arc<dyn BackendEmitter>` from an `AppHandle`.
pub fn create_emitter(app_handle: AppHandle) -> Arc<dyn BackendEmitter> {
    Arc::new(TauriEmitter::new(app_handle))
}

/// Convenience: create a no-op emitter for tests.
pub fn create_noop_emitter() -> Arc<dyn BackendEmitter> {
    Arc::new(NoopEmitter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noop_emitter_does_not_panic() {
        let e = create_noop_emitter();
        e.emit(AppEvent::CoreStarted);
    }
}
