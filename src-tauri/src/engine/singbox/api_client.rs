use crate::utils::{AppError, AppResult};

pub struct SingBoxClient {
    #[allow(dead_code)]
    base_url: String,
}

impl SingBoxClient {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub async fn get_version(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "sing-box api not reachable (mock)".into(),
        ))
    }

    pub async fn get_statistics(&self) -> AppResult<String> {
        Err(AppError::Unsupported(
            "sing-box api not reachable (mock)".into(),
        ))
    }
}
