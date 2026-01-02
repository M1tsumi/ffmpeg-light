//! Audio filter tests. Tests the audio filter enum and FFmpeg filter string generation.

use ffmpeg_light::AudioFilter;

#[test]
fn test_volume_filter() {
    let filter = AudioFilter::Volume(1.5);
    assert_eq!(filter.to_filter_string(), "volume=1.5");
}

#[test]
fn test_volume_half() {
    let filter = AudioFilter::Volume(0.5);
    assert_eq!(filter.to_filter_string(), "volume=0.5");
}

#[test]
fn test_equalizer_all_bands() {
    let filter = AudioFilter::Equalizer {
        bass: Some(2.0),
        mid: Some(0.5),
        treble: Some(-1.0),
    };
    let result = filter.to_filter_string();
    assert!(result.contains("b=2"));
    assert!(result.contains("m=0.5"));
    assert!(result.contains("t=-1"));
}

#[test]
fn test_equalizer_bass_only() {
    let filter = AudioFilter::Equalizer {
        bass: Some(3.0),
        mid: None,
        treble: None,
    };
    let result = filter.to_filter_string();
    assert_eq!(result, "superequalizer=b=3");
}

#[test]
fn test_normalization() {
    let filter = AudioFilter::Normalization {
        target_level: -23.0,
    };
    let result = filter.to_filter_string();
    assert!(result.contains("loudnorm=I=-23"));
}

#[test]
fn test_highpass_filter() {
    let filter = AudioFilter::HighPass { frequency: 80.0 };
    assert_eq!(filter.to_filter_string(), "highpass=f=80");
}

#[test]
fn test_lowpass_filter() {
    let filter = AudioFilter::LowPass {
        frequency: 8000.0,
    };
    assert_eq!(filter.to_filter_string(), "lowpass=f=8000");
}

#[test]
fn test_custom_audio_filter() {
    let custom = "adelay=10000";
    let filter = AudioFilter::Custom(custom.to_string());
    assert_eq!(filter.to_filter_string(), custom);
}

#[test]
fn test_audio_filter_display() {
    let filter = AudioFilter::Volume(2.0);
    assert_eq!(format!("{}", filter), "volume=2");
}

#[test]
fn test_multiple_audio_filters_compatibility() {
    let filters = vec![
        AudioFilter::Normalization {
            target_level: -23.0,
        },
        AudioFilter::LowPass {
            frequency: 12000.0,
        },
    ];

    let filter_strings: Vec<String> = filters.iter().map(|f| f.to_filter_string()).collect();
    let chained = filter_strings.join(",");

    assert!(chained.contains("loudnorm=I=-23"));
    assert!(chained.contains("lowpass=f=12000"));
}
