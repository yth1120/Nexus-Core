// Protocol layer abstraction — inbound / outbound / adapter traits and managers.
//
// Phase 4: trait hierarchy + no-op stubs + lifecycle management.
// Real protocol implementations arrive in Phase 5.

pub mod adapter;
pub mod connection_context;
pub mod http;
pub mod inbound;
pub mod outbound;
pub mod protocol_context;
pub mod protocol_manager;
pub mod protocol_state;
pub mod socks5;

pub use adapter::{NullAdapter, ProtocolAdapter};
pub use connection_context::ConnectionContext;
pub use inbound::{HttpInbound, Inbound, InboundProtocol, MixedInbound, Socks5Inbound};
pub use outbound::{DirectOutbound, Outbound, OutboundProtocol, ProxyOutbound, RejectOutbound};
pub use protocol_context::ProtocolContext;
pub use protocol_manager::ProtocolManager;
pub use protocol_state::{ProtocolState, ProtocolStateCell};
