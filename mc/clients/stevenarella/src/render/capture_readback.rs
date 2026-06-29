use crate::capture::{rgba_buffer_len, CaptureFrameContext, CaptureReadbackError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RendererCaptureState {
    pub(crate) width_px: u32,
    pub(crate) height_px: u32,
    pub(crate) frame_id: u64,
    pub(crate) context_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CaptureFramePlanError {
    ContextUnavailable,
    Readback(CaptureReadbackError),
}

pub(crate) fn available_capture_state(
    width_px: u32,
    height_px: u32,
    frame_id: u64,
) -> RendererCaptureState {
    RendererCaptureState {
        width_px,
        height_px,
        frame_id,
        context_available: true,
    }
}

pub(crate) fn unavailable_capture_state(
    width_px: u32,
    height_px: u32,
    frame_id: u64,
) -> RendererCaptureState {
    RendererCaptureState {
        width_px,
        height_px,
        frame_id,
        context_available: false,
    }
}

pub(crate) fn plan_capture_frame(
    state: RendererCaptureState,
) -> Result<CaptureFrameContext, CaptureFramePlanError> {
    if !state.context_available {
        return Err(CaptureFramePlanError::ContextUnavailable);
    }
    rgba_buffer_len(state.width_px, state.height_px).map_err(CaptureFramePlanError::Readback)?;
    Ok(CaptureFrameContext {
        width_px: state.width_px,
        height_px: state.height_px,
        frame_id: state.frame_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CAPTURE_WIDTH: u32 = 320;
    const TEST_CAPTURE_HEIGHT: u32 = 180;
    const TEST_FRAME_ID: u64 = 7;

    #[test]
    fn capture_frame_plan_returns_readback_context_for_available_renderer() {
        let plan = plan_capture_frame(available_capture_state(
            TEST_CAPTURE_WIDTH,
            TEST_CAPTURE_HEIGHT,
            TEST_FRAME_ID,
        ))
        .unwrap();

        assert_eq!(plan.width_px, TEST_CAPTURE_WIDTH);
        assert_eq!(plan.height_px, TEST_CAPTURE_HEIGHT);
        assert_eq!(plan.frame_id, TEST_FRAME_ID);
    }

    #[test]
    fn capture_frame_plan_rejects_unavailable_or_invalid_contexts() {
        assert_eq!(
            plan_capture_frame(unavailable_capture_state(
                TEST_CAPTURE_WIDTH,
                TEST_CAPTURE_HEIGHT,
                TEST_FRAME_ID,
            )),
            Err(CaptureFramePlanError::ContextUnavailable)
        );
        assert_eq!(
            plan_capture_frame(available_capture_state(
                0,
                TEST_CAPTURE_HEIGHT,
                TEST_FRAME_ID
            )),
            Err(CaptureFramePlanError::Readback(
                CaptureReadbackError::InvalidDimensions {
                    width_px: 0,
                    height_px: TEST_CAPTURE_HEIGHT,
                }
            ))
        );
    }
}
