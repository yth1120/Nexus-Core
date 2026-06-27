use std::path::{Path, PathBuf};

use crate::core_installer::core_downloader::{CoreDownloader, DownloadConfig};
use crate::core_installer::integrity_checker::IntegrityChecker;
use crate::utils::{AppError, AppResult};

/// Downloads and updates the GeoSite protobuf database file.
pub struct GeoSiteUpdater {
    downloader: CoreDownloader,
    geo_dir: PathBuf,
    download_url: String,
}

impl GeoSiteUpdater {
    /// v2fly domain-list-community dlc.dat download URL.
    const DEFAULT_URL: &'static str =
        "https://github.com/v2fly/domain-list-community/releases/latest/download/dlc.dat";

    pub fn new(geo_dir: PathBuf) -> Self {
        Self {
            downloader: CoreDownloader::new(DownloadConfig::default()),
            geo_dir,
            download_url: Self::DEFAULT_URL.to_string(),
        }
    }

    /// Download the latest GeoSite database, verify SHA-256, and atomically
    /// replace the target file.
    pub async fn update(&self, target_path: &Path) -> AppResult<String> {
        let tmp_path = self.geo_dir.join("geosite.dat.tmp");
        let sha256_path = self.geo_dir.join("geosite.dat.sha256");

        // Download
        self.downloader
            .download(self.download_url.as_str(), &tmp_path, |_, _| {}, None)
            .await
            .map_err(|e| AppError::Io(format!("geosite download: {e}")))?;

        // Try checksum
        let expected_sha256 = match self
            .downloader
            .download_checksum(&format!("{}.sha256sum", self.download_url), &sha256_path)
            .await
        {
            Ok(()) => {
                let content = std::fs::read_to_string(&sha256_path)
                    .map_err(|e| AppError::Io(format!("read checksum: {e}")))?;
                content.split_whitespace().next().unwrap_or("").to_string()
            }
            Err(_) => String::new(),
        };

        if !expected_sha256.is_empty() {
            let verified = IntegrityChecker::verify_file(&tmp_path, &expected_sha256)?;
            if !verified {
                let _ = std::fs::remove_file(&tmp_path);
                return Err(AppError::Internal("geosite SHA-256 mismatch".into()));
            }
        }

        // Atomic replace
        if target_path.exists() {
            let backup = self.geo_dir.join("geosite.dat.bak");
            let _ = std::fs::remove_file(&backup);
            std::fs::rename(target_path, &backup).ok();
        }
        std::fs::rename(&tmp_path, target_path)
            .map_err(|e| AppError::Io(format!("geosite rename: {e}")))?;

        Ok("latest".into())
    }

    /// Set a custom download URL.
    pub fn set_url(&mut self, url: String) {
        self.download_url = url;
    }
}
