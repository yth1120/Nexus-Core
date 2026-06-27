use serde::Serialize;

/// Unified error type for the entire application.
/// Implements `thiserror::Error` for ergonomic `?` usage,
/// and `Serialize` so Tauri can return errors to the frontend.
#[derive(Debug, thiserror::Error, Serialize)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("I/O error: {0}")]
    Io(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Unsupported on this platform: {0}")]
    Unsupported(String),

    #[error("Shutdown in progress: {0}")]
    Shutdown(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(e: toml::ser::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        AppError::Database(e.to_string())
    }
}

impl From<notify::Error> for AppError {
    fn from(e: notify::Error) -> Self {
        AppError::Config(e.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Internal(e.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
