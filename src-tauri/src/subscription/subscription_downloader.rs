use crate::utils::{AppError, AppResult};
use std::time::Duration;

use super::update_result::UpdateResult;

pub struct SubscriptionDownloader;

impl SubscriptionDownloader {
    pub async fn download(
        &self,
        url: &str,
        etag: Option<&str>,
        last_modified: Option<&str>,
    ) -> AppResult<UpdateResult> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::Io(format!("build client: {e}")))?;
        let mut req = client.get(url);
        if let Some(e) = etag {
            req = req.header("If-None-Match", e);
        }
        if let Some(lm) = last_modified {
            req = req.header("If-Modified-Since", lm);
        }
        let resp = req
            .send()
            .await
            .map_err(|e| AppError::Io(format!("download {url}: {e}")))?;
        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(UpdateResult {
                status: "unchanged".into(),
                nodes: vec![],
                rules: vec![],
                etag: None,
                last_modified: None,
            });
        }
        let body = resp
            .text()
            .await
            .map_err(|e| AppError::Io(format!("read body: {e}")))?;
        Ok(UpdateResult {
            status: "updated".into(),
            nodes: vec![],
            rules: vec![],
            etag: None,
            last_modified: Some(body),
        })
    }
}
