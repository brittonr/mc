// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

//! Opt-in Stevenarella MCP transport, protocol, registry, and tool adapters.
//!
//! The root module is intentionally a facade. Pure JSON-RPC routing lives in
//! `protocol`; side-effecting stdio/TCP loops live in `transport`; MCP command
//! queues and frame-capture waits live in adapter modules.

pub mod auth;
pub mod capture_adapter;
pub mod control_queue;
pub mod dispatcher;
pub mod protocol;
pub mod registry;
pub mod transport;

pub use auth::{
    TcpAuth, DEFAULT_MCP_TOKEN_ENV, REASON_EMPTY_TOKEN_ENV_NAME, REASON_EMPTY_TOKEN_VALUE,
};
pub use capture_adapter::{McpCaptureToolError, McpCaptureTools};
pub use control_queue::{
    control_command_channel, McpCommandQueueError, McpCommandReceiver, McpCommandSender,
    QueuedMcpCommand, MAX_MCP_COMMANDS_PER_FRAME,
};
pub use dispatcher::{
    handle_jsonrpc_line, handle_jsonrpc_line_with_auth,
    handle_jsonrpc_line_with_auth_and_command_sender, handle_jsonrpc_line_with_auth_and_tools,
};
pub use protocol::{
    route_jsonrpc_line, McpProtocolAction, JSONRPC_INTERNAL_ERROR, JSONRPC_INVALID_REQUEST,
    JSONRPC_METHOD_NOT_FOUND, JSONRPC_PARSE_ERROR, JSONRPC_UNAUTHORIZED,
};
pub use registry::*;
pub use transport::{
    run_jsonrpc_lines, run_jsonrpc_lines_with_auth, run_jsonrpc_lines_with_auth_and_command_sender,
    run_jsonrpc_lines_with_auth_and_tools, run_jsonrpc_lines_with_command_sender,
    start_process_transport, start_process_transport_with_capture, start_transport_runtime,
    start_transport_with_stdio, start_transport_with_stdio_and_commands,
    start_transport_with_stdio_commands_and_capture, validate_process_transport_options,
    validate_transport_options, McpEndpoint, McpTransportError, McpTransportOptions,
    McpTransportRuntime, McpTransportStartError, StartedMcpEndpoint, ValidatedMcpTransport,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capture;
    use crate::control::{
        ControlCommand, ControlOutcome, ControlResponse, CONTROL_OUTCOME_APPLIED_NAME,
        CONTROL_OUTCOME_DEFERRED_NAME,
    };
    use serde_json::{json, Value};
    use std::io::{BufRead, BufReader, Write};
    use std::net::{SocketAddr, TcpStream};
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;
    use std::sync::{mpsc, Arc};
    use std::thread;
    use std::time::Duration;

    const LOOPBACK_LISTEN: &str = "127.0.0.1:4700";
    const LOOPBACK_EPHEMERAL_LISTEN: &str = "127.0.0.1:0";
    const IPV6_LOOPBACK_LISTEN: &str = "[::1]:4700";
    const NON_LOOPBACK_LISTEN: &str = "0.0.0.0:4700";
    const NON_LOOPBACK_EPHEMERAL_LISTEN: &str = "0.0.0.0:0";
    const MALFORMED_LISTEN: &str = "not-a-socket";
    const TOKEN_ENV_NAME: &str = "STEVENARELLA_TEST_MCP_TOKEN";
    const TOKEN_VALUE: &str = "secret-token";
    const QUEUE_TEST_TIMEOUT_MILLIS: u64 = 1;
    const QUEUE_TOOL_TEST_TIMEOUT_MILLIS: u64 = 250;
    const QUEUE_DRAIN_ATTEMPTS: usize = 1_000;
    const QUEUE_DRAIN_SLEEP_MILLIS: u64 = 1;
    const QUEUE_TEST_RESPONSE: &str = "main-thread-handler";
    const TEST_CAPTURE_WIDTH_PX: u32 = 2;
    const TEST_CAPTURE_HEIGHT_PX: u32 = 2;
    const TEST_CAPTURE_FRAME_ID: u64 = 42;
    const TEST_CAPTURE_SEQUENCE_ID: u64 = 0;
    const TEST_ARTIFACT_OUTPUT: &str = "artifact";

    fn synthetic_capture_frame(
        frame: capture::CaptureFrameContext,
    ) -> Result<capture::CapturedRgbaFrame, capture::CaptureReadbackError> {
        let byte_len = capture::rgba_buffer_len(frame.width_px, frame.height_px)?;
        Ok(capture::CapturedRgbaFrame {
            width_px: frame.width_px,
            height_px: frame.height_px,
            frame_id: frame.frame_id,
            rgba_top_left: vec![0; byte_len],
        })
    }

    fn test_capture_frame_context() -> capture::CaptureFrameContext {
        capture::CaptureFrameContext {
            width_px: TEST_CAPTURE_WIDTH_PX,
            height_px: TEST_CAPTURE_HEIGHT_PX,
            frame_id: TEST_CAPTURE_FRAME_ID,
        }
    }

    fn test_capture_tools(
        policy: capture::CapturePolicy,
    ) -> (McpCaptureTools, capture::CaptureRequestReceiver) {
        let (sender, receiver) = capture::capture_request_channel();
        let tools = McpCaptureTools::new(
            sender,
            policy,
            Arc::new(AtomicU64::new(capture::CAPTURE_SEQUENCE_INITIAL)),
        )
        .with_response_timeout(Duration::from_millis(QUEUE_TOOL_TEST_TIMEOUT_MILLIS));
        (tools, receiver)
    }

    fn drain_until_capture(
        receiver: &capture::CaptureRequestReceiver,
        policy: &capture::CapturePolicy,
    ) -> usize {
        for _attempt in 0..QUEUE_DRAIN_ATTEMPTS {
            let drained = try_drain_capture_once(receiver, policy);
            if drained > 0 {
                return drained;
            }
            thread::sleep(Duration::from_millis(QUEUE_DRAIN_SLEEP_MILLIS));
        }
        0
    }

    fn try_drain_capture_once(
        receiver: &capture::CaptureRequestReceiver,
        policy: &capture::CapturePolicy,
    ) -> usize {
        receiver.service_pending_one_shot_with_readback(
            policy,
            test_capture_frame_context(),
            synthetic_capture_frame,
        )
    }

    fn unique_test_capture_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "stevenarella-mcp-capture-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    #[test]
    fn stdio_transport_is_accepted_and_requires_clean_stdout() {
        let options = McpTransportOptions::from_cli(true, None, None);

        let validated = validate_transport_options(&options, |_| None).unwrap();

        assert_eq!(validated.endpoints, vec![McpEndpoint::Stdio]);
        assert!(validated.stdout_must_remain_clean);
    }

    #[test]
    fn loopback_tcp_transport_is_accepted_without_token() {
        let options = McpTransportOptions::from_cli(false, Some(LOOPBACK_LISTEN.to_owned()), None);

        let validated = validate_transport_options(&options, |_| None).unwrap();

        assert_eq!(
            validated.endpoints,
            vec![McpEndpoint::Tcp {
                bind_addr: LOOPBACK_LISTEN.parse().unwrap(),
                auth: TcpAuth::NotRequiredForLoopback,
            }]
        );
        assert!(!validated.stdout_must_remain_clean);
    }

    #[test]
    fn ipv6_loopback_tcp_transport_is_accepted_without_token() {
        let options =
            McpTransportOptions::from_cli(false, Some(IPV6_LOOPBACK_LISTEN.to_owned()), None);

        let validated = validate_transport_options(&options, |_| None).unwrap();

        assert_eq!(
            validated.endpoints,
            vec![McpEndpoint::Tcp {
                bind_addr: IPV6_LOOPBACK_LISTEN.parse().unwrap(),
                auth: TcpAuth::NotRequiredForLoopback,
            }]
        );
    }

    #[test]
    fn non_loopback_tcp_transport_is_rejected_without_token_env() {
        let options =
            McpTransportOptions::from_cli(false, Some(NON_LOOPBACK_LISTEN.to_owned()), None);

        assert_eq!(
            validate_transport_options(&options, |_| None),
            Err(McpTransportError::MissingTokenEnvForNonLoopback {
                bind_addr: NON_LOOPBACK_LISTEN.parse().unwrap(),
            })
        );
    }

    #[test]
    fn non_loopback_tcp_transport_is_rejected_with_empty_token_env_name() {
        let options = McpTransportOptions::from_cli(
            false,
            Some(NON_LOOPBACK_LISTEN.to_owned()),
            Some("  ".to_owned()),
        );

        assert_eq!(
            validate_transport_options(&options, |_| Some(TOKEN_VALUE.to_owned())),
            Err(McpTransportError::InvalidTokenEnvName {
                reason: REASON_EMPTY_TOKEN_ENV_NAME,
            })
        );
    }

    #[test]
    fn non_loopback_tcp_transport_is_rejected_with_missing_token_value() {
        let options = McpTransportOptions::from_cli(
            false,
            Some(NON_LOOPBACK_LISTEN.to_owned()),
            Some(TOKEN_ENV_NAME.to_owned()),
        );

        assert_eq!(
            validate_transport_options(&options, |_| None),
            Err(McpTransportError::MissingTokenValue {
                token_env: TOKEN_ENV_NAME.to_owned(),
            })
        );
    }

    #[test]
    fn non_loopback_tcp_transport_is_rejected_with_empty_token_value() {
        let options = McpTransportOptions::from_cli(
            false,
            Some(NON_LOOPBACK_LISTEN.to_owned()),
            Some(TOKEN_ENV_NAME.to_owned()),
        );

        assert_eq!(
            validate_transport_options(&options, |_| Some("  ".to_owned())),
            Err(McpTransportError::InvalidTokenValue {
                token_env: TOKEN_ENV_NAME.to_owned(),
                reason: REASON_EMPTY_TOKEN_VALUE,
            })
        );
    }

    #[test]
    fn non_loopback_tcp_transport_is_accepted_with_token_env_and_value() {
        let options = McpTransportOptions::from_cli(
            false,
            Some(NON_LOOPBACK_LISTEN.to_owned()),
            Some(format!(" {TOKEN_ENV_NAME} ")),
        );

        let validated = validate_transport_options(&options, |name| {
            assert_eq!(name, TOKEN_ENV_NAME);
            Some(TOKEN_VALUE.to_owned())
        })
        .unwrap();

        assert_eq!(
            validated.endpoints,
            vec![McpEndpoint::Tcp {
                bind_addr: NON_LOOPBACK_LISTEN.parse().unwrap(),
                auth: TcpAuth::TokenEnv {
                    name: TOKEN_ENV_NAME.to_owned(),
                    token: TOKEN_VALUE.to_owned(),
                },
            }]
        );
    }

    #[test]
    fn malformed_listen_address_is_rejected() {
        let options = McpTransportOptions::from_cli(false, Some(MALFORMED_LISTEN.to_owned()), None);

        assert_eq!(
            validate_transport_options(&options, |_| None),
            Err(McpTransportError::MalformedListenAddress(
                MALFORMED_LISTEN.to_owned()
            ))
        );
    }

    #[test]
    fn stdio_transport_runs_jsonrpc_line_loop() {
        let input = br#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}
"#;
        let mut output = Vec::new();

        run_jsonrpc_lines(BufReader::new(&input[..]), &mut output).unwrap();
        let response = String::from_utf8(output).unwrap();

        assert!(response.contains(r#""id":1"#));
        assert!(response.contains(r#""tools":[]"#));
    }

    #[test]
    fn stdio_transport_runtime_starts_line_loop_thread() {
        let validated = ValidatedMcpTransport {
            endpoints: vec![McpEndpoint::Stdio],
            stdout_must_remain_clean: true,
        };
        let input = std::io::Cursor::new(
            br#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}
"#
            .to_vec(),
        );
        let output = Vec::new();

        let runtime = start_transport_with_stdio(validated, input, output).unwrap();

        assert_eq!(runtime.endpoints, vec![StartedMcpEndpoint::Stdio]);
        assert!(runtime.stdout_must_remain_clean);
        assert_eq!(runtime.join_handle_count(), 1);
        assert!(!runtime.command_sender_configured());
    }

    #[test]
    fn stdio_transport_runtime_keeps_command_sender_for_worker_threads() {
        let validated = ValidatedMcpTransport {
            endpoints: vec![McpEndpoint::Stdio],
            stdout_must_remain_clean: true,
        };
        let input = std::io::Cursor::new(Vec::new());
        let output = Vec::new();
        let (sender, _receiver) = control_command_channel();

        let runtime =
            start_transport_with_stdio_and_commands(validated, input, output, Some(sender))
                .unwrap();

        assert_eq!(runtime.endpoints, vec![StartedMcpEndpoint::Stdio]);
        assert!(runtime.command_sender_configured());
    }

    #[test]
    fn tcp_transport_binds_loopback_and_serves_jsonrpc() {
        let options =
            McpTransportOptions::from_cli(false, Some(LOOPBACK_EPHEMERAL_LISTEN.to_owned()), None);
        let validated = validate_transport_options(&options, |_| None).unwrap();

        let runtime = start_transport_runtime(validated).unwrap();
        let StartedMcpEndpoint::Tcp { local_addr } = runtime.endpoints[0] else {
            panic!("expected tcp endpoint");
        };

        let mut stream = TcpStream::connect(local_addr).unwrap();
        stream
            .write_all(
                br#"{"jsonrpc":"2.0","id":7,"method":"ping"}
"#,
            )
            .unwrap();
        let mut response = String::new();
        BufReader::new(stream).read_line(&mut response).unwrap();

        assert!(response.contains(r#""id":7"#));
        assert!(response.contains(r#""result":{}"#));
    }

    #[test]
    fn non_loopback_tcp_transport_requires_token_per_request() {
        let options = McpTransportOptions::from_cli(
            false,
            Some(NON_LOOPBACK_EPHEMERAL_LISTEN.to_owned()),
            Some(TOKEN_ENV_NAME.to_owned()),
        );
        let validated =
            validate_transport_options(&options, |_| Some(TOKEN_VALUE.to_owned())).unwrap();

        let runtime = start_transport_runtime(validated).unwrap();
        let StartedMcpEndpoint::Tcp { local_addr } = runtime.endpoints[0] else {
            panic!("expected tcp endpoint");
        };
        let connect_addr = SocketAddr::from(([127, 0, 0, 1], local_addr.port()));

        let unauthenticated = request_tcp_jsonrpc(
            connect_addr,
            br#"{"jsonrpc":"2.0","id":8,"method":"ping"}
"#,
        );
        assert!(unauthenticated.contains(&JSONRPC_UNAUTHORIZED.to_string()));
        assert!(unauthenticated.contains(r#""unauthorized"#));

        let authenticated = request_tcp_jsonrpc(
            connect_addr,
            br#"{"jsonrpc":"2.0","id":9,"method":"ping","params":{"token":"secret-token"}}
"#,
        );
        assert!(authenticated.contains(r#""id":9"#));
        assert!(authenticated.contains(r#""result":{}"#));
    }

    #[test]
    fn jsonrpc_handler_requires_matching_auth_token_when_configured() {
        let unauthorized = handle_jsonrpc_line_with_auth(
            r#"{"jsonrpc":"2.0","id":3,"method":"ping","params":{"token":"wrong"}}"#,
            Some(TOKEN_VALUE),
        )
        .unwrap();
        assert!(unauthorized.contains(&JSONRPC_UNAUTHORIZED.to_string()));

        let authorized = handle_jsonrpc_line_with_auth(
            r#"{"jsonrpc":"2.0","id":4,"method":"ping","params":{"token":"secret-token"}}"#,
            Some(TOKEN_VALUE),
        )
        .unwrap();
        assert!(authorized.contains(r#""id":4"#));
        assert!(authorized.contains(r#""result":{}"#));
    }

    #[test]
    fn jsonrpc_handler_supports_initialize_and_lists_empty_tools() {
        let initialize = handle_jsonrpc_line(
            r#"{"jsonrpc":"2.0","id":"init","method":"initialize","params":{}}"#,
        )
        .unwrap();
        assert!(initialize.contains(r#""serverInfo"#));
        assert!(initialize.contains(r#""stevenarella"#));

        let tools =
            handle_jsonrpc_line(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#).unwrap();
        assert!(tools.contains(r#""tools":[]"#));
    }

    #[test]
    fn jsonrpc_handler_rejects_malformed_json() {
        let response = handle_jsonrpc_line("not-json").unwrap();

        assert!(response.contains(&JSONRPC_PARSE_ERROR.to_string()));
    }

    #[test]
    fn jsonrpc_tools_list_includes_queue_tool_when_sender_is_configured() {
        let (sender, _receiver) = control_command_channel();

        let response = handle_jsonrpc_line_with_auth_and_command_sender(
            r#"{"jsonrpc":"2.0","id":10,"method":"tools/list"}"#,
            None,
            Some(&sender),
        )
        .unwrap();

        assert!(response.contains(MCP_ENQUEUE_CONTROL_TOOL));
    }

    #[test]
    fn jsonrpc_tools_call_enqueues_control_command_for_main_thread_drain() {
        let (sender, receiver) = control_command_channel();
        let sender =
            sender.with_response_timeout(Duration::from_millis(QUEUE_TOOL_TEST_TIMEOUT_MILLIS));
        let request = json!({
            "jsonrpc": "2.0",
            "id": 11,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_ENQUEUE_CONTROL_TOOL,
                "arguments": {
                    "command": { "action": "status" },
                },
            },
        })
        .to_string();

        let worker = thread::spawn(move || {
            handle_jsonrpc_line_with_auth_and_command_sender(&request, None, Some(&sender))
                .expect("tools/call should return a response")
        });
        let drained = drain_until_command(&receiver, |command| {
            assert_eq!(command, ControlCommand::Status);
            ControlResponse {
                outcome: ControlOutcome::Applied,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        });

        assert_eq!(drained, 1);
        let response: Value = serde_json::from_str(&worker.join().unwrap()).unwrap();
        let text = response["result"]["content"][0]["text"].as_str().unwrap();
        let payload: Value = serde_json::from_str(text).unwrap();
        assert_eq!(payload["outcome"], CONTROL_OUTCOME_APPLIED_NAME);
        assert_eq!(payload["message"], QUEUE_TEST_RESPONSE);
    }

    #[test]
    fn jsonrpc_tools_call_enqueues_capture_command_for_main_thread_drain() {
        let (sender, receiver) = control_command_channel();
        let sender =
            sender.with_response_timeout(Duration::from_millis(QUEUE_TOOL_TEST_TIMEOUT_MILLIS));
        let request = json!({
            "jsonrpc": "2.0",
            "id": 12,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_ENQUEUE_CONTROL_TOOL,
                "arguments": {
                    "command": { "action": "capture_screenshot" },
                },
            },
        })
        .to_string();

        let worker = thread::spawn(move || {
            handle_jsonrpc_line_with_auth_and_command_sender(&request, None, Some(&sender))
                .expect("tools/call should return a response")
        });
        let drained = drain_until_command(&receiver, |command| {
            assert_eq!(command, ControlCommand::CaptureScreenshot);
            ControlResponse {
                outcome: ControlOutcome::Deferred,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        });

        assert_eq!(drained, 1);
        let response: Value = serde_json::from_str(&worker.join().unwrap()).unwrap();
        let text = response["result"]["content"][0]["text"].as_str().unwrap();
        let payload: Value = serde_json::from_str(text).unwrap();
        assert_eq!(payload["outcome"], CONTROL_OUTCOME_DEFERRED_NAME);
        assert_eq!(payload["message"], QUEUE_TEST_RESPONSE);
    }

    #[test]
    fn tools_and_resources_list_include_capture_when_configured() {
        let policy = capture::CapturePolicy::memory();
        let (tools, _receiver) = test_capture_tools(policy);

        let tools_response = handle_jsonrpc_line_with_auth_and_tools(
            r#"{"jsonrpc":"2.0","id":13,"method":"tools/list"}"#,
            None,
            None,
            Some(&tools),
        )
        .unwrap();
        let resources_response = handle_jsonrpc_line_with_auth_and_tools(
            r#"{"jsonrpc":"2.0","id":14,"method":"resources/list"}"#,
            None,
            None,
            Some(&tools),
        )
        .unwrap();

        assert!(tools_response.contains(MCP_CAPTURE_SCREENSHOT_TOOL));
        assert!(tools_response.contains(MCP_CAPTURE_LATEST_FRAME_TOOL));
        assert!(resources_response.contains(MCP_CAPTURE_SCREENSHOT_RESOURCE));
        assert!(resources_response.contains(MCP_CAPTURE_LATEST_FRAME_RESOURCE));
    }

    #[test]
    fn jsonrpc_capture_tool_returns_inline_image_and_metadata() {
        let policy = capture::CapturePolicy::memory();
        let (tools, receiver) = test_capture_tools(policy.clone());
        let request = json!({
            "jsonrpc": "2.0",
            "id": 15,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_CAPTURE_SCREENSHOT_TOOL,
                "arguments": {},
            },
        })
        .to_string();

        let worker = thread::spawn(move || {
            handle_jsonrpc_line_with_auth_and_tools(&request, None, None, Some(&tools))
                .expect("capture tool should return a response")
        });
        let drained = drain_until_capture(&receiver, &policy);

        assert_eq!(drained, 1);
        let response: Value = serde_json::from_str(&worker.join().unwrap()).unwrap();
        let content = response["result"]["content"].as_array().unwrap();
        assert_eq!(content[0]["type"], MCP_CONTENT_TYPE_IMAGE);
        assert_eq!(content[0]["mimeType"], MCP_MIME_IMAGE_PNG);
        assert!(content[0]["data"].as_str().unwrap().len() > 0);
        let metadata: Value = serde_json::from_str(content[1]["text"].as_str().unwrap()).unwrap();
        assert_eq!(metadata["mode"], "screenshot");
        assert_eq!(metadata["output"], MCP_OUTPUT_INLINE);
        assert_eq!(metadata["sequence_id"], TEST_CAPTURE_SEQUENCE_ID);
        assert_eq!(metadata["width_px"], TEST_CAPTURE_WIDTH_PX);
        assert_eq!(metadata["height_px"], TEST_CAPTURE_HEIGHT_PX);
        assert!(metadata["blake3_digest"].as_str().unwrap().len() == capture::BLAKE3_HEX_LENGTH);
    }

    #[test]
    fn jsonrpc_capture_tool_returns_artifact_path_and_digest() {
        let capture_dir = unique_test_capture_dir("artifact-tool");
        let policy = capture::CapturePolicy::local(&capture_dir);
        let (tools, receiver) = test_capture_tools(policy.clone());
        let request = json!({
            "jsonrpc": "2.0",
            "id": 16,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_CAPTURE_SCREENSHOT_TOOL,
                "arguments": { "output": TEST_ARTIFACT_OUTPUT },
            },
        })
        .to_string();

        let worker = thread::spawn(move || {
            handle_jsonrpc_line_with_auth_and_tools(&request, None, None, Some(&tools))
                .expect("capture tool should return a response")
        });
        let drained = drain_until_capture(&receiver, &policy);

        assert_eq!(drained, 1);
        let response: Value = serde_json::from_str(&worker.join().unwrap()).unwrap();
        let content = response["result"]["content"].as_array().unwrap();
        assert_eq!(content.len(), 1);
        let metadata: Value = serde_json::from_str(content[0]["text"].as_str().unwrap()).unwrap();
        let relative_path = metadata["relative_path"].as_str().unwrap();
        assert_eq!(metadata["output"], MCP_OUTPUT_ARTIFACT);
        assert_eq!(metadata["sequence_id"], TEST_CAPTURE_SEQUENCE_ID);
        assert!(capture_dir.join(relative_path).exists());
        assert!(metadata["blake3_digest"].as_str().unwrap().len() == capture::BLAKE3_HEX_LENGTH);
        let _ = std::fs::remove_dir_all(capture_dir);
    }

    #[test]
    fn jsonrpc_capture_resource_read_returns_inline_blob() {
        let policy = capture::CapturePolicy::memory();
        let (tools, receiver) = test_capture_tools(policy.clone());
        let request = json!({
            "jsonrpc": "2.0",
            "id": 17,
            "method": MCP_RESOURCES_READ_METHOD,
            "params": { "uri": MCP_CAPTURE_LATEST_FRAME_RESOURCE },
        })
        .to_string();

        let worker = thread::spawn(move || {
            handle_jsonrpc_line_with_auth_and_tools(&request, None, None, Some(&tools))
                .expect("resource read should return a response")
        });
        let drained = drain_until_capture(&receiver, &policy);

        assert_eq!(drained, 1);
        let response: Value = serde_json::from_str(&worker.join().unwrap()).unwrap();
        let contents = response["result"]["contents"].as_array().unwrap();
        assert_eq!(contents[0]["uri"], MCP_CAPTURE_LATEST_FRAME_RESOURCE);
        assert_eq!(contents[0]["mimeType"], MCP_MIME_IMAGE_PNG);
        assert!(contents[0]["blob"].as_str().unwrap().len() > 0);
        let metadata: Value = serde_json::from_str(contents[1]["text"].as_str().unwrap()).unwrap();
        assert_eq!(metadata["mode"], "latest_frame");
        assert_eq!(metadata["output"], MCP_OUTPUT_INLINE);
    }

    #[test]
    fn jsonrpc_capture_tool_rejects_invalid_format_before_enqueue() {
        let policy = capture::CapturePolicy::memory();
        let (tools, receiver) = test_capture_tools(policy);
        let request = json!({
            "jsonrpc": "2.0",
            "id": 18,
            "method": MCP_TOOLS_CALL_METHOD,
            "params": {
                "name": MCP_CAPTURE_SCREENSHOT_TOOL,
                "arguments": { "format": "webp" },
            },
        })
        .to_string();

        let response = handle_jsonrpc_line_with_auth_and_tools(&request, None, None, Some(&tools))
            .expect("invalid format should return response");

        assert!(response.contains(&JSONRPC_INVALID_REQUEST.to_string()));
        assert_eq!(
            try_drain_capture_once(&receiver, &capture::CapturePolicy::memory()),
            0
        );
    }

    #[test]
    fn command_queue_drains_pending_command_with_main_thread_handler() {
        let (sender, receiver) = control_command_channel();
        let response_receiver = sender
            .enqueue_deferred(ControlCommand::Status)
            .expect("queue should accept command while receiver is alive");

        let drained = receiver.drain_pending_with_handler(|command| {
            assert_eq!(command, ControlCommand::Status);
            ControlResponse {
                outcome: crate::control::ControlOutcome::Applied,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        });

        assert_eq!(drained, 1);
        assert_eq!(
            response_receiver.recv().unwrap(),
            ControlResponse {
                outcome: crate::control::ControlOutcome::Applied,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        );
    }

    #[test]
    fn command_queue_respects_per_frame_drain_limit() {
        let (sender, receiver) = control_command_channel();
        let first_response = sender.enqueue_deferred(ControlCommand::Status).unwrap();
        let second_response = sender.enqueue_deferred(ControlCommand::Disconnect).unwrap();

        let first_drained = receiver.drain_pending_with_limit(1, |command| {
            assert_eq!(command, ControlCommand::Status);
            ControlResponse {
                outcome: crate::control::ControlOutcome::Applied,
                message: None,
            }
        });

        assert_eq!(first_drained, 1);
        assert_eq!(
            first_response.recv().unwrap().outcome,
            crate::control::ControlOutcome::Applied
        );
        assert!(matches!(
            second_response.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));

        let second_drained = receiver.drain_pending_with_handler(|command| {
            assert_eq!(command, ControlCommand::Disconnect);
            ControlResponse {
                outcome: crate::control::ControlOutcome::Rejected,
                message: None,
            }
        });

        assert_eq!(second_drained, 1);
        assert_eq!(
            second_response.recv().unwrap().outcome,
            crate::control::ControlOutcome::Rejected
        );
    }

    #[test]
    fn command_queue_rejects_enqueue_after_receiver_drop() {
        let (sender, receiver) = control_command_channel();
        drop(receiver);

        assert!(matches!(
            sender.enqueue_deferred(ControlCommand::Status),
            Err(McpCommandQueueError::QueueClosed)
        ));
    }

    #[test]
    fn command_queue_reports_timeout_when_main_thread_does_not_drain() {
        let (sender, _receiver) = control_command_channel();
        let sender = sender.with_response_timeout(Duration::from_millis(QUEUE_TEST_TIMEOUT_MILLIS));

        assert_eq!(
            sender.enqueue(ControlCommand::Status),
            Err(McpCommandQueueError::ResponseTimedOut)
        );
    }

    fn drain_until_command<F>(receiver: &McpCommandReceiver, mut handler: F) -> usize
    where
        F: FnMut(ControlCommand) -> ControlResponse,
    {
        for _attempt in 0..QUEUE_DRAIN_ATTEMPTS {
            let drained = receiver.drain_pending_with_handler(&mut handler);
            if drained > 0 {
                return drained;
            }
            thread::sleep(Duration::from_millis(QUEUE_DRAIN_SLEEP_MILLIS));
        }
        0
    }

    fn request_tcp_jsonrpc(connect_addr: SocketAddr, request: &[u8]) -> String {
        let mut stream = TcpStream::connect(connect_addr).unwrap();
        stream.write_all(request).unwrap();
        let mut response = String::new();
        BufReader::new(stream).read_line(&mut response).unwrap();
        response
    }
}
