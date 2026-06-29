// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::model::{
    CaptureArtifactMetadata, CaptureFrameContext, CapturePlan, CapturePolicy, CaptureReadbackError,
    CaptureRequest, CaptureServiceError, CapturedRgbaFrame,
};
use super::persistence::persist_captured_frame_artifact;
use super::validation::{
    validate_capture_request, validate_dimensions, validate_one_shot_capture_request_shape,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServicedCapture {
    pub plan: CapturePlan,
    pub frame: CapturedRgbaFrame,
    pub artifact: Option<CaptureArtifactMetadata>,
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
