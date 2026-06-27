use std::sync::Arc;

use crate::protocol::connection_context::ConnectionContext;
use crate::protocol::outbound::OutboundProtocol;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

/// Rule-matching / node-selection / failover / load-balance engine shell.
///
/// Phase 4 exposes a complete API surface but returns `Direct` / `None` for
/// every method. Real routing logic arrives in Phase 5.
pub struct RouteSelector {
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
}

impl RouteSelector {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self { context }
    }

    /// Match a connection against the rule set. Always returns `Direct` in Phase 4.
    pub fn match_rule(&self, _conn: &ConnectionContext) -> AppResult<OutboundProtocol> {
        Ok(OutboundProtocol::Direct)
    }

    /// Select a node for proxied traffic. Always returns `None` in Phase 4.
    pub fn select_node(&self) -> AppResult<Option<String>> {
        Ok(None)
    }

    /// Pick a failover route if the primary is unavailable. Always `None`.
    pub fn failover(&self) -> AppResult<Option<OutboundProtocol>> {
        Ok(None)
    }

    /// Load-balance across available outbounds. Always `None`.
    pub fn load_balance(&self) -> AppResult<Option<OutboundProtocol>> {
        Ok(None)
    }

    /// Run the full selection pipeline: match → node → failover → load-balance.
    pub fn select(&self, conn: &ConnectionContext) -> AppResult<OutboundProtocol> {
        let primary = self.match_rule(conn)?;
        let _node = self.select_node()?;
        let fallback = self.failover()?;
        let balanced = self.load_balance()?;
        Ok(fallback.or(balanced).unwrap_or(primary))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    use crate::utils::AppResult;

    fn test_conn() -> ConnectionContext {
        ConnectionContext {
            id: "conn-1".into(),
            source: "127.0.0.1:54321".into(),
            destination: "example.com:443".into(),
            protocol: "TCP".into(),
            created_at: 1_000_000,
        }
    }

    #[test]
    fn match_rule_returns_direct() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let sel = RouteSelector::new(ctx);
        let route = sel.match_rule(&test_conn())?;
        assert_eq!(route, OutboundProtocol::Direct);
        Ok(())
    }

    #[test]
    fn select_node_returns_none() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let sel = RouteSelector::new(ctx);
        assert_eq!(sel.select_node()?, None);
        Ok(())
    }

    #[test]
    fn select_pipeline_returns_direct() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let sel = RouteSelector::new(ctx);
        let route = sel.select(&test_conn())?;
        assert_eq!(route, OutboundProtocol::Direct);
        Ok(())
    }
}
