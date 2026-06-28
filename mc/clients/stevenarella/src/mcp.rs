// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::capture::{
    self, CaptureFormat, CaptureMode, CaptureOutput, CapturePolicy, CaptureQueueError,
    CaptureRequest, CaptureRequestSender, ServicedCapture,
};
use crate::control::{ControlCommand, ControlResponse};
use serde_json::{json, Value};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub const DEFAULT_MCP_TOKEN_ENV: &str = "STEVENARELLA_MCP_TOKEN";
pub const MAX_MCP_COMMANDS_PER_FRAME: usize = 64;

const COMMAND_RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;
const CAPTURE_RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;
const REASON_EMPTY_TOKEN_ENV_NAME: &str = "empty_token_env_name";
const REASON_EMPTY_TOKEN_VALUE: &str = "empty_token_value";
const TCP_ACCEPT_IDLE_SLEEP_MILLIS: u64 = 10;
const JSONRPC_PARSE_ERROR: i64 = -32700;
const JSONRPC_INVALID_REQUEST: i64 = -32600;
const JSONRPC_METHOD_NOT_FOUND: i64 = -32601;
const JSONRPC_INTERNAL_ERROR: i64 = -32603;
const JSONRPC_UNAUTHORIZED: i64 = -32001;
const MCP_TOOLS_CALL_METHOD: &str = "tools/call";
const MCP_TOOLS_LIST_METHOD: &str = "tools/list";
const MCP_RESOURCES_LIST_METHOD: &str = "resources/list";
const MCP_RESOURCES_READ_METHOD: &str = "resources/read";
const MCP_ENQUEUE_CONTROL_TOOL: &str = "stevenarella.enqueue_control";
const MCP_CAPTURE_SCREENSHOT_TOOL: &str = "stevenarella.capture_screenshot";
const MCP_CAPTURE_LATEST_FRAME_TOOL: &str = "stevenarella.capture_latest_frame";
const MCP_CAPTURE_SCREENSHOT_RESOURCE: &str = "stevenarella://capture/screenshot";
const MCP_CAPTURE_LATEST_FRAME_RESOURCE: &str = "stevenarella://capture/latest-frame";
const MCP_CONTENT_TYPE_TEXT: &str = "text";
const MCP_CONTENT_TYPE_IMAGE: &str = "image";
const MCP_MIME_APPLICATION_JSON: &str = "application/json";
const MCP_MIME_IMAGE_PNG: &str = "image/png";
const MCP_FIELD_ARGUMENTS: &str = "arguments";
const MCP_FIELD_COMMAND: &str = "command";
const MCP_FIELD_FORMAT: &str = "format";
const MCP_FIELD_INCLUDE_UI: &str = "include_ui";
const MCP_FIELD_NAME: &str = "name";
const MCP_FIELD_OUTPUT: &str = "output";
const MCP_FIELD_RELATIVE_PATH: &str = "relative_path";
const MCP_FIELD_URI: &str = "uri";
const MCP_OUTPUT_ARTIFACT: &str = "artifact";
const MCP_OUTPUT_INLINE: &str = "inline";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct McpTransportOptions {
    pub stdio: bool,
    pub listen: Option<String>,
    pub token_env: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedMcpTransport {
    pub endpoints: Vec<McpEndpoint>,
    pub stdout_must_remain_clean: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpEndpoint {
    Stdio,
    Tcp {
        bind_addr: SocketAddr,
        auth: TcpAuth,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpAuth {
    NotRequiredForLoopback,
    TokenEnv { name: String, token: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartedMcpEndpoint {
    Stdio,
    Tcp { local_addr: SocketAddr },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpTransportError {
    MalformedListenAddress(String),
    MissingTokenEnvForNonLoopback {
        bind_addr: SocketAddr,
    },
    InvalidTokenEnvName {
        reason: &'static str,
    },
    MissingTokenValue {
        token_env: String,
    },
    InvalidTokenValue {
        token_env: String,
        reason: &'static str,
    },
}

#[derive(Debug)]
pub enum McpTransportStartError {
    Io(io::Error),
}

pub struct McpTransportRuntime {
    pub endpoints: Vec<StartedMcpEndpoint>,
    pub stdout_must_remain_clean: bool,
    command_sender: Option<McpCommandSender>,
    shutdown_flags: Vec<Arc<AtomicBool>>,
    join_handles: Vec<JoinHandle<()>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpCommandQueueError {
    QueueClosed,
    ResponseDropped,
    ResponseTimedOut,
}

#[derive(Clone)]
pub struct McpCommandSender {
    sender: mpsc::Sender<QueuedMcpCommand>,
    response_timeout: Duration,
}

pub struct McpCommandReceiver {
    receiver: mpsc::Receiver<QueuedMcpCommand>,
}

pub struct QueuedMcpCommand {
    command: ControlCommand,
    response_sender: mpsc::Sender<ControlResponse>,
}

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
struct McpCaptureToolPayload {
    metadata: Value,
    inline_png_base64: Option<String>,
}

impl Drop for McpTransportRuntime {
    fn drop(&mut self) {
        for shutdown_flag in &self.shutdown_flags {
            shutdown_flag.store(true, Ordering::Release);
        }
    }
}

impl From<io::Error> for McpTransportStartError {
    fn from(err: io::Error) -> Self {
        McpTransportStartError::Io(err)
    }
}

impl McpTransportOptions {
    pub fn from_cli(stdio: bool, listen: Option<String>, token_env: Option<String>) -> Self {
        McpTransportOptions {
            stdio,
            listen,
            token_env,
        }
    }

    pub fn has_transport(&self) -> bool {
        self.stdio || self.listen.is_some()
    }
}

impl McpTransportRuntime {
    pub fn join_handle_count(&self) -> usize {
        self.join_handles.len()
    }

    pub fn command_sender_configured(&self) -> bool {
        self.command_sender.is_some()
    }
}

pub fn control_command_channel() -> (McpCommandSender, McpCommandReceiver) {
    let (sender, receiver) = mpsc::channel();
    (
        McpCommandSender {
            sender,
            response_timeout: Duration::from_millis(COMMAND_RESPONSE_TIMEOUT_MILLIS),
        },
        McpCommandReceiver { receiver },
    )
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

    fn capture_one_shot_from_arguments(
        &self,
        mode: CaptureMode,
        arguments: Option<&Value>,
    ) -> Result<McpCaptureToolPayload, McpCaptureToolError> {
        let sequence_id = self.next_sequence_id.fetch_add(1, Ordering::AcqRel);
        let request = capture_request_from_arguments(arguments, mode, &self.policy, sequence_id)?;
        self.capture_one_shot(request)
    }

    fn capture_one_shot_inline(
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

impl McpCommandSender {
    pub fn enqueue_deferred(
        &self,
        command: ControlCommand,
    ) -> Result<mpsc::Receiver<ControlResponse>, McpCommandQueueError> {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(QueuedMcpCommand {
                command,
                response_sender,
            })
            .map_err(|_| McpCommandQueueError::QueueClosed)?;
        Ok(response_receiver)
    }

    pub fn enqueue(
        &self,
        command: ControlCommand,
    ) -> Result<ControlResponse, McpCommandQueueError> {
        let response_receiver = self.enqueue_deferred(command)?;
        match response_receiver.recv_timeout(self.response_timeout) {
            Ok(response) => Ok(response),
            Err(mpsc::RecvTimeoutError::Timeout) => Err(McpCommandQueueError::ResponseTimedOut),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(McpCommandQueueError::ResponseDropped),
        }
    }

    pub fn with_response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = response_timeout;
        self
    }
}

impl McpCommandReceiver {
    pub fn drain_pending_with_handler<F>(&self, handler: F) -> usize
    where
        F: FnMut(ControlCommand) -> ControlResponse,
    {
        self.drain_pending_with_limit(MAX_MCP_COMMANDS_PER_FRAME, handler)
    }

    pub fn drain_pending_with_limit<F>(&self, limit: usize, mut handler: F) -> usize
    where
        F: FnMut(ControlCommand) -> ControlResponse,
    {
        let mut drained = 0;
        while drained < limit {
            let queued = match self.receiver.try_recv() {
                Ok(queued) => queued,
                Err(mpsc::TryRecvError::Empty) | Err(mpsc::TryRecvError::Disconnected) => break,
            };
            let response = handler(queued.command);
            let _ = queued.response_sender.send(response);
            drained += 1;
        }
        drained
    }
}

pub fn validate_process_transport_options(
    options: &McpTransportOptions,
) -> Result<ValidatedMcpTransport, McpTransportError> {
    validate_transport_options(options, |name| std::env::var(name).ok())
}

pub fn validate_transport_options<F>(
    options: &McpTransportOptions,
    token_lookup: F,
) -> Result<ValidatedMcpTransport, McpTransportError>
where
    F: Fn(&str) -> Option<String>,
{
    let mut endpoints = Vec::new();
    let mut stdout_must_remain_clean = false;

    if options.stdio {
        endpoints.push(McpEndpoint::Stdio);
        stdout_must_remain_clean = true;
    }

    if let Some(listen) = options.listen.as_deref() {
        endpoints.push(validate_tcp_endpoint(
            listen,
            options.token_env.as_deref(),
            &token_lookup,
        )?);
    }

    Ok(ValidatedMcpTransport {
        endpoints,
        stdout_must_remain_clean,
    })
}

pub fn start_process_transport(
    validated: ValidatedMcpTransport,
    command_sender: Option<McpCommandSender>,
) -> Result<McpTransportRuntime, McpTransportStartError> {
    start_transport_with_stdio_commands_and_capture(
        validated,
        io::stdin(),
        io::stdout(),
        command_sender,
        None,
    )
}

pub fn start_process_transport_with_capture(
    validated: ValidatedMcpTransport,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) -> Result<McpTransportRuntime, McpTransportStartError> {
    start_transport_with_stdio_commands_and_capture(
        validated,
        io::stdin(),
        io::stdout(),
        command_sender,
        capture_tools,
    )
}

pub fn start_transport_runtime(
    validated: ValidatedMcpTransport,
) -> Result<McpTransportRuntime, McpTransportStartError> {
    start_transport_runtime_inner(validated, None::<(io::Empty, io::Sink)>, None, None)
}

pub fn start_transport_with_stdio<R, W>(
    validated: ValidatedMcpTransport,
    reader: R,
    writer: W,
) -> Result<McpTransportRuntime, McpTransportStartError>
where
    R: io::Read + Send + 'static,
    W: io::Write + Send + 'static,
{
    start_transport_with_stdio_and_commands(validated, reader, writer, None)
}

pub fn start_transport_with_stdio_and_commands<R, W>(
    validated: ValidatedMcpTransport,
    reader: R,
    writer: W,
    command_sender: Option<McpCommandSender>,
) -> Result<McpTransportRuntime, McpTransportStartError>
where
    R: io::Read + Send + 'static,
    W: io::Write + Send + 'static,
{
    start_transport_with_stdio_commands_and_capture(validated, reader, writer, command_sender, None)
}

pub fn start_transport_with_stdio_commands_and_capture<R, W>(
    validated: ValidatedMcpTransport,
    reader: R,
    writer: W,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) -> Result<McpTransportRuntime, McpTransportStartError>
where
    R: io::Read + Send + 'static,
    W: io::Write + Send + 'static,
{
    start_transport_runtime_inner(
        validated,
        Some((reader, writer)),
        command_sender,
        capture_tools,
    )
}

pub fn run_jsonrpc_lines<R, W>(reader: R, writer: W) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    run_jsonrpc_lines_with_auth(reader, writer, None)
}

pub fn run_jsonrpc_lines_with_auth<R, W>(
    reader: R,
    writer: W,
    required_token: Option<&str>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    run_jsonrpc_lines_with_auth_and_command_sender(reader, writer, required_token, None)
}

pub fn run_jsonrpc_lines_with_command_sender<R, W>(
    reader: R,
    writer: W,
    command_sender: Option<McpCommandSender>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    run_jsonrpc_lines_with_auth_and_command_sender(reader, writer, None, command_sender)
}

pub fn run_jsonrpc_lines_with_auth_and_command_sender<R, W>(
    reader: R,
    writer: W,
    required_token: Option<&str>,
    command_sender: Option<McpCommandSender>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    run_jsonrpc_lines_with_auth_and_tools(reader, writer, required_token, command_sender, None)
}

pub fn run_jsonrpc_lines_with_auth_and_tools<R, W>(
    mut reader: R,
    mut writer: W,
    required_token: Option<&str>,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut line = String::new();
    loop {
        line.clear();
        let read = reader.read_line(&mut line)?;
        if read == 0 {
            return Ok(());
        }
        if let Some(response) = handle_jsonrpc_line_with_auth_and_tools(
            line.trim_end_matches(['\r', '\n']),
            required_token,
            command_sender.as_ref(),
            capture_tools.as_ref(),
        ) {
            writeln!(writer, "{}", response)?;
            writer.flush()?;
        }
    }
}

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
    let value = match serde_json::from_str::<Value>(line) {
        Ok(value) => value,
        Err(_) => {
            return Some(jsonrpc_error(
                Value::Null,
                JSONRPC_PARSE_ERROR,
                "parse error",
            ))
        }
    };

    let Some(object) = value.as_object() else {
        return Some(jsonrpc_error(
            Value::Null,
            JSONRPC_INVALID_REQUEST,
            "request must be an object",
        ));
    };

    let id = object.get("id").cloned();
    let method = object.get("method").and_then(Value::as_str);
    let Some(method) = method else {
        return id.map(|id| jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing method"));
    };

    if let Some(required_token) = required_token {
        if !request_has_token(object.get("params"), required_token) {
            return Some(jsonrpc_error(
                id.unwrap_or(Value::Null),
                JSONRPC_UNAUTHORIZED,
                "unauthorized",
            ));
        }
    }

    match method {
        "initialize" => id.map(|id| {
            jsonrpc_result(
                id,
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
                }),
            )
        }),
        MCP_TOOLS_LIST_METHOD => {
            id.map(|id| jsonrpc_result(id, tools_list_result(command_sender, capture_tools)))
        }
        MCP_TOOLS_CALL_METHOD => {
            id.map(|id| handle_tools_call(id, object.get("params"), command_sender, capture_tools))
        }
        MCP_RESOURCES_LIST_METHOD => {
            id.map(|id| jsonrpc_result(id, resources_list_result(capture_tools)))
        }
        MCP_RESOURCES_READ_METHOD => {
            id.map(|id| handle_resources_read(id, object.get("params"), capture_tools))
        }
        "ping" => id.map(|id| jsonrpc_result(id, json!({}))),
        method if method.starts_with("notifications/") => None,
        _ => id.map(|id| jsonrpc_error(id, JSONRPC_METHOD_NOT_FOUND, "method not found")),
    }
}

fn tools_list_result(
    command_sender: Option<&McpCommandSender>,
    capture_tools: Option<&McpCaptureTools>,
) -> Value {
    let mut tools = Vec::new();
    if command_sender.is_some() {
        tools.push(json!({
            "name": MCP_ENQUEUE_CONTROL_TOOL,
            "description": "Queue one Stevenarella control command for main-thread handling.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "command": { "type": "object" },
                },
                "required": [MCP_FIELD_COMMAND],
                "additionalProperties": false,
            },
        }));
    }
    if capture_tools.is_some() {
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

fn handle_tools_call(
    id: Value,
    params: Option<&Value>,
    command_sender: Option<&McpCommandSender>,
    capture_tools: Option<&McpCaptureTools>,
) -> String {
    let Some(params) = params.and_then(Value::as_object) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing tool params");
    };
    let Some(name) = params.get(MCP_FIELD_NAME).and_then(Value::as_str) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing tool name");
    };
    match name {
        MCP_ENQUEUE_CONTROL_TOOL => handle_control_tools_call(id, params, command_sender),
        MCP_CAPTURE_SCREENSHOT_TOOL => handle_capture_tools_call(
            id,
            params.get(MCP_FIELD_ARGUMENTS),
            capture_tools,
            CaptureMode::Screenshot,
        ),
        MCP_CAPTURE_LATEST_FRAME_TOOL => handle_capture_tools_call(
            id,
            params.get(MCP_FIELD_ARGUMENTS),
            capture_tools,
            CaptureMode::LatestFrame,
        ),
        _ => jsonrpc_error(id, JSONRPC_METHOD_NOT_FOUND, "tool not found"),
    }
}

fn handle_control_tools_call(
    id: Value,
    params: &serde_json::Map<String, Value>,
    command_sender: Option<&McpCommandSender>,
) -> String {
    let Some(command_sender) = command_sender else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "control queue unavailable");
    };
    let Some(arguments) = params.get(MCP_FIELD_ARGUMENTS).and_then(Value::as_object) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing tool arguments");
    };
    let Some(command_value) = arguments.get(MCP_FIELD_COMMAND) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing control command");
    };
    let command = match crate::control::parse_control_command_value(command_value) {
        Ok(command) => command,
        Err(err) => {
            return jsonrpc_error(
                id,
                JSONRPC_INVALID_REQUEST,
                &format!("invalid control command: {err:?}"),
            )
        }
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

    jsonrpc_result(id, control_tool_result(&response))
}

fn handle_capture_tools_call(
    id: Value,
    arguments: Option<&Value>,
    capture_tools: Option<&McpCaptureTools>,
    mode: CaptureMode,
) -> String {
    let Some(capture_tools) = capture_tools else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "capture queue unavailable");
    };
    let payload = match capture_tools.capture_one_shot_from_arguments(mode, arguments) {
        Ok(payload) => payload,
        Err(err) => return mcp_capture_error_response(id, err),
    };
    jsonrpc_result(id, capture_tool_result(&payload))
}

fn resources_list_result(capture_tools: Option<&McpCaptureTools>) -> Value {
    let resources = if capture_tools.is_some() {
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

fn handle_resources_read(
    id: Value,
    params: Option<&Value>,
    capture_tools: Option<&McpCaptureTools>,
) -> String {
    let Some(capture_tools) = capture_tools else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "capture queue unavailable");
    };
    let Some(params) = params.and_then(Value::as_object) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing resource params");
    };
    let Some(uri) = params.get(MCP_FIELD_URI).and_then(Value::as_str) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing resource uri");
    };
    let mode = match uri {
        MCP_CAPTURE_SCREENSHOT_RESOURCE => CaptureMode::Screenshot,
        MCP_CAPTURE_LATEST_FRAME_RESOURCE => CaptureMode::LatestFrame,
        _ => return jsonrpc_error(id, JSONRPC_METHOD_NOT_FOUND, "resource not found"),
    };
    let payload = match capture_tools.capture_one_shot_inline(mode) {
        Ok(payload) => payload,
        Err(err) => return mcp_capture_error_response(id, err),
    };
    jsonrpc_result(id, capture_resource_read_result(uri, &payload))
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

fn mcp_capture_error_response(id: Value, err: McpCaptureToolError) -> String {
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

fn start_transport_runtime_inner<R, W>(
    validated: ValidatedMcpTransport,
    stdio: Option<(R, W)>,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) -> Result<McpTransportRuntime, McpTransportStartError>
where
    R: io::Read + Send + 'static,
    W: io::Write + Send + 'static,
{
    let mut endpoints = Vec::with_capacity(validated.endpoints.len());
    let mut shutdown_flags = Vec::new();
    let mut join_handles = Vec::new();
    let mut stdio = stdio;

    for endpoint in validated.endpoints {
        match endpoint {
            McpEndpoint::Stdio => {
                endpoints.push(StartedMcpEndpoint::Stdio);
                if let Some((reader, writer)) = stdio.take() {
                    let command_sender = command_sender.clone();
                    let capture_tools = capture_tools.clone();
                    join_handles.push(thread::spawn(move || {
                        let reader = BufReader::new(reader);
                        let writer = BufWriter::new(writer);
                        let _ = run_jsonrpc_lines_with_auth_and_tools(
                            reader,
                            writer,
                            None,
                            command_sender,
                            capture_tools,
                        );
                    }));
                }
            }
            McpEndpoint::Tcp { bind_addr, auth } => {
                let listener = TcpListener::bind(bind_addr)?;
                listener.set_nonblocking(true)?;
                let local_addr = listener.local_addr()?;
                let shutdown_flag = Arc::new(AtomicBool::new(false));
                let thread_shutdown_flag = Arc::clone(&shutdown_flag);
                let command_sender = command_sender.clone();
                let capture_tools = capture_tools.clone();
                join_handles.push(thread::spawn(move || {
                    accept_tcp_jsonrpc(
                        listener,
                        auth,
                        thread_shutdown_flag,
                        command_sender,
                        capture_tools,
                    );
                }));
                shutdown_flags.push(shutdown_flag);
                endpoints.push(StartedMcpEndpoint::Tcp { local_addr });
            }
        }
    }

    Ok(McpTransportRuntime {
        endpoints,
        stdout_must_remain_clean: validated.stdout_must_remain_clean,
        command_sender,
        shutdown_flags,
        join_handles,
    })
}

fn accept_tcp_jsonrpc(
    listener: TcpListener,
    auth: TcpAuth,
    shutdown_flag: Arc<AtomicBool>,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) {
    while !shutdown_flag.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                let required_token = auth.required_token().map(ToOwned::to_owned);
                let command_sender = command_sender.clone();
                let capture_tools = capture_tools.clone();
                thread::spawn(move || {
                    let _ = serve_tcp_jsonrpc_stream(
                        stream,
                        required_token,
                        command_sender,
                        capture_tools,
                    );
                });
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(TCP_ACCEPT_IDLE_SLEEP_MILLIS));
            }
            Err(_) => return,
        }
    }
}

fn serve_tcp_jsonrpc_stream(
    stream: TcpStream,
    required_token: Option<String>,
    command_sender: Option<McpCommandSender>,
    capture_tools: Option<McpCaptureTools>,
) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    let writer = BufWriter::new(stream);
    run_jsonrpc_lines_with_auth_and_tools(
        reader,
        writer,
        required_token.as_deref(),
        command_sender,
        capture_tools,
    )
}

fn validate_tcp_endpoint<F>(
    listen: &str,
    token_env: Option<&str>,
    token_lookup: &F,
) -> Result<McpEndpoint, McpTransportError>
where
    F: Fn(&str) -> Option<String>,
{
    let bind_addr = listen
        .parse::<SocketAddr>()
        .map_err(|_| McpTransportError::MalformedListenAddress(listen.to_owned()))?;

    let auth = if bind_addr.ip().is_loopback() {
        TcpAuth::NotRequiredForLoopback
    } else {
        let token_env = normalized_token_env(token_env, bind_addr)?;
        let token = validate_token_value(&token_env, token_lookup)?;
        TcpAuth::TokenEnv {
            name: token_env,
            token,
        }
    };

    Ok(McpEndpoint::Tcp { bind_addr, auth })
}

fn normalized_token_env(
    token_env: Option<&str>,
    bind_addr: SocketAddr,
) -> Result<String, McpTransportError> {
    let Some(token_env) = token_env else {
        return Err(McpTransportError::MissingTokenEnvForNonLoopback { bind_addr });
    };
    let token_env = token_env.trim();
    if token_env.is_empty() {
        return Err(McpTransportError::InvalidTokenEnvName {
            reason: REASON_EMPTY_TOKEN_ENV_NAME,
        });
    }

    Ok(token_env.to_owned())
}

fn validate_token_value<F>(token_env: &str, token_lookup: &F) -> Result<String, McpTransportError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = token_lookup(token_env) else {
        return Err(McpTransportError::MissingTokenValue {
            token_env: token_env.to_owned(),
        });
    };
    let value = value.trim();
    if value.is_empty() {
        return Err(McpTransportError::InvalidTokenValue {
            token_env: token_env.to_owned(),
            reason: REASON_EMPTY_TOKEN_VALUE,
        });
    }

    Ok(value.to_owned())
}

impl TcpAuth {
    fn required_token(&self) -> Option<&str> {
        match self {
            TcpAuth::NotRequiredForLoopback => None,
            TcpAuth::TokenEnv { token, .. } => Some(token.as_str()),
        }
    }
}

fn request_has_token(params: Option<&Value>, required_token: &str) -> bool {
    params
        .and_then(Value::as_object)
        .and_then(|params| params.get("token"))
        .and_then(Value::as_str)
        .map_or(false, |token| token == required_token)
}

fn jsonrpc_result(id: Value, result: Value) -> String {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    })
    .to_string()
}

fn jsonrpc_error(id: Value, code: i64, message: &str) -> String {
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
    use super::*;
    use crate::control::{
        ControlOutcome, CONTROL_OUTCOME_APPLIED_NAME, CONTROL_OUTCOME_DEFERRED_NAME,
    };
    use std::io::Write;

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
