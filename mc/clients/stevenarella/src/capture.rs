// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

mod model;
mod persistence;
mod queue;
mod readback;
mod recording;
mod service;
mod validation;

pub use model::{
    Blake3DigestHex, CaptureArtifactMetadata, CaptureFormat, CaptureFrameContext, CaptureMode,
    CaptureOutput, CapturePersistenceError, CapturePlan, CapturePolicy, CaptureQueueError,
    CaptureReadbackError, CaptureRequest, CaptureServiceError, CaptureValidationError,
    CapturedRgbaFrame, RecordingBounds, RedactionState, BLAKE3_HEX_LENGTH,
    CAPTURE_ARTIFACT_SEQUENCE_WIDTH, CAPTURE_SEQUENCE_INITIAL, DEFAULT_INLINE_RESPONSE_BYTES,
    DEFAULT_MAX_ARTIFACT_BYTES, DEFAULT_MAX_HEIGHT_PX, DEFAULT_MAX_RECORDING_FPS,
    DEFAULT_MAX_RECORDING_FRAMES, DEFAULT_MAX_RECORDING_MILLIS, DEFAULT_MAX_WIDTH_PX,
    DEFAULT_MIN_RECORDING_FPS, MAX_CAPTURE_REQUESTS_PER_FRAME, MAX_PENDING_CAPTURE_REQUESTS,
    MILLIS_PER_SECOND, MIN_FRAME_INTERVAL_MILLIS, RGBA_BYTES_PER_PIXEL,
};
pub use persistence::{encode_png_frame, persist_captured_frame_artifact};
pub use queue::{capture_request_channel, CaptureRequestReceiver, CaptureRequestSender};
pub use readback::{
    captured_rgba_from_bottom_left, normalize_rgba_bottom_left_to_top_left,
    read_current_framebuffer_for_context, read_current_framebuffer_rgba_top_left, rgba_buffer_len,
};
pub use recording::{
    default_recording_relative_dir, service_recording_frame_with_readback, start_recording,
    RecordingServiceOutcome, RecordingSession,
};
pub use service::{service_one_shot_capture_request_with_readback, ServicedCapture};
pub use validation::{
    default_artifact_relative_path, validate_artifact_metadata, validate_capture_request,
    validate_dimensions,
};

#[cfg(test)]
use recording::{recording_cadence_decision, RecordingCadenceSnapshot, RecordingFrameDecision};

#[cfg(test)]
mod tests;
