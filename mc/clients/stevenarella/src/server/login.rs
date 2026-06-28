use crate::format;
use crate::protocol::{self, packet};
use std::str::FromStr;

pub(crate) const FML_NETWORK_VERSION_LEGACY: i64 = 1;
pub(crate) const FML_NETWORK_VERSION_FML2: i64 = 2;
pub(crate) const LEGACY_FML_HANDSHAKE_TAG: &str = "\0FML\0";
pub(crate) const FML2_HANDSHAKE_TAG: &str = "\0FML2\0";
pub(crate) const OPTIONAL_UUID_LOGIN_START_PROTOCOL: i32 = 759;
pub(crate) const VARINT_ENCRYPTION_RESPONSE_PROTOCOL: i32 = 47;
pub(crate) const LOGIN_HANDSHAKE_NEXT_STATE: i32 = 2;
pub(crate) const SHARED_SECRET_BYTES: usize = 16;
pub(crate) const FML_LOGIN_WRAPPER_CHANNEL: &str = "fml:loginwrapper";
pub(crate) const FML_HANDSHAKE_CHANNEL: &str = "fml:handshake";

const WRONG_PACKET_INITIAL: &str = "Wrong packet 1";
const WRONG_PACKET_ENCRYPTED: &str = "Wrong packet 2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoginPhase {
    Initial,
    Encrypted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoginEvent {
    SetCompression(LoginCompression),
    EncryptionRequest(LoginEncryptionRequest),
    Success(LoginOutcome),
    Disconnect(format::Component),
    PluginRequest(LoginPluginRequest),
    Unexpected { packet_debug: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoginCompression {
    pub(crate) threshold: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoginEncryptionRequest {
    pub(crate) server_id: String,
    pub(crate) public_key: Vec<u8>,
    pub(crate) verify_token: Vec<u8>,
    packet_debug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoginPluginRequest {
    pub(crate) message_id: protocol::VarInt,
    pub(crate) channel: String,
    pub(crate) data: Vec<u8>,
    packet_debug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoginOutcome {
    username: String,
    uuid: LoginUuid,
    properties: Option<Vec<packet::PlayerProperty>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LoginUuid {
    HyphenatedString(String),
    Binary(protocol::UUID),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LoginDecision {
    ApplyCompression(LoginCompression),
    BeginEncryption(LoginEncryptionRequest),
    Complete(LoginOutcome),
    Disconnect(format::Component),
    HandlePluginRequest(LoginPluginRequest),
    Fail(LoginFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoginFailure {
    stage: LoginPhase,
    packet_debug: String,
}

pub(crate) fn fml_handshake_tag(fml_network_version: Option<i64>) -> &'static str {
    match fml_network_version {
        Some(FML_NETWORK_VERSION_LEGACY) => LEGACY_FML_HANDSHAKE_TAG,
        Some(FML_NETWORK_VERSION_FML2) => FML2_HANDSHAKE_TAG,
        None => "",
        _ => panic!("unsupported FML network version: {:?}", fml_network_version),
    }
}

pub(crate) fn login_start_uses_optional_uuid(protocol_version: i32) -> bool {
    protocol_version >= OPTIONAL_UUID_LOGIN_START_PROTOCOL
}

pub(crate) fn encryption_response_uses_varint(protocol_version: i32) -> bool {
    protocol_version >= VARINT_ENCRYPTION_RESPONSE_PROTOCOL
}

pub(crate) fn login_event_from_packet(packet: packet::Packet) -> LoginEvent {
    match packet {
        packet::Packet::SetInitialCompression(packet) => {
            LoginEvent::SetCompression(LoginCompression {
                threshold: packet.threshold.0,
            })
        }
        packet::Packet::EncryptionRequest(packet) => {
            let packet_debug = format!("EncryptionRequest({:?})", packet);
            LoginEvent::EncryptionRequest(LoginEncryptionRequest {
                server_id: packet.server_id,
                public_key: packet.public_key.data,
                verify_token: packet.verify_token.data,
                packet_debug,
            })
        }
        packet::Packet::EncryptionRequest_i16(packet) => {
            let packet_debug = format!("EncryptionRequest_i16({:?})", packet);
            LoginEvent::EncryptionRequest(LoginEncryptionRequest {
                server_id: packet.server_id,
                public_key: packet.public_key.data,
                verify_token: packet.verify_token.data,
                packet_debug,
            })
        }
        packet::Packet::LoginSuccess_String(packet) => LoginEvent::Success(LoginOutcome {
            username: packet.username,
            uuid: LoginUuid::HyphenatedString(packet.uuid),
            properties: None,
        }),
        packet::Packet::LoginSuccess_UUID(packet) => LoginEvent::Success(LoginOutcome {
            username: packet.username,
            uuid: LoginUuid::Binary(packet.uuid),
            properties: None,
        }),
        packet::Packet::LoginSuccess_UUID_WithProperties(packet) => {
            LoginEvent::Success(LoginOutcome {
                username: packet.username,
                uuid: LoginUuid::Binary(packet.uuid),
                properties: Some(packet.properties.data),
            })
        }
        packet::Packet::LoginDisconnect(packet) => LoginEvent::Disconnect(packet.reason),
        packet::Packet::LoginPluginRequest(packet) => {
            let packet_debug = format!("LoginPluginRequest({:?})", packet);
            LoginEvent::PluginRequest(LoginPluginRequest {
                message_id: packet.message_id,
                channel: packet.channel,
                data: packet.data,
                packet_debug,
            })
        }
        packet => LoginEvent::Unexpected {
            packet_debug: format!("{:?}", packet),
        },
    }
}

pub(crate) fn decide_login_event(phase: LoginPhase, event: LoginEvent) -> LoginDecision {
    match (phase, event) {
        (_, LoginEvent::SetCompression(compression)) => {
            LoginDecision::ApplyCompression(compression)
        }
        (LoginPhase::Initial, LoginEvent::EncryptionRequest(request)) => {
            LoginDecision::BeginEncryption(request)
        }
        (LoginPhase::Encrypted, LoginEvent::EncryptionRequest(request)) => {
            LoginDecision::Fail(LoginFailure {
                stage: LoginPhase::Encrypted,
                packet_debug: request.packet_debug,
            })
        }
        (LoginPhase::Initial, LoginEvent::Success(outcome))
        | (LoginPhase::Encrypted, LoginEvent::Success(outcome)) => LoginDecision::Complete(outcome),
        (_, LoginEvent::Disconnect(reason)) => LoginDecision::Disconnect(reason),
        (LoginPhase::Encrypted, LoginEvent::PluginRequest(request)) => {
            LoginDecision::HandlePluginRequest(request)
        }
        (LoginPhase::Initial, LoginEvent::PluginRequest(request)) => {
            LoginDecision::Fail(LoginFailure {
                stage: LoginPhase::Initial,
                packet_debug: request.packet_debug,
            })
        }
        (phase, LoginEvent::Unexpected { packet_debug }) => LoginDecision::Fail(LoginFailure {
            stage: phase,
            packet_debug,
        }),
        (phase, event) => LoginDecision::Fail(LoginFailure {
            stage: phase,
            packet_debug: format!("{:?}", event),
        }),
    }
}

impl LoginOutcome {
    pub(crate) fn username(&self) -> &str {
        &self.username
    }

    pub(crate) fn protocol_uuid(&self) -> protocol::UUID {
        match &self.uuid {
            LoginUuid::HyphenatedString(uuid) => protocol::UUID::from_str(uuid).unwrap(),
            LoginUuid::Binary(uuid) => uuid.clone(),
        }
    }

    pub(crate) fn uuid_log_value(&self) -> String {
        match &self.uuid {
            LoginUuid::HyphenatedString(uuid) => uuid.clone(),
            LoginUuid::Binary(uuid) => format!("{:?}", uuid),
        }
    }

    pub(crate) fn property_count(&self) -> Option<usize> {
        self.properties.as_ref().map(Vec::len)
    }

    #[cfg(test)]
    fn properties(&self) -> Option<&[packet::PlayerProperty]> {
        self.properties.as_deref()
    }
}

impl LoginFailure {
    pub(crate) fn message(&self) -> String {
        let prefix = match self.stage {
            LoginPhase::Initial => WRONG_PACKET_INITIAL,
            LoginPhase::Encrypted => WRONG_PACKET_ENCRYPTED,
        };
        format!("{}: {}", prefix, self.packet_debug)
    }

    pub(crate) fn into_protocol_error(self) -> protocol::Error {
        protocol::Error::Err(self.message())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format;
    use crate::protocol::packet::login::clientbound::{
        EncryptionRequest, EncryptionRequest_i16, LoginDisconnect, LoginPluginRequest,
        LoginSuccess_String, LoginSuccess_UUID, LoginSuccess_UUID_WithProperties,
        SetInitialCompression,
    };
    use crate::protocol::packet::status::clientbound::StatusPong;

    const TEST_USERNAME: &str = "steven";
    const TEST_UUID: &str = "123e4567-e89b-12d3-a456-426614174000";
    const MALFORMED_UUID: &str = "not-a-valid-uuid";
    const TEST_COMPRESSION_THRESHOLD: i32 = 256;
    const TEST_MESSAGE_ID: i32 = 77;
    const TEST_STATUS_PING: i64 = 12345;
    const UNSUPPORTED_FML_NETWORK_VERSION_FOR_TEST: i64 = 3;
    const EXPECTED_PROPERTY_COUNT: usize = 1;

    fn test_uuid() -> protocol::UUID {
        protocol::UUID::from_str(TEST_UUID).unwrap()
    }

    fn text_component(text: &str) -> format::Component {
        format::Component::Text(format::TextComponent::new(text))
    }

    fn success_string_packet(uuid: &str) -> packet::Packet {
        packet::Packet::LoginSuccess_String(LoginSuccess_String {
            uuid: uuid.to_owned(),
            username: TEST_USERNAME.to_owned(),
        })
    }

    fn success_uuid_packet() -> packet::Packet {
        packet::Packet::LoginSuccess_UUID(LoginSuccess_UUID {
            uuid: test_uuid(),
            username: TEST_USERNAME.to_owned(),
        })
    }

    fn success_with_properties_packet() -> packet::Packet {
        packet::Packet::LoginSuccess_UUID_WithProperties(LoginSuccess_UUID_WithProperties {
            uuid: test_uuid(),
            username: TEST_USERNAME.to_owned(),
            properties: protocol::LenPrefixed::new(vec![packet::PlayerProperty {
                name: "textures".to_owned(),
                value: "skin".to_owned(),
                signature: Some("signature".to_owned()),
            }]),
        })
    }

    fn varint_encryption_request_packet() -> packet::Packet {
        packet::Packet::EncryptionRequest(EncryptionRequest {
            server_id: "server".to_owned(),
            public_key: protocol::LenPrefixedBytes::new(b"public-key".to_vec()),
            verify_token: protocol::LenPrefixedBytes::new(b"verify-token".to_vec()),
        })
    }

    fn i16_encryption_request_packet() -> packet::Packet {
        packet::Packet::EncryptionRequest_i16(EncryptionRequest_i16 {
            server_id: "legacy-server".to_owned(),
            public_key: protocol::LenPrefixedBytes::new(b"legacy-public-key".to_vec()),
            verify_token: protocol::LenPrefixedBytes::new(b"legacy-verify-token".to_vec()),
        })
    }

    fn compression_packet() -> packet::Packet {
        packet::Packet::SetInitialCompression(SetInitialCompression {
            threshold: protocol::VarInt(TEST_COMPRESSION_THRESHOLD),
        })
    }

    fn disconnect_packet() -> packet::Packet {
        packet::Packet::LoginDisconnect(LoginDisconnect {
            reason: text_component("closed"),
        })
    }

    fn plugin_request_packet() -> packet::Packet {
        packet::Packet::LoginPluginRequest(LoginPluginRequest {
            message_id: protocol::VarInt(TEST_MESSAGE_ID),
            channel: FML_LOGIN_WRAPPER_CHANNEL.to_owned(),
            data: Vec::new(),
        })
    }

    fn status_pong_packet() -> packet::Packet {
        packet::Packet::StatusPong(StatusPong {
            ping: TEST_STATUS_PING,
        })
    }

    fn decide(phase: LoginPhase, packet: packet::Packet) -> LoginDecision {
        decide_login_event(phase, login_event_from_packet(packet))
    }

    #[test]
    fn login_success_string_normalizes_offline_outcome() {
        match decide(LoginPhase::Initial, success_string_packet(TEST_UUID)) {
            LoginDecision::Complete(outcome) => {
                assert_eq!(outcome.username(), TEST_USERNAME);
                assert_eq!(outcome.protocol_uuid(), test_uuid());
                assert_eq!(outcome.uuid_log_value(), TEST_UUID);
                assert_eq!(outcome.property_count(), None);
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn login_success_uuid_normalizes_offline_outcome() {
        match decide(LoginPhase::Initial, success_uuid_packet()) {
            LoginDecision::Complete(outcome) => {
                assert_eq!(outcome.username(), TEST_USERNAME);
                assert_eq!(outcome.protocol_uuid(), test_uuid());
                assert_eq!(outcome.property_count(), None);
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn login_success_with_properties_preserves_property_facts() {
        match decide(LoginPhase::Encrypted, success_with_properties_packet()) {
            LoginDecision::Complete(outcome) => {
                let properties = outcome.properties().unwrap();
                assert_eq!(outcome.username(), TEST_USERNAME);
                assert_eq!(outcome.protocol_uuid(), test_uuid());
                assert_eq!(outcome.property_count(), Some(EXPECTED_PROPERTY_COUNT));
                assert_eq!(properties.len(), EXPECTED_PROPERTY_COUNT);
                assert_eq!(properties[0].name, "textures");
                assert_eq!(properties[0].signature, Some("signature".to_owned()));
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn compression_before_success_returns_apply_then_complete() {
        match decide(LoginPhase::Encrypted, compression_packet()) {
            LoginDecision::ApplyCompression(compression) => {
                assert_eq!(compression.threshold, TEST_COMPRESSION_THRESHOLD);
            }
            decision => panic!("unexpected compression decision: {:?}", decision),
        }
        match decide(LoginPhase::Encrypted, success_uuid_packet()) {
            LoginDecision::Complete(outcome) => {
                assert_eq!(outcome.protocol_uuid(), test_uuid());
            }
            decision => panic!("unexpected success decision: {:?}", decision),
        }
    }

    #[test]
    fn encryption_requests_normalize_to_begin_encryption_decisions() {
        match decide(LoginPhase::Initial, varint_encryption_request_packet()) {
            LoginDecision::BeginEncryption(request) => {
                assert_eq!(request.server_id, "server");
                assert_eq!(request.public_key, b"public-key".to_vec());
                assert_eq!(request.verify_token, b"verify-token".to_vec());
            }
            decision => panic!("unexpected varint encryption decision: {:?}", decision),
        }
        match decide(LoginPhase::Initial, i16_encryption_request_packet()) {
            LoginDecision::BeginEncryption(request) => {
                assert_eq!(request.server_id, "legacy-server");
                assert_eq!(request.public_key, b"legacy-public-key".to_vec());
                assert_eq!(request.verify_token, b"legacy-verify-token".to_vec());
            }
            decision => panic!("unexpected i16 encryption decision: {:?}", decision),
        }
    }

    #[test]
    fn login_disconnect_preserves_disconnect_reason() {
        match decide(LoginPhase::Initial, disconnect_packet()) {
            LoginDecision::Disconnect(reason) => {
                assert_eq!(reason, text_component("closed"));
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn wrong_packets_keep_stage_specific_diagnostics() {
        match decide(LoginPhase::Initial, status_pong_packet()) {
            LoginDecision::Fail(failure) => {
                assert!(failure.message().starts_with("Wrong packet 1: StatusPong"));
            }
            decision => panic!("unexpected initial decision: {:?}", decision),
        }
        match decide(LoginPhase::Encrypted, varint_encryption_request_packet()) {
            LoginDecision::Fail(failure) => {
                assert!(failure
                    .message()
                    .starts_with("Wrong packet 2: EncryptionRequest"));
            }
            decision => panic!("unexpected encrypted decision: {:?}", decision),
        }
    }

    #[test]
    fn malformed_string_uuid_panics_when_shell_reads_protocol_uuid() {
        match decide(LoginPhase::Initial, success_string_packet(MALFORMED_UUID)) {
            LoginDecision::Complete(outcome) => {
                let panic = std::panic::catch_unwind(|| outcome.protocol_uuid());
                assert!(panic.is_err());
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn unsupported_fml_network_version_preserves_panic() {
        let panic = std::panic::catch_unwind(|| {
            fml_handshake_tag(Some(UNSUPPORTED_FML_NETWORK_VERSION_FOR_TEST));
        });
        assert!(panic.is_err());
    }

    #[test]
    fn login_plugin_request_before_encryption_fails_closed() {
        match decide(LoginPhase::Initial, plugin_request_packet()) {
            LoginDecision::Fail(failure) => {
                assert!(failure
                    .message()
                    .starts_with("Wrong packet 1: LoginPluginRequest"));
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }

    #[test]
    fn encrypted_login_accepts_plugin_request_for_shell_handling() {
        match decide(LoginPhase::Encrypted, plugin_request_packet()) {
            LoginDecision::HandlePluginRequest(request) => {
                assert_eq!(request.message_id, protocol::VarInt(TEST_MESSAGE_ID));
                assert_eq!(request.channel, FML_LOGIN_WRAPPER_CHANNEL);
                assert!(request.data.is_empty());
            }
            decision => panic!("unexpected decision: {:?}", decision),
        }
    }
}
