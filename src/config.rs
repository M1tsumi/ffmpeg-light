//! Configuration helpers for locating FFmpeg binaries.

use std::path::{Path, PathBuf};

use crate::command::FfmpegBinaryPaths;
use crate::error::{Error, Result};

/// Determines how the crate should locate `ffmpeg` and `ffprobe`.
#[derive(Clone, Debug)]
pub struct FfmpegLocator {
    paths: FfmpegBinaryPaths,
}

impl FfmpegLocator {
    /// Use binaries discovered on the current `PATH`.
    pub fn system() -> Result<Self> {
        Ok(Self {
            paths: FfmpegBinaryPaths::auto()?,
        })
    }

    /// Use explicitly provided binary paths.
    pub fn with_paths(ffmpeg: impl Into<PathBuf>, ffprobe: impl Into<PathBuf>) -> Result<Self> {
        let ffmpeg = ffmpeg.into();
        let ffprobe = ffprobe.into();
        if !ffmpeg.exists() {
            return Err(Error::BinaryNotFound {
                binary: ffmpeg.display().to_string(),
            });
        }
        if !ffprobe.exists() {
            return Err(Error::BinaryNotFound {
                binary: ffprobe.display().to_string(),
            });
        }
        Ok(Self {
            paths: FfmpegBinaryPaths::with_paths(ffmpeg, ffprobe),
        })
    }

    /// Reuse the given binary paths.
    pub fn from_paths(paths: FfmpegBinaryPaths) -> Self {
        Self { paths }
    }

    /// Raw binary paths.
    pub fn binaries(&self) -> &FfmpegBinaryPaths {
        &self.paths
    }

    /// Path to ffmpeg.
    pub fn ffmpeg(&self) -> &Path {
        self.paths.ffmpeg()
    }

    /// Path to ffprobe.
    pub fn ffprobe(&self) -> &Path {
        self.paths.ffprobe()
    }
}

/// Helper to detect binaries once and reuse them across operations.
pub fn system_locator() -> Result<FfmpegLocator> {
    FfmpegLocator::system()
}
