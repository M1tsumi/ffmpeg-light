#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

//! ffmpeg-light is a small Rust crate that wraps a few common FFmpeg tasks.
//!
//! It focuses on the "80% use cases" like probing media, transcoding, and generating thumbnails,
//! without exposing the full complexity of FFmpeg.
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! ffmpeg-light = "0.1"
//! ```
//!
//! Note: This crate requires `ffmpeg` and `ffprobe` binaries to be installed and available on PATH.
//!
//! ## Example: Probe a video file
//!
//! ```rust,no_run
//! use ffmpeg_light::probe;
//!
//! let result = probe("input.mp4")?;
//! println!("Duration: {:?}", result.duration());
//! # Ok::<(), ffmpeg_light::Error>(())
//! ```
//!
//! ## Example: Transcode to H.264 MP4
//!
//! ```rust,no_run
//! use ffmpeg_light::transcode::TranscodeBuilder;
//!
//! TranscodeBuilder::new()
//!     .input("input.avi")
//!     .output("output.mp4")
//!     .video_codec("libx264")
//!     .run()?;
//! # Ok::<(), ffmpeg_light::Error>(())
//! ```

/// Low-level process helpers for interacting with ffmpeg and ffprobe.
pub mod command;
/// Configuration helpers for locating ffmpeg binaries.
pub mod config;
/// Shared error type and `Result` alias used by the crate.
pub mod error;
/// Small collection of filter helpers used by transcoding.
pub mod filter;
/// Media probing API built on top of `ffprobe` JSON output.
pub mod probe;
/// Thumbnail generation helpers.
pub mod thumbnail;
/// Builder API around common transcoding flows.
pub mod transcode;
/// Shared domain types (timecodes, codecs, stream metadata).
pub mod types;

// Re-export main types for convenience
pub use error::{Error, Result};
pub use filter::{AudioFilter, VideoFilter};
pub use probe::probe;
pub use thumbnail::generate as generate_thumbnail;
pub use transcode::TranscodeBuilder;
pub use types::*;
