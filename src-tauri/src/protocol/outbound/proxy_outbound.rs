use async_trait::async_trait;

use crate::utils::AppResult;

use super::outbound_trait::{Outbound, OutboundProtocol};

#[derive(Debug, Default)]
pub struct ProxyOutbound;

#[async_trait]
impl Outbound for ProxyOutbound {
    async fn connect(&self) -> AppResult<()> {
        tracing::debug!("ProxyOutbound::connect (no-op)");
        Ok(())
    }

    fn protocol(&self) -> OutboundProtocol {
        OutboundProtocol::Proxy
    }
}
