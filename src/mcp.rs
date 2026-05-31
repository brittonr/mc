// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::control::{ControlCommand, ControlOutcome, ControlResponse};
use serde_json::{json, Value};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub const DEFAULT_MCP_TOKEN_ENV: &str = "STEVENARELLA_MCP_TOKEN";
pub const MAX_MCP_COMMANDS_PER_FRAME: usize = 64;

const COMMAND_RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;
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
const MCP_ENQUEUE_CONTROL_TOOL: &str = "stevenarella.enqueue_control";
const MCP_CONTENT_TYPE_TEXT: &str = "text";
const MCP_FIELD_ARGUMENTS: &str = "arguments";
const MCP_FIELD_COMMAND: &str = "command";
const MCP_FIELD_NAME: &str = "name";
const CONTROL_OUTCOME_APPLIED: &str = "applied";
const CONTROL_OUTCOME_REJECTED: &str = "rejected";
const CONTROL_OUTCOME_DEFERRED: &str = "deferred";

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
    start_transport_with_stdio_and_commands(validated, io::stdin(), io::stdout(), command_sender)
}

pub fn start_transport_runtime(
    validated: ValidatedMcpTransport,
) -> Result<McpTransportRuntime, McpTransportStartError> {
    start_transport_runtime_inner(validated, None::<(io::Empty, io::Sink)>, None)
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
    start_transport_runtime_inner(validated, Some((reader, writer)), command_sender)
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
    mut reader: R,
    mut writer: W,
    required_token: Option<&str>,
    command_sender: Option<McpCommandSender>,
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
        if let Some(response) = handle_jsonrpc_line_with_auth_and_command_sender(
            line.trim_end_matches(['\r', '\n']),
            required_token,
            command_sender.as_ref(),
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
        MCP_TOOLS_LIST_METHOD => id.map(|id| jsonrpc_result(id, tools_list_result(command_sender))),
        MCP_TOOLS_CALL_METHOD => {
            id.map(|id| handle_tools_call(id, object.get("params"), command_sender))
        }
        "resources/list" => id.map(|id| jsonrpc_result(id, json!({ "resources": [] }))),
        "ping" => id.map(|id| jsonrpc_result(id, json!({}))),
        method if method.starts_with("notifications/") => None,
        _ => id.map(|id| jsonrpc_error(id, JSONRPC_METHOD_NOT_FOUND, "method not found")),
    }
}

fn tools_list_result(command_sender: Option<&McpCommandSender>) -> Value {
    let tools = if command_sender.is_some() {
        vec![json!({
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
        })]
    } else {
        Vec::new()
    };
    json!({ "tools": tools })
}

fn handle_tools_call(
    id: Value,
    params: Option<&Value>,
    command_sender: Option<&McpCommandSender>,
) -> String {
    let Some(command_sender) = command_sender else {
        return jsonrpc_error(id, JSONRPC_INTERNAL_ERROR, "control queue unavailable");
    };
    let Some(params) = params.and_then(Value::as_object) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing tool params");
    };
    let Some(name) = params.get(MCP_FIELD_NAME).and_then(Value::as_str) else {
        return jsonrpc_error(id, JSONRPC_INVALID_REQUEST, "missing tool name");
    };
    if name != MCP_ENQUEUE_CONTROL_TOOL {
        return jsonrpc_error(id, JSONRPC_METHOD_NOT_FOUND, "tool not found");
    }
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

fn control_tool_result(response: &ControlResponse) -> Value {
    json!({
        "content": [{
            "type": MCP_CONTENT_TYPE_TEXT,
            "text": json!({
                "outcome": control_outcome_name(response.outcome),
                "message": response.message.as_deref(),
            })
            .to_string(),
        }],
        "isError": matches!(response.outcome, ControlOutcome::Rejected),
    })
}

fn control_outcome_name(outcome: ControlOutcome) -> &'static str {
    match outcome {
        ControlOutcome::Applied => CONTROL_OUTCOME_APPLIED,
        ControlOutcome::Rejected => CONTROL_OUTCOME_REJECTED,
        ControlOutcome::Deferred => CONTROL_OUTCOME_DEFERRED,
    }
}

fn start_transport_runtime_inner<R, W>(
    validated: ValidatedMcpTransport,
    stdio: Option<(R, W)>,
    command_sender: Option<McpCommandSender>,
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
                    join_handles.push(thread::spawn(move || {
                        let reader = BufReader::new(reader);
                        let writer = BufWriter::new(writer);
                        let _ =
                            run_jsonrpc_lines_with_command_sender(reader, writer, command_sender);
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
                join_handles.push(thread::spawn(move || {
                    accept_tcp_jsonrpc(listener, auth, thread_shutdown_flag, command_sender);
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
) {
    while !shutdown_flag.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                let required_token = auth.required_token().map(ToOwned::to_owned);
                let command_sender = command_sender.clone();
                thread::spawn(move || {
                    let _ = serve_tcp_jsonrpc_stream(stream, required_token, command_sender);
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
) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);
    let writer = BufWriter::new(stream);
    run_jsonrpc_lines_with_auth_and_command_sender(
        reader,
        writer,
        required_token.as_deref(),
        command_sender,
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
    const QUEUE_TEST_RESPONSE: &str = "main-thread-handler";

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
        assert_eq!(payload["outcome"], CONTROL_OUTCOME_APPLIED);
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
        assert_eq!(payload["outcome"], CONTROL_OUTCOME_DEFERRED);
        assert_eq!(payload["message"], QUEUE_TEST_RESPONSE);
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
            thread::yield_now();
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
