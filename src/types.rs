//! Common domain types shared across the crate.

use std::fmt;
use std::time::Duration;

/// Represents a position in time used for seeking and trimming.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(Duration);

impl Time {
    /// Zero timestamp.
    pub const fn zero() -> Self {
        Self(Duration::from_secs(0))
    }

    /// Create an instance from whole seconds.
    pub fn from_seconds(seconds: u64) -> Self {
        Self(Duration::from_secs(seconds))
    }

    /// Create from a floating-point second representation.
    pub fn from_seconds_f64(seconds: f64) -> Self {
        let nanos = (seconds * 1_000_000_000.0).round();
        let secs = (nanos / 1_000_000_000.0).trunc() as u64;
        let sub_nanos = (nanos % 1_000_000_000.0) as u32;
        Self(Duration::new(secs, sub_nanos))
    }

    /// Create from an existing `Duration`.
    pub const fn from_duration(duration: Duration) -> Self {
        Self(duration)
    }

    /// Convert to std `Duration`.
    pub const fn as_duration(self) -> Duration {
        self.0
    }

    /// Convert to the timestamp format expected by FFmpeg (HH:MM:SS.mmm).
    pub fn to_ffmpeg_timestamp(self) -> String {
        let total_secs = self.0.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        let millis = self.0.subsec_millis();
        format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        Self(duration)
    }
}

impl From<Time> for Duration {
    fn from(value: Time) -> Self {
        value.0
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ffmpeg_timestamp())
    }
}

/// High-level codec representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodecType {
    /// H.264/AVC video.
    H264,
    /// H.265/HEVC video.
    Hevc,
    /// VP9 video.
    Vp9,
    /// AV1 video.
    Av1,
    /// AAC audio.
    Aac,
    /// MP3 audio.
    Mp3,
    /// Opus audio.
    Opus,
    /// PCM S16LE audio.
    PcmS16Le,
    /// Copy stream without re-encoding.
    Copy,
    /// Any other codec string returned by FFmpeg.
    Other(String),
}

impl CodecType {
    /// Create from a codec name string (case insensitive).
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "h264" | "libx264" => CodecType::H264,
            "hevc" | "h265" | "libx265" => CodecType::Hevc,
            "vp9" => CodecType::Vp9,
            "av1" => CodecType::Av1,
            "aac" => CodecType::Aac,
            "mp3" => CodecType::Mp3,
            "opus" => CodecType::Opus,
            "pcm_s16le" => CodecType::PcmS16Le,
            "copy" => CodecType::Copy,
            other => CodecType::Other(other.to_string()),
        }
    }

    /// Convert back into an FFmpeg codec string.
    pub fn as_str(&self) -> &str {
        match self {
            CodecType::H264 => "libx264",
            CodecType::Hevc => "libx265",
            CodecType::Vp9 => "libvpx-vp9",
            CodecType::Av1 => "libaom-av1",
            CodecType::Aac => "aac",
            CodecType::Mp3 => "libmp3lame",
            CodecType::Opus => "libopus",
            CodecType::PcmS16Le => "pcm_s16le",
            CodecType::Copy => "copy",
            CodecType::Other(name) => name,
        }
    }
}

/// Simplified stream classification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamType {
    /// Video stream.
    Video,
    /// Audio stream.
    Audio,
    /// Subtitle stream.
    Subtitle,
    /// Auxiliary data.
    Data,
}

/// Container-level metadata reported by ffprobe.
#[derive(Clone, Debug)]
pub struct FormatInfo {
    /// Name of the format (e.g. "mov,mp4,m4a,3gp,3g2,mj2").
    pub format_name: Option<String>,
    /// Human readable format description.
    pub format_long_name: Option<String>,
    /// Optional duration.
    pub duration: Option<Duration>,
    /// Optional overall bitrate.
    pub bit_rate: Option<u64>,
    /// Optional file size in bytes.
    pub size: Option<u64>,
}

impl FormatInfo {
    /// Create a new instance with defaults.
    pub fn new(
        format_name: Option<String>,
        format_long_name: Option<String>,
        duration: Option<Duration>,
        bit_rate: Option<u64>,
        size: Option<u64>,
    ) -> Self {
        Self {
            format_name,
            format_long_name,
            duration,
            bit_rate,
            size,
        }
    }
}

/// Stream metadata.
#[derive(Clone, Debug)]
pub enum StreamInfo {
    /// Video stream info.
    Video(VideoStreamInfo),
    /// Audio stream info.
    Audio(AudioStreamInfo),
    /// Subtitle stream info.
    Subtitle(SubtitleStreamInfo),
    /// Auxiliary data stream info.
    Data(DataStreamInfo),
}

/// Video stream metadata.
#[derive(Clone, Debug)]
pub struct VideoStreamInfo {
    /// Codec identifier.
    pub codec: CodecType,
    /// Width in pixels.
    pub width: Option<u32>,
    /// Height in pixels.
    pub height: Option<u32>,
    /// Bit rate in bits/sec.
    pub bit_rate: Option<u64>,
    /// Average frame rate (frames per second).
    pub frame_rate: Option<f64>,
}

/// Audio stream metadata.
#[derive(Clone, Debug)]
pub struct AudioStreamInfo {
    /// Codec identifier.
    pub codec: CodecType,
    /// Number of audio channels.
    pub channels: Option<u32>,
    /// Sample rate in Hz.
    pub sample_rate: Option<u32>,
    /// Bit rate in bits/sec.
    pub bit_rate: Option<u64>,
}

/// Subtitle stream metadata.
#[derive(Clone, Debug)]
pub struct SubtitleStreamInfo {
    /// Codec identifier.
    pub codec: CodecType,
    /// Optional language tag (e.g. "eng").
    pub language: Option<String>,
}

/// Misc data stream metadata.
#[derive(Clone, Debug)]
pub struct DataStreamInfo {
    /// Codec identifier.
    pub codec: CodecType,
    /// Optional handler description.
    pub description: Option<String>,
}

/// Top-level probe result.
#[derive(Clone, Debug)]
pub struct ProbeResult {
    format: FormatInfo,
    streams: Vec<StreamInfo>,
}

impl ProbeResult {
    /// Create a new result.
    pub fn new(format: FormatInfo, streams: Vec<StreamInfo>) -> Self {
        Self { format, streams }
    }

    /// Format metadata (container-level details).
    pub fn format(&self) -> &FormatInfo {
        &self.format
    }

    /// All streams reported by ffprobe.
    pub fn streams(&self) -> &[StreamInfo] {
        &self.streams
    }

    /// Convenience helper returning first video stream.
    pub fn first_video(&self) -> Option<&VideoStreamInfo> {
        self.streams.iter().find_map(|stream| match stream {
            StreamInfo::Video(info) => Some(info),
            _ => None,
        })
    }

    /// Convenience helper returning first audio stream.
    pub fn first_audio(&self) -> Option<&AudioStreamInfo> {
        self.streams.iter().find_map(|stream| match stream {
            StreamInfo::Audio(info) => Some(info),
            _ => None,
        })
    }

    /// Duration if reported by ffprobe.
    pub fn duration(&self) -> Option<Duration> {
        self.format.duration
    }
}
