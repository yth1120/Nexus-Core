use async_trait::async_trait;

use crate::utils::AppResult;

use super::inbound_trait::{Inbound, InboundProtocol};

#[derive(Debug, Default)]
pub struct HttpInbound;

#[async_trait]
impl Inbound for HttpInbound {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("HttpInbound::start (no-op)");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("HttpInbound::stop (no-op)");
        Ok(())
    }

    fn protocol(&self) -> InboundProtocol {
        InboundProtocol::Http
    }

    fn address(&self) -> String {
        String::new()
    }
}
