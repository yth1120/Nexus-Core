use std::sync::Arc;

use tauri::State;

use crate::core::CoreManager;
use crate::performance::benchmark::BenchmarkReport;
use crate::performance::memory_report::{MemoryProfiler, MemoryReport};
use crate::performance::profiler::Profiler;
use crate::performance::stress_test::StressTestReport;

#[tauri::command]
pub async fn run_benchmark(core: State<'_, Arc<CoreManager>>) -> Result<BenchmarkReport, String> {
    let report = crate::performance::BenchmarkRunner::run_all().map_err(|e| e.to_string())?;
    core.context()
        .publish(crate::event::AppEvent::BenchmarkCompleted);
    Ok(report)
}

#[tauri::command]
pub async fn run_stress_test(
    core: State<'_, Arc<CoreManager>>,
) -> Result<StressTestReport, String> {
    let report = crate::performance::StressTestRunner::run_all().map_err(|e| e.to_string())?;
    core.context()
        .publish(crate::event::AppEvent::StressTestCompleted);
    Ok(report)
}

#[tauri::command]
pub async fn generate_memory_report(
    _core: State<'_, Arc<CoreManager>>,
) -> Result<MemoryReport, String> {
    let samples = Profiler::sample(10, 100).map_err(|e| e.to_string())?;
    Ok(MemoryProfiler::generate_report(&samples))
}
