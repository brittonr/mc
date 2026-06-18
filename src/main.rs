// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![recursion_limit = "300"]
#![allow(clippy::too_many_arguments)] // match standard gl functions with many arguments
#![allow(clippy::many_single_char_names)] // short variable names provide concise clarity
#![allow(clippy::float_cmp)] // float comparison used to check if changed

use instant::{Duration, Instant};
use log::{error, info, warn};
use std::fs;
use std::path::PathBuf;
extern crate steven_shared as shared;

use structopt::StructOpt;

extern crate steven_protocol;

pub mod ecs;
use steven_protocol::format;
use steven_protocol::nbt;
use steven_protocol::protocol;
pub mod gl;
use steven_protocol::types;
pub mod auth;
pub mod capture;
pub mod chunk_builder;
pub mod console;
pub mod control;
pub mod entity;
#[cfg(not(target_arch = "wasm32"))]
pub mod mcp;
pub mod model;
pub mod render;
pub mod resources;
pub mod screen;
pub mod server;
pub mod settings;
pub mod ui;
pub mod world;

use crate::protocol::mojang;
use cfg_if::cfg_if;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

const CL_BRAND: console::CVar<String> = console::CVar {
    ty: PhantomData,
    name: "cl_brand",
    description: "cl_brand has the value of the clients current 'brand'. e.g. \"Steven\" or \
                  \"Vanilla\"",
    mutable: false,
    serializable: false,
    default: &|| "Steven".to_owned(),
};

#[cfg(not(target_arch = "wasm32"))]
const MCP_STATUS_APPLIED_MESSAGE: &str = "status reported";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CONNECT_STARTED_MESSAGE: &str = "connect started";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CONNECT_ALREADY_ACTIVE_MESSAGE: &str = "connect already active";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CONNECT_ALREADY_CONNECTED_MESSAGE: &str =
    "disconnect before connecting to another server";
#[cfg(not(target_arch = "wasm32"))]
const MCP_DISCONNECT_APPLIED_MESSAGE: &str = "disconnect applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_DISCONNECT_NOT_CONNECTED_MESSAGE: &str = "no active connection to disconnect";
#[cfg(not(target_arch = "wasm32"))]
const MCP_KEY_APPLIED_MESSAGE: &str = "key applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_LOOK_APPLIED_MESSAGE: &str = "look applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_MOUSE_APPLIED_MESSAGE: &str = "mouse applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_USE_ITEM_APPLIED_MESSAGE: &str = "use item applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_ATTACK_APPLIED_MESSAGE: &str = "attack applied";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CHAT_APPLIED_MESSAGE: &str = "chat sent";
#[cfg(not(target_arch = "wasm32"))]
const MCP_RESOURCE_PACK_STATUS_APPLIED_MESSAGE: &str = "resource pack status sent";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CAPTURE_DEFERRED_MESSAGE: &str = "capture queued for next rendered frame";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CAPTURE_QUEUE_CLOSED_MESSAGE: &str = "capture queue closed";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE: &str = "capture queue unavailable";
#[cfg(not(target_arch = "wasm32"))]
const MCP_CAPTURE_REQUEST_INVALID_MESSAGE: &str = "invalid capture request";
#[cfg(not(target_arch = "wasm32"))]
const MCP_REQUIRES_CONNECTED_MESSAGE: &str = "command requires an active connection";
#[cfg(not(target_arch = "wasm32"))]
const MCP_REQUIRES_PLAYER_MESSAGE: &str = "command requires a player entity";
#[cfg(not(target_arch = "wasm32"))]
const MCP_MIN_PITCH_EPSILON_RADIANS: f64 = 0.01;
#[cfg(not(target_arch = "wasm32"))]
const MCP_MIN_PITCH_RADIANS: f64 = std::f64::consts::FRAC_PI_2 + MCP_MIN_PITCH_EPSILON_RADIANS;
#[cfg(not(target_arch = "wasm32"))]
const MCP_MAX_PITCH_RADIANS: f64 =
    std::f64::consts::PI + std::f64::consts::FRAC_PI_2 - MCP_MIN_PITCH_EPSILON_RADIANS;
#[cfg(not(target_arch = "wasm32"))]
const CAPTURE_START_MILLIS: u64 = 0;

pub struct Game {
    renderer: render::Renderer,
    screen_sys: screen::ScreenSystem,
    resource_manager: Arc<RwLock<resources::Manager>>,
    console: Arc<Mutex<console::Console>>,
    vars: Rc<console::Vars>,
    should_close: bool,

    server: server::Server,
    focused: bool,
    chunk_builder: chunk_builder::ChunkBuilder,

    connect_reply: Option<mpsc::Receiver<Result<server::Server, protocol::Error>>>,

    dpi_factor: f64,
    last_mouse_x: f64,
    last_mouse_y: f64,
    last_mouse_xrel: f64,
    last_mouse_yrel: f64,
    is_ctrl_pressed: bool,
    is_logo_pressed: bool,
    is_fullscreen: bool,
    default_protocol_version: i32,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_command_receiver: Option<mcp::McpCommandReceiver>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_capture_request_sender: Option<capture::CaptureRequestSender>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_capture_request_receiver: Option<capture::CaptureRequestReceiver>,
    #[cfg(not(target_arch = "wasm32"))]
    mcp_release_left_after_server_tick: bool,
    #[cfg(not(target_arch = "wasm32"))]
    capture_policy: capture::CapturePolicy,
    #[cfg(not(target_arch = "wasm32"))]
    capture_sequence_id: Arc<AtomicU64>,
    #[cfg(not(target_arch = "wasm32"))]
    active_capture_recording: Option<capture::RecordingSession>,
    #[cfg(not(target_arch = "wasm32"))]
    capture_started_at: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
fn control_key_to_stevenkey(key: control::ControlKey) -> settings::Stevenkey {
    match key {
        control::ControlKey::Forward => settings::Stevenkey::Forward,
        control::ControlKey::Backward => settings::Stevenkey::Backward,
        control::ControlKey::Left => settings::Stevenkey::Left,
        control::ControlKey::Right => settings::Stevenkey::Right,
        control::ControlKey::OpenInventory => settings::Stevenkey::OpenInv,
        control::ControlKey::Sneak => settings::Stevenkey::Sneak,
        control::ControlKey::Sprint => settings::Stevenkey::Sprint,
        control::ControlKey::Jump => settings::Stevenkey::Jump,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn control_command_requires_connected(command: &control::ControlCommand) -> bool {
    !matches!(
        command,
        control::ControlCommand::Status
            | control::ControlCommand::Connect { .. }
            | control::ControlCommand::Disconnect
            | control::ControlCommand::CaptureScreenshot
            | control::ControlCommand::CaptureLatestFrame
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn disconnected_control_rejection(
    command: &control::ControlCommand,
) -> Option<control::ControlResponse> {
    control_command_requires_connected(command)
        .then(|| control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
}

#[cfg(not(target_arch = "wasm32"))]
fn control_status_message(connected: bool, connecting: bool, focused: bool) -> String {
    format!(
        "{MCP_STATUS_APPLIED_MESSAGE}: connected={connected} connecting={connecting} focused={focused}"
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn control_applied_response(message: impl Into<String>) -> control::ControlResponse {
    control::ControlResponse {
        outcome: control::ControlOutcome::Applied,
        message: Some(message.into()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn control_rejected_response(message: impl Into<String>) -> control::ControlResponse {
    control::ControlResponse {
        outcome: control::ControlOutcome::Rejected,
        message: Some(message.into()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn control_deferred_response(message: impl Into<String>) -> control::ControlResponse {
    control::ControlResponse {
        outcome: control::ControlOutcome::Deferred,
        message: Some(message.into()),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn one_shot_mcp_capture_request(
    mode: capture::CaptureMode,
    output: capture::CaptureOutput,
    sequence_id: u64,
) -> capture::CaptureRequest {
    capture::CaptureRequest {
        mode,
        format: capture::CaptureFormat::Png,
        output,
        includes_ui: true,
        recording: None,
        sequence_id: Some(sequence_id),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn one_shot_mcp_capture_output(
    policy: &capture::CapturePolicy,
    mode: capture::CaptureMode,
    sequence_id: u64,
) -> capture::CaptureOutput {
    if policy.has_capture_dir() {
        return capture::CaptureOutput::Artifact {
            relative_path: capture::default_artifact_relative_path(
                mode,
                sequence_id,
                capture::CaptureFormat::Png,
            ),
        };
    }
    capture::CaptureOutput::Inline
}

#[cfg(not(target_arch = "wasm32"))]
fn enqueue_mcp_capture_request(
    sender: Option<&capture::CaptureRequestSender>,
    policy: &capture::CapturePolicy,
    mode: capture::CaptureMode,
    sequence_id: u64,
) -> control::ControlResponse {
    let Some(sender) = sender else {
        return control_rejected_response(MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE);
    };
    let output = one_shot_mcp_capture_output(policy, mode, sequence_id);
    match sender.enqueue_deferred(one_shot_mcp_capture_request(mode, output, sequence_id)) {
        Ok(_) => control_deferred_response(MCP_CAPTURE_DEFERRED_MESSAGE),
        Err(capture::CaptureQueueError::QueueClosed) => {
            control_rejected_response(MCP_CAPTURE_QUEUE_CLOSED_MESSAGE)
        }
        Err(capture::CaptureQueueError::RateLimitExceeded { .. })
        | Err(capture::CaptureQueueError::Validation(_)) => {
            control_rejected_response(MCP_CAPTURE_REQUEST_INVALID_MESSAGE)
        }
    }
}

impl Game {
    pub fn connect_to(&mut self, address: &str) {
        let (protocol_version, forge_mods, fml_network_version) =
            match protocol::Conn::new(address, self.default_protocol_version)
                .and_then(|conn| conn.do_status())
            {
                Ok(res) => {
                    info!(
                        "Detected server protocol version {}",
                        res.0.version.protocol
                    );
                    (
                        res.0.version.protocol,
                        res.0.forge_mods,
                        res.0.fml_network_version,
                    )
                }
                Err(err) => {
                    warn!(
                        "Error pinging server {} to get protocol version: {:?}, defaulting to {}",
                        address, err, self.default_protocol_version
                    );
                    (self.default_protocol_version, vec![], None)
                }
            };

        let (tx, rx) = mpsc::channel();
        self.connect_reply = Some(rx);
        let address = address.to_owned();
        let resources = self.resource_manager.clone();
        let profile = mojang::Profile {
            username: self.vars.get(auth::CL_USERNAME).clone(),
            id: self.vars.get(auth::CL_UUID).clone(),
            access_token: self.vars.get(auth::AUTH_TOKEN).clone(),
        };
        thread::spawn(move || {
            tx.send(server::Server::connect(
                resources,
                profile,
                &address,
                protocol_version,
                forge_mods,
                fml_network_version,
            ))
            .unwrap();
        });
    }

    pub fn tick(&mut self, delta: f64) {
        if !self.server.is_connected() {
            self.renderer.camera.yaw += 0.005 * delta;
            if self.renderer.camera.yaw > ::std::f64::consts::PI * 2.0 {
                self.renderer.camera.yaw = 0.0;
            }
        }

        if let Some(disconnect_reason) = self.server.disconnect_reason.take() {
            self.screen_sys
                .replace_screen(Box::new(screen::ServerList::new(Some(disconnect_reason))));
        }
        if !self.server.is_connected() {
            self.focused = false;
        }

        let mut clear_reply = false;
        if let Some(ref recv) = self.connect_reply {
            if let Ok(server) = recv.try_recv() {
                clear_reply = true;
                match server {
                    Ok(val) => {
                        self.screen_sys.pop_screen();
                        self.focused = true;
                        self.server.remove(&mut self.renderer);
                        self.server = val;
                    }
                    Err(err) => {
                        let msg = match err {
                            protocol::Error::Disconnect(val) => val,
                            err => {
                                let mut msg = format::TextComponent::new(&format!("{}", err));
                                msg.modifier.color = Some(format::Color::Red);
                                format::Component::Text(msg)
                            }
                        };
                        self.screen_sys
                            .replace_screen(Box::new(screen::ServerList::new(Some(msg))));
                    }
                }
            }
        }
        if clear_reply {
            self.connect_reply = None;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn drain_mcp_control_commands(&mut self) -> usize {
        let Some(receiver) = self.mcp_command_receiver.take() else {
            return 0;
        };
        let drained =
            receiver.drain_pending_with_handler(|command| self.apply_mcp_control_command(command));
        self.mcp_command_receiver = Some(receiver);
        drained
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn service_pending_mcp_capture_requests(&mut self) -> usize {
        let Some(receiver) = self.mcp_capture_request_receiver.take() else {
            return 0;
        };
        let frame = self.current_capture_frame_context();
        let serviced = receiver.service_pending_one_shot_with_readback(
            &self.capture_policy,
            frame,
            capture::read_current_framebuffer_for_context,
        );
        self.mcp_capture_request_receiver = Some(receiver);
        serviced
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn service_active_capture_recording(&mut self) {
        let frame = self.current_capture_frame_context();
        let now_millis = duration_to_millis_saturated(self.capture_started_at.elapsed());
        let Some(recording) = self.active_capture_recording.as_mut() else {
            return;
        };
        let outcome = capture::service_recording_frame_with_readback(
            recording,
            &self.capture_policy,
            now_millis,
            frame,
            &mut capture::read_current_framebuffer_for_context,
        );
        match outcome {
            Ok(capture::RecordingServiceOutcome::Captured(metadata)) => info!(
                "Capture recording wrote {:?} digest={}",
                metadata.relative_path,
                metadata.blake3_digest.as_str()
            ),
            Ok(capture::RecordingServiceOutcome::Waiting) => {}
            Ok(capture::RecordingServiceOutcome::Complete) => {
                self.active_capture_recording = None;
                info!("Capture recording complete");
            }
            Err(err) => {
                self.active_capture_recording = None;
                error!("Capture recording stopped: {:?}", err);
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn current_capture_frame_context(&self) -> capture::CaptureFrameContext {
        capture::CaptureFrameContext {
            width_px: self.renderer.width,
            height_px: self.renderer.height,
            frame_id: self.renderer.frame_id as u64,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn next_capture_sequence_id(&mut self) -> u64 {
        self.capture_sequence_id.fetch_add(1, Ordering::AcqRel)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn release_mcp_control_buttons_after_server_tick(&mut self) {
        if self.mcp_release_left_after_server_tick {
            self.server.on_left_mouse_button(false);
            self.mcp_release_left_after_server_tick = false;
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_mcp_control_command(
        &mut self,
        command: control::ControlCommand,
    ) -> control::ControlResponse {
        if !self.server.is_connected() {
            if let Some(response) = disconnected_control_rejection(&command) {
                return response;
            }
        }

        match command {
            control::ControlCommand::Status => control_applied_response(control_status_message(
                self.server.is_connected(),
                self.connect_reply.is_some(),
                self.focused,
            )),
            control::ControlCommand::Connect { address } => self.apply_mcp_connect(&address),
            control::ControlCommand::Disconnect => self.apply_mcp_disconnect(),
            control::ControlCommand::Key { key, down } => {
                self.server.key_press(down, control_key_to_stevenkey(key));
                control_applied_response(MCP_KEY_APPLIED_MESSAGE)
            }
            control::ControlCommand::Look {
                yaw_delta,
                pitch_delta,
            } => self.apply_mcp_look(yaw_delta, pitch_delta),
            control::ControlCommand::Mouse { button, down } => self.apply_mcp_mouse(button, down),
            control::ControlCommand::UseItem => {
                self.server.on_right_mouse_button(true);
                self.server.on_right_click(&mut self.renderer);
                self.server.on_right_mouse_button(false);
                control_applied_response(MCP_USE_ITEM_APPLIED_MESSAGE)
            }
            control::ControlCommand::Attack => {
                self.server.on_left_mouse_button(true);
                self.mcp_release_left_after_server_tick = true;
                control_applied_response(MCP_ATTACK_APPLIED_MESSAGE)
            }
            control::ControlCommand::Chat { message } => {
                self.server
                    .write_packet(protocol::packet::play::serverbound::ChatMessage { message });
                control_applied_response(MCP_CHAT_APPLIED_MESSAGE)
            }
            control::ControlCommand::ResourcePackStatus(decision) => {
                self.server
                    .write_packet(protocol::packet::play::serverbound::ResourcePackStatus {
                        result: protocol::VarInt(decision.status.status_code()),
                    });
                info!(
                    "MC-COMPAT-MILESTONE resource_pack_status_sent offer_id={} status=declined no_external_fetch=true",
                    decision.offer_id
                );
                control_applied_response(MCP_RESOURCE_PACK_STATUS_APPLIED_MESSAGE)
            }
            control::ControlCommand::CaptureScreenshot => {
                let sequence_id = self.next_capture_sequence_id();
                enqueue_mcp_capture_request(
                    self.mcp_capture_request_sender.as_ref(),
                    &self.capture_policy,
                    capture::CaptureMode::Screenshot,
                    sequence_id,
                )
            }
            control::ControlCommand::CaptureLatestFrame => {
                let sequence_id = self.next_capture_sequence_id();
                enqueue_mcp_capture_request(
                    self.mcp_capture_request_sender.as_ref(),
                    &self.capture_policy,
                    capture::CaptureMode::LatestFrame,
                    sequence_id,
                )
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_mcp_connect(&mut self, address: &str) -> control::ControlResponse {
        if self.connect_reply.is_some() {
            return control_rejected_response(MCP_CONNECT_ALREADY_ACTIVE_MESSAGE);
        }
        if self.server.is_connected() {
            return control_rejected_response(MCP_CONNECT_ALREADY_CONNECTED_MESSAGE);
        }
        self.connect_to(address);
        control_applied_response(MCP_CONNECT_STARTED_MESSAGE)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_mcp_disconnect(&mut self) -> control::ControlResponse {
        if self.connect_reply.is_some() {
            self.connect_reply = None;
            return control_applied_response(MCP_DISCONNECT_APPLIED_MESSAGE);
        }
        if !self.server.is_connected() {
            return control_rejected_response(MCP_DISCONNECT_NOT_CONNECTED_MESSAGE);
        }
        self.server.disconnect(None);
        self.focused = false;
        control_applied_response(MCP_DISCONNECT_APPLIED_MESSAGE)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_mcp_look(&mut self, yaw_delta: f64, pitch_delta: f64) -> control::ControlResponse {
        let Some(player) = self.server.player else {
            return control_rejected_response(MCP_REQUIRES_PLAYER_MESSAGE);
        };
        let Some(rotation) = self
            .server
            .entities
            .get_component_mut(player, self.server.rotation)
        else {
            return control_rejected_response(MCP_REQUIRES_PLAYER_MESSAGE);
        };
        rotation.yaw += yaw_delta;
        rotation.pitch = bounded_pitch(rotation.pitch + pitch_delta);
        control_applied_response(MCP_LOOK_APPLIED_MESSAGE)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_mcp_mouse(
        &mut self,
        button: control::MouseButton,
        down: bool,
    ) -> control::ControlResponse {
        match button {
            control::MouseButton::Left => self.server.on_left_mouse_button(down),
            control::MouseButton::Right => {
                self.server.on_right_mouse_button(down);
                if down {
                    self.server.on_right_click(&mut self.renderer);
                }
            }
        }
        control_applied_response(MCP_MOUSE_APPLIED_MESSAGE)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn bounded_pitch(pitch: f64) -> f64 {
    pitch.max(MCP_MIN_PITCH_RADIANS).min(MCP_MAX_PITCH_RADIANS)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod mcp_control_tests {
    use super::*;

    const TEST_YAW_DELTA: f64 = 0.25;
    const TEST_PITCH_DELTA: f64 = -0.125;
    const TEST_PITCH_OFFSET: f64 = 1.0;
    const TEST_CAPTURE_WIDTH_PX: u32 = 2;
    const TEST_CAPTURE_HEIGHT_PX: u32 = 2;
    const TEST_CAPTURE_FRAME_ID: u64 = 42;
    const TEST_CAPTURE_SEQUENCE_ID: u64 = 7;

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

    fn unique_test_capture_dir(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "stevenarella-main-capture-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    #[test]
    fn maps_control_keys_to_internal_steven_keys() {
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Forward),
            settings::Stevenkey::Forward
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Backward),
            settings::Stevenkey::Backward
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Left),
            settings::Stevenkey::Left
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Right),
            settings::Stevenkey::Right
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::OpenInventory),
            settings::Stevenkey::OpenInv
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Sneak),
            settings::Stevenkey::Sneak
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Sprint),
            settings::Stevenkey::Sprint
        );
        assert_eq!(
            control_key_to_stevenkey(control::ControlKey::Jump),
            settings::Stevenkey::Jump
        );
    }

    #[test]
    fn status_connect_and_disconnect_do_not_require_an_existing_connection() {
        assert!(!control_command_requires_connected(
            &control::ControlCommand::Status
        ));
        assert!(!control_command_requires_connected(
            &control::ControlCommand::Connect {
                address: "127.0.0.1:25565".to_owned(),
            }
        ));
        assert!(!control_command_requires_connected(
            &control::ControlCommand::Disconnect
        ));
        assert!(!control_command_requires_connected(
            &control::ControlCommand::CaptureScreenshot
        ));
        assert!(!control_command_requires_connected(
            &control::ControlCommand::CaptureLatestFrame
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::Key {
                key: control::ControlKey::Forward,
                down: true,
            }
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::Look {
                yaw_delta: TEST_YAW_DELTA,
                pitch_delta: TEST_PITCH_DELTA,
            }
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::Mouse {
                button: control::MouseButton::Left,
                down: true,
            }
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::UseItem
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::Attack
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::Chat {
                message: "hello".to_owned(),
            }
        ));
        assert!(control_command_requires_connected(
            &control::ControlCommand::ResourcePackStatus(control::ResourcePackStatusDecision {
                offer_id: "mc-compat-local-resource-pack".to_owned(),
                status: control::ResourcePackStatusResponse::Declined,
            })
        ));
    }

    #[test]
    fn disconnected_operations_return_rejected_response() {
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Status),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Connect {
                address: "127.0.0.1:25565".to_owned(),
            }),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Disconnect),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::CaptureScreenshot),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::CaptureLatestFrame),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Key {
                key: control::ControlKey::Forward,
                down: true,
            }),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Look {
                yaw_delta: TEST_YAW_DELTA,
                pitch_delta: TEST_PITCH_DELTA,
            }),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Mouse {
                button: control::MouseButton::Left,
                down: true,
            }),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::UseItem),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Attack),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Chat {
                message: "hello".to_owned(),
            }),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::ResourcePackStatus(
                control::ResourcePackStatusDecision {
                    offer_id: "mc-compat-local-resource-pack".to_owned(),
                    status: control::ResourcePackStatusResponse::Declined,
                }
            )),
            Some(control_rejected_response(MCP_REQUIRES_CONNECTED_MESSAGE))
        );
    }

    #[test]
    fn status_message_reports_connection_state() {
        assert_eq!(
            control_status_message(true, false, true),
            "status reported: connected=true connecting=false focused=true"
        );
    }

    #[test]
    fn control_capture_enqueue_reaches_post_render_capture_queue() {
        let (sender, receiver) = capture::capture_request_channel();
        let policy = capture::CapturePolicy::memory();
        let response = enqueue_mcp_capture_request(
            Some(&sender),
            &policy,
            capture::CaptureMode::Screenshot,
            TEST_CAPTURE_SEQUENCE_ID,
        );

        let serviced = receiver.service_pending_one_shot_with_readback(
            &policy,
            test_capture_frame_context(),
            synthetic_capture_frame,
        );

        assert_eq!(response.outcome, control::ControlOutcome::Deferred);
        assert_eq!(
            response.message,
            Some(MCP_CAPTURE_DEFERRED_MESSAGE.to_owned())
        );
        assert_eq!(serviced, 1);
    }

    #[test]
    fn control_capture_enqueue_rejects_missing_capture_queue() {
        let policy = capture::CapturePolicy::memory();
        let response = enqueue_mcp_capture_request(
            None,
            &policy,
            capture::CaptureMode::Screenshot,
            TEST_CAPTURE_SEQUENCE_ID,
        );

        assert_eq!(response.outcome, control::ControlOutcome::Rejected);
        assert_eq!(
            response.message,
            Some(MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE.to_owned())
        );
    }

    #[test]
    fn control_capture_enqueue_uses_capture_dir_artifact_output() {
        let capture_dir = unique_test_capture_dir("enqueue-artifact");
        let policy = capture::CapturePolicy::local(&capture_dir);
        let (sender, receiver) = capture::capture_request_channel();
        let response = enqueue_mcp_capture_request(
            Some(&sender),
            &policy,
            capture::CaptureMode::Screenshot,
            TEST_CAPTURE_SEQUENCE_ID,
        );

        let serviced = receiver.service_pending_one_shot_with_readback(
            &policy,
            test_capture_frame_context(),
            synthetic_capture_frame,
        );
        let expected_relative_path = capture::default_artifact_relative_path(
            capture::CaptureMode::Screenshot,
            TEST_CAPTURE_SEQUENCE_ID,
            capture::CaptureFormat::Png,
        );

        assert_eq!(response.outcome, control::ControlOutcome::Deferred);
        assert_eq!(serviced, 1);
        assert!(capture_dir.join(expected_relative_path).exists());
        let _ = std::fs::remove_dir_all(capture_dir);
    }

    #[test]
    fn pitch_bounds_match_window_input_clamp() {
        assert_eq!(
            bounded_pitch(MCP_MIN_PITCH_RADIANS - TEST_PITCH_OFFSET),
            MCP_MIN_PITCH_RADIANS
        );
        assert_eq!(
            bounded_pitch(MCP_MAX_PITCH_RADIANS + TEST_PITCH_OFFSET),
            MCP_MAX_PITCH_RADIANS
        );
        assert_eq!(bounded_pitch(MCP_MIN_PITCH_RADIANS), MCP_MIN_PITCH_RADIANS);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Stevenarella")]
struct Opt {
    /// Server to connect to
    #[structopt(short = "s", long = "server")]
    server: Option<String>,

    /// Username for offline servers
    #[structopt(short = "u", long = "username")]
    username: Option<String>,

    /// Log decoded packets received from network
    #[structopt(short = "n", long = "network-debug")]
    network_debug: bool,

    /// Parse a network packet from a file
    #[structopt(short = "N", long = "network-parse-packet")]
    network_parse_packet: Option<String>,

    /// Protocol version to use in the autodetection ping
    #[structopt(short = "p", long = "default-protocol-version")]
    default_protocol_version: Option<String>,

    /// Enable MCP over stdio. Stdout is reserved for JSON-RPC while active.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-stdio")]
    mcp_stdio: bool,

    /// Enable MCP over a TCP socket, e.g. 127.0.0.1:4700.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-listen")]
    mcp_listen: Option<String>,

    /// Environment variable containing the MCP token for non-loopback TCP binds.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "mcp-token-env")]
    mcp_token_env: Option<String>,

    /// Directory for durable frame capture artifacts.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-dir", parse(from_os_str))]
    capture_dir: Option<PathBuf>,

    /// Startup recording frame rate. Requires --capture-dir and duration or frame count.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-fps")]
    capture_record_fps: Option<u16>,

    /// Startup recording frame count bound.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-frames")]
    capture_record_frames: Option<u32>,

    /// Startup recording duration bound in milliseconds.
    #[cfg(not(target_arch = "wasm32"))]
    #[structopt(long = "capture-record-duration-ms")]
    capture_record_duration_millis: Option<u64>,
}

#[cfg(not(target_arch = "wasm32"))]
fn capture_policy_from_opt(opt: &Opt) -> capture::CapturePolicy {
    match &opt.capture_dir {
        Some(capture_dir) => capture::CapturePolicy::local(capture_dir),
        None => capture::CapturePolicy::memory(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn startup_recording_request_from_opt(opt: &Opt) -> Option<capture::CaptureRequest> {
    let recording_requested = opt.capture_record_fps.is_some()
        || opt.capture_record_frames.is_some()
        || opt.capture_record_duration_millis.is_some();
    if !recording_requested {
        return None;
    }
    let recording = opt
        .capture_record_fps
        .map(|frame_rate_hz| capture::RecordingBounds {
            frame_rate_hz,
            max_frames: opt.capture_record_frames,
            max_duration_millis: opt.capture_record_duration_millis,
        });
    Some(capture::CaptureRequest {
        mode: capture::CaptureMode::Recording,
        format: capture::CaptureFormat::Png,
        output: capture::CaptureOutput::Artifact {
            relative_path: capture::default_recording_relative_dir(
                capture::CAPTURE_SEQUENCE_INITIAL,
            ),
        },
        includes_ui: true,
        recording,
        sequence_id: Some(capture::CAPTURE_SEQUENCE_INITIAL),
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn duration_to_millis_saturated(duration: Duration) -> u64 {
    let millis = duration.as_millis();
    if millis > u128::from(u64::MAX) {
        return u64::MAX;
    }
    millis as u64
}

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate console_error_panic_hook;
        pub use console_error_panic_hook::set_once as set_panic_hook;

        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        #[wasm_bindgen]
        pub fn main() { main2(); }
    } else {
        #[inline]
        pub fn main() { main2(); }
    }
}

fn init_config_dir() {
    if std::path::Path::new("conf.cfg").exists() {
        return;
    }

    if let Some(mut path) = dirs::config_dir() {
        path.push("Stevenarella");
        if !path.exists() {
            std::fs::create_dir_all(path.clone()).unwrap();
        }
        std::env::set_current_dir(path).unwrap();
    }
}

fn main2() {
    #[cfg(target_arch = "wasm32")]
    set_panic_hook();

    init_config_dir();
    let opt = Opt::from_args();
    #[cfg(not(target_arch = "wasm32"))]
    let capture_policy = capture_policy_from_opt(&opt);
    #[cfg(not(target_arch = "wasm32"))]
    let active_capture_recording = match startup_recording_request_from_opt(&opt) {
        Some(request) => {
            match capture::start_recording(request, &capture_policy, CAPTURE_START_MILLIS) {
                Ok(recording) => Some(recording),
                Err(err) => {
                    eprintln!("Invalid capture recording options: {:?}", err);
                    std::process::exit(2);
                }
            }
        }
        None => None,
    };
    let con = Arc::new(Mutex::new(console::Console::new()));
    #[cfg(not(target_arch = "wasm32"))]
    if opt.mcp_stdio {
        con.lock().unwrap().set_terminal_output_enabled(false);
    }
    let proxy = console::ConsoleProxy::new(con.clone());

    log::set_boxed_logger(Box::new(proxy)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    info!("Starting steven");

    #[cfg(not(target_arch = "wasm32"))]
    let (mcp_command_sender, mcp_command_receiver) = mcp::control_command_channel();
    #[cfg(not(target_arch = "wasm32"))]
    let (mcp_capture_request_sender, mcp_capture_request_receiver) =
        capture::capture_request_channel();
    #[cfg(not(target_arch = "wasm32"))]
    let capture_sequence_id = Arc::new(AtomicU64::new(capture::CAPTURE_SEQUENCE_INITIAL));

    #[cfg(not(target_arch = "wasm32"))]
    let _mcp_runtime = {
        let mcp_options = mcp::McpTransportOptions::from_cli(
            opt.mcp_stdio,
            opt.mcp_listen.clone(),
            opt.mcp_token_env.clone(),
        );
        if mcp_options.has_transport() {
            let validated = match mcp::validate_process_transport_options(&mcp_options) {
                Ok(validated) => validated,
                Err(err) => {
                    error!("Invalid MCP transport options: {:?}", err);
                    std::process::exit(2);
                }
            };
            let capture_tools = mcp::McpCaptureTools::new(
                mcp_capture_request_sender.clone(),
                capture_policy.clone(),
                Arc::clone(&capture_sequence_id),
            );
            match mcp::start_process_transport_with_capture(
                validated,
                Some(mcp_command_sender.clone()),
                Some(capture_tools),
            ) {
                Ok(runtime) => Some(runtime),
                Err(err) => {
                    error!("Failed to start MCP transport: {:?}", err);
                    std::process::exit(2);
                }
            }
        } else {
            None
        }
    };

    let (vars, mut vsync) = {
        let mut vars = console::Vars::new();
        vars.register(CL_BRAND);
        console::register_vars(&mut vars);
        auth::register_vars(&mut vars);
        settings::register_vars(&mut vars);
        vars.load_config();
        vars.save_config();
        con.lock().unwrap().configure(&vars);
        let vsync = *vars.get(settings::R_VSYNC);
        (Rc::new(vars), vsync)
    };

    let (res, mut resui) = resources::Manager::new();
    let resource_manager = Arc::new(RwLock::new(res));

    let events_loop = winit::event_loop::EventLoop::new();

    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Stevenarella")
        .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0));

    #[cfg(target_arch = "wasm32")]
    let (context, shader_version, dpi_factor, winit_window) = {
        let winit_window = window_builder.build(&events_loop).unwrap();
        let dpi_factor = winit_window.scale_factor();

        use winit::platform::web::WindowExtWebSys;

        let canvas = winit_window.canvas();

        let html_window = web_sys::window().unwrap();
        let document = html_window.document().unwrap();
        let body = document.body().unwrap();

        body.append_child(&canvas)
            .expect("Append canvas to HTML body");

        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .expect("Failed to get WebGL2 context")
            .expect("Failed to create WebGL2 context, is WebGL2 support enabled? (https://get.webgl.org/webgl2/)")
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        (
            glow::Context::from_webgl2_context(webgl2_context),
            "#version 300 es", // WebGL 2
            dpi_factor,
            winit_window,
        )
    };

    #[cfg(not(target_arch = "wasm32"))]
    let (context, shader_version, dpi_factor, glutin_window) = {
        let glutin_window = glutin::ContextBuilder::new()
            .with_stencil_buffer(0)
            .with_depth_buffer(24)
            .with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            })
            .with_gl_profile(glutin::GlProfile::Core)
            .with_vsync(vsync)
            .build_windowed(window_builder, &events_loop)
            .expect("Could not create glutin window.");
        let dpi_factor = glutin_window.window().scale_factor();

        let glutin_window = unsafe {
            glutin_window
                .make_current()
                .expect("Could not set current context.")
        };

        let context = unsafe {
            glow::Context::from_loader_function(|s| glutin_window.get_proc_address(s) as *const _)
        };

        let shader_version = match glutin_window.get_api() {
            glutin::Api::OpenGl => "#version 150",      // OpenGL 3.2
            glutin::Api::OpenGlEs => "#version 300 es", // OpenGL ES 3.0 (similar to WebGL 2)
            glutin::Api::WebGl => {
                panic!("unexpectedly received WebGl API with glutin, expected to use glow codepath")
            }
        };

        (context, shader_version, dpi_factor, glutin_window)
    };

    gl::init(context);
    info!("Shader version: {}", shader_version);

    let renderer = render::Renderer::new(resource_manager.clone(), shader_version);
    let ui_container = ui::Container::new();

    let mut last_frame = Instant::now();

    let mut screen_sys = screen::ScreenSystem::new();
    if opt.server.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            screen_sys.add_screen(Box::new(screen::Login::new(vars.clone())));
        }

        #[cfg(target_arch = "wasm32")]
        {
            screen_sys.add_screen(Box::new(screen::ServerList::new(None)));
        }
    }

    if let Some(username) = opt.username {
        vars.set(auth::CL_USERNAME, username);
    }

    let textures = renderer.get_textures();
    let default_protocol_version = protocol::versions::protocol_name_to_protocol_version(
        opt.default_protocol_version.unwrap_or_default(),
    );
    let mut game = Game {
        server: server::Server::dummy_server(resource_manager.clone()),
        focused: false,
        renderer,
        screen_sys,
        resource_manager: resource_manager.clone(),
        console: con,
        vars,
        should_close: false,
        chunk_builder: chunk_builder::ChunkBuilder::new(resource_manager, textures),
        connect_reply: None,
        dpi_factor,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        last_mouse_xrel: 0.0,
        last_mouse_yrel: 0.0,
        is_ctrl_pressed: false,
        is_logo_pressed: false,
        is_fullscreen: false,
        default_protocol_version,
        #[cfg(not(target_arch = "wasm32"))]
        mcp_command_receiver: Some(mcp_command_receiver),
        #[cfg(not(target_arch = "wasm32"))]
        mcp_capture_request_sender: Some(mcp_capture_request_sender),
        #[cfg(not(target_arch = "wasm32"))]
        mcp_capture_request_receiver: Some(mcp_capture_request_receiver),
        #[cfg(not(target_arch = "wasm32"))]
        mcp_release_left_after_server_tick: false,
        #[cfg(not(target_arch = "wasm32"))]
        capture_policy,
        #[cfg(not(target_arch = "wasm32"))]
        capture_sequence_id,
        #[cfg(not(target_arch = "wasm32"))]
        active_capture_recording,
        #[cfg(not(target_arch = "wasm32"))]
        capture_started_at: Instant::now(),
    };
    game.renderer.camera.pos = cgmath::Point3::new(0.5, 13.2, 0.5);

    if opt.network_debug {
        protocol::enable_network_debug();
    }

    if let Some(filename) = opt.network_parse_packet {
        let data = fs::read(filename).unwrap();
        protocol::try_parse_packet(data, default_protocol_version);
        return;
    }

    if opt.server.is_some() {
        game.connect_to(&opt.server.unwrap());
    }

    let mut last_resource_version = 0;

    #[cfg(target_arch = "wasm32")]
    let winit_window = Rc::new(RefCell::new(winit_window));

    let game = Rc::new(RefCell::new(game));
    let ui_container = Rc::new(RefCell::new(ui_container));

    #[cfg(target_arch = "wasm32")]
    {
        let winit_window = Rc::clone(&winit_window);
        let game = Rc::clone(&game);
        let ui_container = Rc::clone(&ui_container);

        // Based on https://github.com/grovesNL/glow/blob/2d42c5b105d979efe764191b5b1ce78fab99ffcf/src/web_sys.rs#L3258
        fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.as_ref().unchecked_ref())
                .unwrap();
        }

        let f = Rc::new(RefCell::new(None));

        let mut last_timestamp = None;
        let mut running = true;

        *f.borrow_mut() = Some(Closure::wrap(Box::new({
            let f = f.clone();

            move |timestamp: f64| {
                let dt = last_timestamp.map_or(Duration::from_secs(0), |last_timestamp: f64| {
                    let dt_ms = (timestamp - last_timestamp).max(0.0);
                    let dt_secs = dt_ms / 1000.0;

                    Duration::from_secs_f64(dt_secs)
                });
                last_timestamp = Some(timestamp);

                let winit_window = winit_window.borrow_mut();
                let mut game = game.borrow_mut();
                let mut ui_container = ui_container.borrow_mut();

                tick_all(
                    &winit_window,
                    &mut game,
                    &mut ui_container,
                    &mut last_frame,
                    &mut resui,
                    &mut last_resource_version,
                    &mut vsync,
                );
                println!("render_loop");

                if !running {
                    let _ = f.borrow_mut().take();
                    return;
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut(f64)>));

        request_animation_frame(f.borrow().as_ref().unwrap());
    }

    #[cfg(target_arch = "wasm32")]
    let winit_window = Rc::clone(&winit_window);

    let game = Rc::clone(&game);
    let ui_container = Rc::clone(&ui_container);
    events_loop.run(move |event, _event_loop, control_flow| {
        #[cfg(target_arch = "wasm32")]
        let winit_window = winit_window.borrow_mut();

        #[cfg(not(target_arch = "wasm32"))]
        let winit_window = glutin_window.window();

        let mut game = game.borrow_mut();
        let mut ui_container = ui_container.borrow_mut();

        #[cfg(target_arch = "wasm32")]
        {
            *control_flow = winit::event_loop::ControlFlow::Wait;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            *control_flow = winit::event_loop::ControlFlow::Poll;
        }

        #[cfg(not(target_arch = "wasm32"))]
        if let winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::Resized(physical_size),
            ..
        } = event
        {
            glutin_window.resize(physical_size);
        }

        #[allow(clippy::needless_borrow)] // needless for native, not for web
        if !handle_window_event(&winit_window, &mut game, &mut ui_container, event) {
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            tick_all(
                winit_window,
                &mut game,
                &mut ui_container,
                &mut last_frame,
                &mut resui,
                &mut last_resource_version,
                &mut vsync,
            );

            glutin_window
                .swap_buffers()
                .expect("Failed to swap GL buffers");
        }

        if game.should_close {
            *control_flow = winit::event_loop::ControlFlow::Exit;
        }
    });
}

fn tick_all(
    window: &winit::window::Window,
    game: &mut Game,
    ui_container: &mut ui::Container,
    last_frame: &mut Instant,
    resui: &mut resources::ManagerUI,
    last_resource_version: &mut usize,
    vsync: &mut bool,
) {
    let now = Instant::now();
    let diff = now.duration_since(*last_frame);
    *last_frame = now;
    let frame_time = 1e9f64 / 60.0;
    let delta = (diff.subsec_nanos() as f64) / frame_time;
    let physical_size = window.inner_size();
    let (physical_width, physical_height) = physical_size.into();
    let (width, height) = physical_size.to_logical::<f64>(game.dpi_factor).into();

    let version = {
        let try_res = game.resource_manager.try_write();
        if let Ok(mut res) = try_res {
            res.tick(resui, ui_container, delta);
            res.version()
        } else {
            // TODO: why does game.resource_manager.write() sometimes deadlock?
            //warn!("Failed to obtain mutable reference to resource manager!");
            *last_resource_version
        }
    };
    *last_resource_version = version;

    let vsync_changed = *game.vars.get(settings::R_VSYNC);
    if *vsync != vsync_changed {
        error!("Changing vsync currently requires restarting");
        game.should_close = true;
        // TODO: after https://github.com/tomaka/glutin/issues/693 Allow changing vsync on a Window
        //vsync = vsync_changed;
    }
    let fps_cap = *game.vars.get(settings::R_MAX_FPS);

    game.tick(delta);
    #[cfg(not(target_arch = "wasm32"))]
    game.drain_mcp_control_commands();
    game.server.tick(&mut game.renderer, delta);
    #[cfg(not(target_arch = "wasm32"))]
    game.release_mcp_control_buttons_after_server_tick();

    // Check if window is valid, it might be minimized
    if physical_width == 0 || physical_height == 0 {
        return;
    }

    game.renderer.update_camera(physical_width, physical_height);
    game.server.world.compute_render_list(&mut game.renderer);
    game.chunk_builder
        .tick(&mut game.server.world, &mut game.renderer, version);

    game.screen_sys
        .tick(delta, &mut game.renderer, ui_container);
    /* TODO: open console for chat messages
    if let Some(received_chat_at) = game.server.received_chat_at {
        if Instant::now().duration_since(received_chat_at).as_secs() < 5 {
            game.console.lock().unwrap().activate()
            // TODO: automatically deactivate the console after inactivity
        }
    }
    */
    game.console
        .lock()
        .unwrap()
        .tick(ui_container, &game.renderer, delta, width);
    ui_container.tick(&mut game.renderer, delta, width, height);
    game.renderer.tick(
        &mut game.server.world,
        delta,
        width as u32,
        height as u32,
        physical_width,
        physical_height,
    );
    #[cfg(not(target_arch = "wasm32"))]
    game.service_pending_mcp_capture_requests();
    #[cfg(not(target_arch = "wasm32"))]
    game.service_active_capture_recording();

    if fps_cap > 0 && !*vsync {
        let frame_time = now.elapsed();
        let sleep_interval = Duration::from_millis(1000 / fps_cap as u64);
        if frame_time < sleep_interval {
            thread::sleep(sleep_interval - frame_time);
        }
    }
}

fn handle_window_event<T>(
    window: &winit::window::Window,
    game: &mut Game,
    ui_container: &mut ui::Container,
    event: winit::event::Event<T>,
) -> bool {
    use winit::event::*;
    let cursor_grab_mode = if cfg!(target_os = "macos") {
        winit::window::CursorGrabMode::Locked
    } else {
        winit::window::CursorGrabMode::Confined
    };
    match event {
        Event::MainEventsCleared => return true,
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion {
                delta: (xrel, yrel),
            },
            ..
        } => {
            let (rx, ry) = if xrel > 1000.0 || yrel > 1000.0 {
                // Heuristic for if we were passed an absolute value instead of relative
                // Workaround https://github.com/tomaka/glutin/issues/1084 MouseMotion event returns absolute instead of relative values, when running Linux in a VM
                // Note SDL2 had a hint to handle this scenario:
                // sdl2::hint::set_with_priority("SDL_MOUSE_RELATIVE_MODE_WARP", "1", &sdl2::hint::Hint::Override);
                let s = 8000.0 + 0.01;
                (
                    (xrel - game.last_mouse_xrel) / s,
                    (yrel - game.last_mouse_yrel) / s,
                )
            } else {
                let s = 2000.0 + 0.01;
                (xrel / s, yrel / s)
            };

            game.last_mouse_xrel = xrel;
            game.last_mouse_yrel = yrel;

            use std::f64::consts::PI;

            if game.focused {
                window.set_cursor_grab(cursor_grab_mode).unwrap();
                window.set_cursor_visible(false);
                if let Some(player) = game.server.player {
                    let rotation = game
                        .server
                        .entities
                        .get_component_mut(player, game.server.rotation)
                        .unwrap();
                    rotation.yaw -= rx;
                    rotation.pitch -= ry;
                    if rotation.pitch < (PI / 2.0) + 0.01 {
                        rotation.pitch = (PI / 2.0) + 0.01;
                    }
                    if rotation.pitch > (PI / 2.0) * 3.0 - 0.01 {
                        rotation.pitch = (PI / 2.0) * 3.0 - 0.01;
                    }
                }
            } else {
                window
                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                    .unwrap();
                window.set_cursor_visible(true);
            }
        }

        Event::WindowEvent { event, .. } => {
            match event {
                WindowEvent::ModifiersChanged(modifiers_state) => {
                    game.is_ctrl_pressed = modifiers_state.ctrl();
                    game.is_logo_pressed = modifiers_state.logo();
                }
                WindowEvent::CloseRequested => game.should_close = true,
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    game.dpi_factor = scale_factor;
                }

                WindowEvent::ReceivedCharacter(codepoint) => {
                    if !game.focused && !game.is_ctrl_pressed && !game.is_logo_pressed {
                        ui_container.key_type(game, codepoint);
                    }

                    #[cfg(target_os = "macos")]
                    if game.is_logo_pressed && codepoint == 'q' {
                        game.should_close = true;
                    }
                }

                WindowEvent::MouseInput { state, button, .. } => match (state, button) {
                    (ElementState::Released, MouseButton::Left) => {
                        let physical_size = window.inner_size();
                        let (width, height) =
                            physical_size.to_logical::<f64>(game.dpi_factor).into();

                        if game.server.is_connected()
                            && !game.focused
                            && !game.screen_sys.is_current_closable()
                        {
                            game.focused = true;
                            window.set_cursor_grab(cursor_grab_mode).unwrap();
                            window.set_cursor_visible(false);
                        } else if !game.focused {
                            #[cfg(not(target_arch = "wasm32"))]
                            // TODO: after Pointer Lock https://github.com/rust-windowing/winit/issues/1674
                            window
                                .set_cursor_grab(winit::window::CursorGrabMode::None)
                                .unwrap();
                            window.set_cursor_visible(true);
                            ui_container.click_at(
                                game,
                                game.last_mouse_x,
                                game.last_mouse_y,
                                width,
                                height,
                            );
                        }

                        if game.focused {
                            game.server.on_left_mouse_button(false);
                        }
                    }
                    (ElementState::Pressed, MouseButton::Left) => {
                        if game.focused {
                            game.server.on_left_mouse_button(true);
                        }
                    }
                    (ElementState::Released, MouseButton::Right) => {
                        if game.focused {
                            game.server.on_right_mouse_button(false);
                            game.server.on_right_click(&mut game.renderer);
                        }
                    }
                    (ElementState::Pressed, MouseButton::Right) => {
                        if game.focused {
                            game.server.on_right_mouse_button(true);
                            game.server.on_right_click(&mut game.renderer);
                        }
                    }
                    (_, _) => (),
                },
                WindowEvent::CursorMoved { position, .. } => {
                    let (x, y) = position.to_logical::<f64>(game.dpi_factor).into();
                    game.last_mouse_x = x;
                    game.last_mouse_y = y;

                    if !game.focused {
                        let physical_size = window.inner_size();
                        let (width, height) =
                            physical_size.to_logical::<f64>(game.dpi_factor).into();
                        ui_container.hover_at(game, x, y, width, height);
                    }
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    // TODO: line vs pixel delta? does pixel scrolling (e.g. touchpad) need scaling?
                    match delta {
                        MouseScrollDelta::LineDelta(x, y) => {
                            game.screen_sys.on_scroll(x.into(), y.into());
                        }
                        MouseScrollDelta::PixelDelta(position) => {
                            let (x, y) = position.into();
                            game.screen_sys.on_scroll(x, y);
                        }
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    match (input.state, input.virtual_keycode) {
                        (ElementState::Released, Some(VirtualKeyCode::Escape)) => {
                            if game.focused {
                                window
                                    .set_cursor_grab(winit::window::CursorGrabMode::None)
                                    .unwrap();
                                window.set_cursor_visible(true);
                                game.focused = false;
                                game.screen_sys.replace_screen(Box::new(
                                    screen::SettingsMenu::new(game.vars.clone(), true),
                                ));
                            } else if game.screen_sys.is_current_closable() {
                                window.set_cursor_grab(cursor_grab_mode).unwrap();
                                window.set_cursor_visible(false);
                                game.focused = true;
                                game.screen_sys.pop_screen();
                            }
                        }
                        (ElementState::Pressed, Some(VirtualKeyCode::Grave)) => {
                            game.console.lock().unwrap().toggle();
                        }
                        (ElementState::Pressed, Some(VirtualKeyCode::F11)) => {
                            if !game.is_fullscreen {
                                // TODO: support options for exclusive and simple fullscreen
                                // see https://docs.rs/glutin/0.22.0-alpha5/glutin/window/struct.Window.html#method.set_fullscreen
                                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                                    window.current_monitor(),
                                )));
                            } else {
                                window.set_fullscreen(None);
                            }

                            game.is_fullscreen = !game.is_fullscreen;
                        }
                        (ElementState::Pressed, Some(key)) => {
                            if game.focused {
                                if let Some(steven_key) =
                                    settings::Stevenkey::get_by_keycode(key, &game.vars)
                                {
                                    game.server.key_press(true, steven_key);
                                }
                            } else {
                                let ctrl_pressed = game.is_ctrl_pressed || game.is_logo_pressed;
                                ui_container.key_press(game, key, true, ctrl_pressed);
                            }
                        }
                        (ElementState::Released, Some(key)) => {
                            if game.focused {
                                if let Some(steven_key) =
                                    settings::Stevenkey::get_by_keycode(key, &game.vars)
                                {
                                    game.server.key_press(false, steven_key);
                                }
                            } else {
                                let ctrl_pressed = game.is_ctrl_pressed;
                                ui_container.key_press(game, key, false, ctrl_pressed);
                            }
                        }
                        (_, None) => (),
                    }
                }
                _ => (),
            }
        }

        _ => (),
    }

    false
}
