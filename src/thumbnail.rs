//! Thumbnail generation helpers.

use std::path::Path;

use crate::command::{FfmpegBinaryPaths, FfmpegCommand};
use crate::config::FfmpegLocator;
use crate::error::Result;
use crate::types::Time;

/// Supported output formats.
#[derive(Clone, Debug)]
pub enum ThumbnailFormat {
    /// Portable Network Graphics (png).
    Png,
    /// JPEG (jpg/jpeg).
    Jpeg,
}

impl ThumbnailFormat {
    fn extension(&self) -> &'static str {
        match self {
            ThumbnailFormat::Png => "png",
            ThumbnailFormat::Jpeg => "jpg",
        }
    }

    fn ffmpeg_args(&self) -> &'static [&'static str] {
        match self {
            ThumbnailFormat::Png => &["-f", "image2"],
            ThumbnailFormat::Jpeg => &["-f", "mjpeg"],
        }
    }
}

/// Options for generating a thumbnail.
#[derive(Clone, Debug)]
pub struct ThumbnailOptions {
    time: Time,
    width: Option<u32>,
    height: Option<u32>,
    format: ThumbnailFormat,
}

impl ThumbnailOptions {
    /// Create with the specified timestamp.
    pub fn new(time: Time) -> Self {
        Self {
            time,
            width: None,
            height: None,
            format: ThumbnailFormat::Png,
        }
    }

    /// The timestamp at which the thumbnail will be captured.
    pub fn time(&self) -> Time {
        self.time
    }

    /// Set the output dimensions.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Current width/height configuration.
    pub fn dimensions(&self) -> Option<(u32, u32)> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some((w, h)),
            _ => None,
        }
    }

    /// Choose a specific output format.
    pub fn format(mut self, format: ThumbnailFormat) -> Self {
        self.format = format;
        self
    }

    /// Output format getter.
    pub fn output_format(&self) -> &ThumbnailFormat {
        &self.format
    }
}

/// Generate a thumbnail file.
pub fn generate(
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    options: &ThumbnailOptions,
) -> Result<()> {
    let locator = FfmpegLocator::system()?;
    generate_with_binaries(locator.binaries(), input, output, options)
}

/// Same as [`generate`] but reuses already-discovered binaries.
pub fn generate_with_binaries(
    binaries: &FfmpegBinaryPaths,
    input: impl AsRef<Path>,
    output: impl AsRef<Path>,
    options: &ThumbnailOptions,
) -> Result<()> {
    if let Some(parent) = output.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut cmd = FfmpegCommand::new(binaries.ffmpeg());
    cmd.arg("-y");
    cmd.arg("-ss").arg(options.time.to_ffmpeg_timestamp());
    cmd.arg("-i").arg(input.as_ref());
    cmd.arg("-vframes").arg("1");

    if let (Some(width), Some(height)) = (options.width, options.height) {
        cmd.arg("-vf").arg(format!("scale={width}:{height}"));
    }

    for arg in options.format.ffmpeg_args() {
        cmd.arg(arg);
    }

    let mut output_path = output.as_ref().to_path_buf();
    if output_path.extension().is_none() {
        output_path.set_extension(options.format.extension());
    }
    cmd.arg(output_path);
    cmd.run()
}
