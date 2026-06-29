// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use std::path::PathBuf;

pub const BLAKE3_HEX_LENGTH: usize = 64;
pub const CAPTURE_SEQUENCE_INITIAL: u64 = 0;
pub const CAPTURE_ARTIFACT_SEQUENCE_WIDTH: usize = 6;
pub const MAX_CAPTURE_REQUESTS_PER_FRAME: usize = 1;
pub const MAX_PENDING_CAPTURE_REQUESTS: usize = 1;
pub const MIN_FRAME_INTERVAL_MILLIS: u64 = 1;
pub const MILLIS_PER_SECOND: u64 = 1_000;
pub const RGBA_BYTES_PER_PIXEL: usize = 4;
pub const DEFAULT_MAX_WIDTH_PX: u32 = 7_680;
pub const DEFAULT_MAX_HEIGHT_PX: u32 = 4_320;
pub const DEFAULT_MIN_RECORDING_FPS: u16 = 1;
pub const DEFAULT_MAX_RECORDING_FPS: u16 = 60;
pub const DEFAULT_MAX_RECORDING_FRAMES: u32 = 600;
pub const DEFAULT_MAX_RECORDING_MILLIS: u64 = 10_000;
pub const DEFAULT_MAX_ARTIFACT_BYTES: u64 = 32 * 1024 * 1024;
pub const DEFAULT_INLINE_RESPONSE_BYTES: u64 = 512 * 1024;

pub(super) const FORMAT_PNG: &str = "png";
pub(super) const DEFAULT_SCREENSHOT_ARTIFACT_DIR: &str = "screenshots";
pub(super) const DEFAULT_LATEST_FRAME_ARTIFACT_DIR: &str = "latest-frames";
pub(super) const DEFAULT_RECORDING_ARTIFACT_DIR: &str = "recordings";
pub(super) const DEFAULT_FRAME_ARTIFACT_PREFIX: &str = "frame";
pub(super) const DEFAULT_RECORDING_ARTIFACT_PREFIX: &str = "recording";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureMode {
    Screenshot,
    LatestFrame,
    Recording,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureFormat {
    Png,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedactionState {
    NotReviewed,
    Reviewed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blake3DigestHex(String);

impl Blake3DigestHex {
    pub fn new(value: impl Into<String>) -> Result<Self, CaptureValidationError> {
        let value = value.into();
        if !is_blake3_hex(&value) {
            return Err(CaptureValidationError::InvalidBlake3Digest {
                actual_len: value.len(),
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureOutput {
    Inline,
    Artifact { relative_path: PathBuf },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordingBounds {
    pub frame_rate_hz: u16,
    pub max_frames: Option<u32>,
    pub max_duration_millis: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureRequest {
    pub mode: CaptureMode,
    pub format: CaptureFormat,
    pub output: CaptureOutput,
    pub includes_ui: bool,
    pub recording: Option<RecordingBounds>,
    pub sequence_id: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePolicy {
    pub capture_dir: Option<PathBuf>,
    pub max_width_px: u32,
    pub max_height_px: u32,
    pub min_recording_fps: u16,
    pub max_recording_fps: u16,
    pub max_recording_frames: u32,
    pub max_recording_millis: u64,
    pub max_artifact_bytes: u64,
    pub inline_response_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePlan {
    pub mode: CaptureMode,
    pub format: CaptureFormat,
    pub output: CaptureOutput,
    pub includes_ui: bool,
    pub artifact_path: Option<PathBuf>,
    pub sequence_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureArtifactMetadata {
    pub relative_path: PathBuf,
    pub format: CaptureFormat,
    pub width_px: u32,
    pub height_px: u32,
    pub frame_id: u64,
    pub sequence_id: u64,
    pub byte_len: u64,
    pub blake3_digest: Blake3DigestHex,
    pub includes_ui: bool,
    pub redaction: RedactionState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturedRgbaFrame {
    pub width_px: u32,
    pub height_px: u32,
    pub frame_id: u64,
    pub rgba_top_left: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureReadbackError {
    InvalidDimensions { width_px: u32, height_px: u32 },
    BufferSizeOverflow { width_px: u32, height_px: u32 },
    BufferLengthMismatch { expected: usize, actual: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureFrameContext {
    pub width_px: u32,
    pub height_px: u32,
    pub frame_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureServiceError {
    Validation(CaptureValidationError),
    Readback(CaptureReadbackError),
    Persistence(CapturePersistenceError),
    UiExclusionUnsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureQueueError {
    Validation(CaptureValidationError),
    QueueClosed,
    RateLimitExceeded { pending: usize, max: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapturePersistenceError {
    Validation(CaptureValidationError),
    Readback(CaptureReadbackError),
    Encode(String),
    Io(String),
    MissingArtifactPath,
    ByteLengthOverflow { byte_len: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureValidationError {
    UnsupportedFormat(String),
    MissingCaptureDir,
    MissingSequenceId,
    EmptyArtifactPath,
    ArtifactPathEscapes { relative_path: PathBuf },
    RecordingArtifactOutputRequired,
    RecordingBoundsUnexpected,
    RecordingBoundsRequired,
    RecordingFrameRateOutOfRange { requested: u16, min: u16, max: u16 },
    RecordingDurationRequired,
    RecordingFramesOutOfRange { requested: u32, max: u32 },
    RecordingMillisOutOfRange { requested: u64, max: u64 },
    WidthOutOfRange { requested: u32, max: u32 },
    HeightOutOfRange { requested: u32, max: u32 },
    ArtifactTooLarge { requested: u64, max: u64 },
    InvalidBlake3Digest { actual_len: usize },
}

impl CaptureFormat {
    pub fn from_name(name: &str) -> Result<Self, CaptureValidationError> {
        if name.eq_ignore_ascii_case(FORMAT_PNG) {
            return Ok(Self::Png);
        }
        Err(CaptureValidationError::UnsupportedFormat(name.to_owned()))
    }

    pub fn as_extension(self) -> &'static str {
        match self {
            Self::Png => FORMAT_PNG,
        }
    }
}

impl CapturePolicy {
    pub fn memory() -> Self {
        Self::memory_only()
    }

    pub fn has_capture_dir(&self) -> bool {
        self.capture_dir.is_some()
    }

    pub fn memory_only() -> Self {
        Self {
            capture_dir: None,
            max_width_px: DEFAULT_MAX_WIDTH_PX,
            max_height_px: DEFAULT_MAX_HEIGHT_PX,
            min_recording_fps: DEFAULT_MIN_RECORDING_FPS,
            max_recording_fps: DEFAULT_MAX_RECORDING_FPS,
            max_recording_frames: DEFAULT_MAX_RECORDING_FRAMES,
            max_recording_millis: DEFAULT_MAX_RECORDING_MILLIS,
            max_artifact_bytes: DEFAULT_MAX_ARTIFACT_BYTES,
            inline_response_bytes: DEFAULT_INLINE_RESPONSE_BYTES,
        }
    }

    pub fn local(capture_dir: impl Into<PathBuf>) -> Self {
        Self {
            capture_dir: Some(capture_dir.into()),
            ..Self::memory_only()
        }
    }
}

fn is_blake3_hex(digest: &str) -> bool {
    digest.len() == BLAKE3_HEX_LENGTH && digest.chars().all(|c| c.is_ascii_hexdigit())
}
