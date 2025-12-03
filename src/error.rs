use std::io;

use thiserror::Error;

/// Result alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when invoking FFmpeg or parsing its output.
#[derive(Debug, Error)]
pub enum Error {
    /// The required binary could not be located on the current PATH.
    #[error("binary '{binary}' not found on PATH")]
    BinaryNotFound {
        /// Name or path of the binary that could not be located.
        binary: String,
    },

    /// A spawned command exited with a non-zero status code.
    #[error("{binary} failed (code: {exit_code:?}): {message}")]
    CommandFailed {
        /// Binary that was executed (ffmpeg/ffprobe).
        binary: String,
        /// Exit code if provided by the OS.
        exit_code: Option<i32>,
        /// Captured stderr output (truncated when large).
        message: String,
    },

    /// Errors produced by std::process or other IO operations.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// Errors produced while parsing ffprobe JSON.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// Returned when user input does not satisfy the builder requirements.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// Returned when textual parsing fails (for example, invalid duration strings).
    #[error("parse error: {0}")]
    Parse(String),

    /// Placeholder for functionality that has not yet been implemented.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

impl Error {
    /// Utility to build a `CommandFailed` from a binary label and captured output.
    pub(crate) fn command_failed(binary: &str, exit_code: Option<i32>, stderr: &[u8]) -> Self {
        let message = truncate(stderr);
        Error::CommandFailed {
            binary: binary.to_string(),
            exit_code,
            message,
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
