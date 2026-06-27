use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use zip::write::SimpleFileOptions;

use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};

pub struct BackupManager {
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
}

impl BackupManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self { context }
    }
    pub async fn create_backup(&self, path: &Path) -> AppResult<PathBuf> {
        let file =
            std::fs::File::create(path).map_err(|e| AppError::Io(format!("create backup: {e}")))?;
        let mut zip = zip::ZipWriter::new(file);
        let opts =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("backup.json", opts)
            .map_err(|e| AppError::Io(format!("zip: {e}")))?;
        let data = serde_json::json!({"version": "1.0", "timestamp": chrono::Utc::now().timestamp_millis()});
        zip.write_all(
            serde_json::to_string_pretty(&data)
                .unwrap_or_default()
                .as_bytes(),
        )
        .map_err(|e| AppError::Io(format!("zip write: {e}")))?;
        let finished = zip
            .finish()
            .map_err(|e| AppError::Io(format!("zip finish: {e}")))?;
        drop(finished);
        Ok(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    #[tokio::test]
    async fn creates_backup_zip() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let mgr = BackupManager::new(rt);
        let path = std::env::temp_dir().join(format!("test-backup-{}.zip", uuid::Uuid::new_v4()));
        let result = mgr.create_backup(&path).await?;
        assert!(result.exists());
        let _ = std::fs::remove_file(&path);
        Ok(())
    }
}
