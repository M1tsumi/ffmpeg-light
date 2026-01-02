//! Error handling tests. Validates granular error types and recovery suggestions.

use ffmpeg_light::{Error, Result};

#[test]
fn test_ffmpeg_not_found_error_has_suggestion() {
    let err = Error::FFmpegNotFound {
        suggestion: Some("install ffmpeg with brew".to_string()),
    };
    assert!(err.suggestion().is_some());
    assert_eq!(
        err.suggestion().unwrap(),
        "install ffmpeg with brew".to_string()
    );
}

#[test]
fn test_invalid_input_error_format() {
    let err = Error::InvalidInput("codec is invalid".to_string());
    let msg = err.to_string();
    assert!(msg.contains("invalid input"));
}

#[test]
fn test_processing_error_format() {
    let err = Error::ProcessingError {
        binary: "ffmpeg".to_string(),
        exit_code: Some(1),
        message: "file not found".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("ffmpeg"));
    assert!(msg.contains("file not found"));
}

#[test]
fn test_invalid_input_suggestion_for_input_path() {
    let err = Error::InvalidInput("input path is required".to_string());
    let sugg = err.suggestion();
    assert!(sugg.is_some());
    assert!(sugg.unwrap().contains("input file"));
}

#[test]
fn test_invalid_input_suggestion_for_output_path() {
    let err = Error::InvalidInput("output path is required".to_string());
    let sugg = err.suggestion();
    assert!(sugg.is_some());
    assert!(sugg.unwrap().contains("output"));
}

#[test]
fn test_filter_error_unsupported() {
    let err = Error::FilterError("filter not supported by FFmpeg version".to_string());
    let sugg = err.suggestion();
    assert!(sugg.is_some());
    assert!(sugg.unwrap().contains("FFmpeg version"));
}

#[test]
fn test_timeout_error() {
    let err = Error::TimeoutError("transcode exceeded 5 minutes".to_string());
    let msg = err.to_string();
    assert!(msg.contains("timeout"));
}

#[test]
fn test_processing_error_suggestion() {
    let err = Error::ProcessingError {
        binary: "ffmpeg".to_string(),
        exit_code: Some(2),
        message: "unknown codec".to_string(),
    };
    let sugg = err.suggestion();
    assert!(sugg.is_some());
    let suggestion = sugg.unwrap();
    assert!(suggestion.contains("installed") || suggestion.contains("valid"));
}

#[test]
fn test_result_type_ok() {
    let result: Result<u32> = Ok(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_result_type_err() {
    let result: Result<u32> = Err(Error::InvalidInput("test error".to_string()));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("test error"));
}
