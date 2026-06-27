// TUN virtual network interface framework.
//
// Phase 8: trait hierarchy + mock implementations + route management.
// Real device creation (wintun/tun crates) arrives in Phase 8.5+.

pub mod packet_injector;
pub mod packet_receiver;
pub mod route_table;
pub mod tun_context;
pub mod tun_device;
pub mod tun_manager;
pub mod tun_packet;
pub mod tun_route;
pub mod tun_state;

pub use packet_injector::PacketInjector;
pub use packet_receiver::PacketReceiver;
pub use route_table::{NullRouteTable, RouteTable};
pub use tun_context::TunContext;
pub use tun_device::{NullTunDevice, TunDevice};
pub use tun_manager::TunManager;
pub use tun_packet::TunPacket;
pub use tun_route::RouteManager;
pub use tun_state::{TunState, TunStateCell};
