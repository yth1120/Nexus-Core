// Inbound proxy abstraction layer.
//
// Phase 3: trait + no-op adapter + manager only.
// Phase 7: real HTTP CONNECT and SOCKS5 CONNECT proxy adapters with
// bidirectional tunnel forwarding.

pub mod http_proxy;
pub mod proxy_manager;
pub mod proxy_trait;
pub mod socks5_proxy;
pub mod tunnel;

pub use http_proxy::HttpProxyAdapter;
pub use proxy_manager::{ProxyManager, ProxyStatusDto};
pub use proxy_trait::{NullProxyAdapter, ProxyAdapter, ProxyKind};
pub use socks5_proxy::Socks5ProxyAdapter;
