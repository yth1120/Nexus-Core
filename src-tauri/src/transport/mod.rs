// Transport layer abstraction — listener / stream traits and managers.
//
// Phase 4: trait hierarchy + no-op stubs + lifecycle management.
// Real TCP/UDP stacks arrive in Phase 5.

pub mod listener;
pub mod stream;
pub mod transport_context;
pub mod transport_manager;
pub mod transport_state;

pub use listener::{Listener, ListenerKind, TcpListener, UdpListener};
pub use stream::{TcpStream, TransportStream, UdpStream};
pub use transport_context::TransportContext;
pub use transport_manager::TransportManager;
pub use transport_state::{TransportState, TransportStateCell};
