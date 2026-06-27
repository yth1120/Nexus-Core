// DNS resolution layer.
// Phase 9: real SystemResolver, DNS cache, state machine.

pub mod dns_cache;
pub mod dns_context;
pub mod dns_manager;
pub mod dns_record;
pub mod dns_state;
pub mod dns_trait;
pub mod doh_resolver;
pub mod dot_resolver;
pub mod system_resolver;

pub use dns_cache::DnsCache;
pub use dns_context::DnsContext;
pub use dns_manager::DnsManager;
pub use dns_record::DnsRecord;
pub use dns_state::{DnsState, DnsStateCell};
pub use dns_trait::{DnsKind, DnsResolver, NullDnsResolver};
pub use doh_resolver::DoHResolver;
pub use dot_resolver::DoTResolver;
pub use system_resolver::SystemResolver;
