// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::registry::{
    MCP_FIELD_FORMAT, MCP_FIELD_INCLUDE_UI, MCP_FIELD_OUTPUT, MCP_FIELD_RELATIVE_PATH,
    MCP_OUTPUT_ARTIFACT, MCP_OUTPUT_INLINE,
};
use crate::capture::{
    self, CaptureFormat, CaptureMode, CaptureOutput, CapturePolicy, CaptureQueueError,
    CaptureRequest, CaptureRequestSender, ServicedCapture,
};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

const CAPTURE_RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;

#[derive(Clone)]
pub struct McpCaptureTools {
    sender: CaptureRequestSender,
    policy: CapturePolicy,
    next_sequence_id: Arc<AtomicU64>,
    response_timeout: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpCaptureToolError {
    InvalidArguments(String),
    QueueClosed,
    ResponseDropped,
    ResponseTimedOut,
    CaptureFailed(String),
    InlineTooLarge { requested: u64, max: u64 },
    MissingArtifactMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct McpCaptureToolPayload {
    pub(crate) metadata: Value,
    pub(crate) inline_png_base64: Option<String>,
}

impl McpCaptureTools {
    pub fn new(
        sender: CaptureRequestSender,
        policy: CapturePolicy,
        next_sequence_id: Arc<AtomicU64>,
    ) -> Self {
        Self {
            sender,
            policy,
            next_sequence_id,
            response_timeout: Duration::from_millis(CAPTURE_RESPONSE_TIMEOUT_MILLIS),
        }
    }

    pub fn with_response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = response_timeout;
        self
    }

    pub(crate) fn capture_one_shot_from_arguments(
        &self,
        mode: CaptureMode,
        arguments: Option<&Value>,
    ) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
        let sequence_id = self.next_sequence_id.fetch_add(1, Ordering::AcqRel);
        let request = capture_request_from_arguments(arguments, mode, &self.policy, sequence_id)?;
        self.capture_one_shot(request)
    }

    pub(crate) fn capture_one_shot_inline(
        &self,
        mode: CaptureMode,
    ) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
        let sequence_id = self.next_sequence_id.fetch_add(1, Ordering::AcqRel);
        let request = CaptureRequest {
            mode,
            format: CaptureFormat::Png,
            output: CaptureOutput::Inline,
            includes_ui: true,
            recording: None,
            sequence_id: Some(sequence_id),
        };
        self.capture_one_shot(request)
    }

    fn capture_one_shot(
        &self,
        request: CaptureRequest,
    ) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
        let response_receiver = self
            .sender
            .enqueue_deferred(request)
            .map_err(mcp_capture_queue_error)?;
        let serviced = match response_receiver.recv_timeout(self.response_timeout) {
            Ok(Ok(serviced)) => serviced,
            Ok(Err(err)) => return Err(McpCaptureToolError::CaptureFailed(format!("{err:?}"))),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                return Err(McpCaptureToolError::ResponseTimedOut)
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return Err(McpCaptureToolError::ResponseDropped)
            }
        };
        mcp_capture_payload_from_serviced_capture(serviced, &self.policy)
    }
}

fn capture_request_from_arguments(
    arguments: Option<&Value>,
    mode: CaptureMode,
    policy: &CapturePolicy,
    sequence_id: u64,
) -> Result<CaptureRequest, McpCaptureToolError> {
    let empty_arguments = serde_json::Map::new();
    let arguments = match arguments {
        Some(value) => value.as_object().ok_or_else(|| {
            McpCaptureToolError::InvalidArguments("arguments must be object".to_owned())
        })?,
        None => &empty_arguments,
    };
    let format = match arguments.get(MCP_FIELD_FORMAT).and_then(Value::as_str) {
        Some(format) => CaptureFormat::from_name(format).map_err(|err| {
            McpCaptureToolError::InvalidArguments(format!("invalid format: {err:?}"))
        })?,
        None => CaptureFormat::Png,
    };
    let includes_ui = match arguments.get(MCP_FIELD_INCLUDE_UI) {
        Some(value) => value.as_bool().ok_or_else(|| {
            McpCaptureToolError::InvalidArguments("include_ui must be boolean".to_owned())
        })?,
        None => true,
    };
    let output = capture_output_from_arguments(arguments, mode, format, sequence_id)?;
    let request = CaptureRequest {
        mode,
        format,
        output,
        includes_ui,
        recording: None,
        sequence_id: Some(sequence_id),
    };
    capture::validate_capture_request(&request, policy).map_err(|err| {
        McpCaptureToolError::InvalidArguments(format!("invalid capture request: {err:?}"))
    })?;
    Ok(request)
}

fn capture_output_from_arguments(
    arguments: &serde_json::Map<String, Value>,
    mode: CaptureMode,
    format: CaptureFormat,
    sequence_id: u64,
) -> Result<CaptureOutput, McpCaptureToolError> {
    match arguments.get(MCP_FIELD_OUTPUT).and_then(Value::as_str) {
        Some(MCP_OUTPUT_INLINE) | None => Ok(CaptureOutput::Inline),
        Some(MCP_OUTPUT_ARTIFACT) => Ok(CaptureOutput::Artifact {
            relative_path: capture_relative_path_from_arguments(
                arguments,
                mode,
                format,
                sequence_id,
            )?,
        }),
        Some(output) => Err(McpCaptureToolError::InvalidArguments(format!(
            "unsupported output: {output}"
        ))),
    }
}

fn capture_relative_path_from_arguments(
    arguments: &serde_json::Map<String, Value>,
    mode: CaptureMode,
    format: CaptureFormat,
    sequence_id: u64,
) -> Result<PathBuf, McpCaptureToolError> {
    match arguments.get(MCP_FIELD_RELATIVE_PATH) {
        Some(value) => value.as_str().map(PathBuf::from).ok_or_else(|| {
            McpCaptureToolError::InvalidArguments("relative_path must be string".to_owned())
        }),
        None => Ok(capture::default_artifact_relative_path(
            mode,
            sequence_id,
            format,
        )),
    }
}

fn mcp_capture_payload_from_serviced_capture(
    serviced: ServicedCapture,
    policy: &CapturePolicy,
) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
    match &serviced.plan.output {
        CaptureOutput::Inline => inline_capture_payload(serviced, policy),
        CaptureOutput::Artifact { .. } => artifact_capture_payload(serviced),
    }
}

fn inline_capture_payload(
    serviced: ServicedCapture,
    policy: &CapturePolicy,
) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
    let png_bytes = capture::encode_png_frame(&serviced.frame)
        .map_err(|err| McpCaptureToolError::CaptureFailed(format!("{err:?}")))?;
    let byte_len = u64::try_from(png_bytes.len()).map_err(|_| {
        McpCaptureToolError::CaptureFailed("inline image length overflow".to_owned())
    })?;
    if byte_len > policy.inline_response_bytes {
        return Err(McpCaptureToolError::InlineTooLarge {
            requested: byte_len,
            max: policy.inline_response_bytes,
        });
    }
    let digest = blake3::hash(&png_bytes).to_hex().to_string();
    let metadata = capture_metadata_json(
        serviced.plan.mode,
        serviced.plan.format,
        MCP_OUTPUT_INLINE,
        serviced.frame.width_px,
        serviced.frame.height_px,
        serviced.frame.frame_id,
        serviced.plan.sequence_id,
        byte_len,
        &digest,
        serviced.plan.includes_ui,
        None,
    );
    Ok(McpCaptureToolPayload {
        metadata,
        inline_png_base64: Some(base64::encode(png_bytes)),
    })
}

fn artifact_capture_payload(
    serviced: ServicedCapture,
) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
    let artifact = serviced
        .artifact
        .ok_or(McpCaptureToolError::MissingArtifactMetadata)?;
    let metadata = capture_metadata_json(
        serviced.plan.mode,
        artifact.format,
        MCP_OUTPUT_ARTIFACT,
        artifact.width_px,
        artifact.height_px,
        artifact.frame_id,
        artifact.sequence_id,
        artifact.byte_len,
        artifact.blake3_digest.as_str(),
        artifact.includes_ui,
        Some(artifact.relative_path.to_string_lossy().as_ref()),
    );
    Ok(McpCaptureToolPayload {
        metadata,
        inline_png_base64: None,
    })
}

fn capture_metadata_json(
    mode: CaptureMode,
    format: CaptureFormat,
    output: &str,
    width_px: u32,
    height_px: u32,
    frame_id: u64,
    sequence_id: u64,
    byte_len: u64,
    blake3_digest: &str,
    includes_ui: bool,
    relative_path: Option<&str>,
) -> Value {
    let mut metadata = json!({
        "mode": capture_mode_name(mode),
        "format": format.as_extension(),
        "output": output,
        "width_px": width_px,
        "height_px": height_px,
        "frame_id": frame_id,
        "sequence_id": sequence_id,
        "byte_len": byte_len,
        "blake3_digest": blake3_digest,
        "includes_ui": includes_ui,
        "redaction": "not_reviewed",
    });
    if let Some(relative_path) = relative_path {
        metadata["relative_path"] = Value::String(relative_path.to_owned());
    }
    metadata
}

fn capture_mode_name(mode: CaptureMode) -> &'static str {
    match mode {
        CaptureMode::Screenshot => "screenshot",
        CaptureMode::LatestFrame => "latest_frame",
        CaptureMode::Recording => "recording",
    }
}

fn mcp_capture_queue_error(err: CaptureQueueError) -> McpCaptureToolError {
    match err {
        CaptureQueueError::Validation(err) => {
            McpCaptureToolError::InvalidArguments(format!("invalid capture request: {err:?}"))
        }
        CaptureQueueError::QueueClosed => McpCaptureToolError::QueueClosed,
        CaptureQueueError::RateLimitExceeded { pending, max } => {
            McpCaptureToolError::InvalidArguments(format!(
                "capture rate limit exceeded: pending={pending} max={max}"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEQUENCE_ID: u64 = 0;
    const TEST_FRAME_ID: u64 = 42;
    const TEST_WIDTH_PX: u32 = 2;
    const TEST_HEIGHT_PX: u32 = 2;

    #[test]
    fn capture_metadata_records_inline_identity() {
        let metadata = capture_metadata_json(
            CaptureMode::Screenshot,
            CaptureFormat::Png,
            MCP_OUTPUT_INLINE,
            TEST_WIDTH_PX,
            TEST_HEIGHT_PX,
            TEST_FRAME_ID,
            TEST_SEQUENCE_ID,
            TEST_SEQUENCE_ID,
            &"0".repeat(capture::BLAKE3_HEX_LENGTH),
            true,
            None,
        );

        assert_eq!(metadata["mode"], "screenshot");
        assert_eq!(metadata["output"], MCP_OUTPUT_INLINE);
        assert_eq!(metadata["width_px"], TEST_WIDTH_PX);
        assert_eq!(metadata["height_px"], TEST_HEIGHT_PX);
        assert_eq!(metadata["redaction"], "not_reviewed");
        assert!(metadata.get("relative_path").is_none());
    }

    #[test]
    fn capture_request_rejects_non_object_arguments_before_enqueue() {
        let err = capture_request_from_arguments(
            Some(&json!(true)),
            CaptureMode::Screenshot,
            &CapturePolicy::memory(),
            TEST_SEQUENCE_ID,
        )
        .unwrap_err();

        assert_eq!(
            err,
            McpCaptureToolError::InvalidArguments("arguments must be object".to_owned())
        );
    }
}
