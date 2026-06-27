use async_trait::async_trait;

use crate::utils::{AppError, AppResult};

use super::outbound_trait::{Outbound, OutboundProtocol};

/// The only non-trivial Phase 4 stub: `connect()` returns an error so that the
/// route-selector-to-outbound wiring can be tested declaratively.
#[derive(Debug, Default)]
pub struct RejectOutbound;

#[async_trait]
impl Outbound for RejectOutbound {
    async fn connect(&self) -> AppResult<()> {
        tracing::debug!("RejectOutbound::connect (rejected)");
        Err(AppError::Internal("connection rejected".into()))
    }

    fn protocol(&self) -> OutboundProtocol {
        OutboundProtocol::Reject
    }
}
