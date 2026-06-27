use super::crash_report::CrashReport;
use super::health_report::HealthReport;
use super::system_report::SystemReport;
use crate::runtime::RuntimeContext;
use std::sync::Arc;

pub struct DiagnosticsManager {
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
}

impl DiagnosticsManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self { context }
    }
    pub fn collect_system_report(&self) -> SystemReport {
        SystemReport::collect()
    }
    pub fn collect_health_report(&self) -> HealthReport {
        HealthReport::new()
    }
    pub fn generate_crash_report(&self, panic_msg: &str) -> CrashReport {
        CrashReport::new(panic_msg)
    }

    pub fn save_diagnostics(&self, path: &std::path::Path) -> crate::utils::AppResult<()> {
        let report = serde_json::json!({
            "system": self.collect_system_report(),
            "health": self.collect_health_report(),
        });
        let json = serde_json::to_string_pretty(&report)
            .map_err(|e| crate::utils::AppError::Internal(format!("diagnostics json: {e}")))?;
        std::fs::write(path, json)
            .map_err(|e| crate::utils::AppError::Io(format!("write diagnostics: {e}")))
    }
}
