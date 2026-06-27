use std::path::{Path, PathBuf};

use crate::utils::AppResult;

use super::telemetry_recorder::TelemetryReport;

/// Persists telemetry data to a local JSON file.
/// NO network upload — data stays on the user's machine.
pub struct TelemetryStorage {
    path: PathBuf,
}

impl TelemetryStorage {
    pub fn new(telemetry_dir: &Path) -> Self {
        Self {
            path: telemetry_dir.join("telemetry.json"),
        }
    }

    /// Load telemetry from disk. Returns `None` if the file doesn't exist.
    pub fn load(&self) -> Option<TelemetryReport> {
        if !self.path.exists() {
            return None;
        }
        let data = std::fs::read_to_string(&self.path).ok()?;
        serde_json::from_str(&data).ok()
    }

    /// Save telemetry to disk (atomic write via temp file + rename).
    pub fn save(&self, report: &TelemetryReport) -> AppResult<()> {
        let tmp = self.path.with_extension("tmp");
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| crate::utils::AppError::Config(format!("serialize telemetry: {e}")))?;
        std::fs::write(&tmp, &json)
            .map_err(|e| crate::utils::AppError::Io(format!("write telemetry: {e}")))?;
        std::fs::rename(&tmp, &self.path)
            .map_err(|e| crate::utils::AppError::Io(format!("rename telemetry: {e}")))?;
        Ok(())
    }
}
