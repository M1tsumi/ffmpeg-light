//! Video filter tests. These validate the filter string generation and builder integration.

use ffmpeg_light::filter::{DenoiseStrength, VideoFilter};

#[test]
fn test_scale_filter() {
    let filter = VideoFilter::Scale {
        width: 1920,
        height: 1080,
    };
    assert_eq!(filter.to_filter_string(), "scale=1920:1080");
}

#[test]
fn test_crop_filter() {
    let filter = VideoFilter::Crop {
        width: 1280,
        height: 720,
        x: 0,
        y: 0,
    };
    assert_eq!(filter.to_filter_string(), "crop=1280:720:0:0");
}

#[test]
fn test_rotate_filter() {
    let filter = VideoFilter::Rotate { degrees: 90.0 };
    let result = filter.to_filter_string();
    assert!(result.contains("rotate="));
    assert!(!result.contains("degrees"));
}

#[test]
fn test_flip_horizontal() {
    let filter = VideoFilter::Flip { direction: 'h' };
    assert_eq!(filter.to_filter_string(), "hflip");
}

#[test]
fn test_flip_vertical() {
    let filter = VideoFilter::Flip { direction: 'v' };
    assert_eq!(filter.to_filter_string(), "vflip");
}

#[test]
fn test_brightness_contrast() {
    let filter = VideoFilter::BrightnessContrast {
        brightness: Some(0.2),
        contrast: Some(1.5),
    };
    let result = filter.to_filter_string();
    assert!(result.contains("brightness=0.2"));
    assert!(result.contains("contrast=1.5"));
}

#[test]
fn test_brightness_only() {
    let filter = VideoFilter::BrightnessContrast {
        brightness: Some(0.3),
        contrast: None,
    };
    let result = filter.to_filter_string();
    assert_eq!(result, "eq=brightness=0.3");
}

#[test]
fn test_denoise_light() {
    let filter = VideoFilter::Denoise {
        strength: DenoiseStrength::Light,
    };
    assert_eq!(filter.to_filter_string(), "hqdn3d=1.5:1.5:6:6");
}

#[test]
fn test_denoise_heavy() {
    let filter = VideoFilter::Denoise {
        strength: DenoiseStrength::Heavy,
    };
    assert_eq!(filter.to_filter_string(), "hqdn3d=5:5:6:6");
}

#[test]
fn test_deinterlace() {
    let filter = VideoFilter::Deinterlace;
    assert_eq!(filter.to_filter_string(), "yadif");
}

#[test]
fn test_custom_filter() {
    let custom = "scale=1280:720,fps=30";
    let filter = VideoFilter::Custom(custom.to_string());
    assert_eq!(filter.to_filter_string(), custom);
}

#[test]
fn test_filter_display_trait() {
    let filter = VideoFilter::Scale {
        width: 640,
        height: 480,
    };
    assert_eq!(format!("{}", filter), "scale=640:480");
}
