#![allow(clippy::module_inception)]

// Connection dispatch layer — routing, rule matching, failover.
//
// Phase 4: complete API surface, always returns Direct.
// Real routing logic arrives in Phase 5.

pub mod dispatcher;
pub mod route_selector;

pub use dispatcher::Dispatcher;
pub use route_selector::RouteSelector;
