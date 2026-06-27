// Network core engine layer.
//
// Phase 3: lifecycle state machine + engine orchestration + context bundle.
// All sub-system work is mock/no-op. The real TCP/UDP stack, connection pool,
// packet routing, and NAT traversal arrive in Phase 4.

pub mod network_context;
pub mod network_engine;
pub mod network_state;

pub use network_context::NetworkContext;
pub use network_engine::NetworkEngine;
pub use network_state::{EngineState, NetworkState};
