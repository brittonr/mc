use std::io;
use std::net::{IpAddr, SocketAddr};

use anyhow::Context;
use serde_json::from_str;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use valence_protocol::profile::Property;
use valence_protocol::CompressionThreshold;
use valence_server::client::Properties;
use valence_server::protocol::packets::handshaking::handshake_c2s::HandshakeNextState;

use crate::{ConnectionMode, NewClientInfo};

pub(crate) const MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS: usize = 255;
const OFFLINE_UUID_BYTES: usize = 16;
const BUNGEECORD_MAX_FIELDS: usize = 4;
const BUNGEECORD_IP_FIELD_INDEX: usize = 1;
const BUNGEECORD_UUID_FIELD_INDEX: usize = 2;
const BUNGEECORD_PROPERTIES_FIELD_INDEX: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum HandshakeAddressDecision {
    Accept,
    RejectTooLong {
        utf16_len: usize,
        max_utf16_len: usize,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum SessionPhase {
    Handshake,
    Status,
    Login,
    Play,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct InvalidSessionTransition {
    pub(crate) from: SessionPhase,
    pub(crate) to: SessionPhase,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum LoginProtocolDecision<'a> {
    Accept,
    DisconnectMismatchedVersion { server_version: &'a str },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum CompressionDecision {
    Enable(CompressionThreshold),
    Disabled,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ConnectionIoDecision {
    IgnoreUnexpectedEof,
    Warn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum InitialConnectionDecision {
    Continue,
    WarnTimeout,
}

pub(crate) fn classify_handshake_address(
    connection_mode: &ConnectionMode,
    server_address: &str,
) -> HandshakeAddressDecision {
    if matches!(connection_mode, ConnectionMode::BungeeCord) {
        return HandshakeAddressDecision::Accept;
    }

    let utf16_len = server_address.encode_utf16().count();
    if utf16_len <= MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS {
        HandshakeAddressDecision::Accept
    } else {
        HandshakeAddressDecision::RejectTooLong {
            utf16_len,
            max_utf16_len: MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS,
        }
    }
}

pub(crate) fn phase_after_handshake(next_state: HandshakeNextState) -> SessionPhase {
    match next_state {
        HandshakeNextState::Status => SessionPhase::Status,
        HandshakeNextState::Login => SessionPhase::Login,
    }
}

pub(crate) fn classify_session_transition(
    from: SessionPhase,
    to: SessionPhase,
) -> Result<(), InvalidSessionTransition> {
    let valid = matches!(
        (from, to),
        (
            SessionPhase::Handshake,
            SessionPhase::Status | SessionPhase::Login
        ) | (SessionPhase::Login, SessionPhase::Play)
    );

    if valid {
        Ok(())
    } else {
        Err(InvalidSessionTransition { from, to })
    }
}

pub(crate) fn login_protocol_decision<'a>(
    client_protocol: i32,
    server_protocol: i32,
    server_version: &'a str,
) -> LoginProtocolDecision<'a> {
    if client_protocol == server_protocol {
        LoginProtocolDecision::Accept
    } else {
        LoginProtocolDecision::DisconnectMismatchedVersion { server_version }
    }
}

pub(crate) fn compression_decision(threshold: CompressionThreshold) -> CompressionDecision {
    if threshold.0 > 0 {
        CompressionDecision::Enable(threshold)
    } else {
        CompressionDecision::Disabled
    }
}

pub(crate) fn classify_connection_io_error(kind: io::ErrorKind) -> ConnectionIoDecision {
    if kind == io::ErrorKind::UnexpectedEof {
        ConnectionIoDecision::IgnoreUnexpectedEof
    } else {
        ConnectionIoDecision::Warn
    }
}

pub(crate) fn classify_initial_connection_timeout(timed_out: bool) -> InitialConnectionDecision {
    if timed_out {
        InitialConnectionDecision::WarnTimeout
    } else {
        InitialConnectionDecision::Continue
    }
}

pub(crate) fn default_session_server_url(
    prevent_proxy_connections: bool,
    username: &str,
    auth_digest: &str,
    player_ip: &IpAddr,
) -> String {
    if prevent_proxy_connections {
        format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={auth_digest}&ip={player_ip}")
    } else {
        format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={username}&serverId={auth_digest}")
    }
}

pub(crate) fn offline_uuid(username: &str) -> anyhow::Result<Uuid> {
    Uuid::from_slice(&Sha256::digest(username)[..OFFLINE_UUID_BYTES]).map_err(Into::into)
}

pub(crate) fn offline_profile(
    remote_addr: SocketAddr,
    username: String,
) -> anyhow::Result<NewClientInfo> {
    Ok(NewClientInfo {
        uuid: offline_uuid(username.as_str())?,
        username,
        properties: Properties::default(),
        ip: remote_addr.ip(),
    })
}

pub(crate) fn bungeecord_profile(
    remote_addr: SocketAddr,
    server_address: &str,
    username: String,
) -> anyhow::Result<NewClientInfo> {
    let data = server_address
        .split('\0')
        .take(BUNGEECORD_MAX_FIELDS)
        .collect::<Vec<_>>();

    let ip = match data.get(BUNGEECORD_IP_FIELD_INDEX) {
        Some(ip) => ip.parse()?,
        None => remote_addr.ip(),
    };

    let uuid = match data.get(BUNGEECORD_UUID_FIELD_INDEX) {
        Some(uuid) => uuid.parse()?,
        None => offline_uuid(username.as_str())?,
    };

    let properties = match data.get(BUNGEECORD_PROPERTIES_FIELD_INDEX) {
        Some(properties) => from_str::<Vec<Property>>(properties)
            .context("failed to parse BungeeCord player properties")?,
        None => Vec::new(),
    };

    Ok(NewClientInfo {
        uuid,
        username,
        properties: Properties(properties),
        ip,
    })
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};
    use std::sync::Arc;

    use super::*;

    const CLIENT_PROTOCOL: i32 = 763;
    const SERVER_PROTOCOL: i32 = 763;
    const OTHER_PROTOCOL: i32 = 762;
    const SERVER_VERSION: &str = "1.20.1";
    const POSITIVE_THRESHOLD: i32 = 256;
    const ZERO_THRESHOLD: i32 = 0;
    const NEGATIVE_THRESHOLD: i32 = -1;
    const REMOTE_PORT: u16 = 25565;
    const TEST_UUID: &str = "11111111-2222-3333-4444-555555555555";
    const TEST_USERNAME: &str = "player";
    const TEST_AUTH_DIGEST: &str = "abc123";

    fn remote_addr() -> SocketAddr {
        SocketAddr::from((Ipv4Addr::new(127, 0, 0, 1), REMOTE_PORT))
    }

    #[test]
    fn handshake_address_accepts_normal_and_bungeecord_forwarding_addresses() {
        assert_eq!(
            classify_handshake_address(&ConnectionMode::Offline, "localhost"),
            HandshakeAddressDecision::Accept
        );

        let long_forwarded_address = "a".repeat(MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS + 1);
        assert_eq!(
            classify_handshake_address(&ConnectionMode::BungeeCord, &long_forwarded_address),
            HandshakeAddressDecision::Accept
        );
    }

    #[test]
    fn handshake_address_rejects_too_long_non_bungeecord_addresses() {
        let server_address = "a".repeat(MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS + 1);

        assert_eq!(
            classify_handshake_address(&ConnectionMode::Offline, &server_address),
            HandshakeAddressDecision::RejectTooLong {
                utf16_len: MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS + 1,
                max_utf16_len: MAX_HANDSHAKE_ADDRESS_UTF16_CODE_UNITS,
            }
        );
    }

    #[test]
    fn session_transitions_allow_handshake_targets_and_login_play() {
        assert_eq!(
            phase_after_handshake(HandshakeNextState::Status),
            SessionPhase::Status
        );
        assert_eq!(
            phase_after_handshake(HandshakeNextState::Login),
            SessionPhase::Login
        );
        assert!(classify_session_transition(SessionPhase::Handshake, SessionPhase::Status).is_ok());
        assert!(classify_session_transition(SessionPhase::Handshake, SessionPhase::Login).is_ok());
        assert!(classify_session_transition(SessionPhase::Login, SessionPhase::Play).is_ok());
    }

    #[test]
    fn session_transitions_reject_invalid_state_changes() {
        assert_eq!(
            classify_session_transition(SessionPhase::Status, SessionPhase::Play),
            Err(InvalidSessionTransition {
                from: SessionPhase::Status,
                to: SessionPhase::Play,
            })
        );
    }

    #[test]
    fn login_protocol_accepts_matching_protocol() {
        assert_eq!(
            login_protocol_decision(CLIENT_PROTOCOL, SERVER_PROTOCOL, SERVER_VERSION),
            LoginProtocolDecision::Accept
        );
    }

    #[test]
    fn login_protocol_disconnects_mismatched_protocol() {
        assert_eq!(
            login_protocol_decision(OTHER_PROTOCOL, SERVER_PROTOCOL, SERVER_VERSION),
            LoginProtocolDecision::DisconnectMismatchedVersion {
                server_version: SERVER_VERSION,
            }
        );
    }

    #[test]
    fn compression_decision_enables_positive_threshold() {
        assert_eq!(
            compression_decision(CompressionThreshold(POSITIVE_THRESHOLD)),
            CompressionDecision::Enable(CompressionThreshold(POSITIVE_THRESHOLD))
        );
    }

    #[test]
    fn compression_decision_disables_zero_and_negative_thresholds() {
        assert_eq!(
            compression_decision(CompressionThreshold(ZERO_THRESHOLD)),
            CompressionDecision::Disabled
        );
        assert_eq!(
            compression_decision(CompressionThreshold(NEGATIVE_THRESHOLD)),
            CompressionDecision::Disabled
        );
    }

    #[test]
    fn connection_io_classification_ignores_eof_and_warns_for_other_errors() {
        assert_eq!(
            classify_connection_io_error(io::ErrorKind::UnexpectedEof),
            ConnectionIoDecision::IgnoreUnexpectedEof
        );
        assert_eq!(
            classify_connection_io_error(io::ErrorKind::ConnectionReset),
            ConnectionIoDecision::Warn
        );
    }

    #[test]
    fn initial_connection_timeout_classification_warns_only_on_timeout() {
        assert_eq!(
            classify_initial_connection_timeout(false),
            InitialConnectionDecision::Continue
        );
        assert_eq!(
            classify_initial_connection_timeout(true),
            InitialConnectionDecision::WarnTimeout
        );
    }

    #[test]
    fn offline_profile_uses_remote_ip_and_deterministic_uuid() {
        let profile = offline_profile(remote_addr(), TEST_USERNAME.to_owned()).unwrap();
        let repeated = offline_profile(remote_addr(), TEST_USERNAME.to_owned()).unwrap();

        assert_eq!(profile.uuid, repeated.uuid);
        assert_eq!(profile.username, TEST_USERNAME);
        assert_eq!(profile.ip, remote_addr().ip());
        assert!(profile.properties.0.is_empty());
    }

    #[test]
    fn bungeecord_profile_uses_forwarded_profile_data() {
        let forwarded_ip = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 10));
        let properties = r#"[{"name":"textures","value":"value","signature":"signature"}]"#;
        let server_address = format!("localhost\0{forwarded_ip}\0{TEST_UUID}\0{properties}");
        let profile =
            bungeecord_profile(remote_addr(), &server_address, TEST_USERNAME.to_owned()).unwrap();

        assert_eq!(profile.ip, forwarded_ip);
        assert_eq!(profile.uuid, TEST_UUID.parse::<Uuid>().unwrap());
        assert_eq!(profile.properties.0.len(), 1);
        assert_eq!(profile.properties.0[0].name, "textures");
    }

    #[test]
    fn bungeecord_profile_missing_forwarded_data_uses_offline_fallback() {
        let profile =
            bungeecord_profile(remote_addr(), "localhost", TEST_USERNAME.to_owned()).unwrap();

        assert_eq!(profile.ip, remote_addr().ip());
        assert_eq!(profile.uuid, offline_uuid(TEST_USERNAME).unwrap());
        assert!(profile.properties.0.is_empty());
    }

    #[test]
    fn bungeecord_profile_rejects_malformed_properties() {
        let server_address = format!("localhost\0192.0.2.10\0{TEST_UUID}\0not json");

        assert!(
            bungeecord_profile(remote_addr(), &server_address, TEST_USERNAME.to_owned()).is_err()
        );
    }

    #[test]
    fn default_session_server_url_includes_ip_only_when_proxy_prevention_is_enabled() {
        let player_ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 8));
        let without_ip =
            default_session_server_url(false, TEST_USERNAME, TEST_AUTH_DIGEST, &player_ip);
        let with_ip = default_session_server_url(true, TEST_USERNAME, TEST_AUTH_DIGEST, &player_ip);

        assert_eq!(
            without_ip,
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=player&serverId=abc123"
        );
        assert_eq!(
            with_ip,
            "https://sessionserver.mojang.com/session/minecraft/hasJoined?username=player&serverId=abc123&ip=203.0.113.8"
        );
    }

    #[test]
    fn velocity_connection_mode_remains_supported_by_address_decision() {
        let mode = ConnectionMode::Velocity {
            secret: Arc::from("secret"),
        };

        assert_eq!(
            classify_handshake_address(&mode, "localhost"),
            HandshakeAddressDecision::Accept
        );
    }
}
