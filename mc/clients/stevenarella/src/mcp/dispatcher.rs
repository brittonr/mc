// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::capture_adapter::McpCaptureTools;
use super::control_queue::McpCommandSender;
use super::protocol::{
    capture_resource_read_response, capture_tool_response, control_tool_response, jsonrpc_error,
    mcp_capture_error_response, route_jsonrpc_line, McpProtocolAction, JSONRPC_INTERNAL_ERROR,
};
use super::registry::McpRegistryAvailability;
use crate::capture::CaptureMode;
use crate::control::ControlCommand;
use serde_json::Value;

pub fn handle_jsonrpc_line(line: &str) -> Option<String> {
    handle_jsonrpc_line_with_auth(line, None)
}

pub fn handle_jsonrpc_line_with_auth(line: &str, required_token: Option<&str>) -> Option<String> {
    handle_jsonrpc_line_with_auth_and_command_sender(line, required_token, None)
}

pub fn handle_jsonrpc_line_with_auth_and_command_sender(
    line: &str,
    required_token: Option<&str>,
    command_sender: Option<&McpCommandSender>,
) -> Option<String> {
    handle_jsonrpc_line_with_auth_and_tools(line, required_token, command_sender, None)
}

pub fn handle_jsonrpc_line_with_auth_and_tools(
    line: &str,
    required_token: Option<&str>,
    command_sender: Option<&McpCommandSender>,
    capture_tools: Option<&McpCaptureTools>,
) -> Option<String> {
    let availability = McpRegistryAvailability {
        control: command_sender.is_some(),
        capture: capture_tools.is_some(),
    };
    let action = route_jsonrpc_line(line, required_token, availability);
    execute_protocol_action(action, command_sender, capture_tools)
}

fn execute_protocol_action(
    action: McpProtocolAction,
    command_sender: Option<&McpCommandSender>,
    capture_tools: Option<&McpCaptureTools>,
) -> Option<String> {
    match action {
        McpProtocolAction::NoResponse => None,
        McpProtocolAction::Response(response) => Some(response),
        McpProtocolAction::ControlCall { id, command } => {
            Some(execute_control_tool_call(id, command, command_sender))
        }
        McpProtocolAction::CaptureToolCall {
            id,
            mode,
            arguments,
        } => Some(execute_capture_tool_call(
            id,
            mode,
            arguments.as_ref(),
            capture_tools,
        )),
        McpProtocolAction::CaptureResourceRead { id, uri, mode } => {
            Some(execute_capture_resource_read(id, &uri, mode, capture_tools))
        }
    }
}

fn execute_control_tool_call(
    id: Value,
    command: ControlCommand,
    command_sender: Option<&McpCommandSender>,
) -> String {
    let Some(command_sender) = command_sender else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "control queue unavailable");
    };
    let response = match command_sender.enqueue(command) {
        Ok(response) => response,
        Err(err) => {
            return jsonrpc_error(
                id,
                JSONRPC_INTERNAL_ERROR,
                &format!("control queue failed: {err:?}"),
            )
        }
    };

    control_tool_response(id, &response)
}

fn execute_capture_tool_call(
    id: Value,
    mode: CaptureMode,
    arguments: Option<&Value>,
    capture_tools: Option<&McpCaptureTools>,
) -> String {
    let Some(capture_tools) = capture_tools else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "capture queue unavailable");
    };
    let payload = match capture_tools.capture_one_shot_from_arguments(mode, arguments) {
        Ok(payload) => payload,
        Err(err) => return mcp_capture_error_response(id, err),
    };
    capture_tool_response(id, &payload)
}

fn execute_capture_resource_read(
    id: Value,
    uri: &str,
    mode: CaptureMode,
    capture_tools: Option<&McpCaptureTools>,
) -> String {
    let Some(capture_tools) = capture_tools else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "capture queue unavailable");
    };
    let payload = match capture_tools.capture_one_shot_inline(mode) {
        Ok(payload) => payload,
        Err(err) => return mcp_capture_error_response(id, err),
    };
    capture_resource_read_response(id, uri, &payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatcher_preserves_no_response_notifications() {
        let response =
            handle_jsonrpc_line(r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);

        assert_eq!(response, None);
    }

    #[test]
    fn dispatcher_rejects_closed_control_queue_after_pure_routing() {
        let (sender, receiver) = super::super::control_queue::control_command_channel();
        drop(receiver);
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 301,
            "method": super::super::registry::MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": super::super::registry::MCP_ENQUEUE_CONTROL_TOOL,
                "arguments": { "command": { "action": "status" } },
            },
        })
        .to_string();

        let response =
            handle_jsonrpc_line_with_auth_and_command_sender(&request, None, Some(&sender))
                .expect("closed queue should render an error response");

        assert!(response.contains(&JSONRPC_INTERNAL_ERROR.to_string()));
        assert!(response.contains("control queue failed"));
    }
}
