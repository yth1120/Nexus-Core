use std::io::Write;
use std::path::Path;
use std::time::Duration;

use crate::utils::{AppError, AppResult};

/// Configuration for download retry behaviour.
#[derive(Debug, Clone)]
pub struct DownloadConfig {
    pub max_retries: u32,
    pub timeout_secs: u64,
}

impl Default for DownloadConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            timeout_secs: 300,
        }
    }
}

pub struct CoreDownloader {
    config: DownloadConfig,
}

impl CoreDownloader {
    pub fn new(config: DownloadConfig) -> Self {
        Self { config }
    }

    /// Download a file from `url` to `dest`, calling `on_progress(downloaded, total)`
    /// after each chunk. Supports cancellation via an optional `cancel_token`.
    ///
    /// Retries up to `config.max_retries` times with exponential backoff.
    pub async fn download(
        &self,
        url: &str,
        dest: &Path,
        on_progress: impl Fn(u64, u64) + Send + 'static,
        cancel_token: Option<tokio_util::sync::CancellationToken>,
    ) -> AppResult<()> {
        self.download_inner(url, dest, &on_progress, cancel_token.as_ref(), false)
            .await
    }

    /// Download with resume support. If `dest` already exists and is non-empty,
    /// the download resumes from the existing file size via an HTTP `Range` header.
    pub async fn download_with_resume(
        &self,
        url: &str,
        dest: &Path,
        on_progress: impl Fn(u64, u64) + Send + 'static,
        cancel_token: Option<tokio_util::sync::CancellationToken>,
    ) -> AppResult<()> {
        self.download_inner(url, dest, &on_progress, cancel_token.as_ref(), true)
            .await
    }

    /// Download a checksum file (typically `<url>.sha256`).
    pub async fn download_checksum(&self, checksum_url: &str, dest: &Path) -> AppResult<()> {
        let client = build_client(self.config.timeout_secs)?;
        let resp = client
            .get(checksum_url)
            .send()
            .await
            .map_err(|e| AppError::Io(format!("checksum request: {e}")))?;
        let status = resp.status();
        if !status.is_success() {
            return Err(AppError::Io(format!(
                "checksum download failed: HTTP {status}"
            )));
        }
        let bytes = resp
            .bytes()
            .await
            .map_err(|e| AppError::Io(format!("checksum read: {e}")))?;
        std::fs::write(dest, &bytes).map_err(|e| AppError::Io(format!("checksum write: {e}")))?;
        Ok(())
    }

    // ----- internal -----

    async fn download_inner(
        &self,
        url: &str,
        dest: &Path,
        on_progress: &(impl Fn(u64, u64) + Send + 'static),
        cancel_token: Option<&tokio_util::sync::CancellationToken>,
        allow_resume: bool,
    ) -> AppResult<()> {
        let mut last_error = String::new();

        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                let backoff_secs = if attempt <= 6 {
                    1u64 << (attempt.saturating_sub(1))
                } else {
                    60 // cap at 60 seconds
                };
                let backoff = Duration::from_secs(backoff_secs);
                tracing::warn!(
                    "Download retry {attempt}/{} for {url} after {backoff:?}",
                    self.config.max_retries
                );
                tokio::time::sleep(backoff).await;
            }

            if let Some(ct) = cancel_token {
                if ct.is_cancelled() {
                    return Err(AppError::Internal("download cancelled".into()));
                }
            }

            match self
                .attempt_download(url, dest, on_progress, cancel_token, allow_resume)
                .await
            {
                Ok(()) => return Ok(()),
                Err(e) => {
                    last_error = e.to_string();
                    tracing::warn!("Download attempt {attempt} failed: {last_error}");
                }
            }
        }

        Err(AppError::Io(format!(
            "download failed after {} retries: {last_error}",
            self.config.max_retries
        )))
    }

    async fn attempt_download(
        &self,
        url: &str,
        dest: &Path,
        on_progress: &(impl Fn(u64, u64) + Send + 'static),
        cancel_token: Option<&tokio_util::sync::CancellationToken>,
        allow_resume: bool,
    ) -> AppResult<()> {
        let client = build_client(self.config.timeout_secs)?;
        let mut req = client.get(url);

        // Resume support
        let existing_size: u64 = if allow_resume && dest.exists() {
            std::fs::metadata(dest).map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };

        if existing_size > 0 {
            req = req.header("Range", format!("bytes={existing_size}-"));
        }

        let mut resp = req
            .send()
            .await
            .map_err(|e| AppError::Io(format!("request: {e}")))?;

        let status = resp.status();
        // Accept 200 (full) or 206 (partial content for resume)
        if status != reqwest::StatusCode::OK && status != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(AppError::Io(format!("HTTP {status}")));
        }

        // If we requested a Range but the server responded with 200 instead
        // of 206, the server ignored the Range header — restart from scratch.
        let actual_resume: u64 =
            if existing_size > 0 && status == reqwest::StatusCode::PARTIAL_CONTENT {
                existing_size
            } else if existing_size > 0 && status == reqwest::StatusCode::OK {
                tracing::warn!(
                "Server ignored Range request (returned 200 instead of 206), restarting download"
            );
                0
            } else {
                0
            };

        let total = resp.content_length().map(|t| t + actual_resume);

        // Open file for writing (append if resuming, truncate if restarting)
        let mut file = if actual_resume > 0 {
            std::fs::OpenOptions::new()
                .append(true)
                .open(dest)
                .map_err(|e| AppError::Io(format!("open for append: {e}")))?
        } else {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::File::create(dest).map_err(|e| AppError::Io(format!("create file: {e}")))?
        };

        let mut downloaded = actual_resume;

        // Read chunks one at a time from the response body.
        loop {
            if let Some(ct) = cancel_token {
                if ct.is_cancelled() {
                    return Err(AppError::Internal("download cancelled".into()));
                }
            }

            let chunk = match resp.chunk().await {
                Ok(Some(c)) => c,
                Ok(None) => break, // EOF
                Err(e) => return Err(AppError::Io(format!("chunk: {e}"))),
            };

            file.write_all(&chunk)
                .map_err(|e| AppError::Io(format!("write chunk: {e}")))?;
            downloaded += chunk.len() as u64;
            if let Some(t) = total {
                on_progress(downloaded, t);
            }
        }

        file.flush()
            .map_err(|e| AppError::Io(format!("flush: {e}")))?;

        Ok(())
    }
}

fn build_client(timeout_secs: u64) -> AppResult<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .user_agent("NexusCore/2.4")
        .build()
        .map_err(|e| AppError::Io(format!("build client: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spawn a local HTTP server that serves `body` at `/test` and `sha256_body`
    /// at `/test.sha256`. Returns the base URL.
    async fn spawn_test_server(body: Vec<u8>, sha256_body: String) -> AppResult<String> {
        use tokio::io::AsyncReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| AppError::Io(format!("bind: {e}")))?;
        let addr = listener
            .local_addr()
            .map_err(|e| AppError::Io(format!("addr: {e}")))?;
        let base_url = format!("http://{addr}");

        let body = std::sync::Arc::new(body);
        let sha256_body = std::sync::Arc::new(sha256_body);

        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    break;
                };
                let body = body.clone();
                let sha256_body = sha256_body.clone();
                tokio::spawn(async move {
                    let mut buf = [0u8; 4096];
                    let n = stream.read(&mut buf).await.unwrap_or(0);
                    let req = String::from_utf8_lossy(&buf[..n]);
                    let (status, content_type, resp_body) = if req.contains("/test.sha256") {
                        ("200 OK", "text/plain", sha256_body.as_bytes().to_vec())
                    } else {
                        ("200 OK", "application/octet-stream", body.to_vec())
                    };
                    let response = format!(
                        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\n\r\n",
                        resp_body.len()
                    );
                    let _ = stream.write_all(response.as_bytes()).await;
                    let _ = stream.write_all(&resp_body).await;
                    let _ = stream.flush().await;
                });
            }
        });

        Ok(base_url)
    }

    #[tokio::test]
    async fn downloads_file() -> AppResult<()> {
        let server = spawn_test_server(b"hello world".to_vec(), String::new()).await?;
        let tmp = std::env::temp_dir().join(format!("dl-test-{}", uuid::Uuid::new_v4()));
        let dest = tmp.join("out.bin");
        let dl = CoreDownloader::new(DownloadConfig::default());
        dl.download(&format!("{server}/test"), &dest, |_, _| {}, None)
            .await?;
        let content =
            std::fs::read_to_string(&dest).map_err(|e| AppError::Io(format!("read: {e}")))?;
        assert_eq!(content, "hello world");
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[tokio::test]
    async fn download_with_progress() -> AppResult<()> {
        let data = vec![0u8; 10000];
        let server = spawn_test_server(data.clone(), String::new()).await?;
        let tmp = std::env::temp_dir().join(format!("dl-prog-{}", uuid::Uuid::new_v4()));
        let dest = tmp.join("out.bin");
        let progress = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(0));
        let p_clone = progress.clone();
        let dl = CoreDownloader::new(DownloadConfig::default());
        dl.download(
            &format!("{server}/test"),
            &dest,
            move |d, _| {
                p_clone.store(d, std::sync::atomic::Ordering::SeqCst);
            },
            None,
        )
        .await?;
        assert!(progress.load(std::sync::atomic::Ordering::SeqCst) > 0);
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[tokio::test]
    async fn download_checksum_works() -> AppResult<()> {
        let sha = "abc123def";
        let server = spawn_test_server(vec![], sha.to_string()).await?;
        let tmp = std::env::temp_dir().join(format!("dl-cs-{}", uuid::Uuid::new_v4()));
        let dest = tmp.join("checksum.sha256");
        let dl = CoreDownloader::new(DownloadConfig::default());
        dl.download_checksum(&format!("{server}/test.sha256"), &dest)
            .await?;
        let content =
            std::fs::read_to_string(&dest).map_err(|e| AppError::Io(format!("read: {e}")))?;
        assert_eq!(content.trim(), sha);
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }

    #[tokio::test]
    async fn cancel_stops_download() -> AppResult<()> {
        let data = vec![0u8; 500_000]; // large enough to hit chunks
        let server = spawn_test_server(data, String::new()).await?;
        let tmp = std::env::temp_dir().join(format!("dl-cancel-{}", uuid::Uuid::new_v4()));
        let dest = tmp.join("out.bin");
        let ct = tokio_util::sync::CancellationToken::new();
        let ct_child = ct.clone();
        let dl = CoreDownloader::new(DownloadConfig {
            max_retries: 0,
            timeout_secs: 30,
        });
        ct.cancel(); // cancel immediately
        let result = dl
            .download(&format!("{server}/test"), &dest, |_, _| {}, Some(ct_child))
            .await;
        assert!(result.is_err());
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
