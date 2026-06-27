use async_trait::async_trait;

use crate::utils::{AppError, AppResult};

// ---- trait ----

#[async_trait]
pub trait ReleaseProvider: Send + Sync {
    /// Return the latest version tag for the given core.
    async fn latest_version(&self, core: &str) -> AppResult<String>;

    /// Build a direct-download URL for the given core / version / platform.
    async fn download_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String>;

    /// Return release notes (markdown body) for the given version.
    async fn release_notes(&self, core: &str, version: &str) -> AppResult<String>;

    /// Return the URL of the SHA-256 checksum file for a release.
    async fn checksum_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String>;
}

// ---- GitHub Release Provider ----

/// Known cores and their GitHub repository coordinates.
const KNOWN_CORES: &[(&str, &str, &str)] = &[
    ("sing-box", "SagerNet", "sing-box"),
    ("mihomo", "MetaCubeX", "mihomo"),
];

fn repo_owner_and_name(core: &str) -> AppResult<(&'static str, &'static str)> {
    KNOWN_CORES
        .iter()
        .find(|(name, _, _)| *name == core)
        .map(|(_, owner, repo)| (*owner, *repo))
        .ok_or_else(|| AppError::NotFound(format!("unknown core: {core}")))
}

pub struct GithubReleaseProvider {
    client: reqwest::Client,
}

impl GithubReleaseProvider {
    pub fn new() -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent("NexusCore/2.4")
            .build()
            .map_err(|e| AppError::Io(format!("build github client: {e}")))?;
        Ok(Self { client })
    }
}

impl Default for GithubReleaseProvider {
    fn default() -> Self {
        Self::new().expect("GithubReleaseProvider client build")
    }
}

#[async_trait]
impl ReleaseProvider for GithubReleaseProvider {
    async fn latest_version(&self, core: &str) -> AppResult<String> {
        let (owner, repo) = repo_owner_and_name(core)?;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| AppError::Io(format!("github api: {e}")))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Io(format!(
                "github api returned HTTP {status} for {core}"
            )));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Io(format!("github json: {e}")))?;

        let tag = json["tag_name"]
            .as_str()
            .unwrap_or("0.0.0")
            .trim_start_matches('v')
            .to_string();

        Ok(tag)
    }

    async fn download_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String> {
        let (owner, repo) = repo_owner_and_name(core)?;
        let ext = if cfg!(target_os = "windows") {
            "zip"
        } else {
            "tar.gz"
        };
        // Use the standard GitHub release asset naming convention.
        // sing-box: sing-box-1.11.0-windows-amd64.zip
        // mihomo: mihomo-windows-amd64-1.19.0.zip
        let filename = match core {
            "sing-box" => format!("{core}-{version}-{platform}.{ext}"),
            "mihomo" => format!("{core}-{platform}-{version}.{ext}"),
            _ => format!("{core}-{platform}-{version}.{ext}"),
        };
        Ok(format!(
            "https://github.com/{owner}/{repo}/releases/download/v{version}/{filename}"
        ))
    }

    async fn release_notes(&self, core: &str, version: &str) -> AppResult<String> {
        let (owner, repo) = repo_owner_and_name(core)?;
        let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/tags/v{version}");

        let resp = self
            .client
            .get(&url)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| AppError::Io(format!("github api: {e}")))?;

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AppError::Io(format!("github json: {e}")))?;

        Ok(json["body"]
            .as_str()
            .unwrap_or("No release notes available.")
            .to_string())
    }

    async fn checksum_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String> {
        let (owner, repo) = repo_owner_and_name(core)?;
        let ext = if cfg!(target_os = "windows") {
            "zip"
        } else {
            "tar.gz"
        };
        // Checksum files often follow the pattern: <asset>.sha256 or SHA256SUMS
        let filename = match core {
            "sing-box" => {
                format!("{core}-{version}-{platform}.{ext}.sha256")
            }
            "mihomo" => format!("{core}-{platform}-{version}.{ext}.sha256"),
            _ => format!("{core}-{platform}-{version}.{ext}.sha256"),
        };
        Ok(format!(
            "https://github.com/{owner}/{repo}/releases/download/v{version}/{filename}"
        ))
    }
}

// ---- Mirror Release Provider ----

pub struct MirrorReleaseProvider {
    base_url: String,
    client: reqwest::Client,
}

impl MirrorReleaseProvider {
    pub fn new(base_url: String) -> AppResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent("NexusCore/2.4")
            .build()
            .map_err(|e| AppError::Io(format!("build mirror client: {e}")))?;
        Ok(Self { base_url, client })
    }
}

#[async_trait]
impl ReleaseProvider for MirrorReleaseProvider {
    async fn latest_version(&self, core: &str) -> AppResult<String> {
        let url = format!("{}/{core}/latest.txt", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Io(format!("mirror request: {e}")))?;
        if !resp.status().is_success() {
            return Err(AppError::Unsupported(format!(
                "mirror returned HTTP {} for {core}",
                resp.status()
            )));
        }
        resp.text()
            .await
            .map(|s| s.trim().to_string())
            .map_err(|e| AppError::Io(format!("mirror read: {e}")))
    }

    async fn download_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String> {
        let ext = if cfg!(target_os = "windows") {
            "zip"
        } else {
            "tar.gz"
        };
        Ok(format!(
            "{base}/{core}/{core}-{version}-{platform}.{ext}",
            base = self.base_url
        ))
    }

    async fn release_notes(&self, core: &str, version: &str) -> AppResult<String> {
        let url = format!("{}/{core}/{version}/notes.txt", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => resp
                .text()
                .await
                .map_err(|e| AppError::Io(format!("mirror notes: {e}"))),
            _ => Ok("No release notes available.".into()),
        }
    }

    async fn checksum_url(&self, core: &str, version: &str, platform: &str) -> AppResult<String> {
        let ext = if cfg!(target_os = "windows") {
            "zip"
        } else {
            "tar.gz"
        };
        Ok(format!(
            "{base}/{core}/{core}-{version}-{platform}.{ext}.sha256",
            base = self.base_url
        ))
    }
}

// ---- tests ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_owner_name_for_known_cores() {
        assert!(repo_owner_and_name("sing-box").is_ok());
        assert!(repo_owner_and_name("mihomo").is_ok());
        assert!(repo_owner_and_name("unknown").is_err());
    }

    #[tokio::test]
    async fn github_provider_builds_urls() -> AppResult<()> {
        let p = GithubReleaseProvider::new()?;
        let url = p
            .download_url("sing-box", "1.11.0", "windows-amd64")
            .await?;
        assert!(url.contains("sing-box"));
        assert!(url.contains("1.11.0"));
        assert!(url.contains("windows-amd64"));
        Ok(())
    }

    #[tokio::test]
    async fn mirror_provider_builds_urls() -> AppResult<()> {
        let p = MirrorReleaseProvider::new("https://mirror.example.com".into())?;
        let url = p.download_url("sing-box", "1.0.0", "linux-amd64").await?;
        assert!(url.starts_with("https://mirror.example.com/"));
        Ok(())
    }
}
