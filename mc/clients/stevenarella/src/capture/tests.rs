use super::*;
use std::path::PathBuf;

const TEST_CAPTURE_DIR: &str = "capture-root";
const TEST_ARTIFACT_PATH: &str = "screens/frame-0001.png";
const TEST_WIDTH_PX: u32 = 1_920;
const TEST_HEIGHT_PX: u32 = 1_080;
const TEST_FRAME_ID: u64 = 42;
const TEST_SEQUENCE_ID: u64 = 7;
const TEST_NEXT_SEQUENCE_ID: u64 = 8;
const TEST_BYTE_LEN: u64 = 4_096;
const TEST_RECORDING_FPS: u16 = 30;
const TEST_RECORDING_FRAMES: u32 = 10;
const TEST_RECORDING_ONE_FRAME: u32 = 1;
const TEST_RECORDING_TWO_FRAMES: u32 = 2;
const TEST_NOW_MILLIS: u64 = 1_000;
const TEST_TOO_EARLY_MILLIS: u64 = 1_010;
const TEST_NEXT_FRAME_MILLIS: u64 = 1_034;
const TEST_AFTER_RECORDING_LIMIT_MILLIS: u64 = 1_068;
const TEST_ARTIFACT_TOO_SMALL_BYTES: u64 = 1;
const TEST_BLAKE3: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const TEST_READBACK_WIDTH_PX: u32 = 2;
const TEST_READBACK_HEIGHT_PX: u32 = 2;
const TEST_OPAQUE_ALPHA: u8 = u8::MAX;
const TEST_BOTTOM_LEFT_PIXEL: [u8; RGBA_BYTES_PER_PIXEL] = [10, 20, 30, TEST_OPAQUE_ALPHA];
const TEST_BOTTOM_RIGHT_PIXEL: [u8; RGBA_BYTES_PER_PIXEL] = [40, 50, 60, TEST_OPAQUE_ALPHA];
const TEST_TOP_LEFT_PIXEL: [u8; RGBA_BYTES_PER_PIXEL] = [70, 80, 90, TEST_OPAQUE_ALPHA];
const TEST_TOP_RIGHT_PIXEL: [u8; RGBA_BYTES_PER_PIXEL] = [100, 110, 120, TEST_OPAQUE_ALPHA];
const TEST_PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

fn inline_capture_request(mode: CaptureMode, includes_ui: bool) -> CaptureRequest {
    CaptureRequest {
        mode,
        format: CaptureFormat::Png,
        output: CaptureOutput::Inline,
        includes_ui,
        recording: None,
        sequence_id: Some(TEST_SEQUENCE_ID),
    }
}

fn synthetic_frame(frame: CaptureFrameContext) -> Result<CapturedRgbaFrame, CaptureReadbackError> {
    let len = rgba_buffer_len(frame.width_px, frame.height_px)?;
    Ok(CapturedRgbaFrame {
        width_px: frame.width_px,
        height_px: frame.height_px,
        frame_id: frame.frame_id,
        rgba_top_left: vec![0; len],
    })
}

fn test_frame() -> CaptureFrameContext {
    CaptureFrameContext {
        width_px: TEST_READBACK_WIDTH_PX,
        height_px: TEST_READBACK_HEIGHT_PX,
        frame_id: TEST_FRAME_ID,
    }
}

fn unique_test_capture_dir(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "stevenarella-capture-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&path);
    path
}

fn artifact_request(relative_path: PathBuf) -> CaptureRequest {
    CaptureRequest {
        mode: CaptureMode::Screenshot,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact { relative_path },
        includes_ui: true,
        recording: None,
        sequence_id: Some(TEST_SEQUENCE_ID),
    }
}

fn bounded_recording_request(relative_path: PathBuf) -> CaptureRequest {
    CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact { relative_path },
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: Some(TEST_RECORDING_TWO_FRAMES),
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    }
}

#[test]
fn valid_screenshot_artifact_request_is_planned() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Screenshot,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact {
            relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        },
        includes_ui: true,
        recording: None,
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let plan = validate_capture_request(&request, &policy).expect("request should pass");

    assert_eq!(plan.mode, CaptureMode::Screenshot);
    assert_eq!(plan.format, CaptureFormat::Png);
    assert_eq!(
        plan.artifact_path,
        Some(PathBuf::from(TEST_CAPTURE_DIR).join(TEST_ARTIFACT_PATH))
    );
}

#[test]
fn valid_recording_bounds_are_accepted() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact {
            relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        },
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: Some(TEST_RECORDING_FRAMES),
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let plan = validate_capture_request(&request, &policy).expect("recording should pass");

    assert_eq!(plan.mode, CaptureMode::Recording);
}

#[test]
fn valid_artifact_metadata_is_accepted() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let metadata = CaptureArtifactMetadata {
        relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        format: CaptureFormat::Png,
        width_px: TEST_WIDTH_PX,
        height_px: TEST_HEIGHT_PX,
        frame_id: TEST_FRAME_ID,
        sequence_id: TEST_SEQUENCE_ID,
        byte_len: TEST_BYTE_LEN,
        blake3_digest: Blake3DigestHex::new(TEST_BLAKE3).expect("digest should pass"),
        includes_ui: true,
        redaction: RedactionState::NotReviewed,
    };

    let path = validate_artifact_metadata(&metadata, &policy).expect("metadata should pass");

    assert_eq!(
        path,
        PathBuf::from(TEST_CAPTURE_DIR).join(TEST_ARTIFACT_PATH)
    );
}

#[test]
fn one_shot_artifact_capture_writes_png_and_metadata() {
    let capture_dir = unique_test_capture_dir("one-shot-artifact");
    let policy = CapturePolicy::local(&capture_dir);
    let relative_path = default_artifact_relative_path(
        CaptureMode::Screenshot,
        TEST_SEQUENCE_ID,
        CaptureFormat::Png,
    );
    let mut readback = synthetic_frame;

    let capture = service_one_shot_capture_request_with_readback(
        artifact_request(relative_path.clone()),
        &policy,
        test_frame(),
        &mut readback,
    )
    .expect("artifact capture should pass");
    let metadata = capture.artifact.expect("artifact metadata should exist");
    let artifact_path = capture_dir.join(&metadata.relative_path);
    let artifact_bytes = std::fs::read(&artifact_path).expect("artifact should be readable");

    assert_eq!(metadata.relative_path, relative_path);
    assert_eq!(metadata.sequence_id, TEST_SEQUENCE_ID);
    assert_eq!(metadata.width_px, TEST_READBACK_WIDTH_PX);
    assert_eq!(metadata.height_px, TEST_READBACK_HEIGHT_PX);
    assert_eq!(metadata.byte_len, artifact_bytes.len() as u64);
    assert_eq!(
        metadata.blake3_digest.as_str(),
        blake3::hash(&artifact_bytes).to_hex().as_str()
    );
    assert!(artifact_bytes.starts_with(TEST_PNG_SIGNATURE));
    let _ = std::fs::remove_dir_all(capture_dir);
}

#[test]
fn one_shot_artifact_size_guard_rejects_before_write() {
    let capture_dir = unique_test_capture_dir("artifact-size-guard");
    let mut policy = CapturePolicy::local(&capture_dir);
    policy.max_artifact_bytes = TEST_ARTIFACT_TOO_SMALL_BYTES;
    let relative_path = default_artifact_relative_path(
        CaptureMode::Screenshot,
        TEST_SEQUENCE_ID,
        CaptureFormat::Png,
    );
    let mut readback = synthetic_frame;

    let err = service_one_shot_capture_request_with_readback(
        artifact_request(relative_path.clone()),
        &policy,
        test_frame(),
        &mut readback,
    )
    .expect_err("oversized artifact should fail");

    match err {
        CaptureServiceError::Persistence(CapturePersistenceError::Validation(
            CaptureValidationError::ArtifactTooLarge { requested, max },
        )) => {
            assert!(requested > max);
            assert_eq!(max, TEST_ARTIFACT_TOO_SMALL_BYTES);
        }
        other => panic!("unexpected error: {other:?}"),
    }
    assert!(!capture_dir.join(relative_path).exists());
    let _ = std::fs::remove_dir_all(capture_dir);
}

#[test]
fn bounded_recording_writes_contained_frames_and_respects_fps() {
    let capture_dir = unique_test_capture_dir("bounded-recording");
    let policy = CapturePolicy::local(&capture_dir);
    let recording_dir = default_recording_relative_dir(TEST_SEQUENCE_ID);
    let mut session = start_recording(
        bounded_recording_request(recording_dir.clone()),
        &policy,
        TEST_NOW_MILLIS,
    )
    .expect("recording should start");
    let mut readback = synthetic_frame;

    let first = service_recording_frame_with_readback(
        &mut session,
        &policy,
        TEST_NOW_MILLIS,
        test_frame(),
        &mut readback,
    )
    .expect("first frame should capture");
    assert!(matches!(first, RecordingServiceOutcome::Captured(_)));

    let wait = service_recording_frame_with_readback(
        &mut session,
        &policy,
        TEST_TOO_EARLY_MILLIS,
        test_frame(),
        &mut readback,
    )
    .expect("early frame should wait");
    assert_eq!(wait, RecordingServiceOutcome::Waiting);

    let second = service_recording_frame_with_readback(
        &mut session,
        &policy,
        TEST_NEXT_FRAME_MILLIS,
        test_frame(),
        &mut readback,
    )
    .expect("second frame should capture");
    let RecordingServiceOutcome::Captured(metadata) = second else {
        panic!("second frame should capture");
    };
    assert_eq!(metadata.sequence_id, TEST_NEXT_SEQUENCE_ID);
    assert!(metadata.relative_path.starts_with(&recording_dir));
    assert!(capture_dir.join(&metadata.relative_path).exists());

    let complete = service_recording_frame_with_readback(
        &mut session,
        &policy,
        TEST_AFTER_RECORDING_LIMIT_MILLIS,
        test_frame(),
        &mut readback,
    )
    .expect("recording should complete");
    assert_eq!(complete, RecordingServiceOutcome::Complete);
    assert_eq!(session.frames_captured(), TEST_RECORDING_TWO_FRAMES);
    assert!(session.is_completed());
    let _ = std::fs::remove_dir_all(capture_dir);
}

#[test]
fn recording_inline_output_is_rejected() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Inline,
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: Some(TEST_RECORDING_FRAMES),
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let err = validate_capture_request(&request, &policy).expect_err("inline rejected");

    assert_eq!(err, CaptureValidationError::RecordingArtifactOutputRequired);
}

#[test]
fn rgba_readback_normalizes_gl_bottom_left_origin() {
    let rgba_bottom_left = [
        TEST_BOTTOM_LEFT_PIXEL,
        TEST_BOTTOM_RIGHT_PIXEL,
        TEST_TOP_LEFT_PIXEL,
        TEST_TOP_RIGHT_PIXEL,
    ]
    .concat();
    let expected_top_left = [
        TEST_TOP_LEFT_PIXEL,
        TEST_TOP_RIGHT_PIXEL,
        TEST_BOTTOM_LEFT_PIXEL,
        TEST_BOTTOM_RIGHT_PIXEL,
    ]
    .concat();

    let frame = captured_rgba_from_bottom_left(
        TEST_READBACK_WIDTH_PX,
        TEST_READBACK_HEIGHT_PX,
        TEST_FRAME_ID,
        &rgba_bottom_left,
    )
    .expect("valid RGBA readback should normalize");

    assert_eq!(frame.width_px, TEST_READBACK_WIDTH_PX);
    assert_eq!(frame.height_px, TEST_READBACK_HEIGHT_PX);
    assert_eq!(frame.frame_id, TEST_FRAME_ID);
    assert_eq!(frame.rgba_top_left, expected_top_left);
}

#[test]
fn rgba_readback_rejects_wrong_buffer_length() {
    let expected = rgba_buffer_len(TEST_READBACK_WIDTH_PX, TEST_READBACK_HEIGHT_PX)
        .expect("fixture dimensions should pass");
    let actual = expected - RGBA_BYTES_PER_PIXEL;
    let short_buffer = vec![0; actual];

    let err = normalize_rgba_bottom_left_to_top_left(
        TEST_READBACK_WIDTH_PX,
        TEST_READBACK_HEIGHT_PX,
        &short_buffer,
    )
    .expect_err("short buffer rejected");

    assert_eq!(
        err,
        CaptureReadbackError::BufferLengthMismatch { expected, actual }
    );
}

#[test]
fn rgba_readback_rejects_empty_dimensions() {
    let err = rgba_buffer_len(TEST_READBACK_WIDTH_PX, 0).expect_err("height rejected");

    assert_eq!(
        err,
        CaptureReadbackError::InvalidDimensions {
            width_px: TEST_READBACK_WIDTH_PX,
            height_px: 0,
        }
    );
}

#[test]
fn unsupported_format_fails_closed() {
    let err = CaptureFormat::from_name("webp").expect_err("webp not supported yet");

    assert_eq!(
        err,
        CaptureValidationError::UnsupportedFormat("webp".to_owned())
    );
}

#[test]
fn artifact_path_escape_is_rejected() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Screenshot,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact {
            relative_path: PathBuf::from("../outside.png"),
        },
        includes_ui: true,
        recording: None,
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let err = validate_capture_request(&request, &policy).expect_err("escape rejected");

    assert_eq!(
        err,
        CaptureValidationError::ArtifactPathEscapes {
            relative_path: PathBuf::from("../outside.png"),
        }
    );
}

#[test]
fn recording_without_explicit_bounds_is_rejected() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact {
            relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        },
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: None,
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let err = validate_capture_request(&request, &policy).expect_err("unbounded rejected");

    assert_eq!(err, CaptureValidationError::RecordingDurationRequired);
}

#[test]
fn invalid_digest_is_rejected() {
    let err = Blake3DigestHex::new("not-a-blake3-digest").expect_err("digest rejected");

    assert_eq!(
        err,
        CaptureValidationError::InvalidBlake3Digest {
            actual_len: "not-a-blake3-digest".len(),
        }
    );
}

#[test]
fn zero_dimensions_are_rejected() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);

    let err = validate_dimensions(0, TEST_HEIGHT_PX, &policy).expect_err("width rejected");

    assert_eq!(
        err,
        CaptureValidationError::WidthOutOfRange {
            requested: 0,
            max: DEFAULT_MAX_WIDTH_PX,
        }
    );
}

#[test]
fn capture_queue_services_one_screenshot_after_frame_readback() {
    let (sender, receiver) = capture_request_channel();
    let response = sender
        .enqueue_deferred(inline_capture_request(CaptureMode::Screenshot, true))
        .expect("capture request queued");
    let policy = CapturePolicy::memory();

    let serviced =
        receiver.service_pending_one_shot_with_readback(&policy, test_frame(), synthetic_frame);
    let capture = response
        .recv()
        .expect("capture response sent")
        .expect("capture serviced");

    assert_eq!(serviced, 1);
    assert_eq!(capture.plan.mode, CaptureMode::Screenshot);
    assert_eq!(capture.frame.frame_id, TEST_FRAME_ID);
    assert_eq!(capture.frame.width_px, TEST_READBACK_WIDTH_PX);
    assert_eq!(capture.frame.height_px, TEST_READBACK_HEIGHT_PX);
}

#[test]
fn capture_queue_services_one_latest_frame_after_frame_readback() {
    let (sender, receiver) = capture_request_channel();
    let response = sender
        .enqueue_deferred(inline_capture_request(CaptureMode::LatestFrame, true))
        .expect("latest-frame request queued");
    let policy = CapturePolicy::memory();

    let serviced =
        receiver.service_pending_one_shot_with_readback(&policy, test_frame(), synthetic_frame);
    let capture = response
        .recv()
        .expect("capture response sent")
        .expect("capture serviced");

    assert_eq!(serviced, 1);
    assert_eq!(capture.plan.mode, CaptureMode::LatestFrame);
    assert_eq!(capture.frame.frame_id, TEST_FRAME_ID);
}

#[test]
fn one_shot_capture_rejects_ui_exclusion_before_readback() {
    let mut readback_called = false;
    let mut readback = |frame| {
        readback_called = true;
        synthetic_frame(frame)
    };
    let policy = CapturePolicy::memory();

    let err = service_one_shot_capture_request_with_readback(
        inline_capture_request(CaptureMode::Screenshot, false),
        &policy,
        test_frame(),
        &mut readback,
    )
    .expect_err("ui exclusion is rejected before readback");

    assert_eq!(err, CaptureServiceError::UiExclusionUnsupported);
    assert!(!readback_called);
}

#[test]
fn capture_queue_rejects_recording_request_before_enqueue() {
    let (sender, _receiver) = capture_request_channel();
    let request = CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Inline,
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: Some(TEST_RECORDING_FRAMES),
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let err = match sender.enqueue_deferred(request) {
        Ok(_) => panic!("recording must not enter one-shot queue"),
        Err(err) => err,
    };

    assert_eq!(
        err,
        CaptureQueueError::Validation(CaptureValidationError::RecordingBoundsUnexpected)
    );
}

#[test]
fn focused_validation_covers_valid_screenshot_metadata() {
    let capture_dir = unique_test_capture_dir("focused-metadata");
    let policy = CapturePolicy::local(&capture_dir);
    let relative_path = default_artifact_relative_path(
        CaptureMode::Screenshot,
        TEST_SEQUENCE_ID,
        CaptureFormat::Png,
    );
    let mut readback = synthetic_frame;

    let capture = service_one_shot_capture_request_with_readback(
        artifact_request(relative_path.clone()),
        &policy,
        test_frame(),
        &mut readback,
    )
    .expect("screenshot metadata should pass");
    let metadata = capture.artifact.expect("metadata should exist");

    assert_eq!(metadata.relative_path, relative_path);
    assert_eq!(metadata.format, CaptureFormat::Png);
    assert_eq!(metadata.width_px, TEST_READBACK_WIDTH_PX);
    assert_eq!(metadata.height_px, TEST_READBACK_HEIGHT_PX);
    assert_eq!(metadata.frame_id, TEST_FRAME_ID);
    assert_eq!(metadata.sequence_id, TEST_SEQUENCE_ID);
    assert!(metadata.byte_len > 0);
    assert_eq!(metadata.blake3_digest.as_str().len(), BLAKE3_HEX_LENGTH);
    assert!(metadata.includes_ui);
    assert_eq!(metadata.redaction, RedactionState::NotReviewed);
    let _ = std::fs::remove_dir_all(capture_dir);
}

#[test]
fn focused_validation_covers_vertical_flip_normalization() {
    let rgba_bottom_left = [
        TEST_BOTTOM_LEFT_PIXEL,
        TEST_BOTTOM_RIGHT_PIXEL,
        TEST_TOP_LEFT_PIXEL,
        TEST_TOP_RIGHT_PIXEL,
    ]
    .concat();
    let expected_top_left = [
        TEST_TOP_LEFT_PIXEL,
        TEST_TOP_RIGHT_PIXEL,
        TEST_BOTTOM_LEFT_PIXEL,
        TEST_BOTTOM_RIGHT_PIXEL,
    ]
    .concat();

    let frame = captured_rgba_from_bottom_left(
        TEST_READBACK_WIDTH_PX,
        TEST_READBACK_HEIGHT_PX,
        TEST_FRAME_ID,
        &rgba_bottom_left,
    )
    .expect("focused vertical flip should pass");

    assert_eq!(frame.rgba_top_left, expected_top_left);
}

#[test]
fn focused_validation_rejects_invalid_format() {
    let err = CaptureFormat::from_name("webp").expect_err("invalid format rejected");

    assert_eq!(
        err,
        CaptureValidationError::UnsupportedFormat("webp".to_owned())
    );
}

#[test]
fn focused_validation_rejects_path_traversal() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = artifact_request(PathBuf::from("../escape.png"));

    let err = validate_capture_request(&request, &policy).expect_err("escape rejected");

    assert_eq!(
        err,
        CaptureValidationError::ArtifactPathEscapes {
            relative_path: PathBuf::from("../escape.png"),
        }
    );
}

#[test]
fn focused_validation_rejects_capture_rate_limit() {
    let (sender, _receiver) = capture_request_channel();
    let first = sender.enqueue_deferred(inline_capture_request(CaptureMode::Screenshot, true));
    let second = sender.enqueue_deferred(inline_capture_request(CaptureMode::LatestFrame, true));

    assert!(first.is_ok());
    assert_eq!(
        second.expect_err("second pending capture should be rate-limited"),
        CaptureQueueError::RateLimitExceeded {
            pending: MAX_PENDING_CAPTURE_REQUESTS,
            max: MAX_PENDING_CAPTURE_REQUESTS,
        }
    );
}

#[test]
fn capture_queue_reports_closed_after_pending_receiver_drop() {
    let (sender, receiver) = capture_request_channel();
    let first = sender.enqueue_deferred(inline_capture_request(CaptureMode::Screenshot, true));
    drop(receiver);

    let second = sender.enqueue_deferred(inline_capture_request(CaptureMode::LatestFrame, true));

    assert!(first.is_ok());
    assert_eq!(
        second.expect_err("closed receiver should beat pending rate limit"),
        CaptureQueueError::QueueClosed
    );
}

#[test]
fn focused_validation_rejects_unbounded_recording() {
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    let request = CaptureRequest {
        mode: CaptureMode::Recording,
        format: CaptureFormat::Png,
        output: CaptureOutput::Artifact {
            relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        },
        includes_ui: true,
        recording: Some(RecordingBounds {
            frame_rate_hz: TEST_RECORDING_FPS,
            max_frames: None,
            max_duration_millis: None,
        }),
        sequence_id: Some(TEST_SEQUENCE_ID),
    };

    let err = validate_capture_request(&request, &policy).expect_err("unbounded rejected");

    assert_eq!(err, CaptureValidationError::RecordingDurationRequired);
}

#[test]
fn latest_frame_default_artifact_path_uses_stable_directory_and_width() {
    let path = default_artifact_relative_path(
        CaptureMode::LatestFrame,
        TEST_SEQUENCE_ID,
        CaptureFormat::Png,
    );

    assert_eq!(
        path,
        PathBuf::from("latest-frames").join("frame-000007.png")
    );
}

#[test]
fn recording_cadence_decision_is_pure_over_session_snapshot() {
    let bounds = RecordingBounds {
        frame_rate_hz: TEST_RECORDING_FPS,
        max_frames: Some(TEST_RECORDING_FRAMES),
        max_duration_millis: Some(TEST_AFTER_RECORDING_LIMIT_MILLIS),
    };
    let fresh_snapshot = RecordingCadenceSnapshot {
        bounds,
        started_at_millis: TEST_NOW_MILLIS,
        last_capture_at_millis: None,
        frames_captured: 0,
        completed: false,
    };
    let just_captured_snapshot = RecordingCadenceSnapshot {
        last_capture_at_millis: Some(TEST_NOW_MILLIS),
        frames_captured: TEST_RECORDING_ONE_FRAME,
        ..fresh_snapshot
    };
    let exhausted_snapshot = RecordingCadenceSnapshot {
        frames_captured: TEST_RECORDING_FRAMES,
        ..fresh_snapshot
    };

    assert_eq!(
        recording_cadence_decision(fresh_snapshot, TEST_NOW_MILLIS),
        RecordingFrameDecision::Capture
    );
    assert_eq!(
        recording_cadence_decision(just_captured_snapshot, TEST_TOO_EARLY_MILLIS),
        RecordingFrameDecision::Waiting
    );
    assert_eq!(
        recording_cadence_decision(just_captured_snapshot, TEST_NEXT_FRAME_MILLIS),
        RecordingFrameDecision::Capture
    );
    assert_eq!(
        recording_cadence_decision(exhausted_snapshot, TEST_NEXT_FRAME_MILLIS),
        RecordingFrameDecision::Complete
    );
}

#[test]
fn metadata_validation_rejects_oversized_artifact() {
    let mut policy = CapturePolicy::local(TEST_CAPTURE_DIR);
    policy.max_artifact_bytes = TEST_ARTIFACT_TOO_SMALL_BYTES;
    let metadata = CaptureArtifactMetadata {
        relative_path: PathBuf::from(TEST_ARTIFACT_PATH),
        format: CaptureFormat::Png,
        width_px: TEST_WIDTH_PX,
        height_px: TEST_HEIGHT_PX,
        frame_id: TEST_FRAME_ID,
        sequence_id: TEST_SEQUENCE_ID,
        byte_len: TEST_BYTE_LEN,
        blake3_digest: Blake3DigestHex::new(TEST_BLAKE3).expect("digest is valid"),
        includes_ui: true,
        redaction: RedactionState::NotReviewed,
    };

    let err = validate_artifact_metadata(&metadata, &policy).expect_err("oversize rejected");

    assert_eq!(
        err,
        CaptureValidationError::ArtifactTooLarge {
            requested: TEST_BYTE_LEN,
            max: TEST_ARTIFACT_TOO_SMALL_BYTES,
        }
    );
}

#[test]
fn recording_frame_rate_above_policy_is_rejected() {
    let mut request = bounded_recording_request(PathBuf::from(TEST_ARTIFACT_PATH));
    request.recording = Some(RecordingBounds {
        frame_rate_hz: DEFAULT_MAX_RECORDING_FPS + 1,
        max_frames: Some(TEST_RECORDING_FRAMES),
        max_duration_millis: None,
    });
    let policy = CapturePolicy::local(TEST_CAPTURE_DIR);

    let err = validate_capture_request(&request, &policy).expect_err("fps rejected");

    assert_eq!(
        err,
        CaptureValidationError::RecordingFrameRateOutOfRange {
            requested: DEFAULT_MAX_RECORDING_FPS + 1,
            min: DEFAULT_MIN_RECORDING_FPS,
            max: DEFAULT_MAX_RECORDING_FPS,
        }
    );
}
