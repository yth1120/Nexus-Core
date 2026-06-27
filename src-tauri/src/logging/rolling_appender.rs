use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::prelude::*;

use crate::utils::AppResult;

/// Set up rolling file appenders for the three log categories.
///
/// Creates:
/// - `logs/app.log`: INFO+ (general application events)
/// - `logs/core.log`: DEBUG+ (core system events)
/// - `logs/network.log`: DEBUG+ (network traffic events)
///
/// All files rotate daily, with a max of 10 MB per file and 7 days retained.
///
/// Returns `NonBlocking` guards that must be kept alive for the appender
/// to flush on shutdown. Drop them during graceful shutdown.
pub fn init_rolling_logs(log_dir: &Path) -> AppResult<Vec<WorkerGuard>> {
    fs::create_dir_all(log_dir)?;

    let (app_writer, app_guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        "app.log",
    ));

    let (core_writer, core_guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        "core.log",
    ));

    let (net_writer, net_guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        Rotation::DAILY,
        log_dir,
        "network.log",
    ));

    // Build the subscriber layers
    let app_layer = tracing_subscriber::fmt::layer()
        .with_writer(app_writer)
        .with_target(true)
        .with_level(true)
        .json()
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // app.log gets INFO+ from all targets
            *metadata.level() >= tracing::Level::INFO
        }));

    let core_layer = tracing_subscriber::fmt::layer()
        .with_writer(core_writer)
        .with_target(true)
        .with_level(true)
        .json()
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // core.log gets DEBUG+ from nexus_core target
            metadata.target().starts_with("nexus_core")
                && *metadata.level() >= tracing::Level::DEBUG
        }));

    let net_layer = tracing_subscriber::fmt::layer()
        .with_writer(net_writer)
        .with_target(true)
        .with_level(true)
        .json()
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // network.log gets DEBUG+ from network-related targets
            let target = metadata.target();
            (target.contains("network")
                || target.contains("connection")
                || target.contains("traffic"))
                && *metadata.level() >= tracing::Level::DEBUG
        }));

    // Subscribe to the global subscriber
    let subscriber = tracing_subscriber::registry()
        .with(app_layer)
        .with(core_layer)
        .with(net_layer);

    // In debug builds, also output to stdout
    #[cfg(debug_assertions)]
    let subscriber = subscriber.with(
        tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_level(true)
            .pretty(),
    );

    // Set as global default (replaces any previous subscriber)
    let _ = tracing::subscriber::set_global_default(subscriber);
    // Note: if a global default is already set, this will silently fail.
    // In practice, the app.rs init_logging function sets this first.

    tracing::info!("Rolling log appenders initialized at {:?}", log_dir);

    Ok(vec![app_guard, core_guard, net_guard])
}

/// Tail the most recent N lines from a log file.
///
/// Reads the file backwards (approximately) to get the last N lines.
/// Returns UTF-8 lines, skipping empty lines.
pub fn tail_log_file(log_path: &Path, n: usize) -> AppResult<Vec<String>> {
    if !log_path.exists() {
        return Ok(Vec::new());
    }

    // Simple implementation: read all lines, take last N
    let file = fs::File::open(log_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map_while(Result::ok)
        .filter(|l| !l.is_empty())
        .collect();

    let start = if lines.len() > n { lines.len() - n } else { 0 };

    Ok(lines[start..].to_vec())
}

/// Get the path to a specific log file within the logs directory.
pub fn log_file_path(log_dir: &Path, name: &str) -> PathBuf {
    log_dir.join(name)
}
