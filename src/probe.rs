//! Media probing utilities built on top of `ffprobe`.

use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

use serde::Deserialize;

#[cfg(feature = "tokio")]
use crate::command::ffprobe_json_async;
use crate::command::{ffprobe_json, FfmpegBinaryPaths};
use crate::config::FfmpegLocator;
use crate::error::Result;
use crate::types::{
    AudioStreamInfo, CodecType, DataStreamInfo, FormatInfo, ProbeResult, StreamInfo,
    SubtitleStreamInfo, VideoStreamInfo,
};

/// Probe a file using binaries discovered on the current PATH.
pub fn probe(path: impl AsRef<Path>) -> Result<ProbeResult> {
    let locator = FfmpegLocator::system()?;
    probe_with_locator(&locator, path)
}

/// Async variant of [`probe`] (requires the `tokio` feature).
#[cfg(feature = "tokio")]
pub async fn probe_async(path: impl AsRef<Path>) -> Result<ProbeResult> {
    let locator = FfmpegLocator::system()?;
    probe_with_locator_async(&locator, path).await
}

/// Probe a file with a pre-configured locator (useful for custom binary paths).
pub fn probe_with_locator(locator: &FfmpegLocator, path: impl AsRef<Path>) -> Result<ProbeResult> {
    probe_with_binaries(locator.binaries(), path)
}

/// Async variant of [`probe_with_locator`] (requires the `tokio` feature).
#[cfg(feature = "tokio")]
pub async fn probe_with_locator_async(
    locator: &FfmpegLocator,
    path: impl AsRef<Path>,
) -> Result<ProbeResult> {
    probe_with_binaries_async(locator.binaries(), path).await
}

/// Probe a file using already-resolved binaries.
pub fn probe_with_binaries(
    paths: &FfmpegBinaryPaths,
    path: impl AsRef<Path>,
) -> Result<ProbeResult> {
    let json = ffprobe_json(paths, path.as_ref())?;
    parse_probe_output(&json)
}

/// Async variant of [`probe_with_binaries`] (requires the `tokio` feature).
#[cfg(feature = "tokio")]
pub async fn probe_with_binaries_async(
    paths: &FfmpegBinaryPaths,
    path: impl AsRef<Path>,
) -> Result<ProbeResult> {
    let json = ffprobe_json_async(paths, path.as_ref()).await?;
    parse_probe_output(&json)
}

fn parse_probe_output(json: &str) -> Result<ProbeResult> {
    let data: FfprobeOutput = serde_json::from_str(json)?;
    let format = data
        .format
        .map(format_info_from_ffprobe)
        .unwrap_or_else(|| FormatInfo::new(None, None, None, None, None));
    let streams = data
        .streams
        .into_iter()
        .filter_map(stream_info_from_ffprobe)
        .collect();
    Ok(ProbeResult::new(format, streams))
}

#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    format: Option<FfprobeFormat>,
    #[serde(default)]
    streams: Vec<FfprobeStream>,
}

#[derive(Debug, Deserialize)]
struct FfprobeFormat {
    format_name: Option<String>,
    format_long_name: Option<String>,
    duration: Option<String>,
    bit_rate: Option<String>,
    size: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    bit_rate: Option<String>,
    avg_frame_rate: Option<String>,
    channels: Option<u32>,
    sample_rate: Option<String>,
    tags: Option<HashMap<String, String>>,
}

fn format_info_from_ffprobe(format: FfprobeFormat) -> FormatInfo {
    FormatInfo::new(
        format.format_name,
        format.format_long_name,
        parse_duration(format.duration.as_deref()),
        parse_u64(format.bit_rate.as_deref()),
        parse_u64(format.size.as_deref()),
    )
}

fn stream_info_from_ffprobe(stream: FfprobeStream) -> Option<StreamInfo> {
    let codec = stream
        .codec_name
        .as_deref()
        .map(CodecType::from_name)
        .unwrap_or_else(|| CodecType::Other("unknown".into()));
    match stream.codec_type.as_deref() {
        Some("video") => Some(StreamInfo::Video(VideoStreamInfo {
            codec,
            width: stream.width,
            height: stream.height,
            bit_rate: parse_u64(stream.bit_rate.as_deref()),
            frame_rate: parse_ratio(stream.avg_frame_rate.as_deref()),
        })),
        Some("audio") => Some(StreamInfo::Audio(AudioStreamInfo {
            codec,
            channels: stream.channels,
            sample_rate: parse_u32(stream.sample_rate.as_deref()),
            bit_rate: parse_u64(stream.bit_rate.as_deref()),
        })),
        Some("subtitle") => {
            let language = stream
                .tags
                .as_ref()
                .and_then(|tags| tags.get("language").cloned());
            Some(StreamInfo::Subtitle(SubtitleStreamInfo { codec, language }))
        }
        Some("data") => Some(StreamInfo::Data(DataStreamInfo {
            codec,
            description: stream
                .tags
                .as_ref()
                .and_then(|tags| tags.get("title").cloned()),
        })),
        _ => None,
    }
}

fn parse_duration(raw: Option<&str>) -> Option<Duration> {
    raw.and_then(|value| value.parse::<f64>().ok())
        .map(Duration::from_secs_f64)
}

fn parse_u64(raw: Option<&str>) -> Option<u64> {
    raw.and_then(|value| value.parse().ok())
}

fn parse_u32(raw: Option<&str>) -> Option<u32> {
    raw.and_then(|value| value.parse().ok())
}

fn parse_ratio(raw: Option<&str>) -> Option<f64> {
    let raw = raw?;
    if raw == "0/0" || raw == "0" {
        return None;
    }
    if let Some((num, den)) = raw.split_once('/') {
        let num: f64 = num.parse().ok()?;
        let den: f64 = den.parse().ok()?;
        if den.abs() < f64::EPSILON {
            return None;
        }
        Some(num / den)
    } else {
        raw.parse().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::parse_ratio;

    #[test]
    fn ratio_parsing() {
        assert_eq!(parse_ratio(Some("30000/1001")), Some(30_000.0 / 1_001.0));
        assert_eq!(parse_ratio(Some("0/0")), None);
        assert_eq!(parse_ratio(Some("59.94")), Some(59.94));
        assert_eq!(parse_ratio(None), None);
    }
}
