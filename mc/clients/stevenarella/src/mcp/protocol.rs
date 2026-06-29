// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::auth::request_has_token;
use super::capture_adapter::{McpCaptureToolError, McpCaptureToolPayload};
use super::registry::{
    capture_mode_for_resource_uri, capture_mode_for_tool_name, resources_list_result,
    tools_list_result, McpRegistryAvailability, MCP_CONTENT_TYPE_IMAGE, MCP_CONTENT_TYPE_TEXT,
    MCP_ENQUEUE_CONTROL_TOOL, MCP_FIELD_ARGUMENTS, MCP_FIELD_COMMAND, MCP_FIELD_NAME,
    MCP_FIELD_URI, MCP_MIME_APPLICATION_JSON, MCP_MIME_IMAGE_PNG, MCP_RESOURCES_LIST_METHOD,
    MCP_RESOURCES_READ_METHOD, MCP_TOOLS_CALL_METHOD, MCP_TOOLS_LIST_METHOD,
};
use crate::capture::CaptureMode;
use crate::control::{ControlCommand, ControlResponse};
use serde_json::{json, Value};

pub const JSONRPC_PARSE_ERROR: i64 = -32700;
pub const JSONRPC_INVALID_REQUEST: i64 = -32600;
pub const JSONRPC_METHOD_NOT_FOUND: i64 = -32601;
pub const JSONRPC_INTERNAL_ERROR: i64 = -32603;
pub const JSONRPC_UNAUTHORIZED: i64 = -32001;

const MCP_INITIALIZE_METHOD: &str = "initialize";
const MCP_PING_METHOD: &str = "ping";
const MCP_NOTIFICATIONS_PREFIX: &str = "notifications/";

#[derive(Debug, Clone, PartialEq)]
pub enum McpProtocolAction {
    NoResponse,
    Response(String),
    ControlCall {
        id: Value,
        command: ControlCommand,
    },
    CaptureToolCall {
        id: Value,
        mode: CaptureMode,
        arguments: Option<Value>,
    },
    CaptureResourceRead {
        id: Value,
        uri: String,
        mode: CaptureMode,
    },
}

pub fn route_jsonrpc_line(
    line: &str,
    required_token: Option<&str>,
    availability: McpRegistryAvailability,
) -> McpProtocolAction {
    let value = match serde_json::from_str::<Value>(line) {
        Ok(value) => value,
        Err(_) => {
            return McpProtocolAction::Response(jsonrpc_error(
                Value::Null,
                JSONRPC_PARSE_ERROR,
                "parse error",
            ))
        }
    };
    route_jsonrpc_value(value, required_token, availability)
}

fn route_jsonrpc_value(
    value: Value,
    required_token: Option<&str>,
    availability: McpRegistryAvailability,
) -> McpProtocolAction {
    let Some(object) = value.as_object() else {
        return McpProtocolAction::Response(jsonrpc_error(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            "request must be an object",
        ));
    };

    let id = object.get("id").cloned();
    let method = object.get("method").and_then(Value::as_str);
    let Some(method) = method else {
        return optional_error(id, JSONRPC_INVALID_REQUEST, "missing method");
    };

    if let Some(required_token) = required_token {
        if !request_has_token(object.get("params"), required_token) {
            return McpProtocolAction::Response(jsonrpc_error(
                id.unwrap_or(Value::Null),
                JSONRPC_UNAUTHORIZED,
                "unauthorized",
            ));
        }
    }

    match method {
        MCP_INITIALIZE_METHOD => optional_result(id, initialize_result()),
        MCP_TOOLS_LIST_METHOD => optional_result(id, tools_list_result(availability)),
        MCP_TOOLS_CALL_METHOD => route_tools_call(id, object.get("params"), availability),
        MCP_RESOURCES_LIST_METHOD => optional_result(id, resources_list_result(availability)),
        MCP_RESOURCES_READ_METHOD => route_resources_read(id, object.get("params"), availability),
        MCP_PING_METHOD => optional_result(id, json!({})),
        method if method.starts_with(MCP_NOTIFICATIONS_PREFIX) => McpProtocolAction::NoResponse,
        _ => optional_error(id, JSONRPC_METHOD_NOT_FOUND, "method not found"),
    }
}

fn route_tools_call(
    id: Option<Value>,
    params: Option<&Value>,
    availability: McpRegistryAvailability,
) -> McpProtocolAction {
    let Some(id) = id else {
        return McpProtocolAction::NoResponse;
    };
    let Some(params) = params.and_then(Value::as_object) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing tool params",
        ));
    };
    let Some(name) = params.get(MCP_FIELD_NAME).and_then(Value::as_str) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing tool name",
        ));
    };

    if name == MCP_ENQUEUE_CONTROL_TOOL {
        return route_control_tool_call(id, params, availability);
    }
    if let Some(mode) = capture_mode_for_tool_name(name) {
        if !availability.capture {
            return McpProtocolAction::Response(jsonrpc_error(
                id,
                JSONRPC_INTERNAL_ERROR,
                "capture queue unavailable",
            ));
        }
        return McpProtocolAction::CaptureToolCall {
            id,
            mode,
            arguments: params.get(MCP_FIELD_ARGUMENTS).cloned(),
        };
    }

    McpProtocolAction::Response(jsonrpc_error(
        id,
        JSONRPC_METHOD_NOT_FOUND,
        "tool not found",
    ))
}

fn route_control_tool_call(
    id: Value,
    params: &serde_json::Map<String, Value>,
    availability: McpRegistryAvailability,
) -> McpProtocolAction {
    if !availability.control {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INTERNAL_ERROR,
            "control queue unavailable",
        ));
    }
    let Some(arguments) = params.get(MCP_FIELD_ARGUMENTS).and_then(Value::as_object) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing tool arguments",
        ));
    };
    let Some(command_value) = arguments.get(MCP_FIELD_COMMAND) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing control command",
        ));
    };
    let command = match crate::control::parse_control_command_value(command_value) {
        Ok(command) => command,
        Err(err) => {
            return McpProtocolAction::Response(jsonrpc_error(
                id,
                JSONRPC_INVALID_REQUEST,
                &format!("invalid control command: {err:?}"),
            ))
        }
    };

    McpProtocolAction::ControlCall { id, command }
}

fn route_resources_read(
    id: Option<Value>,
    params: Option<&Value>,
    availability: McpRegistryAvailability,
) -> McpProtocolAction {
    let Some(id) = id else {
        return McpProtocolAction::NoResponse;
    };
    if !availability.capture {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INTERNAL_ERROR,
            "capture queue unavailable",
        ));
    }
    let Some(params) = params.and_then(Value::as_object) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing resource params",
        ));
    };
    let Some(uri) = params.get(MCP_FIELD_URI).and_then(Value::as_str) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_INVALID_REQUEST,
            "missing resource uri",
        ));
    };
    let Some(mode) = capture_mode_for_resource_uri(uri) else {
        return McpProtocolAction::Response(jsonrpc_error(
            id,
            JSONRPC_METHOD_NOT_FOUND,
            "resource not found",
        ));
    };

    McpProtocolAction::CaptureResourceRead {
        id,
        uri: uri.to_owned(),
        mode,
    }
}

fn optional_result(id: Option<Value>, result: Value) -> McpProtocolAction {
    id.map(|id| McpProtocolAction::Response(jsonrpc_result(id, result)))
        .unwrap_or(McpProtocolAction::NoResponse)
}

fn optional_error(id: Option<Value>, code: i64, message: &str) -> McpProtocolAction {
    id.map(|id| McpProtocolAction::Response(jsonrpc_error(id, code, message)))
        .unwrap_or(McpProtocolAction::NoResponse)
}

fn initialize_result() -> Value {
    json!({
        "protocolVersion": "2024-11-05",
        "serverInfo": {
            "name": "stevenarella",
            "version": env!("CARGO_PKG_VERSION"),
        },
        "capabilities": {
            "tools": {},
            "resources": {},
        },
    })
}

pub(crate) fn control_tool_response(id: Value, response: &ControlResponse) -> String {
    jsonrpc_result(id, control_tool_result(response))
}

pub(crate) fn capture_tool_response(id: Value, payload: &McpCaptureToolPayload) -> String {
    jsonrpc_result(id, capture_tool_result(payload))
}

pub(crate) fn capture_resource_read_response(
    id: Value,
    uri: &str,
    payload: &McpCaptureToolPayload,
) -> String {
    jsonrpc_result(id, capture_resource_read_result(uri, payload))
}

fn control_tool_result(response: &ControlResponse) -> Value {
    json!({
        "content": [{
            "type": MCP_CONTENT_TYPE_TEXT,
            "text": json!({
                "outcome": response.outcome.as_str(),
                "message": response.message.as_deref(),
            })
            .to_string(),
        }],
        "isError": response.is_error(),
    })
}

fn capture_tool_result(payload: &McpCaptureToolPayload) -> Value {
    let mut content = Vec::new();
    if let Some(inline_png_base64) = &payload.inline_png_base64 {
        content.push(json!({
            "type": MCP_CONTENT_TYPE_IMAGE,
            "mimeType": MCP_MIME_IMAGE_PNG,
            "data": inline_png_base64,
        }));
    }
    content.push(json!({
        "type": MCP_CONTENT_TYPE_TEXT,
        "text": payload.metadata.to_string(),
    }));
    json!({
        "content": content,
        "isError": false,
    })
}

fn capture_resource_read_result(uri: &str, payload: &McpCaptureToolPayload) -> Value {
    let mut contents = Vec::new();
    if let Some(inline_png_base64) = &payload.inline_png_base64 {
        contents.push(json!({
            "uri": uri,
            "mimeType": MCP_MIME_IMAGE_PNG,
            "blob": inline_png_base64,
        }));
    }
    contents.push(json!({
        "uri": format!("{uri}#metadata"),
        "mimeType": MCP_MIME_APPLICATION_JSON,
        "text": payload.metadata.to_string(),
    }));
    json!({ "contents": contents })
}

pub(crate) fn mcp_capture_error_response(id: Value, err: McpCaptureToolError) -> String {
    let code = match &err {
        McpCaptureToolError::InvalidArguments(_) => JSONRPC_INVALID_REQUEST,
        McpCaptureToolError::QueueClosed
        | McpCaptureToolError::ResponseDropped
        | McpCaptureToolError::ResponseTimedOut
        | McpCaptureToolError::CaptureFailed(_)
        | McpCaptureToolError::InlineTooLarge { .. }
        | McpCaptureToolError::MissingArtifactMetadata => JSONRPC_INTERNAL_ERROR,
    };
    jsonrpc_error(id, code, &format!("capture failed: {err:?}"))
}

fn jsonrpc_result(id: Value, result: Value) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    })
    .to_string()
}

pub(crate) fn jsonrpc_error(id: Value, code: i64, message: &str) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message,
        },
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::super::registry::{MCP_CAPTURE_SCREENSHOT_TOOL, MCP_OUTPUT_INLINE};
    use super::*;
    use crate::control::{ControlOutcome, CONTROL_OUTCOME_APPLIED_NAME};

    const TOKEN_VALUE: &str = "secret-token";
    const TEST_ID: i64 = 101;

    fn all_available() -> McpRegistryAvailability {
        McpRegistryAvailability {
            control: true,
            capture: true,
        }
    }

    fn none_available() -> McpRegistryAvailability {
        McpRegistryAvailability {
            control: false,
            capture: false,
        }
    }

    #[test]
    fn pure_route_returns_control_effect_without_queue_side_effects() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": TEST_ID,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_ENQUEUE_CONTROL_TOOL,
                "arguments": {
                    "command": { "action": "status" },
                },
            },
        })
        .to_string();

        let action = route_jsonrpc_line(&request, None, all_available());

        assert_eq!(
            action,
            McpProtocolAction::ControlCall {
                id: json!(TEST_ID),
                command: ControlCommand::Status,
            }
        );
    }

    #[test]
    fn pure_route_returns_capture_effect_without_capture_queue_side_effects() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": TEST_ID,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_CAPTURE_SCREENSHOT_TOOL,
                "arguments": { "output": MCP_OUTPUT_INLINE },
            },
        })
        .to_string();

        let action = route_jsonrpc_line(&request, None, all_available());

        assert_eq!(
            action,
            McpProtocolAction::CaptureToolCall {
                id: json!(TEST_ID),
                mode: CaptureMode::Screenshot,
                arguments: Some(json!({ "output": MCP_OUTPUT_INLINE })),
            }
        );
    }

    #[test]
    fn pure_route_rejects_unknown_method_without_adapters() {
        let action = route_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":1,"method":"missing"}"#,
            None,
            none_available(),
        );

        let McpProtocolAction::Response(response) = action else {
            panic!("expected method-not-found response");
        };
        assert!(response.contains(&JSONRPC_METHOD_NOT_FOUND.to_string()));
    }

    #[test]
    fn pure_route_rejects_unknown_tool_name_without_adapter_side_effects() {
        let request = json!({
            "jsonrpc": "2.0",
            "id": TEST_ID,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": { "name": "stevenarella.missing_tool" },
        })
        .to_string();

        let action = route_jsonrpc_line(&request, None, all_available());

        let McpProtocolAction::Response(response) = action else {
            panic!("expected tool-not-found response");
        };
        assert!(response.contains(&JSONRPC_METHOD_NOT_FOUND.to_string()));
        assert!(response.contains("tool not found"));
    }

    #[test]
    fn pure_route_rejects_unauthorized_request_before_tool_routing() {
        let action = route_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":1,"method":"ping","params":{"token":"wrong"}}"#,
            Some(TOKEN_VALUE),
            none_available(),
        );

        let McpProtocolAction::Response(response) = action else {
            panic!("expected unauthorized response");
        };
        assert!(response.contains(&JSONRPC_UNAUTHORIZED.to_string()));
    }

    #[test]
    fn shell_helper_renders_control_response_after_adapter_result() {
        let payload = control_tool_result(&ControlResponse {
            outcome: ControlOutcome::Applied,
            message: None,
        });
        let text = payload["content"][0]["text"].as_str().unwrap();
        let parsed: Value = serde_json::from_str(text).unwrap();

        assert_eq!(parsed["outcome"], CONTROL_OUTCOME_APPLIED_NAME);
        assert_eq!(parsed["message"], Value::Null);
    }
}
