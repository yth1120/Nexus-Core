use crate::event::{AppEvent, EventBus};

/// Publish an `engine:registered` event.
pub(crate) fn publish_engine_registered(bus: &EventBus, engine_type: &str) {
    bus.publish(AppEvent::EngineRegistered {
        engine_type: engine_type.to_string(),
    });
}

/// Publish an `engine:unregistered` event.
pub(crate) fn publish_engine_unregistered(bus: &EventBus, engine_type: &str) {
    bus.publish(AppEvent::EngineUnregistered {
        engine_type: engine_type.to_string(),
    });
}

/// Publish an `engine:switched` event.
pub(crate) fn publish_engine_switched(bus: &EventBus, from: &str, to: &str) {
    bus.publish(AppEvent::EngineSwitched {
        from: from.to_string(),
        to: to.to_string(),
    });
}

/// Publish `engine:starting` / `engine:started` / `engine:stopping` / `engine:stopped`.
pub(crate) fn publish_engine_lifecycle(bus: &EventBus, event: AppEvent) {
    bus.publish(event);
}

/// Publish `engine:health` result.
#[allow(dead_code)]
pub(crate) fn publish_engine_health(bus: &EventBus, engine_type: &str, healthy: bool) {
    bus.publish(AppEvent::EngineHealth {
        engine_type: engine_type.to_string(),
        healthy,
    });
}
