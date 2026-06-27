use async_trait::async_trait;

use crate::utils::AppResult;

/// Abstraction over a protocol-level adapter (e.g., an external proxy core).
///
/// Phase 4 is architecture only — all implementations are no-op.
/// Real adapters arrive in Phase 5.
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    async fn initialize(&self) -> AppResult<()>;
    async fn shutdown(&self) -> AppResult<()>;
}
