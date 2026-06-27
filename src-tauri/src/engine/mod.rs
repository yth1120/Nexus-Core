// Pluggable engine architecture — registry, factory, lifecycle, process management.
//
// Phase 5: engine trait + mock implementations for Native / SingBox / Mihomo /
// Xray / Plugin. Hot-switch and process supervision framework. No real proxy,
// TUN, DNS, or encryption is implemented.

pub mod engine_context;
pub mod engine_event;
pub mod engine_factory;
pub mod engine_manager;
pub mod engine_registry;
pub mod engine_state;
pub mod engine_trait;
pub mod external;
pub mod mihomo;
pub mod native;
pub mod plugin;
pub mod process_manager;
pub mod process_supervisor;
pub mod singbox;
pub mod xray;

pub use engine_context::EngineContext;
pub use engine_factory::EngineFactory;
pub use engine_manager::EngineManager;
pub use engine_registry::EngineRegistry;
pub use engine_state::{EngineState, EngineStateCell};
pub use engine_trait::{Engine, EngineCapability, EngineType};
pub use mihomo::MihomoEngine;
pub use native::NativeEngine;
pub use plugin::{PluginEngine, PluginLoader};
pub use process_manager::ProcessManager;
pub use process_supervisor::ProcessSupervisor;
pub use singbox::SingBoxEngine;
pub use xray::XrayEngine;
