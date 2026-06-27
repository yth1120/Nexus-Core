use async_trait::async_trait;

use crate::utils::AppResult;

use super::outbound_trait::{Outbound, OutboundProtocol};

#[derive(Debug, Default)]
pub struct DirectOutbound;

#[async_trait]
impl Outbound for DirectOutbound {
    async fn connect(&self) -> AppResult<()> {
        tracing::debug!("DirectOutbound::connect (no-op)");
        Ok(())
    }

    fn protocol(&self) -> OutboundProtocol {
        OutboundProtocol::Direct
    }
}
