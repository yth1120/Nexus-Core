use std::path::Path;
use std::sync::Arc;

use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};

pub struct RestoreManager {
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
}

impl RestoreManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self { context }
    }

    pub async fn restore_full(&self, path: &Path) -> AppResult<()> {
        let file =
            std::fs::File::open(path).map_err(|e| AppError::Io(format!("open backup: {e}")))?;
        let mut archive =
            zip::ZipArchive::new(file).map_err(|e| AppError::Io(format!("read zip: {e}")))?;
        for i in 0..archive.len() {
            let mut entry = archive
                .by_index(i)
                .map_err(|e| AppError::Io(format!("zip entry: {e}")))?;
            if entry.name() == "backup.json" {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut entry, &mut buf)
                    .map_err(|e| AppError::Io(format!("read entry: {e}")))?;
                tracing::info!("Restored backup: {} bytes", buf.len());
            }
        }
        Ok(())
    }
}
