use std::path::Path;

use crate::core_installer::core_downloader::{CoreDownloader, DownloadConfig};
use crate::utils::{AppError, AppResult};

use super::checksum::ChecksumVerifier;

/// Downloads geo database files with progress, retry, and checksum verification.
pub struct GeoDownloader {
    downloader: CoreDownloader,
}

impl GeoDownloader {
    pub fn new() -> Self {
        Self {
            downloader: CoreDownloader::new(DownloadConfig::default()),
        }
    }

    /// Download a file from `url` to `dest`, optionally verifying against a
    /// checksum file at `checksum_url`.
    pub async fn download(
        &self,
        url: &str,
        dest: &Path,
        checksum_url: Option<&str>,
    ) -> AppResult<()> {
        self.downloader
            .download(
                url,
                dest,
                |_, _| {}, // progress callback (can be wired to events later)
                None,      // no cancel token
            )
            .await
            .map_err(|e| AppError::Io(format!("geo download: {e}")))?;

        // Optional checksum verification
        if let Some(cs_url) = checksum_url {
            let cs_path = dest.with_extension("sha256");
            match self.downloader.download_checksum(cs_url, &cs_path).await {
                Ok(()) => {
                    let expected = ChecksumVerifier::parse_checksum_file(&cs_path)?;
                    let verified = ChecksumVerifier::verify(dest, &expected)?;
                    if !verified {
                        let _ = std::fs::remove_file(dest);
                        return Err(AppError::Internal("checksum mismatch".into()));
                    }
                }
                Err(_) => {
                    // Checksum not available — proceed without verification
                    tracing::debug!("No checksum available for {url}");
                }
            }
        }

        Ok(())
    }

    /// Download with ETag support for conditional requests.
    /// Returns `true` if the file was updated, `false` if unchanged (304).
    pub async fn download_if_modified(
        &self,
        url: &str,
        dest: &Path,
        etag_file: &Path,
    ) -> AppResult<bool> {
        let client = reqwest::Client::builder()
            .user_agent("NexusCore/2.4")
            .build()
            .map_err(|e| AppError::Io(format!("client: {e}")))?;

        let mut req = client.get(url);

        // Read stored ETag if available
        if etag_file.exists() {
            if let Ok(etag) = std::fs::read_to_string(etag_file) {
                let etag = etag.trim().to_string();
                if !etag.is_empty() {
                    req = req.header("If-None-Match", &etag);
                }
            }
        }

        let resp = req
            .send()
            .await
            .map_err(|e| AppError::Io(format!("request: {e}")))?;

        if resp.status() == reqwest::StatusCode::NOT_MODIFIED {
            return Ok(false); // unchanged
        }

        // Store new ETag
        if let Some(etag) = resp.headers().get("etag") {
            if let Ok(val) = etag.to_str() {
                let _ = std::fs::write(etag_file, val);
            }
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|e| AppError::Io(format!("read: {e}")))?;

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(dest, &bytes).map_err(|e| AppError::Io(format!("write: {e}")))?;

        Ok(true)
    }
}

impl Default for GeoDownloader {
    fn default() -> Self {
        Self::new()
    }
}
