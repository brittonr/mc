// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use std::path::PathBuf;

use super::model::{
    CaptureArtifactMetadata, CaptureFrameContext, CaptureMode, CaptureOutput,
    CapturePersistenceError, CapturePlan, CapturePolicy, CaptureReadbackError, CaptureRequest,
    CaptureServiceError, CaptureValidationError, CapturedRgbaFrame, RecordingBounds,
    CAPTURE_ARTIFACT_SEQUENCE_WIDTH, DEFAULT_FRAME_ARTIFACT_PREFIX, DEFAULT_RECORDING_ARTIFACT_DIR,
    DEFAULT_RECORDING_ARTIFACT_PREFIX, MILLIS_PER_SECOND, MIN_FRAME_INTERVAL_MILLIS,
};
use super::persistence::persist_captured_frame_artifact;
use super::validation::{contained_artifact_path, validate_capture_request, validate_dimensions};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RecordingFrameDecision {
    Capture,
    Waiting,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RecordingCadenceSnapshot {
    pub bounds: RecordingBounds,
    pub started_at_millis: u64,
    pub last_capture_at_millis: Option<u64>,
    pub frames_captured: u32,
    pub completed: bool,
}

pub fn default_recording_relative_dir(sequence_id: u64) -> PathBuf {
    PathBuf::from(DEFAULT_RECORDING_ARTIFACT_DIR).join(format!(
        "{DEFAULT_RECORDING_ARTIFACT_PREFIX}-{sequence_id:0width$}",
        width = CAPTURE_ARTIFACT_SEQUENCE_WIDTH
    ))
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
    match recording_frame_decision(session, now_millis) {
        RecordingFrameDecision::Complete => {
            session.completed = true;
            return Ok(RecordingServiceOutcome::Complete);
        }
        RecordingFrameDecision::Waiting => return Ok(RecordingServiceOutcome::Waiting),
        RecordingFrameDecision::Capture => {}
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

pub(super) fn recording_frame_decision(
    session: &RecordingSession,
    now_millis: u64,
) -> RecordingFrameDecision {
    recording_cadence_decision(
        RecordingCadenceSnapshot {
            bounds: session.bounds,
            started_at_millis: session.started_at_millis,
            last_capture_at_millis: session.last_capture_at_millis,
            frames_captured: session.frames_captured,
            completed: session.completed,
        },
        now_millis,
    )
}

pub(super) fn recording_cadence_decision(
    snapshot: RecordingCadenceSnapshot,
    now_millis: u64,
) -> RecordingFrameDecision {
    if snapshot.completed || recording_limit_reached(snapshot, now_millis) {
        return RecordingFrameDecision::Complete;
    }
    if recording_frame_due(snapshot, now_millis) {
        return RecordingFrameDecision::Capture;
    }
    RecordingFrameDecision::Waiting
}

fn recording_frame_due(snapshot: RecordingCadenceSnapshot, now_millis: u64) -> bool {
    match snapshot.last_capture_at_millis {
        None => true,
        Some(last_capture_at_millis) => {
            now_millis.saturating_sub(last_capture_at_millis)
                >= recording_frame_interval_millis(snapshot.bounds)
        }
    }
}

fn recording_limit_reached(snapshot: RecordingCadenceSnapshot, now_millis: u64) -> bool {
    if let Some(max_frames) = snapshot.bounds.max_frames {
        if snapshot.frames_captured >= max_frames {
            return true;
        }
    }
    if let Some(max_duration_millis) = snapshot.bounds.max_duration_millis {
        return now_millis.saturating_sub(snapshot.started_at_millis) > max_duration_millis;
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
