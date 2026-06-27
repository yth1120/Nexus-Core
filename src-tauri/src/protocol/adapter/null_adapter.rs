use async_trait::async_trait;

use crate::utils::AppResult;

use super::adapter_trait::ProtocolAdapter;

/// Default no-op protocol adapter.
#[derive(Debug, Default)]
pub struct NullAdapter;

#[async_trait]
impl ProtocolAdapter for NullAdapter {
    async fn initialize(&self) -> AppResult<()> {
        tracing::debug!("NullAdapter::initialize (no-op)");
        Ok(())
    }

    async fn shutdown(&self) -> AppResult<()> {
        tracing::debug!("NullAdapter::shutdown (no-op)");
        Ok(())
    }
}
