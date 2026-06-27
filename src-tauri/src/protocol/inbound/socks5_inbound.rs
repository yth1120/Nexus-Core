use async_trait::async_trait;

use crate::utils::AppResult;

use super::inbound_trait::{Inbound, InboundProtocol};

#[derive(Debug, Default)]
pub struct Socks5Inbound;

#[async_trait]
impl Inbound for Socks5Inbound {
    async fn start(&self) -> AppResult<()> {
        tracing::debug!("Socks5Inbound::start (no-op)");
        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        tracing::debug!("Socks5Inbound::stop (no-op)");
        Ok(())
    }

    fn protocol(&self) -> InboundProtocol {
        InboundProtocol::Socks5
    }

    fn address(&self) -> String {
        String::new()
    }
}
