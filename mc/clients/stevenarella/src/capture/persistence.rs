// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use std::fs;
use std::io::Cursor;

use super::model::{
    Blake3DigestHex, CaptureArtifactMetadata, CaptureOutput, CapturePersistenceError, CapturePlan,
    CapturePolicy, CaptureReadbackError, CapturedRgbaFrame, RedactionState,
};
use super::readback::rgba_buffer_len;
use super::validation::{validate_artifact_metadata, validate_artifact_size, validate_dimensions};

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
