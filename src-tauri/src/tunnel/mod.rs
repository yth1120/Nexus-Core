// Tunnel / traffic-capture abstraction layer.
//
// Phase 3: trait + no-op adapter + manager only. Real System-proxy / TUN / TAP
// device creation and IP packet processing arrive in Phase 4.

pub mod tunnel_manager;
pub mod tunnel_trait;

pub use tunnel_manager::TunnelManager;
pub use tunnel_trait::{NullTunnelAdapter, TunnelAdapter, TunnelKind};
