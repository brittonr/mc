// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::capture::CaptureMode;
use serde_json::{json, Value};

pub const MCP_TOOLS_CALL_METHOD: &str = "tools/call";
pub const MCP_TOOLS_LIST_METHOD: &str = "tools/list";
pub const MCP_RESOURCES_LIST_METHOD: &str = "resources/list";
pub const MCP_RESOURCES_READ_METHOD: &str = "resources/read";
pub const MCP_ENQUEUE_CONTROL_TOOL: &str = "stevenarella.enqueue_control";
pub const MCP_CAPTURE_SCREENSHOT_TOOL: &str = "stevenarella.capture_screenshot";
pub const MCP_CAPTURE_LATEST_FRAME_TOOL: &str = "stevenarella.capture_latest_frame";
pub const MCP_CAPTURE_SCREENSHOT_RESOURCE: &str = "stevenarella://capture/screenshot";
pub const MCP_CAPTURE_LATEST_FRAME_RESOURCE: &str = "stevenarella://capture/latest-frame";
pub const MCP_CONTENT_TYPE_TEXT: &str = "text";
pub const MCP_CONTENT_TYPE_IMAGE: &str = "image";
pub const MCP_MIME_APPLICATION_JSON: &str = "application/json";
pub const MCP_MIME_IMAGE_PNG: &str = "image/png";
pub const MCP_FIELD_ARGUMENTS: &str = "arguments";
pub const MCP_FIELD_COMMAND: &str = "command";
pub const MCP_FIELD_FORMAT: &str = "format";
pub const MCP_FIELD_INCLUDE_UI: &str = "include_ui";
pub const MCP_FIELD_NAME: &str = "name";
pub const MCP_FIELD_OUTPUT: &str = "output";
pub const MCP_FIELD_RELATIVE_PATH: &str = "relative_path";
pub const MCP_FIELD_URI: &str = "uri";
pub const MCP_OUTPUT_ARTIFACT: &str = "artifact";
pub const MCP_OUTPUT_INLINE: &str = "inline";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McpRegistryAvailability {
    pub control: bool,
    pub capture: bool,
}

pub fn tools_list_result(availability: McpRegistryAvailability) -> Value {
    let mut tools = Vec::new();
    if availability.control {
        tools.push(control_tool_schema());
    }
    if availability.capture {
        tools.push(capture_tool_schema(
            MCP_CAPTURE_SCREENSHOT_TOOL,
            "Capture one rendered Stevenarella screenshot.",
        ));
        tools.push(capture_tool_schema(
            MCP_CAPTURE_LATEST_FRAME_TOOL,
            "Capture the latest rendered Stevenarella frame.",
        ));
    }
    json!({ "tools": tools })
}

pub fn resources_list_result(availability: McpRegistryAvailability) -> Value {
    let resources = if availability.capture {
        vec![
            json!({
                "uri": MCP_CAPTURE_SCREENSHOT_RESOURCE,
                "name": "Stevenarella screenshot",
                "description": "Capture one screenshot from the next rendered frame.",
                "mimeType": MCP_MIME_IMAGE_PNG,
            }),
            json!({
                "uri": MCP_CAPTURE_LATEST_FRAME_RESOURCE,
                "name": "Stevenarella latest frame",
                "description": "Capture the latest rendered frame.",
                "mimeType": MCP_MIME_IMAGE_PNG,
            }),
        ]
    } else {
        Vec::new()
    };
    json!({ "resources": resources })
}

pub fn capture_mode_for_tool_name(name: &str) -> Option<CaptureMode> {
    match name {
        MCP_CAPTURE_SCREENSHOT_TOOL => Some(CaptureMode::Screenshot),
        MCP_CAPTURE_LATEST_FRAME_TOOL => Some(CaptureMode::LatestFrame),
        _ => None,
    }
}

pub fn capture_mode_for_resource_uri(uri: &str) -> Option<CaptureMode> {
    match uri {
        MCP_CAPTURE_SCREENSHOT_RESOURCE => Some(CaptureMode::Screenshot),
        MCP_CAPTURE_LATEST_FRAME_RESOURCE => Some(CaptureMode::LatestFrame),
        _ => None,
    }
}

fn control_tool_schema() -> Value {
    json!({
        "name": MCP_ENQUEUE_CONTROL_TOOL,
        "description": "Queue one Stevenarella control command for main-thread handling.",
        "inputSchema": {
            "type": "object",
            "properties": {
                MCP_FIELD_COMMAND: { "type": "object" },
            },
            "required": [MCP_FIELD_COMMAND],
            "additionalProperties": false,
        },
    })
}

fn capture_tool_schema(name: &str, description: &str) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": {
            "type": "object",
            "properties": {
                MCP_FIELD_OUTPUT: { "type": "string", "enum": [MCP_OUTPUT_INLINE, MCP_OUTPUT_ARTIFACT] },
                MCP_FIELD_FORMAT: { "type": "string", "enum": ["png"] },
                MCP_FIELD_RELATIVE_PATH: { "type": "string" },
                MCP_FIELD_INCLUDE_UI: { "type": "boolean" },
            },
            "additionalProperties": false,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_lists_enabled_tools_and_resources() {
        let availability = McpRegistryAvailability {
            control: true,
            capture: true,
        };

        let tools = tools_list_result(availability).to_string();
        let resources = resources_list_result(availability).to_string();

        assert!(tools.contains(MCP_ENQUEUE_CONTROL_TOOL));
        assert!(tools.contains(MCP_CAPTURE_SCREENSHOT_TOOL));
        assert!(resources.contains(MCP_CAPTURE_SCREENSHOT_RESOURCE));
        assert!(resources.contains(MCP_CAPTURE_LATEST_FRAME_RESOURCE));
    }

    #[test]
    fn registry_omits_unavailable_surfaces() {
        let availability = McpRegistryAvailability {
            control: false,
            capture: false,
        };

        assert_eq!(tools_list_result(availability), json!({ "tools": [] }));
        assert_eq!(
            resources_list_result(availability),
            json!({ "resources": [] })
        );
        assert_eq!(capture_mode_for_tool_name("unknown"), None);
        assert_eq!(capture_mode_for_resource_uri("unknown"), None);
    }
}
