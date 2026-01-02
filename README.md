# ffmpeg-light

A Rust crate that wraps common FFmpeg tasks without the CLI memorization. Transcode, probe, filter, and process video/audio with a clean, type-safe API.

## Features

- **Video & Audio Filters**: Scale, crop, rotate, flip, denoise, deinterlace, volume control, equalization, normalization, and more
- **Fluent Builder API**: Chain operations naturally with `TranscodeBuilder`
- **Robust Error Handling**: Granular error types with contextual recovery suggestions
- **Media Probing**: Extract duration, codecs, resolutions, and stream information
- **Thumbnail Generation**: Extract frames at specific timestamps with size control
- **Filter Composition**: Build complex filter chains with automatic optimization
- **Zero Breaking Changes**: All v0.1.0 APIs remain fully supported

## Why this exists

FFmpeg is powerful but the CLI has hundreds of flags. Using raw C bindings means managing C library builds. This crate sits in the middle: spawns the `ffmpeg`/`ffprobe` binaries you already have and gives you a type-safe Rust API.

## Install

```toml
[dependencies]
ffmpeg-light = "0.2"
```

Requires `ffmpeg` and `ffprobe` on `PATH`. Get them via:
- macOS: `brew install ffmpeg`
- Linux: `apt install ffmpeg` (or equivalent)
- Windows: [ffmpeg.org](https://ffmpeg.org/download.html)

## Quick Start

### Probe a file

```rust,no_run
use ffmpeg_light::probe;

fn main() -> ffmpeg_light::Result<()> {
    let info = probe("input.mp4")?;
    println!("Duration: {:?}", info.duration());
    Ok(())
}
```

### Transcode with filters

```rust,no_run
use ffmpeg_light::{AudioFilter, TranscodeBuilder, VideoFilter};

fn main() -> ffmpeg_light::Result<()> {
    TranscodeBuilder::new()
        .input("input.avi")
        .output("output.mp4")
        .video_codec("libx264")
        .audio_codec("aac")
        .video_bitrate(2500)
        .add_video_filter(VideoFilter::Scale {
            width: 1280,
            height: 720,
        })
        .add_audio_filter(AudioFilter::Normalization {
            target_level: -23.0,
        })
        .run()?;
    Ok(())
}
```

### Apply multiple filters

```rust,no_run
use ffmpeg_light::{AudioFilter, TranscodeBuilder, VideoFilter};

fn main() -> ffmpeg_light::Result<()> {
    TranscodeBuilder::new()
        .input("raw.mov")
        .output("processed.mp4")
        .add_video_filter(VideoFilter::Crop {
            width: 1920,
            height: 1080,
            x: 0,
            y: 0,
        })
        .add_video_filter(VideoFilter::Denoise {
            strength: ffmpeg_light::filter::DenoiseStrength::Medium,
        })
        .add_audio_filter(AudioFilter::Normalization {
            target_level: -23.0,
        })
        .video_codec("libx264")
        .audio_codec("aac")
        .run()?;
    Ok(())
}
```

### Grab a thumbnail

```rust,no_run
use ffmpeg_light::{thumbnail::ThumbnailOptions, types::Time};

fn main() -> ffmpeg_light::Result<()> {
    let options = ThumbnailOptions::new(Time::from_seconds_f64(12.5));
    ffmpeg_light::generate_thumbnail("input.mp4", "thumb.png", &options)?;
    Ok(())
}
```

## API Overview

### Transcoding & Filters

- `TranscodeBuilder`: Fluent API for configuring transcoding jobs
  - `.video_codec()`, `.audio_codec()`: Set output codecs
  - `.video_bitrate()`, `.audio_bitrate()`: Control quality/file size
  - `.add_video_filter()`, `.add_audio_filter()`: Chain filters
  - `.preset()`: Encoding preset (e.g., "fast", "medium", "slow")
  - `.size()`: Shortcut for `Scale` filter
  - `.run()`: Execute the transcode job

### Video Filters

- `Scale`: Resize video
- `Crop`: Extract a region
- `Trim`: Cut a time range
- `Rotate`: Rotate by degrees
- `Flip`: Mirror horizontally or vertically
- `BrightnessContrast`: Adjust brightness/contrast
- `Denoise`: Reduce noise (Light/Medium/Heavy)
- `Deinterlace`: Convert interlaced to progressive
- `Custom`: Raw FFmpeg filter syntax

### Audio Filters

- `Volume`: Adjust audio level
- `Equalizer`: 3-band EQ (bass, mid, treble)
- `Normalization`: Normalize to target loudness
- `HighPass`, `LowPass`: Frequency filtering
- `Custom`: Raw FFmpeg audio filter syntax

### Media Inspection

- `probe(path)`: Get file duration, codecs, resolution, frame rate, and bit rates
- `ProbeResult`: Video/audio stream metadata

### Thumbnails

- `generate_thumbnail(input, output, options)`: Extract frame at timestamp
- `ThumbnailOptions`: Control timestamp, size, and format

## Error Handling

The crate provides granular error types with recovery suggestions:

```rust,ignore
use ffmpeg_light::Error;

match some_operation() {
    Err(Error::FFmpegNotFound { suggestion }) => {
        eprintln!("FFmpeg not found. Help: {:?}", suggestion);
    }
    Err(Error::InvalidInput(msg)) => {
        eprintln!("Bad parameters: {}", msg);
    }
    Err(Error::FilterError(msg)) => {
        eprintln!("Filter problem: {}", msg);
    }
    Ok(_) => {}
    Err(e) => eprintln!("Error: {}", e),
}
```

## Design notes

- The crate shells out to `ffmpeg`/`ffprobe`. That keeps builds fast, works anywhere binaries exist, and avoids shipping unsafe C bindings.
- Arguments are never handed to a shell; every value goes directly to `std::process::Command`.
- If you need the full FFmpeg surface area, drop down to the CLI; this crate handles the frequent, repetitive tasks.

## Supported Platforms

Tested on Linux, macOS, and Windows. Requires `ffmpeg` and `ffprobe` on `PATH`:

- **macOS**: `brew install ffmpeg`
- **Linux**: `apt install ffmpeg` (or distro equivalent)
- **Windows**: Download from [ffmpeg.org](https://ffmpeg.org/download.html)

Paths on Windows work best without spaces. The builder handles argument quoting automatically.

## v0.2.0 Release Highlights

- ✅ **8 Video Filters**: Scale, Crop, Trim, Rotate, Flip, BrightnessContrast, Denoise, Deinterlace
- ✅ **6 Audio Filters**: Volume, Equalizer, Normalization, HighPass, LowPass, Custom
- ✅ **Granular Error Types**: FFmpegNotFound, ProcessingError, InvalidInput, FilterError, TimeoutError
- ✅ **Builder Accessors**: Inspect configured filters, codecs, and settings
- ✅ **41 Integration Tests**: Comprehensive coverage of filters, error handling, and builder composition
- ✅ **Zero Breaking Changes**: All v0.1.0 APIs remain fully compatible

## Roadmap

- Async command helpers behind a `tokio` feature gate
- Batch processing with parallel transcoding
- Hardware acceleration (NVIDIA NVENC, Intel QuickSync, Apple VideoToolbox)
- Streaming segment output (HLS/DASH)
- Subtitle and chapter handling

## License

Licensed under the [Apache License, Version 2.0](./LICENSE).
