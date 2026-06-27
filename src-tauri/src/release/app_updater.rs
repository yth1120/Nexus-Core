use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::core_installer::core_downloader::{CoreDownloader, DownloadConfig};
use crate::core_installer::integrity_checker::IntegrityChecker;
use crate::utils::{AppError, AppResult};

/// Information about an available update.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    pub release_notes: String,
    pub download_url: String,
    pub checksum: String,
    pub size_bytes: u64,
}

/// Version comparison result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionStatus {
    UpToDate,
    UpdateAvailable,
    NewerThanLatest, // dev build
}

/// Checks for and downloads updates to the Nexus Core application itself.
///
/// This is separate from Phase 13's engine/binary updater. It queries
/// GitHub Releases for new versions of the Nexus Core desktop application.
pub struct AppUpdater {
    current_version: String,
    update_channel: String,
    downloader: CoreDownloader,
    repo_owner: String,
    repo_name: String,
}

impl AppUpdater {
    /// Create a new app updater.
    ///
    /// `current_version` should be the version from Cargo.toml (e.g. "2.4.1").
    /// `channel` is "stable" or "beta".
    pub fn new(current_version: String, channel: String) -> Self {
        Self {
            current_version,
            update_channel: channel,
            downloader: CoreDownloader::new(DownloadConfig::default()),
            repo_owner: "NexusCore".into(),
            repo_name: "Nexus-Core".into(),
        }
    }

    /// Check GitHub Releases for a newer version.
    /// Returns `None` if current is up-to-date.
    pub async fn check_update(&self) -> AppResult<Option<UpdateInfo>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.repo_owner, self.repo_name
        );

        let client = reqwest::Client::builder()
            .user_agent("NexusCore/2.4")
            .build()
            .map_err(|e| AppError::Io(format!("build client: {e}")))?;

        let resp = client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| AppError::Io(format!("github api: {e}")))?;

        if !resp.status().is_success() {
            return Ok(None); // gracefully handle API errors
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Io(format!("json: {e}")))?;

        let latest_version = json["tag_name"]
            .as_str()
            .unwrap_or("0.0.0")
            .trim_start_matches('v')
            .to_string();

        match Self::compare_versions(&self.current_version, &latest_version) {
            VersionStatus::UpdateAvailable => {
                let notes = json["body"]
                    .as_str()
                    .unwrap_or("No release notes")
                    .to_string();

                // Find the appropriate platform asset
                let platform = Self::platform_string();
                let assets = json["assets"].as_array();
                let (download_url, checksum, size) = if let Some(assets) = assets {
                    let asset = assets.iter().find(|a| {
                        a["name"]
                            .as_str()
                            .map(|n| n.contains(&platform))
                            .unwrap_or(false)
                    });
                    match asset {
                        Some(a) => (
                            a["browser_download_url"].as_str().unwrap_or("").to_string(),
                            String::new(), // checksum not in release API asset
                            a["size"].as_u64().unwrap_or(0),
                        ),
                        None => (String::new(), String::new(), 0),
                    }
                } else {
                    (String::new(), String::new(), 0)
                };

                Ok(Some(UpdateInfo {
                    version: latest_version,
                    release_notes: notes,
                    download_url,
                    checksum,
                    size_bytes: size,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Download an update package.
    pub async fn download_update(&self, info: &UpdateInfo, dest: &Path) -> AppResult<()> {
        self.downloader
            .download(&info.download_url, dest, |_, _| {}, None)
            .await
            .map_err(|e| AppError::Io(format!("update download: {e}")))
    }

    /// Verify an update package against its expected checksum.
    pub fn verify_update(&self, path: &Path, expected_hash: &str) -> AppResult<bool> {
        IntegrityChecker::verify_file(path, expected_hash)
    }

    /// Compare two semver strings.
    pub fn compare_versions(current: &str, latest: &str) -> VersionStatus {
        let cur_parts: Vec<u32> = current.split('.').filter_map(|p| p.parse().ok()).collect();
        let lat_parts: Vec<u32> = latest.split('.').filter_map(|p| p.parse().ok()).collect();

        match cur_parts.cmp(&lat_parts) {
            std::cmp::Ordering::Less => VersionStatus::UpdateAvailable,
            std::cmp::Ordering::Equal => VersionStatus::UpToDate,
            std::cmp::Ordering::Greater => VersionStatus::NewerThanLatest,
        }
    }

    /// Update channel.
    pub fn channel(&self) -> &str {
        &self.update_channel
    }

    /// Current version.
    pub fn current_version(&self) -> &str {
        &self.current_version
    }

    fn platform_string() -> String {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        };
        let arch = if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else if cfg!(target_arch = "aarch64") {
            "aarch64"
        } else {
            "x86_64"
        };
        format!("{os}-{arch}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_versions_detects_update() {
        assert_eq!(
            AppUpdater::compare_versions("2.4.0", "2.5.0"),
            VersionStatus::UpdateAvailable
        );
        assert_eq!(
            AppUpdater::compare_versions("2.4.1", "2.4.1"),
            VersionStatus::UpToDate
        );
        assert_eq!(
            AppUpdater::compare_versions("3.0.0", "2.9.9"),
            VersionStatus::NewerThanLatest
        );
    }

    #[test]
    fn platform_string_is_non_empty() {
        let ps = AppUpdater::platform_string();
        assert!(!ps.is_empty());
        assert!(ps.contains("-"));
    }

    #[test]
    fn updater_has_version() {
        let updater = AppUpdater::new("2.4.1".into(), "stable".into());
        assert_eq!(updater.current_version(), "2.4.1");
        assert_eq!(updater.channel(), "stable");
    }
}
