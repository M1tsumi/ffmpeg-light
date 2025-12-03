//! Transcoding helpers built on top of the CLI `ffmpeg` binary.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::command::{FfmpegBinaryPaths, FfmpegCommand};
use crate::config::FfmpegLocator;
use crate::error::{Error, Result};
use crate::filter::VideoFilter;

/// Builder-style API for spinning up simple ffmpeg jobs.
#[derive(Debug, Default)]
pub struct TranscodeBuilder {
    binaries: Option<FfmpegBinaryPaths>,
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    video_codec: Option<String>,
    audio_codec: Option<String>,
    video_bitrate: Option<u32>,
    audio_bitrate: Option<u32>,
    frame_rate: Option<f64>,
    preset: Option<String>,
    filters: Vec<VideoFilter>,
    extra_args: Vec<OsString>,
    overwrite: bool,
}

impl TranscodeBuilder {
    /// Create a new builder with sensible defaults (overwrite enabled).
    pub fn new() -> Self {
        Self {
            overwrite: true,
            ..Self::default()
        }
    }

    /// Use pre-discovered binaries instead of searching PATH every call.
    pub fn with_binaries(mut self, binaries: &FfmpegBinaryPaths) -> Self {
        self.binaries = Some(binaries.clone());
        self
    }

    /// Pin the builder to a specific locator.
    pub fn with_locator(mut self, locator: &FfmpegLocator) -> Self {
        self.binaries = Some(locator.binaries().clone());
        self
    }

    /// Input media path.
    pub fn input<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.input = Some(path.as_ref().to_path_buf());
        self
    }

    /// Output media path.
    pub fn output<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output = Some(path.as_ref().to_path_buf());
        self
    }

    /// Desired video codec (e.g. `libx264`).
    pub fn video_codec(mut self, codec: impl Into<String>) -> Self {
        self.video_codec = Some(codec.into());
        self
    }

    /// Desired audio codec (e.g. `aac`).
    pub fn audio_codec(mut self, codec: impl Into<String>) -> Self {
        self.audio_codec = Some(codec.into());
        self
    }

    /// Target video bitrate in kbps.
    pub fn video_bitrate(mut self, kbps: u32) -> Self {
        self.video_bitrate = Some(kbps);
        self
    }

    /// Target audio bitrate in kbps.
    pub fn audio_bitrate(mut self, kbps: u32) -> Self {
        self.audio_bitrate = Some(kbps);
        self
    }

    /// Target frame rate.
    pub fn frame_rate(mut self, fps: f64) -> Self {
        self.frame_rate = Some(fps);
        self
    }

    /// Apply a named preset (maps to `-preset`).
    pub fn preset(mut self, preset: impl Into<String>) -> Self {
        self.preset = Some(preset.into());
        self
    }

    /// Convenience helper to scale output.
    pub fn size(self, width: u32, height: u32) -> Self {
        self.add_filter(VideoFilter::Scale { width, height })
    }

    /// Push a filter into the video filter graph.
    pub fn add_filter(mut self, filter: VideoFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Pass a raw argument for advanced cases.
    pub fn extra_arg(mut self, arg: impl Into<OsString>) -> Self {
        self.extra_args.push(arg.into());
        self
    }

    /// Control whether ffmpeg should overwrite the output file.
    pub fn overwrite(mut self, enabled: bool) -> Self {
        self.overwrite = enabled;
        self
    }

    fn resolve_binaries(binaries: Option<FfmpegBinaryPaths>) -> Result<FfmpegBinaryPaths> {
        if let Some(paths) = binaries {
            return Ok(paths);
        }
        Ok(FfmpegLocator::system()?.binaries().clone())
    }

    fn validate(self) -> Result<ValidatedTranscode> {
        let Self {
            binaries,
            input,
            output,
            video_codec,
            audio_codec,
            video_bitrate,
            audio_bitrate,
            frame_rate,
            preset,
            filters,
            extra_args,
            overwrite,
        } = self;

        let input = input.ok_or_else(|| Error::InvalidInput("input path is required".into()))?;
        let output = output.ok_or_else(|| Error::InvalidInput("output path is required".into()))?;

        Ok(ValidatedTranscode {
            binaries: Self::resolve_binaries(binaries)?,
            input,
            output,
            video_codec,
            audio_codec,
            video_bitrate,
            audio_bitrate,
            frame_rate,
            preset,
            filters,
            extra_args,
            overwrite,
        })
    }

    /// Execute ffmpeg with the configured arguments.
    pub fn run(self) -> Result<()> {
        let validated = self.validate()?;
        validated.run()
    }
}

struct ValidatedTranscode {
    binaries: FfmpegBinaryPaths,
    input: PathBuf,
    output: PathBuf,
    video_codec: Option<String>,
    audio_codec: Option<String>,
    video_bitrate: Option<u32>,
    audio_bitrate: Option<u32>,
    frame_rate: Option<f64>,
    preset: Option<String>,
    filters: Vec<VideoFilter>,
    extra_args: Vec<OsString>,
    overwrite: bool,
}

impl ValidatedTranscode {
    fn run(self) -> Result<()> {
        let mut cmd = FfmpegCommand::new(self.binaries.ffmpeg());
        cmd.arg(if self.overwrite { "-y" } else { "-n" });
        cmd.arg("-i").arg(&self.input);

        if let Some(codec) = self.video_codec {
            cmd.arg("-c:v").arg(codec);
        }
        if let Some(codec) = self.audio_codec {
            cmd.arg("-c:a").arg(codec);
        }
        if let Some(kbps) = self.video_bitrate {
            cmd.arg("-b:v").arg(format!("{kbps}k"));
        }
        if let Some(kbps) = self.audio_bitrate {
            cmd.arg("-b:a").arg(format!("{kbps}k"));
        }
        if let Some(fps) = self.frame_rate {
            cmd.arg("-r").arg(format!("{fps}"));
        }
        if let Some(preset) = self.preset {
            cmd.arg("-preset").arg(preset);
        }

        let mut filter_strings: Vec<String> = Vec::new();
        for filter in self.filters {
            filter_strings.push(filter.to_filter_string());
        }
        if !filter_strings.is_empty() {
            cmd.arg("-vf").arg(filter_strings.join(","));
        }

        for arg in self.extra_args {
            cmd.arg(arg);
        }

        cmd.arg(&self.output);
        cmd.run()
    }
}
