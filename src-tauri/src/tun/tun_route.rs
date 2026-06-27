use std::sync::Arc;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::route_table::RouteTable;

pub struct RouteManager {
    context: Arc<RuntimeContext>,
    table: RwLock<Arc<dyn RouteTable>>,
}

impl RouteManager {
    pub fn new(context: Arc<RuntimeContext>, table: Arc<dyn RouteTable>) -> Self {
        Self {
            context,
            table: RwLock::new(table),
        }
    }

    pub fn set_table(&self, table: Arc<dyn RouteTable>) {
        *self.table.write() = table;
    }

    pub async fn create_route(&self, dest: &str, gateway: &str) -> AppResult<()> {
        let table = self.table.read().clone();
        table.add(dest, gateway).await?;
        self.context.publish(AppEvent::RouteCreated {
            dest: dest.to_string(),
        });
        Ok(())
    }

    pub async fn delete_route(&self, dest: &str) -> AppResult<()> {
        let table = self.table.read().clone();
        table.remove(dest).await?;
        self.context.publish(AppEvent::RouteDeleted {
            dest: dest.to_string(),
        });
        Ok(())
    }

    pub async fn restore_routes(&self) -> AppResult<()> {
        let table = self.table.read().clone();
        table.restore().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;

    #[tokio::test]
    async fn create_route_unsupported_with_null_table() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let t: Arc<dyn RouteTable> = Arc::new(super::super::route_table::NullRouteTable);
        let rm = RouteManager::new(rt, t);
        assert!(rm.create_route("0.0.0.0/0", "10.0.0.1").await.is_err());
        Ok(())
    }
}
