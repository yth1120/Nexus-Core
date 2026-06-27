// Runtime dependency-injection layer.
//
// Holds `RuntimeContext`, the single container every Phase 3 network-core
// manager is constructed from. Distinct from `core::runtime::Runtime`, which
// coordinates the Phase 1/2 background monitor tasks.

pub mod runtime_context;
pub mod shutdown_token;

pub use runtime_context::RuntimeContext;
pub use shutdown_token::ShutdownToken;
