// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use serde_json::{Map, Value};

pub const MAX_SERVER_ADDRESS_CHARS: usize = 255;
pub const MAX_CHAT_MESSAGE_CHARS: usize = 256;
pub const MAX_RESOURCE_PACK_OFFER_ID_CHARS: usize = 128;
pub const MAX_RESOURCE_PACK_URL_CHARS: usize = 2048;
pub const SIGN_EDITOR_LINE_COUNT: usize = 4;
pub const MAX_SIGN_EDITOR_LINE_CHARS: usize = 64;
pub const MAX_ABSOLUTE_LOOK_DELTA_RADIANS: f64 = std::f64::consts::PI;
pub const RESOURCE_PACK_STATUS_DECLINED_CODE: i32 = 1;

const FIELD_ACTION: &str = "action";
const FIELD_ADDRESS: &str = "address";
const FIELD_BUTTON: &str = "button";
const FIELD_DOWN: &str = "down";
const FIELD_KEY: &str = "key";
const FIELD_MESSAGE: &str = "message";
const FIELD_CLAIMS_BROAD_SIGN_EDITING: &str = "claim_broad_sign_editing";
const FIELD_LINES: &str = "lines";
const FIELD_OFFER_ID: &str = "offer_id";
const FIELD_OFFER_RECEIVED: &str = "offer_received";
const FIELD_OPEN_OBSERVED: &str = "open_observed";
const FIELD_OPEN_POSITION: &str = "open_position";
const FIELD_PITCH_DELTA: &str = "pitch_delta";
const FIELD_POSITION: &str = "position";
const FIELD_RESOURCE_PACK_STATUS: &str = "status";
const FIELD_URL: &str = "url";
const FIELD_X: &str = "x";
const FIELD_Y: &str = "y";
const FIELD_Z: &str = "z";
const FIELD_YAW_DELTA: &str = "yaw_delta";

const ACTION_STATUS: &str = "status";
const ACTION_CONNECT: &str = "connect";
const ACTION_DISCONNECT: &str = "disconnect";
const ACTION_KEY: &str = "key";
const ACTION_LOOK: &str = "look";
const ACTION_MOUSE: &str = "mouse";
const ACTION_USE_ITEM: &str = "use_item";
const ACTION_USE_ITEM_ALIAS: &str = "use-item";
const ACTION_ATTACK: &str = "attack";
const ACTION_CHAT: &str = "chat";
const ACTION_RESOURCE_PACK_STATUS: &str = "resource_pack_status";
const ACTION_RESOURCE_PACK_STATUS_ALIAS: &str = "resource-pack-status";
const ACTION_SIGN_EDITOR_UPDATE: &str = "sign_editor_update";
const ACTION_SIGN_EDITOR_UPDATE_ALIAS: &str = "sign-editor-update";
const ACTION_CAPTURE_SCREENSHOT: &str = "capture_screenshot";
const ACTION_CAPTURE_SCREENSHOT_ALIAS: &str = "capture-screenshot";
const ACTION_CAPTURE_LATEST_FRAME: &str = "capture_latest_frame";
const ACTION_CAPTURE_LATEST_FRAME_ALIAS: &str = "capture-latest-frame";

const KEY_FORWARD: &str = "forward";
const KEY_BACKWARD: &str = "backward";
const KEY_LEFT: &str = "left";
const KEY_RIGHT: &str = "right";
const KEY_OPEN_INVENTORY: &str = "open_inventory";
const KEY_OPEN_INVENTORY_ALIAS: &str = "open-inventory";
const KEY_OPEN_INV: &str = "open_inv";
const KEY_OPEN_INV_ALIAS: &str = "open-inv";
const KEY_SNEAK: &str = "sneak";
const KEY_SPRINT: &str = "sprint";
const KEY_JUMP: &str = "jump";

const BUTTON_LEFT: &str = "left";
const BUTTON_RIGHT: &str = "right";

const REASON_EMPTY: &str = "empty";
const REASON_EMPTY_OR_WHITESPACE: &str = "empty_or_whitespace";
const REASON_EXPECTED_ARRAY: &str = "expected_array";
const REASON_EXPECTED_BOOL: &str = "expected_bool";
const REASON_EXPECTED_NUMBER: &str = "expected_number";
const REASON_EXPECTED_OBJECT: &str = "expected_object";
const REASON_EXPECTED_STRING: &str = "expected_string";
const REASON_EXTERNAL_RESOURCE_PACK_URL: &str = "external_or_off_scope_url";
const REASON_NOT_FINITE: &str = "not_finite";
const REASON_OFFER_NOT_RECEIVED: &str = "offer_not_received";
const REASON_OPEN_NOT_OBSERVED: &str = "open_not_observed";
const REASON_OVERCLAIM: &str = "overclaim";
const REASON_POSITION_MISMATCH: &str = "position_mismatch";
const REASON_UNSUPPORTED_RESOURCE_PACK_STATUS: &str = "unsupported_resource_pack_status";
const REASON_WRONG_LINE_COUNT: &str = "wrong_line_count";
const RESOURCE_PACK_STATUS_DECLINED: &str = "declined";
const RESOURCE_PACK_LOCAL_HTTP_PREFIXES: &[&str] = &[
    "http://localhost",
    "http://127.0.0.1",
    "http://[::1]",
    "https://localhost",
    "https://127.0.0.1",
    "https://[::1]",
];
const RESOURCE_PACK_LOCAL_FILE_PREFIX: &str = "file://";

pub const CONTROL_OUTCOME_APPLIED_NAME: &str = "applied";
pub const CONTROL_OUTCOME_REJECTED_NAME: &str = "rejected";
pub const CONTROL_OUTCOME_DEFERRED_NAME: &str = "deferred";

#[derive(Debug, Clone, PartialEq)]
pub enum ControlCommand {
    Status,
    Connect { address: String },
    Disconnect,
    Key { key: ControlKey, down: bool },
    Look { yaw_delta: f64, pitch_delta: f64 },
    Mouse { button: MouseButton, down: bool },
    UseItem,
    Attack,
    Chat { message: String },
    ResourcePackStatus(ResourcePackStatusDecision),
    SignEditorUpdate(SignEditorUpdateDecision),
    CaptureScreenshot,
    CaptureLatestFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlShellAction {
    ReportStatus,
    Connect,
    Disconnect,
    Key,
    Look,
    Mouse,
    UseItem,
    Attack,
    Chat,
    ResourcePackStatus,
    SignEditorUpdate,
    CaptureScreenshot,
    CaptureLatestFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlCommandFacts {
    pub action_name: &'static str,
    pub requires_connection: bool,
    pub shell_action: ControlShellAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourcePackStatusDecision {
    pub offer_id: String,
    pub status: ResourcePackStatusResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePackStatusResponse {
    Declined,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignEditorUpdateDecision {
    pub position: BlockPosition,
    pub lines: [String; SIGN_EDITOR_LINE_COUNT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlKey {
    Forward,
    Backward,
    Left,
    Right,
    OpenInventory,
    Sneak,
    Sprint,
    Jump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlOutcome {
    Applied,
    Rejected,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlResponse {
    pub outcome: ControlOutcome,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControlError {
    MalformedJson(String),
    ExpectedObject,
    MissingField(&'static str),
    InvalidField {
        field: &'static str,
        reason: &'static str,
    },
    UnknownAction(String),
    UnknownKey(String),
    UnknownMouseButton(String),
    ValueTooLong {
        field: &'static str,
        max_chars: usize,
        actual_chars: usize,
    },
    OutOfRange {
        field: &'static str,
        max_abs: f64,
        actual: f64,
    },
}

pub fn parse_control_command(input: &str) -> Result<ControlCommand, ControlError> {
    let value: Value =
        serde_json::from_str(input).map_err(|err| ControlError::MalformedJson(err.to_string()))?;
    parse_control_command_value(&value)
}

pub fn parse_control_command_value(value: &Value) -> Result<ControlCommand, ControlError> {
    let object = value.as_object().ok_or(ControlError::ExpectedObject)?;
    let action = required_string(object, FIELD_ACTION)?;

    match action {
        ACTION_STATUS => Ok(ControlCommand::Status),
        ACTION_CONNECT => parse_connect(object),
        ACTION_DISCONNECT => Ok(ControlCommand::Disconnect),
        ACTION_KEY => parse_key_command(object),
        ACTION_LOOK => parse_look(object),
        ACTION_MOUSE => parse_mouse(object),
        ACTION_USE_ITEM | ACTION_USE_ITEM_ALIAS => Ok(ControlCommand::UseItem),
        ACTION_ATTACK => Ok(ControlCommand::Attack),
        ACTION_CHAT => parse_chat(object),
        ACTION_RESOURCE_PACK_STATUS | ACTION_RESOURCE_PACK_STATUS_ALIAS => {
            parse_resource_pack_status(object)
        }
        ACTION_SIGN_EDITOR_UPDATE | ACTION_SIGN_EDITOR_UPDATE_ALIAS => {
            parse_sign_editor_update(object)
        }
        ACTION_CAPTURE_SCREENSHOT | ACTION_CAPTURE_SCREENSHOT_ALIAS => {
            Ok(ControlCommand::CaptureScreenshot)
        }
        ACTION_CAPTURE_LATEST_FRAME | ACTION_CAPTURE_LATEST_FRAME_ALIAS => {
            Ok(ControlCommand::CaptureLatestFrame)
        }
        _ => Err(ControlError::UnknownAction(action.to_owned())),
    }
}

impl ControlKey {
    pub fn from_name(name: &str) -> Result<Self, ControlError> {
        match name {
            KEY_FORWARD => Ok(ControlKey::Forward),
            KEY_BACKWARD => Ok(ControlKey::Backward),
            KEY_LEFT => Ok(ControlKey::Left),
            KEY_RIGHT => Ok(ControlKey::Right),
            KEY_OPEN_INVENTORY | KEY_OPEN_INVENTORY_ALIAS | KEY_OPEN_INV | KEY_OPEN_INV_ALIAS => {
                Ok(ControlKey::OpenInventory)
            }
            KEY_SNEAK => Ok(ControlKey::Sneak),
            KEY_SPRINT => Ok(ControlKey::Sprint),
            KEY_JUMP => Ok(ControlKey::Jump),
            _ => Err(ControlError::UnknownKey(name.to_owned())),
        }
    }
}

impl MouseButton {
    pub fn from_name(name: &str) -> Result<Self, ControlError> {
        match name {
            BUTTON_LEFT => Ok(MouseButton::Left),
            BUTTON_RIGHT => Ok(MouseButton::Right),
            _ => Err(ControlError::UnknownMouseButton(name.to_owned())),
        }
    }
}

impl ResourcePackStatusResponse {
    pub fn status_code(self) -> i32 {
        match self {
            ResourcePackStatusResponse::Declined => RESOURCE_PACK_STATUS_DECLINED_CODE,
        }
    }
}

impl BlockPosition {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl ControlCommand {
    pub fn facts(&self) -> ControlCommandFacts {
        match self {
            ControlCommand::Status => ControlCommandFacts {
                action_name: ACTION_STATUS,
                requires_connection: false,
                shell_action: ControlShellAction::ReportStatus,
            },
            ControlCommand::Connect { .. } => ControlCommandFacts {
                action_name: ACTION_CONNECT,
                requires_connection: false,
                shell_action: ControlShellAction::Connect,
            },
            ControlCommand::Disconnect => ControlCommandFacts {
                action_name: ACTION_DISCONNECT,
                requires_connection: false,
                shell_action: ControlShellAction::Disconnect,
            },
            ControlCommand::Key { .. } => ControlCommandFacts {
                action_name: ACTION_KEY,
                requires_connection: true,
                shell_action: ControlShellAction::Key,
            },
            ControlCommand::Look { .. } => ControlCommandFacts {
                action_name: ACTION_LOOK,
                requires_connection: true,
                shell_action: ControlShellAction::Look,
            },
            ControlCommand::Mouse { .. } => ControlCommandFacts {
                action_name: ACTION_MOUSE,
                requires_connection: true,
                shell_action: ControlShellAction::Mouse,
            },
            ControlCommand::UseItem => ControlCommandFacts {
                action_name: ACTION_USE_ITEM,
                requires_connection: true,
                shell_action: ControlShellAction::UseItem,
            },
            ControlCommand::Attack => ControlCommandFacts {
                action_name: ACTION_ATTACK,
                requires_connection: true,
                shell_action: ControlShellAction::Attack,
            },
            ControlCommand::Chat { .. } => ControlCommandFacts {
                action_name: ACTION_CHAT,
                requires_connection: true,
                shell_action: ControlShellAction::Chat,
            },
            ControlCommand::ResourcePackStatus(_) => ControlCommandFacts {
                action_name: ACTION_RESOURCE_PACK_STATUS,
                requires_connection: true,
                shell_action: ControlShellAction::ResourcePackStatus,
            },
            ControlCommand::SignEditorUpdate(_) => ControlCommandFacts {
                action_name: ACTION_SIGN_EDITOR_UPDATE,
                requires_connection: true,
                shell_action: ControlShellAction::SignEditorUpdate,
            },
            ControlCommand::CaptureScreenshot => ControlCommandFacts {
                action_name: ACTION_CAPTURE_SCREENSHOT,
                requires_connection: false,
                shell_action: ControlShellAction::CaptureScreenshot,
            },
            ControlCommand::CaptureLatestFrame => ControlCommandFacts {
                action_name: ACTION_CAPTURE_LATEST_FRAME,
                requires_connection: false,
                shell_action: ControlShellAction::CaptureLatestFrame,
            },
        }
    }

    pub fn action_name(&self) -> &'static str {
        self.facts().action_name
    }

    pub fn requires_connection(&self) -> bool {
        self.facts().requires_connection
    }
}

impl ControlOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            ControlOutcome::Applied => CONTROL_OUTCOME_APPLIED_NAME,
            ControlOutcome::Rejected => CONTROL_OUTCOME_REJECTED_NAME,
            ControlOutcome::Deferred => CONTROL_OUTCOME_DEFERRED_NAME,
        }
    }
}

impl ControlResponse {
    pub fn applied(message: impl Into<String>) -> Self {
        Self {
            outcome: ControlOutcome::Applied,
            message: Some(message.into()),
        }
    }

    pub fn rejected(message: impl Into<String>) -> Self {
        Self {
            outcome: ControlOutcome::Rejected,
            message: Some(message.into()),
        }
    }

    pub fn deferred(message: impl Into<String>) -> Self {
        Self {
            outcome: ControlOutcome::Deferred,
            message: Some(message.into()),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.outcome, ControlOutcome::Rejected)
    }
}

pub fn control_command_facts(command: &ControlCommand) -> ControlCommandFacts {
    command.facts()
}

pub fn control_command_requires_connection(command: &ControlCommand) -> bool {
    command.requires_connection()
}

fn parse_connect(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let raw_address = required_string(object, FIELD_ADDRESS)?;
    let address = raw_address.trim();
    validate_nonempty_string(FIELD_ADDRESS, address)?;
    validate_max_chars(FIELD_ADDRESS, address, MAX_SERVER_ADDRESS_CHARS)?;
    Ok(ControlCommand::Connect {
        address: address.to_owned(),
    })
}

fn parse_key_command(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let key = ControlKey::from_name(required_string(object, FIELD_KEY)?)?;
    let down = required_bool(object, FIELD_DOWN)?;
    Ok(ControlCommand::Key { key, down })
}

fn parse_look(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let yaw_delta = required_bounded_f64(object, FIELD_YAW_DELTA)?;
    let pitch_delta = required_bounded_f64(object, FIELD_PITCH_DELTA)?;
    Ok(ControlCommand::Look {
        yaw_delta,
        pitch_delta,
    })
}

fn parse_mouse(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let button = MouseButton::from_name(required_string(object, FIELD_BUTTON)?)?;
    let down = required_bool(object, FIELD_DOWN)?;
    Ok(ControlCommand::Mouse { button, down })
}

fn parse_chat(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let message = required_string(object, FIELD_MESSAGE)?;
    validate_nonblank_string(FIELD_MESSAGE, message)?;
    validate_max_chars(FIELD_MESSAGE, message, MAX_CHAT_MESSAGE_CHARS)?;
    Ok(ControlCommand::Chat {
        message: message.to_owned(),
    })
}

fn parse_resource_pack_status(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    let offer_id =
        required_trimmed_nonblank_string(object, FIELD_OFFER_ID, MAX_RESOURCE_PACK_OFFER_ID_CHARS)?;
    let url = required_trimmed_nonblank_string(object, FIELD_URL, MAX_RESOURCE_PACK_URL_CHARS)?;
    validate_owned_local_resource_pack_url(url)?;
    validate_offer_received(object)?;
    let status =
        parse_resource_pack_status_response(required_string(object, FIELD_RESOURCE_PACK_STATUS)?)?;
    Ok(ControlCommand::ResourcePackStatus(
        ResourcePackStatusDecision {
            offer_id: offer_id.to_owned(),
            status,
        },
    ))
}

fn parse_sign_editor_update(object: &Map<String, Value>) -> Result<ControlCommand, ControlError> {
    reject_sign_editor_overclaims(object)?;
    let position = required_position(object, FIELD_POSITION)?;
    let open_position = required_position(object, FIELD_OPEN_POSITION)?;
    validate_open_observed(object)?;
    validate_matching_position(position, open_position)?;
    let lines = required_sign_lines(object)?;
    Ok(ControlCommand::SignEditorUpdate(SignEditorUpdateDecision {
        position,
        lines,
    }))
}

fn required_string<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
) -> Result<&'a str, ControlError> {
    object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_str()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_STRING,
        })
}

fn required_bool(object: &Map<String, Value>, field: &'static str) -> Result<bool, ControlError> {
    object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_bool()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_BOOL,
        })
}

fn required_object<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
) -> Result<&'a Map<String, Value>, ControlError> {
    object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_object()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_OBJECT,
        })
}

fn required_array<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
) -> Result<&'a Vec<Value>, ControlError> {
    object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_array()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_ARRAY,
        })
}

fn required_i32(object: &Map<String, Value>, field: &'static str) -> Result<i32, ControlError> {
    let value = object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_i64()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_NUMBER,
        })?;
    i32::try_from(value).map_err(|_| ControlError::InvalidField {
        field,
        reason: REASON_EXPECTED_NUMBER,
    })
}

fn required_bounded_f64(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<f64, ControlError> {
    let value = object
        .get(field)
        .ok_or(ControlError::MissingField(field))?
        .as_f64()
        .ok_or(ControlError::InvalidField {
            field,
            reason: REASON_EXPECTED_NUMBER,
        })?;

    if !value.is_finite() {
        return Err(ControlError::InvalidField {
            field,
            reason: REASON_NOT_FINITE,
        });
    }

    if value.abs() > MAX_ABSOLUTE_LOOK_DELTA_RADIANS {
        return Err(ControlError::OutOfRange {
            field,
            max_abs: MAX_ABSOLUTE_LOOK_DELTA_RADIANS,
            actual: value,
        });
    }

    Ok(value)
}

fn validate_nonempty_string(field: &'static str, value: &str) -> Result<(), ControlError> {
    if value.is_empty() {
        return Err(ControlError::InvalidField {
            field,
            reason: REASON_EMPTY,
        });
    }

    Ok(())
}

fn required_trimmed_nonblank_string<'a>(
    object: &'a Map<String, Value>,
    field: &'static str,
    max_chars: usize,
) -> Result<&'a str, ControlError> {
    let value = required_string(object, field)?.trim();
    validate_nonempty_string(field, value)?;
    validate_max_chars(field, value, max_chars)?;
    Ok(value)
}

fn required_position(
    object: &Map<String, Value>,
    field: &'static str,
) -> Result<BlockPosition, ControlError> {
    let position = required_object(object, field)?;
    Ok(BlockPosition::new(
        required_i32(position, FIELD_X)?,
        required_i32(position, FIELD_Y)?,
        required_i32(position, FIELD_Z)?,
    ))
}

fn required_sign_lines(
    object: &Map<String, Value>,
) -> Result<[String; SIGN_EDITOR_LINE_COUNT], ControlError> {
    let values = required_array(object, FIELD_LINES)?;
    if values.len() != SIGN_EDITOR_LINE_COUNT {
        return Err(ControlError::InvalidField {
            field: FIELD_LINES,
            reason: REASON_WRONG_LINE_COUNT,
        });
    }
    let mut lines = Vec::with_capacity(SIGN_EDITOR_LINE_COUNT);
    for value in values {
        let Some(line) = value.as_str() else {
            return Err(ControlError::InvalidField {
                field: FIELD_LINES,
                reason: REASON_EXPECTED_STRING,
            });
        };
        validate_max_chars(FIELD_LINES, line, MAX_SIGN_EDITOR_LINE_CHARS)?;
        lines.push(line.to_owned());
    }
    Ok(lines
        .try_into()
        .expect("line count checked before array conversion"))
}

fn validate_nonblank_string(field: &'static str, value: &str) -> Result<(), ControlError> {
    if value.trim().is_empty() {
        return Err(ControlError::InvalidField {
            field,
            reason: REASON_EMPTY_OR_WHITESPACE,
        });
    }

    Ok(())
}

fn validate_owned_local_resource_pack_url(url: &str) -> Result<(), ControlError> {
    if url.starts_with(RESOURCE_PACK_LOCAL_FILE_PREFIX)
        || RESOURCE_PACK_LOCAL_HTTP_PREFIXES
            .iter()
            .any(|prefix| url.starts_with(prefix))
    {
        return Ok(());
    }
    Err(ControlError::InvalidField {
        field: FIELD_URL,
        reason: REASON_EXTERNAL_RESOURCE_PACK_URL,
    })
}

fn validate_offer_received(object: &Map<String, Value>) -> Result<(), ControlError> {
    if required_bool(object, FIELD_OFFER_RECEIVED)? {
        return Ok(());
    }
    Err(ControlError::InvalidField {
        field: FIELD_OFFER_RECEIVED,
        reason: REASON_OFFER_NOT_RECEIVED,
    })
}

fn validate_open_observed(object: &Map<String, Value>) -> Result<(), ControlError> {
    if required_bool(object, FIELD_OPEN_OBSERVED)? {
        return Ok(());
    }
    Err(ControlError::InvalidField {
        field: FIELD_OPEN_OBSERVED,
        reason: REASON_OPEN_NOT_OBSERVED,
    })
}

fn validate_matching_position(
    position: BlockPosition,
    open_position: BlockPosition,
) -> Result<(), ControlError> {
    if position == open_position {
        return Ok(());
    }
    Err(ControlError::InvalidField {
        field: FIELD_OPEN_POSITION,
        reason: REASON_POSITION_MISMATCH,
    })
}

fn reject_sign_editor_overclaims(object: &Map<String, Value>) -> Result<(), ControlError> {
    if object
        .get(FIELD_CLAIMS_BROAD_SIGN_EDITING)
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(ControlError::InvalidField {
            field: FIELD_CLAIMS_BROAD_SIGN_EDITING,
            reason: REASON_OVERCLAIM,
        });
    }
    Ok(())
}

fn parse_resource_pack_status_response(
    value: &str,
) -> Result<ResourcePackStatusResponse, ControlError> {
    match value {
        RESOURCE_PACK_STATUS_DECLINED => Ok(ResourcePackStatusResponse::Declined),
        _ => Err(ControlError::InvalidField {
            field: FIELD_RESOURCE_PACK_STATUS,
            reason: REASON_UNSUPPORTED_RESOURCE_PACK_STATUS,
        }),
    }
}

fn validate_max_chars(
    field: &'static str,
    value: &str,
    max_chars: usize,
) -> Result<(), ControlError> {
    let actual_chars = value.chars().count();
    if actual_chars > max_chars {
        return Err(ControlError::ValueTooLong {
            field,
            max_chars,
            actual_chars,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const VALID_LOOK_YAW_DELTA: f64 = 0.25;
    const VALID_LOOK_PITCH_DELTA: f64 = -0.125;
    const OVERSIZED_CHAT_EXTRA_CHARS: usize = 1;
    const OVERSIZED_RESOURCE_PACK_URL_EXTRA_CHARS: usize = 1;
    const TEST_RESOURCE_PACK_OFFER_ID: &str = "mc-compat-local-resource-pack";
    const TEST_RESOURCE_PACK_LOCAL_URL: &str = "http://127.0.0.1:25565/resource-pack.zip";
    const TEST_RESOURCE_PACK_EXTERNAL_URL: &str = "https://example.com/resource-pack.zip";
    const TEST_CONNECT_ADDRESS: &str = "127.0.0.1:25565";
    const TEST_CHAT_MESSAGE: &str = "/help";
    const TEST_RESPONSE_MESSAGE: &str = "ok";
    const TEST_SIGN_X: i32 = 28;
    const TEST_SIGN_Y: i32 = 64;
    const TEST_SIGN_Z: i32 = 0;
    const TEST_WRONG_SIGN_X: i32 = 29;
    const TEST_OVERSIZED_SIGN_LINE_EXTRA_CHARS: usize = 1;

    #[test]
    fn parses_valid_initial_command_set() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "status" })),
            Ok(ControlCommand::Status)
        );
        assert_eq!(
            parse_control_command_value(
                &json!({ "action": "connect", "address": " 127.0.0.1:25565 " })
            ),
            Ok(ControlCommand::Connect {
                address: "127.0.0.1:25565".to_owned(),
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "disconnect" })),
            Ok(ControlCommand::Disconnect)
        );
        assert_eq!(
            parse_control_command_value(
                &json!({ "action": "key", "key": "forward", "down": true })
            ),
            Ok(ControlCommand::Key {
                key: ControlKey::Forward,
                down: true,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "look",
                "yaw_delta": VALID_LOOK_YAW_DELTA,
                "pitch_delta": VALID_LOOK_PITCH_DELTA,
            })),
            Ok(ControlCommand::Look {
                yaw_delta: VALID_LOOK_YAW_DELTA,
                pitch_delta: VALID_LOOK_PITCH_DELTA,
            })
        );
        assert_eq!(
            parse_control_command_value(
                &json!({ "action": "mouse", "button": "right", "down": false })
            ),
            Ok(ControlCommand::Mouse {
                button: MouseButton::Right,
                down: false,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "use-item" })),
            Ok(ControlCommand::UseItem)
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "attack" })),
            Ok(ControlCommand::Attack)
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "chat", "message": "/help" })),
            Ok(ControlCommand::Chat {
                message: "/help".to_owned(),
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "capture_screenshot" })),
            Ok(ControlCommand::CaptureScreenshot)
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "capture_latest_frame" })),
            Ok(ControlCommand::CaptureLatestFrame)
        );
    }

    #[test]
    fn parses_resource_pack_status_decline_for_owned_local_offer() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource-pack-status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": TEST_RESOURCE_PACK_LOCAL_URL,
                "status": "declined",
                "offer_received": true,
            })),
            Ok(ControlCommand::ResourcePackStatus(
                ResourcePackStatusDecision {
                    offer_id: TEST_RESOURCE_PACK_OFFER_ID.to_owned(),
                    status: ResourcePackStatusResponse::Declined,
                }
            ))
        );
        assert_eq!(
            ResourcePackStatusResponse::Declined.status_code(),
            RESOURCE_PACK_STATUS_DECLINED_CODE
        );
    }

    #[test]
    fn rejects_resource_pack_offer_with_blank_identity() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": "   ",
                "url": TEST_RESOURCE_PACK_LOCAL_URL,
                "status": "declined",
                "offer_received": true,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_OFFER_ID,
                reason: REASON_EMPTY,
            })
        );
    }

    #[test]
    fn rejects_external_resource_pack_url_before_protocol_response() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": TEST_RESOURCE_PACK_EXTERNAL_URL,
                "status": "declined",
                "offer_received": true,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_URL,
                reason: REASON_EXTERNAL_RESOURCE_PACK_URL,
            })
        );
    }

    #[test]
    fn rejects_unsupported_resource_pack_status() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": TEST_RESOURCE_PACK_LOCAL_URL,
                "status": "accepted",
                "offer_received": true,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_RESOURCE_PACK_STATUS,
                reason: REASON_UNSUPPORTED_RESOURCE_PACK_STATUS,
            })
        );
    }

    #[test]
    fn rejects_resource_pack_status_without_offer_state() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": TEST_RESOURCE_PACK_LOCAL_URL,
                "status": "declined",
                "offer_received": false,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_OFFER_RECEIVED,
                reason: REASON_OFFER_NOT_RECEIVED,
            })
        );
    }

    #[test]
    fn rejects_oversized_resource_pack_url_before_redaction_boundary() {
        let oversized_url = format!(
            "file://{}",
            "x".repeat(MAX_RESOURCE_PACK_URL_CHARS + OVERSIZED_RESOURCE_PACK_URL_EXTRA_CHARS)
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": oversized_url,
                "status": "declined",
                "offer_received": true,
            })),
            Err(ControlError::ValueTooLong {
                field: FIELD_URL,
                max_chars: MAX_RESOURCE_PACK_URL_CHARS,
                actual_chars: MAX_RESOURCE_PACK_URL_CHARS
                    + RESOURCE_PACK_LOCAL_FILE_PREFIX.chars().count()
                    + OVERSIZED_RESOURCE_PACK_URL_EXTRA_CHARS,
            })
        );
    }

    #[test]
    fn parses_sign_editor_update_for_matching_open_state() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign-editor-update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign", "Edit"],
            })),
            Ok(ControlCommand::SignEditorUpdate(SignEditorUpdateDecision {
                position: BlockPosition::new(TEST_SIGN_X, TEST_SIGN_Y, TEST_SIGN_Z),
                lines: [
                    "MC".to_owned(),
                    "Compat".to_owned(),
                    "Sign".to_owned(),
                    "Edit".to_owned(),
                ],
            }))
        );
    }

    #[test]
    fn rejects_sign_editor_update_without_open_state() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": false,
                "lines": ["MC", "Compat", "Sign", "Edit"],
            })),
            Err(ControlError::InvalidField {
                field: FIELD_OPEN_OBSERVED,
                reason: REASON_OPEN_NOT_OBSERVED,
            })
        );
    }

    #[test]
    fn rejects_sign_editor_update_for_wrong_open_position() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_WRONG_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign", "Edit"],
            })),
            Err(ControlError::InvalidField {
                field: FIELD_OPEN_POSITION,
                reason: REASON_POSITION_MISMATCH,
            })
        );
    }

    #[test]
    fn rejects_sign_editor_malformed_payload() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", true, "Edit"],
            })),
            Err(ControlError::InvalidField {
                field: FIELD_LINES,
                reason: REASON_EXPECTED_STRING,
            })
        );
    }

    #[test]
    fn rejects_sign_editor_wrong_line_count() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign"],
            })),
            Err(ControlError::InvalidField {
                field: FIELD_LINES,
                reason: REASON_WRONG_LINE_COUNT,
            })
        );
    }

    #[test]
    fn rejects_sign_editor_overlong_line() {
        let overlong_line =
            "x".repeat(MAX_SIGN_EDITOR_LINE_CHARS + TEST_OVERSIZED_SIGN_LINE_EXTRA_CHARS);
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", overlong_line, "Edit"],
            })),
            Err(ControlError::ValueTooLong {
                field: FIELD_LINES,
                max_chars: MAX_SIGN_EDITOR_LINE_CHARS,
                actual_chars: MAX_SIGN_EDITOR_LINE_CHARS + TEST_OVERSIZED_SIGN_LINE_EXTRA_CHARS,
            })
        );
    }

    #[test]
    fn rejects_sign_editor_overclaim() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign", "Edit"],
                "claim_broad_sign_editing": true,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_CLAIMS_BROAD_SIGN_EDITING,
                reason: REASON_OVERCLAIM,
            })
        );
    }

    #[test]
    fn parses_all_key_aliases() {
        assert_eq!(ControlKey::from_name("backward"), Ok(ControlKey::Backward));
        assert_eq!(ControlKey::from_name("left"), Ok(ControlKey::Left));
        assert_eq!(ControlKey::from_name("right"), Ok(ControlKey::Right));
        assert_eq!(
            ControlKey::from_name("open_inventory"),
            Ok(ControlKey::OpenInventory)
        );
        assert_eq!(
            ControlKey::from_name("open-inventory"),
            Ok(ControlKey::OpenInventory)
        );
        assert_eq!(
            ControlKey::from_name("open_inv"),
            Ok(ControlKey::OpenInventory)
        );
        assert_eq!(
            ControlKey::from_name("open-inv"),
            Ok(ControlKey::OpenInventory)
        );
        assert_eq!(ControlKey::from_name("sneak"), Ok(ControlKey::Sneak));
        assert_eq!(ControlKey::from_name("sprint"), Ok(ControlKey::Sprint));
        assert_eq!(ControlKey::from_name("jump"), Ok(ControlKey::Jump));
    }

    fn assert_command_facts(
        command: ControlCommand,
        action_name: &'static str,
        requires_connection: bool,
        shell_action: ControlShellAction,
    ) {
        let facts = command.facts();
        assert_eq!(facts.action_name, action_name);
        assert_eq!(facts.requires_connection, requires_connection);
        assert_eq!(facts.shell_action, shell_action);
        assert_eq!(command.action_name(), action_name);
        assert_eq!(control_command_facts(&command), facts);
        assert_eq!(
            control_command_requires_connection(&command),
            requires_connection
        );
    }

    #[test]
    fn classifies_command_facts_without_game_state() {
        assert_command_facts(
            ControlCommand::Status,
            ACTION_STATUS,
            false,
            ControlShellAction::ReportStatus,
        );
        assert_command_facts(
            ControlCommand::Connect {
                address: TEST_CONNECT_ADDRESS.to_owned(),
            },
            ACTION_CONNECT,
            false,
            ControlShellAction::Connect,
        );
        assert_command_facts(
            ControlCommand::Disconnect,
            ACTION_DISCONNECT,
            false,
            ControlShellAction::Disconnect,
        );
        assert_command_facts(
            ControlCommand::Key {
                key: ControlKey::Forward,
                down: true,
            },
            ACTION_KEY,
            true,
            ControlShellAction::Key,
        );
        assert_command_facts(
            ControlCommand::Look {
                yaw_delta: VALID_LOOK_YAW_DELTA,
                pitch_delta: VALID_LOOK_PITCH_DELTA,
            },
            ACTION_LOOK,
            true,
            ControlShellAction::Look,
        );
        assert_command_facts(
            ControlCommand::Mouse {
                button: MouseButton::Left,
                down: true,
            },
            ACTION_MOUSE,
            true,
            ControlShellAction::Mouse,
        );
        assert_command_facts(
            ControlCommand::UseItem,
            ACTION_USE_ITEM,
            true,
            ControlShellAction::UseItem,
        );
        assert_command_facts(
            ControlCommand::Attack,
            ACTION_ATTACK,
            true,
            ControlShellAction::Attack,
        );
        assert_command_facts(
            ControlCommand::Chat {
                message: TEST_CHAT_MESSAGE.to_owned(),
            },
            ACTION_CHAT,
            true,
            ControlShellAction::Chat,
        );
        assert_command_facts(
            ControlCommand::ResourcePackStatus(ResourcePackStatusDecision {
                offer_id: TEST_RESOURCE_PACK_OFFER_ID.to_owned(),
                status: ResourcePackStatusResponse::Declined,
            }),
            ACTION_RESOURCE_PACK_STATUS,
            true,
            ControlShellAction::ResourcePackStatus,
        );
        assert_command_facts(
            ControlCommand::SignEditorUpdate(SignEditorUpdateDecision {
                position: BlockPosition::new(TEST_SIGN_X, TEST_SIGN_Y, TEST_SIGN_Z),
                lines: [
                    "MC".to_owned(),
                    "Compat".to_owned(),
                    "Sign".to_owned(),
                    "Edit".to_owned(),
                ],
            }),
            ACTION_SIGN_EDITOR_UPDATE,
            true,
            ControlShellAction::SignEditorUpdate,
        );
        assert_command_facts(
            ControlCommand::CaptureScreenshot,
            ACTION_CAPTURE_SCREENSHOT,
            false,
            ControlShellAction::CaptureScreenshot,
        );
        assert_command_facts(
            ControlCommand::CaptureLatestFrame,
            ACTION_CAPTURE_LATEST_FRAME,
            false,
            ControlShellAction::CaptureLatestFrame,
        );
    }

    #[test]
    fn classifies_response_vocabulary_without_mcp_transport() {
        let applied = ControlResponse::applied(TEST_RESPONSE_MESSAGE);
        let rejected = ControlResponse::rejected(TEST_RESPONSE_MESSAGE);
        let deferred = ControlResponse::deferred(TEST_RESPONSE_MESSAGE);

        assert_eq!(applied.outcome.as_str(), CONTROL_OUTCOME_APPLIED_NAME);
        assert_eq!(rejected.outcome.as_str(), CONTROL_OUTCOME_REJECTED_NAME);
        assert_eq!(deferred.outcome.as_str(), CONTROL_OUTCOME_DEFERRED_NAME);
        assert!(!applied.is_error());
        assert!(rejected.is_error());
        assert!(!deferred.is_error());
    }

    #[test]
    fn rejects_non_object_and_action_schema_mismatches() {
        assert_eq!(
            parse_control_command_value(&json!(null)),
            Err(ControlError::ExpectedObject)
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": false })),
            Err(ControlError::InvalidField {
                field: FIELD_ACTION,
                reason: REASON_EXPECTED_STRING,
            })
        );
    }

    #[test]
    fn rejects_missing_payloads_for_commands() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "connect" })),
            Err(ControlError::MissingField(FIELD_ADDRESS))
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "look",
                "pitch_delta": VALID_LOOK_PITCH_DELTA,
            })),
            Err(ControlError::MissingField(FIELD_YAW_DELTA))
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "mouse", "down": true })),
            Err(ControlError::MissingField(FIELD_BUTTON))
        );
        assert_eq!(
            parse_control_command_value(&json!({ "action": "chat" })),
            Err(ControlError::MissingField(FIELD_MESSAGE))
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "url": TEST_RESOURCE_PACK_LOCAL_URL,
                "status": "declined",
                "offer_received": true,
            })),
            Err(ControlError::MissingField(FIELD_OFFER_ID))
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign", "Edit"],
            })),
            Err(ControlError::MissingField(FIELD_POSITION))
        );
    }

    #[test]
    fn rejects_command_payload_schema_mismatches() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "connect", "address": false })),
            Err(ControlError::InvalidField {
                field: FIELD_ADDRESS,
                reason: REASON_EXPECTED_STRING,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "key",
                "key": "jump",
                "down": "true",
            })),
            Err(ControlError::InvalidField {
                field: FIELD_DOWN,
                reason: REASON_EXPECTED_BOOL,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "look",
                "yaw_delta": VALID_LOOK_YAW_DELTA,
                "pitch_delta": "north",
            })),
            Err(ControlError::InvalidField {
                field: FIELD_PITCH_DELTA,
                reason: REASON_EXPECTED_NUMBER,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "mouse",
                "button": "left",
                "down": "true",
            })),
            Err(ControlError::InvalidField {
                field: FIELD_DOWN,
                reason: REASON_EXPECTED_BOOL,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "resource_pack_status",
                "offer_id": TEST_RESOURCE_PACK_OFFER_ID,
                "url": false,
                "status": "declined",
                "offer_received": true,
            })),
            Err(ControlError::InvalidField {
                field: FIELD_URL,
                reason: REASON_EXPECTED_STRING,
            })
        );
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "sign_editor_update",
                "position": [],
                "open_position": { "x": TEST_SIGN_X, "y": TEST_SIGN_Y, "z": TEST_SIGN_Z },
                "open_observed": true,
                "lines": ["MC", "Compat", "Sign", "Edit"],
            })),
            Err(ControlError::InvalidField {
                field: FIELD_POSITION,
                reason: REASON_EXPECTED_OBJECT,
            })
        );
    }

    #[test]
    fn rejects_missing_action_field() {
        assert_eq!(
            parse_control_command_value(&json!({ "key": "forward", "down": true })),
            Err(ControlError::MissingField(FIELD_ACTION))
        );
    }

    #[test]
    fn rejects_unknown_action() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "teleport" })),
            Err(ControlError::UnknownAction("teleport".to_owned()))
        );
    }

    #[test]
    fn rejects_invalid_key_name() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "key", "key": "fly", "down": true })),
            Err(ControlError::UnknownKey("fly".to_owned()))
        );
    }

    #[test]
    fn rejects_invalid_mouse_button() {
        assert_eq!(
            parse_control_command_value(
                &json!({ "action": "mouse", "button": "middle", "down": true })
            ),
            Err(ControlError::UnknownMouseButton("middle".to_owned()))
        );
    }

    #[test]
    fn rejects_missing_required_command_field() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "key", "key": "jump" })),
            Err(ControlError::MissingField(FIELD_DOWN))
        );
    }

    #[test]
    fn rejects_oversized_chat_message() {
        let oversized = "x".repeat(MAX_CHAT_MESSAGE_CHARS + OVERSIZED_CHAT_EXTRA_CHARS);
        assert_eq!(
            parse_control_command_value(&json!({ "action": "chat", "message": oversized })),
            Err(ControlError::ValueTooLong {
                field: FIELD_MESSAGE,
                max_chars: MAX_CHAT_MESSAGE_CHARS,
                actual_chars: MAX_CHAT_MESSAGE_CHARS + OVERSIZED_CHAT_EXTRA_CHARS,
            })
        );
    }

    #[test]
    fn rejects_blank_chat_message() {
        assert_eq!(
            parse_control_command_value(&json!({ "action": "chat", "message": "   " })),
            Err(ControlError::InvalidField {
                field: FIELD_MESSAGE,
                reason: REASON_EMPTY_OR_WHITESPACE,
            })
        );
    }

    #[test]
    fn rejects_out_of_range_look_delta() {
        assert_eq!(
            parse_control_command_value(&json!({
                "action": "look",
                "yaw_delta": MAX_ABSOLUTE_LOOK_DELTA_RADIANS * 2.0,
                "pitch_delta": VALID_LOOK_PITCH_DELTA,
            })),
            Err(ControlError::OutOfRange {
                field: FIELD_YAW_DELTA,
                max_abs: MAX_ABSOLUTE_LOOK_DELTA_RADIANS,
                actual: MAX_ABSOLUTE_LOOK_DELTA_RADIANS * 2.0,
            })
        );
    }

    #[test]
    fn parses_json_string_entrypoint() {
        assert_eq!(
            parse_control_command(r#"{"action":"mouse","button":"left","down":true}"#),
            Ok(ControlCommand::Mouse {
                button: MouseButton::Left,
                down: true,
            })
        );
    }

    #[test]
    fn rejects_malformed_json_entrypoint() {
        match parse_control_command("not-json") {
            Err(ControlError::MalformedJson(_)) => {}
            other => panic!("expected malformed json, got {:?}", other),
        }
    }
}
