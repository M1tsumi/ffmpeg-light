//! Video and audio filter definitions. Filters can be composed into chains for complex processing.

use std::fmt;

use crate::types::Time;

/// Video filters for common editing tasks.
#[derive(Clone, Debug, PartialEq)]
pub enum VideoFilter {
    /// Scale video to the provided width/height. Use -1 for one dimension to preserve aspect ratio.
    Scale {
        width: u32,
        height: u32,
    },
    /// Trim video between start and optional end timestamps.
    Trim {
        start: Time,
        end: Option<Time>,
    },
    /// Crop video to specified dimensions at given offset.
    Crop {
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    },
    /// Rotate video by angle in degrees. Typically 90, 180, or 270.
    Rotate {
        degrees: f64,
    },
    /// Flip video horizontally or vertically. 'h' for horizontal, 'v' for vertical.
    Flip {
        direction: char,
    },
    /// Adjust brightness and contrast. Brightness range: -1.0 to 1.0, Contrast: 0.0 to 2.0.
    BrightnessContrast {
        brightness: Option<f32>,
        contrast: Option<f32>,
    },
    /// Remove noise with denoise filter (light, medium, heavy).
    Denoise {
        strength: DenoiseStrength,
    },
    /// Deinterlace interlaced video (useful for old TV recordings).
    Deinterlace,
    /// Custom filter string for advanced use-cases. FFmpeg syntax.
    Custom(String),
}

/// Denoise filter strength options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenoiseStrength {
    Light,
    Medium,
    Heavy,
}

impl DenoiseStrength {
    fn to_filter_value(&self) -> &'static str {
        match self {
            DenoiseStrength::Light => "hqdn3d=1.5:1.5:6:6",
            DenoiseStrength::Medium => "hqdn3d=3:3:6:6",
            DenoiseStrength::Heavy => "hqdn3d=5:5:6:6",
        }
    }
}

impl VideoFilter {
    /// Convert filter to FFmpeg `-vf` format string.
    pub fn to_filter_string(&self) -> String {
        match self {
            VideoFilter::Scale { width, height } => format!("scale={width}:{height}"),
            VideoFilter::Trim { start, end } => match end {
                Some(end) => format!("trim=start={start}:end={end}"),
                None => format!("trim=start={start}"),
            },
            VideoFilter::Crop { width, height, x, y } => {
                format!("crop={width}:{height}:{x}:{y}")
            }
            VideoFilter::Rotate { degrees } => {
                // FFmpeg rotate expects radians, but we use degrees for API simplicity
                let radians = degrees * std::f64::consts::PI / 180.0;
                format!("rotate={radians}")
            }
            VideoFilter::Flip { direction } => match direction {
                'h' => "hflip".to_string(),
                'v' => "vflip".to_string(),
                _ => "hflip".to_string(), // default to horizontal
            },
            VideoFilter::BrightnessContrast {
                brightness,
                contrast,
            } => {
                let mut parts = Vec::new();
                if let Some(b) = brightness {
                    parts.push(format!("brightness={b}"));
                }
                if let Some(c) = contrast {
                    parts.push(format!("contrast={c}"));
                }
                if parts.is_empty() {
                    "eq".to_string()
                } else {
                    format!("eq={}", parts.join(":"))
                }
            }
            VideoFilter::Denoise { strength } => strength.to_filter_value().to_string(),
            VideoFilter::Deinterlace => "yadif".to_string(),
            VideoFilter::Custom(raw) => raw.clone(),
        }
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_filter_string())
    }
}

/// Audio filters for sound processing.
#[derive(Clone, Debug, PartialEq)]
pub enum AudioFilter {
    /// Adjust volume. 1.0 = no change, 0.5 = half volume, 2.0 = double.
    Volume(f32),
    /// Equalization with bass, mid, and treble adjustments in dB.
    Equalizer {
        bass: Option<f32>,
        mid: Option<f32>,
        treble: Option<f32>,
    },
    /// Normalize audio to prevent clipping. Target level in dBFS.
    Normalization {
        target_level: f32,
    },
    /// High-pass filter to remove low frequencies. Frequency in Hz.
    HighPass {
        frequency: f32,
    },
    /// Low-pass filter to remove high frequencies. Frequency in Hz.
    LowPass {
        frequency: f32,
    },
    /// Custom audio filter for advanced use-cases. FFmpeg syntax.
    Custom(String),
}

impl AudioFilter {
    /// Convert filter to FFmpeg `-af` format string.
    pub fn to_filter_string(&self) -> String {
        match self {
            AudioFilter::Volume(vol) => format!("volume={vol}"),
            AudioFilter::Equalizer { bass, mid, treble } => {
                let mut parts = Vec::new();
                if let Some(b) = bass {
                    parts.push(format!("b={b}"));
                }
                if let Some(m) = mid {
                    parts.push(format!("m={m}"));
                }
                if let Some(t) = treble {
                    parts.push(format!("t={t}"));
                }
                if parts.is_empty() {
                    "superequalizer".to_string()
                } else {
                    format!("superequalizer={}", parts.join(":"))
                }
            }
            AudioFilter::Normalization { target_level } => {
                format!("anlmdn=m=I:p=0.05,loudnorm=I={target_level}")
            }
            AudioFilter::HighPass { frequency } => {
                format!("highpass=f={frequency}")
            }
            AudioFilter::LowPass { frequency } => {
                format!("lowpass=f={frequency}")
            }
            AudioFilter::Custom(raw) => raw.clone(),
        }
    }
}

impl fmt::Display for AudioFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_filter_string())
    }
}
