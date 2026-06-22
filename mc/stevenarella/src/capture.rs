// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::gl;
use std::fs;
use std::io::Cursor;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};

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

const FORMAT_PNG: &str = "png";
const DEFAULT_SCREENSHOT_ARTIFACT_DIR: &str = "screenshots";
const DEFAULT_LATEST_FRAME_ARTIFACT_DIR: &str = "latest-frames";
const DEFAULT_RECORDING_ARTIFACT_DIR: &str = "recordings";
const DEFAULT_FRAME_ARTIFACT_PREFIX: &str = "frame";
const DEFAULT_RECORDING_ARTIFACT_PREFIX: &str = "recording";
const FRAMEBUFFER_READ_ORIGIN_X: i32 = 0;
const FRAMEBUFFER_READ_ORIGIN_Y: i32 = 0;

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
pub struct ServicedCapture {
    pub plan: CapturePlan,
    pub frame: CapturedRgbaFrame,
    pub artifact: Option<CaptureArtifactMetadata>,
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
pub struct RecordingSession {
    plan: CapturePlan,
    bounds: RecordingBounds,
    started_at_millis: u64,
    last_capture_at_millis: Option<u64>,
    frames_captured: u32,
    next_sequence_id: u64,
    completed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordingServiceOutcome {
    Captured(CaptureArtifactMetadata),
    Waiting,
    Complete,
}

#[derive(Clone)]
pub struct CaptureRequestSender {
    sender: mpsc::Sender<QueuedCaptureRequest>,
    pending_requests: Arc<AtomicUsize>,
    receiver_open: Arc<AtomicBool>,
}

pub struct CaptureRequestReceiver {
    receiver: mpsc::Receiver<QueuedCaptureRequest>,
    pending_requests: Arc<AtomicUsize>,
    receiver_open: Arc<AtomicBool>,
}

struct QueuedCaptureRequest {
    request: CaptureRequest,
    response_sender: mpsc::Sender<Result<ServicedCapture, CaptureServiceError>>,
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

pub fn default_artifact_relative_path(
    mode: CaptureMode,
    sequence_id: u64,
    format: CaptureFormat,
) -> PathBuf {
    let artifact_dir = match mode {
        CaptureMode::Screenshot => DEFAULT_SCREENSHOT_ARTIFACT_DIR,
        CaptureMode::LatestFrame => DEFAULT_LATEST_FRAME_ARTIFACT_DIR,
        CaptureMode::Recording => DEFAULT_RECORDING_ARTIFACT_DIR,
    };
    PathBuf::from(artifact_dir).join(format!(
        "{DEFAULT_FRAME_ARTIFACT_PREFIX}-{sequence_id:0width$}.{extension}",
        width = CAPTURE_ARTIFACT_SEQUENCE_WIDTH,
        extension = format.as_extension()
    ))
}

pub fn default_recording_relative_dir(sequence_id: u64) -> PathBuf {
    PathBuf::from(DEFAULT_RECORDING_ARTIFACT_DIR).join(format!(
        "{DEFAULT_RECORDING_ARTIFACT_PREFIX}-{sequence_id:0width$}",
        width = CAPTURE_ARTIFACT_SEQUENCE_WIDTH
    ))
}

pub fn validate_capture_request(
    request: &CaptureRequest,
    policy: &CapturePolicy,
) -> Result<CapturePlan, CaptureValidationError> {
    let sequence_id = request
        .sequence_id
        .ok_or(CaptureValidationError::MissingSequenceId)?;
    if request.mode == CaptureMode::Recording && matches!(&request.output, CaptureOutput::Inline) {
        return Err(CaptureValidationError::RecordingArtifactOutputRequired);
    }

    let artifact_path = match &request.output {
        CaptureOutput::Inline => None,
        CaptureOutput::Artifact { relative_path } => {
            Some(contained_artifact_path(policy, relative_path)?)
        }
    };

    match (request.mode, request.recording) {
        (CaptureMode::Recording, Some(recording)) => validate_recording_bounds(recording, policy)?,
        (CaptureMode::Recording, None) => {
            return Err(CaptureValidationError::RecordingBoundsRequired)
        }
        (_, Some(_)) => return Err(CaptureValidationError::RecordingBoundsUnexpected),
        (_, None) => {}
    }

    Ok(CapturePlan {
        mode: request.mode,
        format: request.format,
        output: request.output.clone(),
        includes_ui: request.includes_ui,
        artifact_path,
        sequence_id,
    })
}

pub fn capture_request_channel() -> (CaptureRequestSender, CaptureRequestReceiver) {
    let (sender, receiver) = mpsc::channel();
    let pending_requests = Arc::new(AtomicUsize::new(0));
    let receiver_open = Arc::new(AtomicBool::new(true));
    (
        CaptureRequestSender {
            sender,
            pending_requests: Arc::clone(&pending_requests),
            receiver_open: Arc::clone(&receiver_open),
        },
        CaptureRequestReceiver {
            receiver,
            pending_requests,
            receiver_open,
        },
    )
}

impl CaptureRequestSender {
    pub fn enqueue_deferred(
        &self,
        request: CaptureRequest,
    ) -> Result<mpsc::Receiver<Result<ServicedCapture, CaptureServiceError>>, CaptureQueueError>
    {
        validate_one_shot_capture_request_shape(&request).map_err(CaptureQueueError::Validation)?;
        ensure_capture_receiver_open(&self.receiver_open)?;
        reserve_pending_capture_slot(&self.pending_requests)?;
        let (response_sender, response_receiver) = mpsc::channel();
        let send_result = self.sender.send(QueuedCaptureRequest {
            request,
            response_sender,
        });
        if send_result.is_err() {
            release_pending_capture_slot(&self.pending_requests);
            return Err(CaptureQueueError::QueueClosed);
        }
        Ok(response_receiver)
    }
}

fn ensure_capture_receiver_open(receiver_open: &AtomicBool) -> Result<(), CaptureQueueError> {
    if receiver_open.load(Ordering::Acquire) {
        return Ok(());
    }
    Err(CaptureQueueError::QueueClosed)
}

fn reserve_pending_capture_slot(pending_requests: &AtomicUsize) -> Result<(), CaptureQueueError> {
    pending_requests
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |pending| {
            (pending < MAX_PENDING_CAPTURE_REQUESTS).then_some(pending + 1)
        })
        .map(|_| ())
        .map_err(|pending| CaptureQueueError::RateLimitExceeded {
            pending,
            max: MAX_PENDING_CAPTURE_REQUESTS,
        })
}

fn release_pending_capture_slot(pending_requests: &AtomicUsize) {
    let _ = pending_requests.fetch_update(Ordering::AcqRel, Ordering::Acquire, |pending| {
        pending.checked_sub(1)
    });
}

impl Drop for CaptureRequestReceiver {
    fn drop(&mut self) {
        self.receiver_open.store(false, Ordering::Release);
    }
}

impl CaptureRequestReceiver {
    pub fn service_pending_one_shot_with_readback<F>(
        &self,
        policy: &CapturePolicy,
        frame: CaptureFrameContext,
        mut readback: F,
    ) -> usize
    where
        F: FnMut(CaptureFrameContext) -> Result<CapturedRgbaFrame, CaptureReadbackError>,
    {
        let mut serviced = 0;
        while serviced < MAX_CAPTURE_REQUESTS_PER_FRAME {
            let queued = match self.receiver.try_recv() {
                Ok(queued) => {
                    release_pending_capture_slot(&self.pending_requests);
                    queued
                }
                Err(mpsc::TryRecvError::Empty) | Err(mpsc::TryRecvError::Disconnected) => break,
            };
            let response = service_one_shot_capture_request_with_readback(
                queued.request,
                policy,
                frame,
                &mut readback,
            );
            let _ = queued.response_sender.send(response);
            serviced += 1;
        }
        serviced
    }
}

pub fn service_one_shot_capture_request_with_readback<F>(
    request: CaptureRequest,
    policy: &CapturePolicy,
    frame: CaptureFrameContext,
    readback: &mut F,
) -> Result<ServicedCapture, CaptureServiceError>
where
    F: FnMut(CaptureFrameContext) -> Result<CapturedRgbaFrame, CaptureReadbackError>,
{
    validate_one_shot_capture_request_shape(&request).map_err(CaptureServiceError::Validation)?;
    let plan =
        validate_capture_request(&request, policy).map_err(CaptureServiceError::Validation)?;
    validate_dimensions(frame.width_px, frame.height_px, policy)
        .map_err(CaptureServiceError::Validation)?;
    if !plan.includes_ui {
        return Err(CaptureServiceError::UiExclusionUnsupported);
    }
    let captured_frame = readback(frame).map_err(CaptureServiceError::Readback)?;
    let artifact = persist_captured_frame_artifact(&plan, &captured_frame, policy)
        .map_err(CaptureServiceError::Persistence)?;
    Ok(ServicedCapture {
        plan,
        frame: captured_frame,
        artifact,
    })
}

fn validate_one_shot_capture_request_shape(
    request: &CaptureRequest,
) -> Result<(), CaptureValidationError> {
    match request.mode {
        CaptureMode::Screenshot | CaptureMode::LatestFrame => Ok(()),
        CaptureMode::Recording => Err(CaptureValidationError::RecordingBoundsUnexpected),
    }
}

pub fn persist_captured_frame_artifact(
    plan: &CapturePlan,
    frame: &CapturedRgbaFrame,
    policy: &CapturePolicy,
) -> Result<Option<CaptureArtifactMetadata>, CapturePersistenceError> {
    let CaptureOutput::Artifact { relative_path } = &plan.output else {
        return Ok(None);
    };
    let artifact_path = plan
        .artifact_path
        .as_ref()
        .ok_or(CapturePersistenceError::MissingArtifactPath)?;
    validate_dimensions(frame.width_px, frame.height_px, policy)
        .map_err(CapturePersistenceError::Validation)?;

    let png_bytes = encode_png_frame(frame)?;
    let byte_len = u64::try_from(png_bytes.len()).map_err(|_| {
        CapturePersistenceError::ByteLengthOverflow {
            byte_len: png_bytes.len(),
        }
    })?;
    validate_artifact_size(byte_len, policy).map_err(CapturePersistenceError::Validation)?;

    if let Some(parent) = artifact_path.parent() {
        fs::create_dir_all(parent).map_err(|err| CapturePersistenceError::Io(err.to_string()))?;
    }
    fs::write(artifact_path, &png_bytes)
        .map_err(|err| CapturePersistenceError::Io(err.to_string()))?;

    let digest = Blake3DigestHex::new(blake3::hash(&png_bytes).to_hex().to_string())
        .map_err(CapturePersistenceError::Validation)?;
    let metadata = CaptureArtifactMetadata {
        relative_path: relative_path.clone(),
        format: plan.format,
        width_px: frame.width_px,
        height_px: frame.height_px,
        frame_id: frame.frame_id,
        sequence_id: plan.sequence_id,
        byte_len,
        blake3_digest: digest,
        includes_ui: plan.includes_ui,
        redaction: RedactionState::NotReviewed,
    };
    validate_artifact_metadata(&metadata, policy).map_err(CapturePersistenceError::Validation)?;
    Ok(Some(metadata))
}

pub fn start_recording(
    request: CaptureRequest,
    policy: &CapturePolicy,
    started_at_millis: u64,
) -> Result<RecordingSession, CaptureValidationError> {
    let bounds = request
        .recording
        .ok_or(CaptureValidationError::RecordingBoundsRequired)?;
    let plan = validate_capture_request(&request, policy)?;
    if plan.mode != CaptureMode::Recording {
        return Err(CaptureValidationError::RecordingBoundsUnexpected);
    }
    Ok(RecordingSession {
        next_sequence_id: plan.sequence_id,
        plan,
        bounds,
        started_at_millis,
        last_capture_at_millis: None,
        frames_captured: 0,
        completed: false,
    })
}

pub fn service_recording_frame_with_readback<F>(
    session: &mut RecordingSession,
    policy: &CapturePolicy,
    now_millis: u64,
    frame: CaptureFrameContext,
    readback: &mut F,
) -> Result<RecordingServiceOutcome, CaptureServiceError>
where
    F: FnMut(CaptureFrameContext) -> Result<CapturedRgbaFrame, CaptureReadbackError>,
{
    if session.completed || recording_limit_reached(session, now_millis) {
        session.completed = true;
        return Ok(RecordingServiceOutcome::Complete);
    }
    if !recording_frame_due(session, now_millis) {
        return Ok(RecordingServiceOutcome::Waiting);
    }
    if !session.plan.includes_ui {
        return Err(CaptureServiceError::UiExclusionUnsupported);
    }
    validate_dimensions(frame.width_px, frame.height_px, policy)
        .map_err(CaptureServiceError::Validation)?;

    let relative_path = recording_frame_relative_path(session)?;
    let artifact_path =
        contained_artifact_path(policy, &relative_path).map_err(CaptureServiceError::Validation)?;
    let frame_plan = CapturePlan {
        mode: CaptureMode::Recording,
        format: session.plan.format,
        output: CaptureOutput::Artifact { relative_path },
        includes_ui: session.plan.includes_ui,
        artifact_path: Some(artifact_path),
        sequence_id: session.next_sequence_id,
    };
    let captured_frame = readback(frame).map_err(CaptureServiceError::Readback)?;
    let metadata = persist_captured_frame_artifact(&frame_plan, &captured_frame, policy)
        .map_err(CaptureServiceError::Persistence)?
        .ok_or(CaptureServiceError::Persistence(
            CapturePersistenceError::MissingArtifactPath,
        ))?;
    session.last_capture_at_millis = Some(now_millis);
    session.frames_captured = session.frames_captured.saturating_add(1);
    session.next_sequence_id = session.next_sequence_id.saturating_add(1);
    Ok(RecordingServiceOutcome::Captured(metadata))
}

fn recording_frame_due(session: &RecordingSession, now_millis: u64) -> bool {
    match session.last_capture_at_millis {
        None => true,
        Some(last_capture_at_millis) => {
            now_millis.saturating_sub(last_capture_at_millis)
                >= recording_frame_interval_millis(session.bounds)
        }
    }
}

fn recording_limit_reached(session: &RecordingSession, now_millis: u64) -> bool {
    if let Some(max_frames) = session.bounds.max_frames {
        if session.frames_captured >= max_frames {
            return true;
        }
    }
    if let Some(max_duration_millis) = session.bounds.max_duration_millis {
        return now_millis.saturating_sub(session.started_at_millis) > max_duration_millis;
    }
    false
}

fn recording_frame_interval_millis(bounds: RecordingBounds) -> u64 {
    let requested_fps = u64::from(bounds.frame_rate_hz);
    (MILLIS_PER_SECOND / requested_fps).max(MIN_FRAME_INTERVAL_MILLIS)
}

fn recording_frame_relative_path(
    session: &RecordingSession,
) -> Result<PathBuf, CaptureServiceError> {
    let CaptureOutput::Artifact { relative_path } = &session.plan.output else {
        return Err(CaptureServiceError::Validation(
            CaptureValidationError::RecordingArtifactOutputRequired,
        ));
    };
    Ok(relative_path.join(format!(
        "{DEFAULT_FRAME_ARTIFACT_PREFIX}-{sequence_id:0width$}.{extension}",
        sequence_id = session.next_sequence_id,
        width = CAPTURE_ARTIFACT_SEQUENCE_WIDTH,
        extension = session.plan.format.as_extension()
    )))
}

impl RecordingSession {
    pub fn frames_captured(&self) -> u32 {
        self.frames_captured
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
}

pub fn encode_png_frame(frame: &CapturedRgbaFrame) -> Result<Vec<u8>, CapturePersistenceError> {
    let expected_len = rgba_buffer_len(frame.width_px, frame.height_px)
        .map_err(CapturePersistenceError::Readback)?;
    if frame.rgba_top_left.len() != expected_len {
        return Err(CapturePersistenceError::Readback(
            CaptureReadbackError::BufferLengthMismatch {
                expected: expected_len,
                actual: frame.rgba_top_left.len(),
            },
        ));
    }
    let Some(image) =
        image::RgbaImage::from_raw(frame.width_px, frame.height_px, frame.rgba_top_left.clone())
    else {
        return Err(CapturePersistenceError::Readback(
            CaptureReadbackError::BufferLengthMismatch {
                expected: expected_len,
                actual: frame.rgba_top_left.len(),
            },
        ));
    };
    let mut png_bytes = Vec::new();
    image::DynamicImage::ImageRgba8(image)
        .write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .map_err(|err| CapturePersistenceError::Encode(err.to_string()))?;
    Ok(png_bytes)
}

pub fn validate_artifact_metadata(
    metadata: &CaptureArtifactMetadata,
    policy: &CapturePolicy,
) -> Result<PathBuf, CaptureValidationError> {
    validate_dimensions(metadata.width_px, metadata.height_px, policy)?;
    validate_artifact_size(metadata.byte_len, policy)?;
    contained_artifact_path(policy, &metadata.relative_path)
}

pub fn validate_dimensions(
    width_px: u32,
    height_px: u32,
    policy: &CapturePolicy,
) -> Result<(), CaptureValidationError> {
    if width_px == 0 || width_px > policy.max_width_px {
        return Err(CaptureValidationError::WidthOutOfRange {
            requested: width_px,
            max: policy.max_width_px,
        });
    }
    if height_px == 0 || height_px > policy.max_height_px {
        return Err(CaptureValidationError::HeightOutOfRange {
            requested: height_px,
            max: policy.max_height_px,
        });
    }
    Ok(())
}

pub fn read_current_framebuffer_rgba_top_left(
    width_px: u32,
    height_px: u32,
    frame_id: u64,
) -> Result<CapturedRgbaFrame, CaptureReadbackError> {
    read_current_framebuffer_for_context(CaptureFrameContext {
        width_px,
        height_px,
        frame_id,
    })
}

pub fn read_current_framebuffer_for_context(
    frame: CaptureFrameContext,
) -> Result<CapturedRgbaFrame, CaptureReadbackError> {
    let expected_len = rgba_buffer_len(frame.width_px, frame.height_px)?;
    let mut rgba_bottom_left = vec![0; expected_len];
    gl::read_pixels_rgba(
        FRAMEBUFFER_READ_ORIGIN_X,
        FRAMEBUFFER_READ_ORIGIN_Y,
        frame.width_px,
        frame.height_px,
        &mut rgba_bottom_left,
    );
    captured_rgba_from_bottom_left(
        frame.width_px,
        frame.height_px,
        frame.frame_id,
        &rgba_bottom_left,
    )
}

pub fn captured_rgba_from_bottom_left(
    width_px: u32,
    height_px: u32,
    frame_id: u64,
    rgba_bottom_left: &[u8],
) -> Result<CapturedRgbaFrame, CaptureReadbackError> {
    Ok(CapturedRgbaFrame {
        width_px,
        height_px,
        frame_id,
        rgba_top_left: normalize_rgba_bottom_left_to_top_left(
            width_px,
            height_px,
            rgba_bottom_left,
        )?,
    })
}

pub fn normalize_rgba_bottom_left_to_top_left(
    width_px: u32,
    height_px: u32,
    rgba_bottom_left: &[u8],
) -> Result<Vec<u8>, CaptureReadbackError> {
    let expected_len = rgba_buffer_len(width_px, height_px)?;
    if rgba_bottom_left.len() != expected_len {
        return Err(CaptureReadbackError::BufferLengthMismatch {
            expected: expected_len,
            actual: rgba_bottom_left.len(),
        });
    }

    let row_stride = rgba_row_stride_bytes(width_px, height_px)?;
    let mut rgba_top_left = vec![0; expected_len];
    for top_row in 0..height_px as usize {
        let bottom_row = (height_px as usize) - top_row - 1;
        let source_start = bottom_row * row_stride;
        let target_start = top_row * row_stride;
        let source_end = source_start + row_stride;
        let target_end = target_start + row_stride;
        rgba_top_left[target_start..target_end]
            .copy_from_slice(&rgba_bottom_left[source_start..source_end]);
    }

    Ok(rgba_top_left)
}

pub fn rgba_buffer_len(width_px: u32, height_px: u32) -> Result<usize, CaptureReadbackError> {
    validate_readback_dimensions(width_px, height_px)?;
    let row_stride = rgba_row_stride_bytes(width_px, height_px)?;
    row_stride
        .checked_mul(height_px as usize)
        .ok_or(CaptureReadbackError::BufferSizeOverflow {
            width_px,
            height_px,
        })
}

fn validate_readback_dimensions(width_px: u32, height_px: u32) -> Result<(), CaptureReadbackError> {
    if width_px == 0 || height_px == 0 {
        return Err(CaptureReadbackError::InvalidDimensions {
            width_px,
            height_px,
        });
    }
    Ok(())
}

fn rgba_row_stride_bytes(width_px: u32, height_px: u32) -> Result<usize, CaptureReadbackError> {
    (width_px as usize).checked_mul(RGBA_BYTES_PER_PIXEL).ok_or(
        CaptureReadbackError::BufferSizeOverflow {
            width_px,
            height_px,
        },
    )
}

fn validate_artifact_size(
    byte_len: u64,
    policy: &CapturePolicy,
) -> Result<(), CaptureValidationError> {
    if byte_len > policy.max_artifact_bytes {
        return Err(CaptureValidationError::ArtifactTooLarge {
            requested: byte_len,
            max: policy.max_artifact_bytes,
        });
    }
    Ok(())
}

fn validate_recording_bounds(
    recording: RecordingBounds,
    policy: &CapturePolicy,
) -> Result<(), CaptureValidationError> {
    if recording.frame_rate_hz < policy.min_recording_fps
        || recording.frame_rate_hz > policy.max_recording_fps
    {
        return Err(CaptureValidationError::RecordingFrameRateOutOfRange {
            requested: recording.frame_rate_hz,
            min: policy.min_recording_fps,
            max: policy.max_recording_fps,
        });
    }

    if recording.max_frames.is_none() && recording.max_duration_millis.is_none() {
        return Err(CaptureValidationError::RecordingDurationRequired);
    }

    if let Some(max_frames) = recording.max_frames {
        if max_frames == 0 || max_frames > policy.max_recording_frames {
            return Err(CaptureValidationError::RecordingFramesOutOfRange {
                requested: max_frames,
                max: policy.max_recording_frames,
            });
        }
    }

    if let Some(max_duration_millis) = recording.max_duration_millis {
        if max_duration_millis == 0 || max_duration_millis > policy.max_recording_millis {
            return Err(CaptureValidationError::RecordingMillisOutOfRange {
                requested: max_duration_millis,
                max: policy.max_recording_millis,
            });
        }
    }

    Ok(())
}

fn contained_artifact_path(
    policy: &CapturePolicy,
    relative_path: &Path,
) -> Result<PathBuf, CaptureValidationError> {
    let capture_dir = policy
        .capture_dir
        .as_ref()
        .ok_or(CaptureValidationError::MissingCaptureDir)?;
    validate_relative_artifact_path(relative_path)?;
    Ok(capture_dir.join(relative_path))
}

fn validate_relative_artifact_path(relative_path: &Path) -> Result<(), CaptureValidationError> {
    let mut components_seen = false;
    for component in relative_path.components() {
        components_seen = true;
        match component {
            Component::Normal(name) if !name.is_empty() => {}
            _ => {
                return Err(CaptureValidationError::ArtifactPathEscapes {
                    relative_path: relative_path.to_path_buf(),
                })
            }
        }
    }
    if !components_seen {
        return Err(CaptureValidationError::EmptyArtifactPath);
    }
    Ok(())
}

fn is_blake3_hex(digest: &str) -> bool {
    digest.len() == BLAKE3_HEX_LENGTH && digest.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn synthetic_frame(
        frame: CaptureFrameContext,
    ) -> Result<CapturedRgbaFrame, CaptureReadbackError> {
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
        let second =
            sender.enqueue_deferred(inline_capture_request(CaptureMode::LatestFrame, true));

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

        let second =
            sender.enqueue_deferred(inline_capture_request(CaptureMode::LatestFrame, true));

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
}
