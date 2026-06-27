use async_trait::async_trait;

use crate::utils::AppResult;

use super::inbound_trait::{Inbound, InboundProtocol};

#[derive(Debug, Default)]
pub struct MixedInbound;

#[async_trait]
impl Inbound for MixedInbound {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("MixedInbound::start (no-op)");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("MixedInbound::stop (no-op)");
        Ok(())
    }

    fn protocol(&self) -> InboundProtocol {
        InboundProtocol::Mixed
    }

    fn address(&self) -> String {
        String::new()
    }
}
