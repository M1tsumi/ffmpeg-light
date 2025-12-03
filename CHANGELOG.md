# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-03

### Added
- Initial release of `ffmpeg-light` crate.
- `probe` function for media file inspection via `ffprobe` JSON output.
- `TranscodeBuilder` for transcoding with codec, bitrate, preset, size, and filter options.
- `generate_thumbnail` function with `ThumbnailOptions` for timestamp/size/format controls.
- `VideoFilter` enum for common filters (`Scale`, `Fps`, `Pad`, `Custom`).
- Typed structs for probe results (`ProbeResult`, `VideoStreamInfo`, `AudioStreamInfo`).
- `Time` type for duration and timestamp handling.
- Unified error handling via `Error` and `Result` types.
- Apache-2.0 license.
- README with quick-start examples.
- Showcase documentation for probe, transcode, and thumbnail workflows.
