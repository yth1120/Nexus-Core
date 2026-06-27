use crate::utils::{AppError, AppResult};

pub struct ClashApiClient {
    #[allow(dead_code)]
    base_url: String,
}

impl ClashApiClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_version(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "clash api not reachable (mock)".into(),
        ))
    }
    pub async fn get_traffic(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "clash api not reachable (mock)".into(),
        ))
    }
    pub async fn get_connections(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "clash api not reachable (mock)".into(),
        ))
    }
    pub async fn get_proxies(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "clash api not reachable (mock)".into(),
        ))
    }
    pub async fn set_proxy(&self, _name: &str, _group: &str) -> AppResult<()> {
        Err(AppError::Unsupported(
            "clash api not reachable (mock)".into(),
        ))
    }
}
