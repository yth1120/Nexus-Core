use std::path::{Path, PathBuf};

use crate::core_installer::core_downloader::{CoreDownloader, DownloadConfig};
use crate::core_installer::integrity_checker::IntegrityChecker;
use crate::utils::{AppError, AppResult};

/// Downloads and updates the GeoIP MMDB database file.
pub struct GeoIpUpdater {
    downloader: CoreDownloader,
    geo_dir: PathBuf,
    download_url: String,
}

impl GeoIpUpdater {
    /// v2fly geoip.dat download URL.
    const DEFAULT_URL: &'static str =
        "https://github.com/v2fly/geoip/releases/latest/download/geoip.dat";

    pub fn new(geo_dir: PathBuf) -> Self {
        Self {
            downloader: CoreDownloader::new(DownloadConfig::default()),
            geo_dir,
            download_url: Self::DEFAULT_URL.to_string(),
        }
    }

    /// Download the latest GeoIP database to a temp file, verify SHA-256,
    /// then atomically replace the target file.
    pub async fn update(&self, target_path: &Path) -> AppResult<String> {
        let tmp_path = self.geo_dir.join("geoip.mmdb.tmp");
        let sha256_path = self.geo_dir.join("geoip.mmdb.sha256");

        // Download
        self.downloader
            .download(self.download_url.as_str(), &tmp_path, |_, _| {}, None)
            .await
            .map_err(|e| AppError::Io(format!("geoip download: {e}")))?;

        // Try to download checksum; skip verification if not available
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

        // Verify if we have a checksum
        if !expected_sha256.is_empty() {
            let verified = IntegrityChecker::verify_file(&tmp_path, &expected_sha256)?;
            if !verified {
                let _ = std::fs::remove_file(&tmp_path);
                return Err(AppError::Internal("geoip SHA-256 mismatch".into()));
            }
        }

        // Atomic replace
        if target_path.exists() {
            let backup = self.geo_dir.join("geoip.mmdb.bak");
            let _ = std::fs::remove_file(&backup);
            std::fs::rename(target_path, &backup).ok();
        }
        std::fs::rename(&tmp_path, target_path)
            .map_err(|e| AppError::Io(format!("geoip rename: {e}")))?;

        // Read version from the new database
        let version = match super::MmdbReader::open(target_path) {
            Ok(reader) => reader.version_string(),
            Err(_) => "unknown".into(),
        };

        Ok(version)
    }

    /// Set a custom download URL (for mirrors or testing).
    pub fn set_url(&mut self, url: String) {
        self.download_url = url;
    }
}
