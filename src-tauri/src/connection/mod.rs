// Active-connection management layer.

pub mod connection_manager;
pub mod connection_state;
pub mod connection_task;

pub use connection_manager::ConnectionManager;
pub use connection_state::{ActiveConnection, ConnectionState};
pub use connection_task::ConnectionTask;
