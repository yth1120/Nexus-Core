use async_trait::async_trait;
use serde::Serialize;

use crate::utils::AppResult;

/// The kind of inbound protocol an [`Inbound`] adapter handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InboundProtocol {
    Http,
    Socks5,
    Mixed,
    Transparent,
}

/// Abstraction over an inbound connection handler (listener).
#[async_trait]
pub trait Inbound: Send + Sync {
    async fn start(&self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;
    fn protocol(&self) -> InboundProtocol;
    fn address(&self) -> String;
}
