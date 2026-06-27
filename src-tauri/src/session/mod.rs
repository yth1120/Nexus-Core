// Session management layer.
//
// Phase 3: tracks a single in-memory session bound to a profile/node and
// publishes lifecycle events. No persistence or real connection yet.

pub mod session_manager;
pub mod session_state;

pub use session_manager::SessionManager;
pub use session_state::Session;
