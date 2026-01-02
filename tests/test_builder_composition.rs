//! Builder pattern and filter composition tests.

use ffmpeg_light::{AudioFilter, TranscodeBuilder, VideoFilter};

#[test]
fn test_builder_with_video_filter() {
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .add_video_filter(VideoFilter::Scale {
            width: 1280,
            height: 720,
        });

    assert!(builder.input_path().is_some());
}

#[test]
fn test_builder_with_audio_filter() {
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .add_audio_filter(AudioFilter::Volume(1.5));

    assert!(builder.input_path().is_some());
}

#[test]
fn test_builder_with_multiple_video_filters() {
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .add_video_filter(VideoFilter::Scale {
            width: 1280,
            height: 720,
        })
        .add_video_filter(VideoFilter::Denoise {
            strength: ffmpeg_light::filter::DenoiseStrength::Light,
        });

    assert_eq!(builder.video_filters().len(), 2);
}

#[test]
fn test_builder_with_multiple_audio_filters() {
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .add_audio_filter(AudioFilter::Normalization {
            target_level: -23.0,
        })
        .add_audio_filter(AudioFilter::LowPass {
            frequency: 12000.0,
        });

    assert_eq!(builder.audio_filters().len(), 2);
}

#[test]
fn test_builder_mixed_filters() {
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .size(1920, 1080)
        .add_video_filter(VideoFilter::Flip { direction: 'h' })
        .add_audio_filter(AudioFilter::Volume(1.2))
        .video_codec("libx264")
        .audio_codec("aac");

    assert_eq!(builder.video_filters().len(), 2);
    assert_eq!(builder.audio_filters().len(), 1);
    assert!(builder.video_codec_ref().is_some());
    assert!(builder.audio_codec_ref().is_some());
}

#[test]
fn test_builder_backward_compat_add_filter() {
    #[allow(deprecated)]
    let builder = TranscodeBuilder::new()
        .input("input.mp4")
        .output("output.mp4")
        .add_filter(VideoFilter::Scale {
            width: 1280,
            height: 720,
        });

    assert_eq!(builder.video_filters().len(), 1);
}

#[test]
fn test_builder_chaining() {
    let builder = TranscodeBuilder::new()
        .input("input.avi")
        .output("output.mp4")
        .video_codec("libx264")
        .video_bitrate(2500)
        .audio_codec("aac")
        .audio_bitrate(128)
        .frame_rate(30.0)
        .preset("medium")
        .size(1280, 720);

    assert_eq!(builder.input_path().and_then(|p| p.to_str()), Some("input.avi"));
    assert_eq!(builder.output_path().and_then(|p| p.to_str()), Some("output.mp4"));
    assert_eq!(builder.video_codec_ref(), Some("libx264"));
    assert_eq!(builder.video_bitrate_value(), Some(2500));
    assert_eq!(builder.audio_codec_ref(), Some("aac"));
    assert_eq!(builder.frame_rate_value(), Some(30.0));
    assert_eq!(builder.preset_value(), Some("medium"));
}

#[test]
fn test_builder_overwrite_flag() {
    let builder1 = TranscodeBuilder::new().overwrite(true);
    assert!(builder1.overwrite_enabled());

    let builder2 = TranscodeBuilder::new().overwrite(false);
    assert!(!builder2.overwrite_enabled());
}

#[test]
fn test_crop_and_denoise_composition() {
    let crop = VideoFilter::Crop {
        width: 1920,
        height: 1080,
        x: 0,
        y: 0,
    };
    let denoise = VideoFilter::Denoise {
        strength: ffmpeg_light::filter::DenoiseStrength::Medium,
    };

    let builder = TranscodeBuilder::new()
        .input("raw_video.mov")
        .output("processed.mp4")
        .add_video_filter(crop)
        .add_video_filter(denoise);

    assert_eq!(builder.video_filters().len(), 2);
    match &builder.video_filters()[0] {
        VideoFilter::Crop { .. } => (),
        _ => panic!("First filter should be crop"),
    }
    match &builder.video_filters()[1] {
        VideoFilter::Denoise { .. } => (),
        _ => panic!("Second filter should be denoise"),
    }
}
