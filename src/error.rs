use std::io;

use thiserror::Error;

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when invoking FFmpeg or parsing its output.
#[derive(Debug, Error)]
pub enum Error {
    /// The required binary could not be located on the current PATH.
    #[error("ffmpeg binary not found on PATH")]
    FFmpegNotFound {
        /// Suggestion for resolving the issue.
        suggestion: Option<String>,
    },

    /// A spawned command exited with a non-zero status code.
    #[error("{binary} failed (code: {exit_code:?}): {message}")]
    ProcessingError {
        /// Binary that was executed (ffmpeg/ffprobe).
        binary: String,
        /// Exit code if provided by the OS.
        exit_code: Option<i32>,
        /// Captured stderr output (truncated when large).
        message: String,
    },

    /// Invalid input parameters or missing required values.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Errors from filter configuration or composition.
    #[error("filter error: {0}")]
    FilterError(String),

    /// Timeout waiting for process completion.
    #[error("timeout: {0}")]
    TimeoutError(String),

    /// Errors produced by std::process or other IO operations.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Errors produced while parsing ffprobe JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Returned when textual parsing fails (for example, invalid duration strings).
    #[error("parse error: {0}")]
    Parse(String),

    /// Placeholder for functionality that has not yet been implemented.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

impl Error {
    /// Utility to build a `ProcessingError` from a binary label and captured output.
    pub(crate) fn command_failed(binary: &str, exit_code: Option<i32>, stderr: &[u8]) -> Self {
        let message = truncate(stderr);
        Error::ProcessingError {
            binary: binary.to_string(),
            exit_code,
            message,
        }
    }

    /// Suggestion for resolving this error (if available).
    pub fn suggestion(&self) -> Option<String> {
        match self {
            Error::FFmpegNotFound { suggestion } => suggestion.clone(),
            Error::InvalidInput(msg) => {
                if msg.contains("input path") {
                    Some("ensure input file exists and path is correct".to_string())
                } else if msg.contains("output path") {
                    Some("ensure output directory exists".to_string())
                } else {
                    Some("check your parameters".to_string())
                }
            }
            Error::ProcessingError { .. } => {
                Some("check FFmpeg is installed and your parameters are valid".to_string())
            }
            Error::FilterError(msg) => {
                if msg.contains("unsupported") || msg.contains("not supported") {
                    Some("check FFmpeg version supports this filter".to_string())
                } else {
                    Some("review filter parameters and syntax".to_string())
                }
            }
            _ => None,
        }
    }
}

fn truncate(message: &[u8]) -> String {
    const MAX: usize = 4096;
    let mut text = String::from_utf8_lossy(message).into_owned();
    if text.len() > MAX {
        text.truncate(MAX);
        text.push_str("…");
    }
    text
}
