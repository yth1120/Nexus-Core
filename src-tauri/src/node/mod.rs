// Node management layer.
//
// Phase 3: tracks the current node selection and returns mock latency. Real
// node connections and latency probes arrive in Phase 4.

pub mod node_manager;

pub use node_manager::NodeManager;
