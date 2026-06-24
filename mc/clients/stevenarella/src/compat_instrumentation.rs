// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

//! Opt-in compatibility instrumentation boundaries.
//!
//! This module is the harness-only shell for Stevenarella MCP control and
//! frame-capture instrumentation. Core startup passes explicit CLI options in;
//! default options keep MCP command and capture queues absent.

use crate::capture::{self, CapturePolicy, CaptureRequestReceiver, CaptureRequestSender};
use crate::mcp::{self, McpCommandReceiver, McpTransportRuntime};
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatInstrumentationMode {
    Disabled,
    McpControlAndCapture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatInstrumentationOptions {
    pub mcp: mcp::McpTransportOptions,
}

pub struct StartedCompatInstrumentation {
    pub mcp_runtime: Option<McpTransportRuntime>,
    pub mcp_command_receiver: Option<McpCommandReceiver>,
    pub mcp_capture_request_sender: Option<CaptureRequestSender>,
    pub mcp_capture_request_receiver: Option<CaptureRequestReceiver>,
    pub capture_sequence_id: Arc<AtomicU64>,
}

#[derive(Debug)]
pub enum CompatInstrumentationStartError {
    Transport(mcp::McpTransportError),
    Start(mcp::McpTransportStartError),
}

impl From<mcp::McpTransportError> for CompatInstrumentationStartError {
    fn from(err: mcp::McpTransportError) -> Self {
        CompatInstrumentationStartError::Transport(err)
    }
}

impl From<mcp::McpTransportStartError> for CompatInstrumentationStartError {
    fn from(err: mcp::McpTransportStartError) -> Self {
        CompatInstrumentationStartError::Start(err)
    }
}

impl CompatInstrumentationOptions {
    pub fn from_cli(stdio: bool, listen: Option<String>, token_env: Option<String>) -> Self {
        CompatInstrumentationOptions {
            mcp: mcp::McpTransportOptions::from_cli(stdio, listen, token_env),
        }
    }

    pub fn mode(&self) -> CompatInstrumentationMode {
        mode_for_mcp_options(&self.mcp)
    }

    pub fn reserves_stdout(&self) -> bool {
        mcp_options_reserve_stdout(&self.mcp)
    }
}

impl StartedCompatInstrumentation {
    pub fn disabled() -> Self {
        StartedCompatInstrumentation {
            mcp_runtime: None,
            mcp_command_receiver: None,
            mcp_capture_request_sender: None,
            mcp_capture_request_receiver: None,
            capture_sequence_id: initial_capture_sequence_id(),
        }
    }
}

pub fn mode_for_mcp_options(options: &mcp::McpTransportOptions) -> CompatInstrumentationMode {
    if options.has_transport() {
        return CompatInstrumentationMode::McpControlAndCapture;
    }
    CompatInstrumentationMode::Disabled
}

pub fn mcp_options_reserve_stdout(options: &mcp::McpTransportOptions) -> bool {
    options.stdio
}

pub fn start_process_instrumentation(
    options: &CompatInstrumentationOptions,
    capture_policy: CapturePolicy,
) -> Result<StartedCompatInstrumentation, CompatInstrumentationStartError> {
    match options.mode() {
        CompatInstrumentationMode::Disabled => Ok(StartedCompatInstrumentation::disabled()),
        CompatInstrumentationMode::McpControlAndCapture => {
            start_mcp_control_and_capture(&options.mcp, capture_policy)
        }
    }
}

fn start_mcp_control_and_capture(
    options: &mcp::McpTransportOptions,
    capture_policy: CapturePolicy,
) -> Result<StartedCompatInstrumentation, CompatInstrumentationStartError> {
    let validated = mcp::validate_process_transport_options(options)?;
    let (mcp_command_sender, mcp_command_receiver) = mcp::control_command_channel();
    let (mcp_capture_request_sender, mcp_capture_request_receiver) =
        capture::capture_request_channel();
    let capture_sequence_id = initial_capture_sequence_id();
    let capture_tools = mcp::McpCaptureTools::new(
        mcp_capture_request_sender.clone(),
        capture_policy,
        Arc::clone(&capture_sequence_id),
    );
    let mcp_runtime = mcp::start_process_transport_with_capture(
        validated,
        Some(mcp_command_sender),
        Some(capture_tools),
    )?;

    Ok(StartedCompatInstrumentation {
        mcp_runtime: Some(mcp_runtime),
        mcp_command_receiver: Some(mcp_command_receiver),
        mcp_capture_request_sender: Some(mcp_capture_request_sender),
        mcp_capture_request_receiver: Some(mcp_capture_request_receiver),
        capture_sequence_id,
    })
}

fn initial_capture_sequence_id() -> Arc<AtomicU64> {
    Arc::new(AtomicU64::new(capture::CAPTURE_SEQUENCE_INITIAL))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    const LOOPBACK_LISTEN: &str = "127.0.0.1:0";
    const TOKEN_ENV_NAME: &str = "STEVENARELLA_TEST_MCP_TOKEN";

    #[test]
    fn default_options_disable_mcp_instrumentation() {
        let options = CompatInstrumentationOptions::from_cli(false, None, None);

        assert_eq!(options.mode(), CompatInstrumentationMode::Disabled);
        assert!(!options.reserves_stdout());
    }

    #[test]
    fn stdio_options_enable_mcp_instrumentation_and_reserve_stdout() {
        let options = CompatInstrumentationOptions::from_cli(true, None, None);

        assert_eq!(
            options.mode(),
            CompatInstrumentationMode::McpControlAndCapture
        );
        assert!(options.reserves_stdout());
    }

    #[test]
    fn tcp_options_enable_mcp_instrumentation_without_reserving_stdout() {
        let options = CompatInstrumentationOptions::from_cli(
            false,
            Some(LOOPBACK_LISTEN.to_owned()),
            Some(TOKEN_ENV_NAME.to_owned()),
        );

        assert_eq!(
            options.mode(),
            CompatInstrumentationMode::McpControlAndCapture
        );
        assert!(!options.reserves_stdout());
    }

    #[test]
    fn disabled_instrumentation_has_no_harness_queues() {
        let instrumentation = StartedCompatInstrumentation::disabled();

        assert!(instrumentation.mcp_runtime.is_none());
        assert!(instrumentation.mcp_command_receiver.is_none());
        assert!(instrumentation.mcp_capture_request_sender.is_none());
        assert!(instrumentation.mcp_capture_request_receiver.is_none());
        assert_eq!(
            instrumentation.capture_sequence_id.load(Ordering::Acquire),
            capture::CAPTURE_SEQUENCE_INITIAL
        );
    }
}
