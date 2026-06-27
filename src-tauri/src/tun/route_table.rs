use async_trait::async_trait;

use crate::utils::{AppError, AppResult};

#[async_trait]
pub trait RouteTable: Send + Sync {
    async fn add(&self, _dest: &str, _gateway: &str) -> AppResult<()>;
    async fn remove(&self, _dest: &str) -> AppResult<()>;
    async fn restore(&self) -> AppResult<()>;
}

#[derive(Debug, Default)]
pub struct NullRouteTable;

#[async_trait]
impl RouteTable for NullRouteTable {
    async fn add(&self, _dest: &str, _gateway: &str) -> AppResult<()> {
        Err(AppError::Unsupported("route table not available".into()))
    }
    async fn remove(&self, _dest: &str) -> AppResult<()> {
        Err(AppError::Unsupported("route table not available".into()))
    }
    async fn restore(&self) -> AppResult<()> {
        Err(AppError::Unsupported("route table not available".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn null_route_table_returns_unsupported() {
        let t = NullRouteTable;
        assert!(t.add("0.0.0.0/0", "10.0.0.1").await.is_err());
        assert!(t.restore().await.is_err());
    }
}
