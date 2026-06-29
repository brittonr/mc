// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::gl;

use super::model::{
    CaptureFrameContext, CaptureReadbackError, CapturedRgbaFrame, RGBA_BYTES_PER_PIXEL,
};

const FRAMEBUFFER_READ_ORIGIN_X: i32 = 0;
const FRAMEBUFFER_READ_ORIGIN_Y: i32 = 0;

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
