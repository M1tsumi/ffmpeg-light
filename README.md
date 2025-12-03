# ffmpeg-light

A small Rust crate that wraps a few common FFmpeg tasks without asking you to memorize the entire CLI.

## Why this exists

When you just want to probe a file, transcode it to H.264, or grab a thumbnail, the full FFmpeg CLI can feel like overkill. On the other hand, pulling in a massive set of raw bindings is equally daunting. `ffmpeg-light` sits in the middle: it spawns the `ffmpeg`/`ffprobe` binaries you already have installed and gives you a tidy Rust API for the 80% use cases.

This crate is **young**. Expect the API to move a bit until we get feedback from real projects.

## Installation

```toml
[dependencies]
ffmpeg-light = "0.1"
```

You’ll also need the `ffmpeg` and `ffprobe` binaries available on your `PATH`. On macOS that might be `brew install ffmpeg`; on Windows, grab the latest build and add it to your environment variables.

## Quick start

### Probe a media file

```rust,no_run
use ffmpeg_light::probe;

fn main() -> ffmpeg_light::Result<()> {
    let info = probe("input.mp4")?;
    if let Some(duration) = info.duration() {
        println!("duration: {:?}", duration);
    }
    Ok(())
}
```

### Transcode to H.264 MP4

```rust,no_run
use ffmpeg_light::transcode::TranscodeBuilder;

fn main() -> ffmpeg_light::Result<()> {
    TranscodeBuilder::new()
        .input("input.avi")
        .output("output.mp4")
        .video_codec("libx264")
        .audio_codec("aac")
        .video_bitrate(2_500)
        .size(1280, 720)
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

## Features

- Media probing built on `ffprobe` JSON output.
- A `TranscodeBuilder` that covers basic codecs, bitrates, presets, filters, and custom args.
- Thumbnail generation with timestamp/size/format controls.
- Simple filter enum so you don’t have to concat raw filter strings.
- Optional logging via the `tracing` feature.
- Optional async command execution via the `tokio` feature (roadmap).

## Design notes

- The crate shells out to `ffmpeg`/`ffprobe`. That keeps builds fast, works anywhere binaries exist, and avoids shipping unsafe bindings.
- We never hand your input to a shell. Every argument is passed directly to `std::process::Command`.
- If you need the full FFmpeg surface area, you can still drop down to the CLI; this crate is for the frequent, boring chores.

## Platform notes

The library is tested on Linux, macOS, and Windows as long as FFmpeg is on `PATH`. Windows users should prefer paths without spaces or wrap them in `PathBuf` to avoid quoting issues (the builder handles the quoting, but FFmpeg can still trip over exotic characters).

## Roadmap

- Async command helpers behind a `tokio` feature gate.
- More preset filters (denoise, deinterlace, etc.).
- Higher-level profiles for “transcode for web” or “extract audio only”.
- Better integration tests once we ship sample fixtures.

## License

Licensed under the [Apache License, Version 2.0](./LICENSE).
