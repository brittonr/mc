// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use std::path::{Component, Path, PathBuf};

use super::model::{
    CaptureArtifactMetadata, CaptureFormat, CaptureMode, CaptureOutput, CapturePlan, CapturePolicy,
    CaptureRequest, CaptureValidationError, RecordingBounds, CAPTURE_ARTIFACT_SEQUENCE_WIDTH,
    DEFAULT_FRAME_ARTIFACT_PREFIX, DEFAULT_LATEST_FRAME_ARTIFACT_DIR,
    DEFAULT_RECORDING_ARTIFACT_DIR, DEFAULT_SCREENSHOT_ARTIFACT_DIR,
};

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

pub(super) fn validate_one_shot_capture_request_shape(
    request: &CaptureRequest,
) -> Result<(), CaptureValidationError> {
    match request.mode {
        CaptureMode::Screenshot | CaptureMode::LatestFrame => Ok(()),
        CaptureMode::Recording => Err(CaptureValidationError::RecordingBoundsUnexpected),
    }
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

pub(super) fn validate_artifact_size(
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

pub(super) fn validate_recording_bounds(
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

pub(super) fn contained_artifact_path(
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
