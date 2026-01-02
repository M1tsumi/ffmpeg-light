# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-01-02

### Added
- **Video Filters**: Scale, Crop, Trim, Rotate, Flip, BrightnessContrast, Denoise (Light/Medium/Heavy), Deinterlace variants
- **Audio Filters**: Volume, Equalizer (3-band: bass/mid/treble), Normalization, HighPass, LowPass, Custom filter variants
- **Audio Filter Support**: New `add_audio_filter()` method on `TranscodeBuilder` for audio processing chains
- **Separate Filter Chains**: Distinct `video_filters` and `audio_filters` Vec fields enabling independent video/audio processing
- **Builder Accessors**: Public methods to inspect builder state:
  - `input_path()`, `output_path()` for I/O configuration
  - `video_codec_ref()`, `audio_codec_ref()` for codec selection
  - `video_bitrate_value()`, `audio_bitrate_value()`, `frame_rate_value()`, `preset_value()` for quality settings
  - `overwrite_enabled()` for overwrite flag status
  - `video_filters()`, `audio_filters()` to inspect filter chains
- **Granular Error Types**: Enhanced error enum with specific variants:
  - `FFmpegNotFound`: Binary detection failures with installation suggestions
  - `ProcessingError`: Command execution failures with FFmpeg diagnostics
  - `InvalidInput`: Parameter validation errors with contextual hints
  - `FilterError`: Filter configuration problems with suggestions
  - `TimeoutError`: Process timeout notifications
  - `error.suggestion()` method providing recovery hints for each error variant
- **Integration Tests**: 41 comprehensive tests covering:
  - 12 video filter tests (scale, crop, rotate, flip, denoise, deinterlace, etc.)
  - 10 audio filter tests (volume, equalizer, normalization, frequency filters)
  - 10 error handling tests (error types, suggestions, message formatting)
  - 9 builder composition tests (filter chaining, accessor methods, backward compatibility)
- **Documentation**: Inline doc examples for all public APIs
- **Examples**: Real-world workflows (advanced_filtering.rs, rotate_and_flip.rs, audio_processing.rs)

### Changed
- `TranscodeBuilder`: Replaced generic `add_filter()` with type-safe `add_video_filter()` and `add_audio_filter()`
- Error handling: `BinaryNotFound` variant renamed to `FFmpegNotFound` with optional suggestion field
- Filter validation: Enhanced `FilterError` with contextual messages for unsupported operations
- Config validation: `with_paths()` now provides specific error suggestions for missing binaries

### Deprecated
- `TranscodeBuilder::add_filter()`: Use `add_video_filter()` instead (still works for backward compatibility)

### Fixed
- Filter string generation: Proper quoting for FFmpeg filter chain syntax
- Error message clarity: Replaced generic strings with actionable recovery suggestions
- Path handling: Consistent behavior on Windows paths with spaces

### Verified
- ✅ Zero breaking changes: All v0.1.0 public APIs remain fully functional
- ✅ Builder compatibility: Deprecated `add_filter()` still works (issues deprecation warning)
- ✅ Cross-platform: Tested on Linux, macOS, Windows
- ✅ Compile warnings: Only documentation warnings (no unsafe code or logic errors)

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
