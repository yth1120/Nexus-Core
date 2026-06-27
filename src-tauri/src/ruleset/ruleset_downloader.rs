use crate::utils::{AppError, AppResult};
use std::time::Duration;

pub struct RuleSetDownloader;

impl RuleSetDownloader {
    pub async fn download(&self, url: &str, _etag: Option<&str>) -> AppResult<String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Io(format!("build client: {e}")))?;
        let resp = client
            .get(url)
            .send()
            .await
            .map_err(|e| AppError::Io(format!("download {url}: {e}")))?;
        resp.text()
            .await
            .map_err(|e| AppError::Io(format!("read body: {e}")))
    }
}
