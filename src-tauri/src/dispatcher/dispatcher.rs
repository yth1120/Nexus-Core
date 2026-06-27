use std::sync::Arc;

use crate::event::AppEvent;
use crate::protocol::connection_context::ConnectionContext;
use crate::protocol::outbound::OutboundProtocol;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::route_selector::RouteSelector;

/// Routes incoming connections through the protocol and transport layers.
///
/// Phase 4 always dispatches to `Direct`. The real dispatch pipeline
/// (Connection → Rule → Outbound → Adapter) arrives in Phase 5.
pub struct Dispatcher {
    context: Arc<RuntimeContext>,
    route_selector: Arc<RouteSelector>,
}

impl Dispatcher {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        let route_selector = Arc::new(RouteSelector::new(context.clone()));
        Self {
            context,
            route_selector,
        }
    }

    /// Dispatch a connection — always returns `Direct` in Phase 4.
    pub async fn dispatch(&self, conn: &ConnectionContext) -> AppResult<OutboundProtocol> {
        let route = self.route_selector.select(conn)?;
        self.context.publish(AppEvent::ConnectionDispatched {
            connection_id: conn.id.clone(),
            route: format!("{:?}", route),
        });
        tracing::debug!("Dispatcher: {} -> {:?}", conn.id, route);
        Ok(route)
    }

    /// Select a route without publishing an event.
    pub fn select_route(&self, conn: &ConnectionContext) -> AppResult<OutboundProtocol> {
        self.route_selector.select(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    use crate::utils::AppResult;

    #[tokio::test]
    async fn dispatch_always_returns_direct() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let disp = Dispatcher::new(ctx);
        let conn = ConnectionContext {
            id: "c1".into(),
            source: "s".into(),
            destination: "d".into(),
            protocol: "TCP".into(),
            created_at: 0,
        };
        let route = disp.dispatch(&conn).await?;
        assert_eq!(route, OutboundProtocol::Direct);
        Ok(())
    }
}
