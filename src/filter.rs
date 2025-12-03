//! Helper types for common FFmpeg video filters.

use std::fmt;

use crate::types::Time;

/// Filters supported by the high-level API.
#[derive(Clone, Debug, PartialEq)]
pub enum VideoFilter {
    /// Scale video to the provided width/height.
    Scale {
        /// Target width in pixels.
        width: u32,
        /// Target height in pixels.
        height: u32,
    },
    /// Trim video between `start` and optional `end` timestamps.
    Trim {
        /// Starting timestamp for the trim window.
        start: Time,
        /// Optional end timestamp; `None` trims until the end of the input.
        end: Option<Time>,
    },
    /// Custom filter string for advanced use-cases.
    Custom(String),
}

impl VideoFilter {
    /// Serialize into an FFmpeg `-vf` snippet.
    pub fn to_filter_string(&self) -> String {
        match self {
            VideoFilter::Scale { width, height } => format!("scale={width}:{height}"),
            VideoFilter::Trim { start, end } => match end {
                Some(end) => format!("trim=start={start}:end={end}"),
                None => format!("trim=start={start}"),
            },
            VideoFilter::Custom(raw) => raw.clone(),
        }
    }
}

impl fmt::Display for VideoFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_filter_string())
    }
}
