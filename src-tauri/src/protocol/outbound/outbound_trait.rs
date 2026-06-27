use async_trait::async_trait;
use serde::Serialize;

use crate::utils::AppResult;

/// The kind of outbound route an [`Outbound`] adapter represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutboundProtocol {
    Direct,
    Proxy,
    Reject,
}

/// Abstraction over an outbound connection target.
#[async_trait]
pub trait Outbound: Send + Sync {
    async fn connect(&self) -> AppResult<()>;
    fn protocol(&self) -> OutboundProtocol;
}
