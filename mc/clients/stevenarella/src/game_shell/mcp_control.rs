use log::info;
use steven_shared as shared;

use crate::{capture, control, protocol, settings, Game};

const MCP_STATUS_APPLIED_MESSAGE: &str = "status reported";
const MCP_CONNECT_STARTED_MESSAGE: &str = "connect started";
const MCP_CONNECT_ALREADY_ACTIVE_MESSAGE: &str = "connect already active";
const MCP_CONNECT_ALREADY_CONNECTED_MESSAGE: &str =
    "disconnect before connecting to another server";
const MCP_DISCONNECT_APPLIED_MESSAGE: &str = "disconnect applied";
const MCP_DISCONNECT_NOT_CONNECTED_MESSAGE: &str = "no active connection to disconnect";
const MCP_KEY_APPLIED_MESSAGE: &str = "key applied";
const MCP_LOOK_APPLIED_MESSAGE: &str = "look applied";
const MCP_MOUSE_APPLIED_MESSAGE: &str = "mouse applied";
const MCP_USE_ITEM_APPLIED_MESSAGE: &str = "use item applied";
const MCP_ATTACK_APPLIED_MESSAGE: &str = "attack applied";
const MCP_CHAT_APPLIED_MESSAGE: &str = "chat sent";
const MCP_RESOURCE_PACK_STATUS_APPLIED_MESSAGE: &str = "resource pack status sent";
const MCP_SIGN_EDITOR_UPDATE_APPLIED_MESSAGE: &str = "sign editor update sent";
const MCP_SIGN_EDITOR_OPEN_MISSING_MESSAGE: &str = "sign editor open state missing";
const MCP_SIGN_EDITOR_OPEN_POSITION_MISMATCH_MESSAGE: &str = "sign editor open position mismatch";
const MCP_CAPTURE_DEFERRED_MESSAGE: &str = "capture queued for next rendered frame";
const MCP_CAPTURE_QUEUE_CLOSED_MESSAGE: &str = "capture queue closed";
const MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE: &str = "capture queue unavailable";
const MCP_CAPTURE_REQUEST_INVALID_MESSAGE: &str = "invalid capture request";
const MCP_REQUIRES_CONNECTED_MESSAGE: &str = "command requires an active connection";
const MCP_REQUIRES_PLAYER_MESSAGE: &str = "command requires a player entity";
const MCP_MIN_PITCH_EPSILON_RADIANS: f64 = 0.01;
const MCP_MIN_PITCH_RADIANS: f64 = std::f64::consts::FRAC_PI_2 + MCP_MIN_PITCH_EPSILON_RADIANS;
const MCP_MAX_PITCH_RADIANS: f64 =
    std::f64::consts::PI + std::f64::consts::FRAC_PI_2 - MCP_MIN_PITCH_EPSILON_RADIANS;
const SIGN_EDITOR_LINE_INDEX_1: usize = 0;
const SIGN_EDITOR_LINE_INDEX_2: usize = 1;
const SIGN_EDITOR_LINE_INDEX_3: usize = 2;
const SIGN_EDITOR_LINE_INDEX_4: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ControlShellState {
    pub connected: bool,
    pub connecting: bool,
    pub focused: bool,
    pub player_rotation_available: bool,
    pub sign_editor_open_position: Option<control::BlockPosition>,
    pub capture_queue_available: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ControlCommandPlan {
    Respond(control::ControlResponse),
    Connect {
        address: String,
        response: control::ControlResponse,
    },
    CancelConnect {
        response: control::ControlResponse,
    },
    Disconnect {
        response: control::ControlResponse,
    },
    Key {
        key: settings::Stevenkey,
        down: bool,
        response: control::ControlResponse,
    },
    Look {
        yaw_delta: f64,
        pitch_delta: f64,
        response: control::ControlResponse,
    },
    Mouse {
        button: control::MouseButton,
        down: bool,
        response: control::ControlResponse,
    },
    UseItem {
        response: control::ControlResponse,
    },
    Attack {
        response: control::ControlResponse,
    },
    Chat {
        message: String,
        response: control::ControlResponse,
    },
    ResourcePackStatus {
        decision: control::ResourcePackStatusDecision,
        response: control::ControlResponse,
    },
    SignEditorUpdate {
        decision: control::SignEditorUpdateDecision,
        response: control::ControlResponse,
    },
    Capture {
        mode: capture::CaptureMode,
        response: control::ControlResponse,
    },
}

pub(crate) fn plan_mcp_control_command(
    command: control::ControlCommand,
    state: ControlShellState,
) -> ControlCommandPlan {
    if !state.connected {
        if let Some(response) = disconnected_control_rejection(&command) {
            return ControlCommandPlan::Respond(response);
        }
    }

    match command {
        control::ControlCommand::Status => {
            ControlCommandPlan::Respond(control::ControlResponse::applied(control_status_message(
                state.connected,
                state.connecting,
                state.focused,
            )))
        }
        control::ControlCommand::Connect { address } => plan_connect(address, state),
        control::ControlCommand::Disconnect => plan_disconnect(state),
        control::ControlCommand::Key { key, down } => ControlCommandPlan::Key {
            key: control_key_to_stevenkey(key),
            down,
            response: control::ControlResponse::applied(MCP_KEY_APPLIED_MESSAGE),
        },
        control::ControlCommand::Look {
            yaw_delta,
            pitch_delta,
        } => plan_look(yaw_delta, pitch_delta, state),
        control::ControlCommand::Mouse { button, down } => ControlCommandPlan::Mouse {
            button,
            down,
            response: control::ControlResponse::applied(MCP_MOUSE_APPLIED_MESSAGE),
        },
        control::ControlCommand::UseItem => ControlCommandPlan::UseItem {
            response: control::ControlResponse::applied(MCP_USE_ITEM_APPLIED_MESSAGE),
        },
        control::ControlCommand::Attack => ControlCommandPlan::Attack {
            response: control::ControlResponse::applied(MCP_ATTACK_APPLIED_MESSAGE),
        },
        control::ControlCommand::Chat { message } => ControlCommandPlan::Chat {
            message,
            response: control::ControlResponse::applied(MCP_CHAT_APPLIED_MESSAGE),
        },
        control::ControlCommand::ResourcePackStatus(decision) => {
            ControlCommandPlan::ResourcePackStatus {
                decision,
                response: control::ControlResponse::applied(
                    MCP_RESOURCE_PACK_STATUS_APPLIED_MESSAGE,
                ),
            }
        }
        control::ControlCommand::SignEditorUpdate(decision) => {
            plan_sign_editor_update(decision, state)
        }
        control::ControlCommand::CaptureScreenshot => {
            plan_capture(capture::CaptureMode::Screenshot, state)
        }
        control::ControlCommand::CaptureLatestFrame => {
            plan_capture(capture::CaptureMode::LatestFrame, state)
        }
    }
}

pub(crate) fn bounded_pitch(pitch: f64) -> f64 {
    pitch.max(MCP_MIN_PITCH_RADIANS).min(MCP_MAX_PITCH_RADIANS)
}

fn plan_connect(address: String, state: ControlShellState) -> ControlCommandPlan {
    if state.connecting {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_CONNECT_ALREADY_ACTIVE_MESSAGE,
        ));
    }
    if state.connected {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_CONNECT_ALREADY_CONNECTED_MESSAGE,
        ));
    }
    ControlCommandPlan::Connect {
        address,
        response: control::ControlResponse::applied(MCP_CONNECT_STARTED_MESSAGE),
    }
}

fn plan_disconnect(state: ControlShellState) -> ControlCommandPlan {
    if state.connecting {
        return ControlCommandPlan::CancelConnect {
            response: control::ControlResponse::applied(MCP_DISCONNECT_APPLIED_MESSAGE),
        };
    }
    if !state.connected {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_DISCONNECT_NOT_CONNECTED_MESSAGE,
        ));
    }
    ControlCommandPlan::Disconnect {
        response: control::ControlResponse::applied(MCP_DISCONNECT_APPLIED_MESSAGE),
    }
}

fn plan_look(yaw_delta: f64, pitch_delta: f64, state: ControlShellState) -> ControlCommandPlan {
    if !state.player_rotation_available {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_REQUIRES_PLAYER_MESSAGE,
        ));
    }
    ControlCommandPlan::Look {
        yaw_delta,
        pitch_delta,
        response: control::ControlResponse::applied(MCP_LOOK_APPLIED_MESSAGE),
    }
}

fn plan_sign_editor_update(
    decision: control::SignEditorUpdateDecision,
    state: ControlShellState,
) -> ControlCommandPlan {
    let Some(open_position) = state.sign_editor_open_position else {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_SIGN_EDITOR_OPEN_MISSING_MESSAGE,
        ));
    };
    if open_position != decision.position {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_SIGN_EDITOR_OPEN_POSITION_MISMATCH_MESSAGE,
        ));
    }
    ControlCommandPlan::SignEditorUpdate {
        decision,
        response: control::ControlResponse::applied(MCP_SIGN_EDITOR_UPDATE_APPLIED_MESSAGE),
    }
}

fn plan_capture(mode: capture::CaptureMode, state: ControlShellState) -> ControlCommandPlan {
    if !state.capture_queue_available {
        return ControlCommandPlan::Respond(control::ControlResponse::rejected(
            MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE,
        ));
    }
    ControlCommandPlan::Capture {
        mode,
        response: control::ControlResponse::deferred(MCP_CAPTURE_DEFERRED_MESSAGE),
    }
}

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

fn disconnected_control_rejection(
    command: &control::ControlCommand,
) -> Option<control::ControlResponse> {
    command
        .requires_connection()
        .then(|| control::ControlResponse::rejected(MCP_REQUIRES_CONNECTED_MESSAGE))
}

fn control_status_message(connected: bool, connecting: bool, focused: bool) -> String {
    format!(
        "{MCP_STATUS_APPLIED_MESSAGE}: connected={connected} connecting={connecting} focused={focused}"
    )
}

fn control_block_position_to_shared(position: control::BlockPosition) -> shared::Position {
    shared::Position::new(position.x, position.y, position.z)
}

fn shared_position_to_control_block(position: shared::Position) -> control::BlockPosition {
    control::BlockPosition::new(position.x, position.y, position.z)
}

#[cfg(test)]
fn sign_editor_open_matches(
    open_position: control::BlockPosition,
    expected: control::BlockPosition,
) -> bool {
    open_position == expected
}

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

fn capture_queue_error_response(err: capture::CaptureQueueError) -> control::ControlResponse {
    match err {
        capture::CaptureQueueError::QueueClosed => {
            control::ControlResponse::rejected(MCP_CAPTURE_QUEUE_CLOSED_MESSAGE)
        }
        capture::CaptureQueueError::RateLimitExceeded { .. }
        | capture::CaptureQueueError::Validation(_) => {
            control::ControlResponse::rejected(MCP_CAPTURE_REQUEST_INVALID_MESSAGE)
        }
    }
}

fn enqueue_mcp_capture_request(
    sender: Option<&capture::CaptureRequestSender>,
    policy: &capture::CapturePolicy,
    mode: capture::CaptureMode,
    sequence_id: u64,
) -> control::ControlResponse {
    let Some(sender) = sender else {
        return control::ControlResponse::rejected(MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE);
    };
    let output = one_shot_mcp_capture_output(policy, mode, sequence_id);
    match sender.enqueue_deferred(one_shot_mcp_capture_request(mode, output, sequence_id)) {
        Ok(_) => control::ControlResponse::deferred(MCP_CAPTURE_DEFERRED_MESSAGE),
        Err(err) => capture_queue_error_response(err),
    }
}

impl Game {
    pub fn drain_mcp_control_commands(&mut self) -> usize {
        let Some(receiver) = self.mcp_command_receiver.take() else {
            return 0;
        };
        let drained =
            receiver.drain_pending_with_handler(|command| self.apply_mcp_control_command(command));
        self.mcp_command_receiver = Some(receiver);
        drained
    }

    pub fn release_mcp_control_buttons_after_server_tick(&mut self) {
        if self.mcp_release_left_after_server_tick {
            self.server.on_left_mouse_button(false);
            self.mcp_release_left_after_server_tick = false;
        }
    }

    fn apply_mcp_control_command(
        &mut self,
        command: control::ControlCommand,
    ) -> control::ControlResponse {
        let state = self.mcp_control_shell_state();
        let plan = plan_mcp_control_command(command, state);
        self.apply_mcp_control_plan(plan)
    }

    fn mcp_control_shell_state(&self) -> ControlShellState {
        let player_rotation_available = self
            .server
            .player
            .and_then(|player| {
                self.server
                    .entities
                    .get_component(player, self.server.rotation)
            })
            .is_some();
        ControlShellState {
            connected: self.server.is_connected(),
            connecting: self.connect_reply.is_some(),
            focused: self.focused,
            player_rotation_available,
            sign_editor_open_position: self
                .server
                .sign_editor_open_position()
                .map(shared_position_to_control_block),
            capture_queue_available: self.mcp_capture_request_sender.is_some(),
        }
    }

    fn apply_mcp_control_plan(&mut self, plan: ControlCommandPlan) -> control::ControlResponse {
        match plan {
            ControlCommandPlan::Respond(response) => response,
            ControlCommandPlan::Connect { address, response } => {
                self.connect_to(&address);
                response
            }
            ControlCommandPlan::CancelConnect { response } => {
                self.connect_reply = None;
                response
            }
            ControlCommandPlan::Disconnect { response } => {
                self.server.disconnect(None);
                self.focused = false;
                response
            }
            ControlCommandPlan::Key {
                key,
                down,
                response,
            } => {
                self.server.key_press(down, key);
                response
            }
            ControlCommandPlan::Look {
                yaw_delta,
                pitch_delta,
                response,
            } => self.apply_mcp_look(yaw_delta, pitch_delta, response),
            ControlCommandPlan::Mouse {
                button,
                down,
                response,
            } => {
                self.apply_mcp_mouse(button, down);
                response
            }
            ControlCommandPlan::UseItem { response } => {
                self.server.on_right_mouse_button(true);
                self.server.on_right_click(&mut self.renderer);
                self.server.on_right_mouse_button(false);
                response
            }
            ControlCommandPlan::Attack { response } => {
                self.server.on_left_mouse_button(true);
                self.mcp_release_left_after_server_tick = true;
                response
            }
            ControlCommandPlan::Chat { message, response } => {
                self.server
                    .write_packet(protocol::packet::play::serverbound::ChatMessage { message });
                response
            }
            ControlCommandPlan::ResourcePackStatus { decision, response } => {
                self.server
                    .write_packet(protocol::packet::play::serverbound::ResourcePackStatus {
                        result: protocol::VarInt(decision.status.status_code()),
                    });
                info!(
                    "MC-COMPAT-MILESTONE resource_pack_status_sent offer_id={} status=declined no_external_fetch=true",
                    decision.offer_id
                );
                response
            }
            ControlCommandPlan::SignEditorUpdate { decision, response } => {
                self.apply_mcp_sign_editor_update(decision, response)
            }
            ControlCommandPlan::Capture { mode, .. } => {
                let sequence_id = self.next_capture_sequence_id();
                enqueue_mcp_capture_request(
                    self.mcp_capture_request_sender.as_ref(),
                    &self.capture_policy,
                    mode,
                    sequence_id,
                )
            }
        }
    }

    fn apply_mcp_sign_editor_update(
        &mut self,
        decision: control::SignEditorUpdateDecision,
        response: control::ControlResponse,
    ) -> control::ControlResponse {
        let expected_position = control_block_position_to_shared(decision.position);
        self.server
            .write_packet(protocol::packet::play::serverbound::SetSign {
                location: expected_position,
                line1: decision.lines[SIGN_EDITOR_LINE_INDEX_1].clone(),
                line2: decision.lines[SIGN_EDITOR_LINE_INDEX_2].clone(),
                line3: decision.lines[SIGN_EDITOR_LINE_INDEX_3].clone(),
                line4: decision.lines[SIGN_EDITOR_LINE_INDEX_4].clone(),
            });
        info!(
            "MC-COMPAT-MILESTONE sign_update_sent position={},{},{} line_count={}",
            expected_position.x,
            expected_position.y,
            expected_position.z,
            control::SIGN_EDITOR_LINE_COUNT
        );
        response
    }

    fn apply_mcp_look(
        &mut self,
        yaw_delta: f64,
        pitch_delta: f64,
        response: control::ControlResponse,
    ) -> control::ControlResponse {
        let Some(player) = self.server.player else {
            return control::ControlResponse::rejected(MCP_REQUIRES_PLAYER_MESSAGE);
        };
        let Some(rotation) = self
            .server
            .entities
            .get_component_mut(player, self.server.rotation)
        else {
            return control::ControlResponse::rejected(MCP_REQUIRES_PLAYER_MESSAGE);
        };
        rotation.yaw += yaw_delta;
        rotation.pitch = bounded_pitch(rotation.pitch + pitch_delta);
        response
    }

    fn apply_mcp_mouse(&mut self, button: control::MouseButton, down: bool) {
        match button {
            control::MouseButton::Left => self.server.on_left_mouse_button(down),
            control::MouseButton::Right => {
                self.server.on_right_mouse_button(down);
                if down {
                    self.server.on_right_click(&mut self.renderer);
                }
            }
        }
    }

    fn next_capture_sequence_id(&mut self) -> u64 {
        self.capture_sequence_id
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ADDRESS: &str = "127.0.0.1:25565";
    const TEST_CHAT_MESSAGE: &str = "hello";
    const TEST_YAW_DELTA: f64 = 0.25;
    const TEST_PITCH_DELTA: f64 = -0.125;
    const TEST_PITCH_OFFSET: f64 = 1.0;
    const TEST_CAPTURE_WIDTH_PX: u32 = 2;
    const TEST_CAPTURE_HEIGHT_PX: u32 = 2;
    const TEST_CAPTURE_FRAME_ID: u64 = 42;
    const TEST_CAPTURE_SEQUENCE_ID: u64 = 7;
    const TEST_SIGN_X: i32 = 28;
    const TEST_SIGN_Y: i32 = 64;
    const TEST_SIGN_Z: i32 = 0;
    const TEST_POSITION_OFFSET: i32 = 1;
    const OUT_OF_RANGE_LOOK_DELTA: f64 =
        control::MAX_ABSOLUTE_LOOK_DELTA_RADIANS + TEST_PITCH_OFFSET;

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

    fn unique_test_capture_dir(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "stevenarella-main-capture-{name}-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        path
    }

    fn test_sign_position() -> control::BlockPosition {
        control::BlockPosition::new(TEST_SIGN_X, TEST_SIGN_Y, TEST_SIGN_Z)
    }

    fn test_sign_update() -> control::SignEditorUpdateDecision {
        control::SignEditorUpdateDecision {
            position: test_sign_position(),
            lines: [
                "MC".to_owned(),
                "Compat".to_owned(),
                "Sign".to_owned(),
                "Edit".to_owned(),
            ],
        }
    }

    fn disconnected_state() -> ControlShellState {
        ControlShellState {
            connected: false,
            connecting: false,
            focused: false,
            player_rotation_available: false,
            sign_editor_open_position: None,
            capture_queue_available: true,
        }
    }

    fn connected_state() -> ControlShellState {
        ControlShellState {
            connected: true,
            connecting: false,
            focused: true,
            player_rotation_available: true,
            sign_editor_open_position: Some(test_sign_position()),
            capture_queue_available: true,
        }
    }

    fn connecting_state() -> ControlShellState {
        ControlShellState {
            connected: false,
            connecting: true,
            focused: false,
            player_rotation_available: false,
            sign_editor_open_position: None,
            capture_queue_available: true,
        }
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
        assert!(!control::control_command_requires_connection(
            &control::ControlCommand::Status
        ));
        assert!(!control::control_command_requires_connection(
            &control::ControlCommand::Connect {
                address: TEST_ADDRESS.to_owned(),
            }
        ));
        assert!(!control::control_command_requires_connection(
            &control::ControlCommand::Disconnect
        ));
        assert!(!control::control_command_requires_connection(
            &control::ControlCommand::CaptureScreenshot
        ));
        assert!(!control::control_command_requires_connection(
            &control::ControlCommand::CaptureLatestFrame
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::Key {
                key: control::ControlKey::Forward,
                down: true,
            }
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::Look {
                yaw_delta: TEST_YAW_DELTA,
                pitch_delta: TEST_PITCH_DELTA,
            }
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::Mouse {
                button: control::MouseButton::Left,
                down: true,
            }
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::UseItem
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::Attack
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::Chat {
                message: TEST_CHAT_MESSAGE.to_owned(),
            }
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::ResourcePackStatus(control::ResourcePackStatusDecision {
                offer_id: "mc-compat-local-resource-pack".to_owned(),
                status: control::ResourcePackStatusResponse::Declined,
            })
        ));
        assert!(control::control_command_requires_connection(
            &control::ControlCommand::SignEditorUpdate(test_sign_update())
        ));
    }

    #[test]
    fn sign_editor_open_match_is_position_exact() {
        let expected = test_sign_position();
        assert!(sign_editor_open_matches(expected, expected));
        assert!(!sign_editor_open_matches(
            control::BlockPosition::new(
                TEST_SIGN_X + TEST_POSITION_OFFSET,
                TEST_SIGN_Y,
                TEST_SIGN_Z
            ),
            expected
        ));
    }

    #[test]
    fn maps_control_block_position_to_protocol_position() {
        assert_eq!(
            control_block_position_to_shared(test_sign_position()),
            shared::Position::new(TEST_SIGN_X, TEST_SIGN_Y, TEST_SIGN_Z)
        );
    }

    #[test]
    fn disconnected_operations_return_rejected_response() {
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Status),
            None
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Connect {
                address: TEST_ADDRESS.to_owned(),
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
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Look {
                yaw_delta: TEST_YAW_DELTA,
                pitch_delta: TEST_PITCH_DELTA,
            }),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Mouse {
                button: control::MouseButton::Left,
                down: true,
            }),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::UseItem),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Attack),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::Chat {
                message: TEST_CHAT_MESSAGE.to_owned(),
            }),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::ResourcePackStatus(
                control::ResourcePackStatusDecision {
                    offer_id: "mc-compat-local-resource-pack".to_owned(),
                    status: control::ResourcePackStatusResponse::Declined,
                }
            )),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            disconnected_control_rejection(&control::ControlCommand::SignEditorUpdate(
                test_sign_update()
            )),
            Some(control::ControlResponse::rejected(
                MCP_REQUIRES_CONNECTED_MESSAGE
            ))
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
    fn plans_status_connect_and_disconnect_actions() {
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::Status, connected_state()),
            ControlCommandPlan::Respond(control::ControlResponse::applied(
                "status reported: connected=true connecting=false focused=true"
            ))
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Connect {
                    address: TEST_ADDRESS.to_owned(),
                },
                disconnected_state()
            ),
            ControlCommandPlan::Connect {
                address: TEST_ADDRESS.to_owned(),
                response: control::ControlResponse::applied(MCP_CONNECT_STARTED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Connect {
                    address: TEST_ADDRESS.to_owned(),
                },
                connecting_state()
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_CONNECT_ALREADY_ACTIVE_MESSAGE
            ))
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Connect {
                    address: TEST_ADDRESS.to_owned(),
                },
                connected_state()
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_CONNECT_ALREADY_CONNECTED_MESSAGE
            ))
        );
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::Disconnect, connecting_state()),
            ControlCommandPlan::CancelConnect {
                response: control::ControlResponse::applied(MCP_DISCONNECT_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::Disconnect, connected_state()),
            ControlCommandPlan::Disconnect {
                response: control::ControlResponse::applied(MCP_DISCONNECT_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::Disconnect, disconnected_state()),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_DISCONNECT_NOT_CONNECTED_MESSAGE
            ))
        );
    }

    #[test]
    fn plans_connected_movement_and_protocol_actions() {
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Key {
                    key: control::ControlKey::Forward,
                    down: true,
                },
                connected_state()
            ),
            ControlCommandPlan::Key {
                key: settings::Stevenkey::Forward,
                down: true,
                response: control::ControlResponse::applied(MCP_KEY_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Look {
                    yaw_delta: TEST_YAW_DELTA,
                    pitch_delta: TEST_PITCH_DELTA,
                },
                connected_state()
            ),
            ControlCommandPlan::Look {
                yaw_delta: TEST_YAW_DELTA,
                pitch_delta: TEST_PITCH_DELTA,
                response: control::ControlResponse::applied(MCP_LOOK_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Mouse {
                    button: control::MouseButton::Right,
                    down: true,
                },
                connected_state()
            ),
            ControlCommandPlan::Mouse {
                button: control::MouseButton::Right,
                down: true,
                response: control::ControlResponse::applied(MCP_MOUSE_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::UseItem, connected_state()),
            ControlCommandPlan::UseItem {
                response: control::ControlResponse::applied(MCP_USE_ITEM_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(control::ControlCommand::Attack, connected_state()),
            ControlCommandPlan::Attack {
                response: control::ControlResponse::applied(MCP_ATTACK_APPLIED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Chat {
                    message: TEST_CHAT_MESSAGE.to_owned(),
                },
                connected_state()
            ),
            ControlCommandPlan::Chat {
                message: TEST_CHAT_MESSAGE.to_owned(),
                response: control::ControlResponse::applied(MCP_CHAT_APPLIED_MESSAGE),
            }
        );
    }

    #[test]
    fn plans_sign_editor_update_only_for_matching_open_state() {
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::SignEditorUpdate(test_sign_update()),
                connected_state()
            ),
            ControlCommandPlan::SignEditorUpdate {
                decision: test_sign_update(),
                response: control::ControlResponse::applied(MCP_SIGN_EDITOR_UPDATE_APPLIED_MESSAGE),
            }
        );

        let mut missing_open_state = connected_state();
        missing_open_state.sign_editor_open_position = None;
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::SignEditorUpdate(test_sign_update()),
                missing_open_state
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_SIGN_EDITOR_OPEN_MISSING_MESSAGE
            ))
        );

        let mut mismatched_open_state = connected_state();
        mismatched_open_state.sign_editor_open_position = Some(control::BlockPosition::new(
            TEST_SIGN_X + TEST_POSITION_OFFSET,
            TEST_SIGN_Y,
            TEST_SIGN_Z,
        ));
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::SignEditorUpdate(test_sign_update()),
                mismatched_open_state
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_SIGN_EDITOR_OPEN_POSITION_MISMATCH_MESSAGE
            ))
        );
    }

    #[test]
    fn plans_capture_actions_and_rejects_unavailable_queue() {
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::CaptureScreenshot,
                connected_state()
            ),
            ControlCommandPlan::Capture {
                mode: capture::CaptureMode::Screenshot,
                response: control::ControlResponse::deferred(MCP_CAPTURE_DEFERRED_MESSAGE),
            }
        );
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::CaptureLatestFrame,
                connected_state()
            ),
            ControlCommandPlan::Capture {
                mode: capture::CaptureMode::LatestFrame,
                response: control::ControlResponse::deferred(MCP_CAPTURE_DEFERRED_MESSAGE),
            }
        );

        let mut unavailable_queue = connected_state();
        unavailable_queue.capture_queue_available = false;
        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::CaptureScreenshot,
                unavailable_queue
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_CAPTURE_QUEUE_UNAVAILABLE_MESSAGE
            ))
        );
    }

    #[test]
    fn plan_rejects_missing_player_before_look_side_effects() {
        let mut missing_player = connected_state();
        missing_player.player_rotation_available = false;

        assert_eq!(
            plan_mcp_control_command(
                control::ControlCommand::Look {
                    yaw_delta: TEST_YAW_DELTA,
                    pitch_delta: TEST_PITCH_DELTA,
                },
                missing_player
            ),
            ControlCommandPlan::Respond(control::ControlResponse::rejected(
                MCP_REQUIRES_PLAYER_MESSAGE
            ))
        );
    }

    #[test]
    fn out_of_range_look_input_is_rejected_before_shell_plan() {
        let command = format!(
            r#"{{"action":"look","yaw_delta":{},"pitch_delta":0.0}}"#,
            OUT_OF_RANGE_LOOK_DELTA
        );

        match control::parse_control_command(&command) {
            Err(control::ControlError::OutOfRange { field, actual, .. }) => {
                assert_eq!(field, "yaw_delta");
                assert_eq!(actual, OUT_OF_RANGE_LOOK_DELTA);
            }
            other => panic!("expected out-of-range look rejection, got {other:?}"),
        }
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
    fn capture_queue_errors_preserve_response_vocabulary() {
        assert_eq!(
            capture_queue_error_response(capture::CaptureQueueError::QueueClosed),
            control::ControlResponse::rejected(MCP_CAPTURE_QUEUE_CLOSED_MESSAGE)
        );
        assert_eq!(
            capture_queue_error_response(capture::CaptureQueueError::Validation(
                capture::CaptureValidationError::RecordingBoundsUnexpected
            )),
            control::ControlResponse::rejected(MCP_CAPTURE_REQUEST_INVALID_MESSAGE)
        );
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
