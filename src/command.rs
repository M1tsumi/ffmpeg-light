//! Low-level helpers for invoking the `ffmpeg` and `ffprobe` binaries.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

#[cfg(feature = "tokio")]
use tokio::process::Command as TokioCommand;

use which::which;

use crate::error::{Error, Result};

/// Paths to ffmpeg/ffprobe binaries used by the crate.
#[derive(Debug, Clone)]
pub struct FfmpegBinaryPaths {
    ffmpeg: PathBuf,
    ffprobe: PathBuf,
}

impl FfmpegBinaryPaths {
    /// Locate binaries on PATH.
    pub fn auto() -> Result<Self> {
        let ffmpeg = which("ffmpeg").map_err(|_| Error::FFmpegNotFound {
            suggestion: Some("install ffmpeg with 'brew install ffmpeg' (macOS), 'apt install ffmpeg' (Linux), or download from ffmpeg.org".to_string()),
        })?;
        let ffprobe = which("ffprobe").map_err(|_| Error::FFmpegNotFound {
            suggestion: Some("ffprobe comes with ffmpeg installation".to_string()),
        })?;
        Ok(Self { ffmpeg, ffprobe })
    }

    /// Override binaries manually.
    pub fn with_paths<P, Q>(ffmpeg: P, ffprobe: Q) -> Self
    where
        P: Into<PathBuf>,
        Q: Into<PathBuf>,
    {
        Self {
            ffmpeg: ffmpeg.into(),
            ffprobe: ffprobe.into(),
        }
    }

    /// Path to the ffmpeg binary.
    pub fn ffmpeg(&self) -> &Path {
        &self.ffmpeg
    }

    /// Path to the ffprobe binary.
    pub fn ffprobe(&self) -> &Path {
        &self.ffprobe
    }
}

/// Builder around `ffmpeg` command invocations.
#[derive(Debug)]
pub struct FfmpegCommand {
    binary: PathBuf,
    args: Vec<OsString>,
}

impl FfmpegCommand {
    /// Start building a command using the provided binary path.
    pub fn new(binary: impl Into<PathBuf>) -> Self {
        Self {
            binary: binary.into(),
            args: Vec::new(),
        }
    }

    /// Append an argument.
    pub fn arg<T: AsRef<OsStr>>(&mut self, arg: T) -> &mut Self {
        self.args.push(arg.as_ref().into());
        self
    }

    /// Append multiple arguments.
    pub fn args<T: AsRef<OsStr>>(&mut self, args: &[T]) -> &mut Self {
        self.args.extend(args.iter().map(|arg| arg.as_ref().into()));
        self
    }

    fn spawn_command(&self) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.args(&self.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::piped());
        cmd
    }

    #[cfg(feature = "tokio")]
    fn spawn_async_command(&self) -> TokioCommand {
        let mut cmd = TokioCommand::new(&self.binary);
        cmd.args(&self.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::piped());
        cmd
    }

    /// Run the command and inherit stdout.
    pub fn run(&self) -> Result<()> {
        let output = self.run_with_output()?;
        if !output.status.success() {
            return Err(Error::command_failed(
                display_path(&self.binary),
                output.status.code(),
                &output.stderr,
            ));
        }
        Ok(())
    }

    /// Run the command and capture stdout/stderr.
    pub fn run_with_output(&self) -> Result<Output> {
        let mut cmd = self.spawn_command();
        let output = cmd.output()?;
        Ok(output)
    }

    /// Run the command asynchronously (requires the `tokio` feature).
    #[cfg(feature = "tokio")]
    pub async fn run_async(&self) -> Result<()> {
        let output = self.run_with_output_async().await?;
        if !output.status.success() {
            return Err(Error::command_failed(
                display_path(&self.binary),
                output.status.code(),
                &output.stderr,
            ));
        }
        Ok(())
    }

    /// Run the command asynchronously and capture stdout/stderr (requires `tokio`).
    #[cfg(feature = "tokio")]
    pub async fn run_with_output_async(&self) -> Result<Output> {
        let mut cmd = self.spawn_async_command();
        let output = cmd.output().await?;
        Ok(output)
    }
}

/// Specialized command for `ffprobe` returning JSON output.
pub struct FfprobeCommand {
    binary: PathBuf,
    input: PathBuf,
    extra_args: Vec<OsString>,
}

impl FfprobeCommand {
    /// Create a command targeting a specific input file.
    pub fn new(binary: impl Into<PathBuf>, input: impl Into<PathBuf>) -> Self {
        Self {
            binary: binary.into(),
            input: input.into(),
            extra_args: Vec::new(),
        }
    }

    /// Add extra arguments (before ffprobe defaults, e.g. -v quiet).
    pub fn arg<T: AsRef<OsStr>>(&mut self, arg: T) -> &mut Self {
        self.extra_args.push(arg.as_ref().into());
        self
    }

    fn build_command(&self) -> Command {
        let mut cmd = Command::new(&self.binary);
        cmd.arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams");
        for arg in &self.extra_args {
            cmd.arg(arg);
        }
        cmd.arg(&self.input)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    #[cfg(feature = "tokio")]
    fn build_async_command(&self) -> TokioCommand {
        let mut cmd = TokioCommand::new(&self.binary);
        cmd.arg("-v")
            .arg("quiet")
            .arg("-print_format")
            .arg("json")
            .arg("-show_format")
            .arg("-show_streams");
        for arg in &self.extra_args {
            cmd.arg(arg);
        }
        cmd.arg(&self.input)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        cmd
    }

    /// Execute ffprobe and fetch the captured output.
    pub fn run(&self) -> Result<Output> {
        let output = self.build_command().output()?;
        if !output.status.success() {
            return Err(Error::command_failed(
                display_path(&self.binary),
                output.status.code(),
                &output.stderr,
            ));
        }
        Ok(output)
    }

    /// Async variant of [`run`] (requires `tokio`).
    #[cfg(feature = "tokio")]
    pub async fn run_async(&self) -> Result<Output> {
        let output = self.build_async_command().output().await?;
        if !output.status.success() {
            return Err(Error::command_failed(
                display_path(&self.binary),
                output.status.code(),
                &output.stderr,
            ));
        }
        Ok(output)
    }
}

/// Convenience to run ffprobe and return stdout as string.
pub fn ffprobe_json(paths: &FfmpegBinaryPaths, input: impl AsRef<Path>) -> Result<String> {
    let cmd = FfprobeCommand::new(paths.ffprobe(), input.as_ref());
    let output = cmd.run()?;
    let json = String::from_utf8(output.stdout).map_err(|err| Error::Parse(err.to_string()))?;
    Ok(json)
}

/// Async helper returning the ffprobe JSON payload.
#[cfg(feature = "tokio")]
pub async fn ffprobe_json_async(
    paths: &FfmpegBinaryPaths,
    input: impl AsRef<Path>,
) -> Result<String> {
    let mut cmd = FfprobeCommand::new(paths.ffprobe(), input.as_ref());
    let output = cmd.run_async().await?;
    let json = String::from_utf8(output.stdout).map_err(|err| Error::Parse(err.to_string()))?;
    Ok(json)
}

fn display_path(path: &Path) -> &str {
    path.to_str().unwrap_or("<invalid utf8 path>")
}

#[cfg(test)]
impl FfmpegCommand {
    pub(crate) fn test_binary(&self) -> &Path {
        &self.binary
    }

    pub(crate) fn test_args(&self) -> &[OsString] {
        &self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stringify_args(cmd: &FfmpegCommand) -> Vec<String> {
        cmd.test_args()
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn ffmpeg_command_collects_args_in_order() {
        let mut cmd = FfmpegCommand::new("/usr/bin/ffmpeg");
        cmd.arg("-y")
            .arg("-i")
            .arg("input.mp4")
            .args(&[OsStr::new("-c:v"), OsStr::new("libx264")]);

        assert_eq!(cmd.test_binary(), Path::new("/usr/bin/ffmpeg"));
        assert_eq!(
            stringify_args(&cmd),
            vec!["-y", "-i", "input.mp4", "-c:v", "libx264"]
        );
    }

    #[test]
    fn ffprobe_command_includes_json_flags() {
        let cmd = FfprobeCommand::new("/usr/bin/ffprobe", "video.mkv");
        let process = cmd.build_command();
        let args = process
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            args,
            vec![
                "-v",
                "quiet",
                "-print_format",
                "json",
                "-show_format",
                "-show_streams",
                "video.mkv"
            ]
        );
    }
}
