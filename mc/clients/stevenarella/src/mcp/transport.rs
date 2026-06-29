// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::auth::{validate_tcp_auth, TcpAuth};
use super::capture_adapter::McpCaptureTools;
use super::control_queue::McpCommandSender;
use super::dispatcher::{
    handle_jsonrpc_line_with_auth_and_command_sender, handle_jsonrpc_line_with_auth_and_tools,
};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

const TCP_ACCEPT_IDLE_SLEEP_MILLIS: u64 = 10;

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
    let auth = validate_tcp_auth(bind_addr, token_env, token_lookup)?;

    Ok(McpEndpoint::Tcp { bind_addr, auth })
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOOPBACK_LISTEN: &str = "127.0.0.1:4700";

    #[test]
    fn transport_options_report_opt_in_state() {
        let disabled = McpTransportOptions::from_cli(false, None, None);
        let stdio = McpTransportOptions::from_cli(true, None, None);
        let tcp = McpTransportOptions::from_cli(false, Some(LOOPBACK_LISTEN.to_owned()), None);

        assert!(!disabled.has_transport());
        assert!(stdio.has_transport());
        assert!(tcp.has_transport());
    }

    #[test]
    fn validation_preserves_stdio_stdout_contract() {
        let options = McpTransportOptions::from_cli(true, None, None);

        let validated = validate_transport_options(&options, |_| None).unwrap();

        assert_eq!(validated.endpoints, vec![McpEndpoint::Stdio]);
        assert!(validated.stdout_must_remain_clean);
    }
}
