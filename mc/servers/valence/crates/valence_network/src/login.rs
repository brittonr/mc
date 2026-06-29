use std::net::SocketAddr;

use anyhow::{bail, ensure, Context};
use hmac::digest::Update;
use hmac::{Hmac, Mac};
use num_bigint::BigInt;
use reqwest::StatusCode;
use rsa::Pkcs1v15Encrypt;
use serde::Deserialize;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use valence_lang::keys;
use valence_protocol::profile::Property;
use valence_protocol::Decode;
use valence_server::client::Properties;
use valence_server::protocol::packets::login::{
    LoginCompressionS2c, LoginDisconnectS2c, LoginHelloC2s, LoginHelloS2c, LoginKeyC2s,
    LoginQueryRequestS2c, LoginQueryResponseC2s, LoginSuccessS2c,
};
use valence_server::protocol::{RawBytes, VarInt};
use valence_server::text::{Color, IntoText};
use valence_server::{ident, Text, MINECRAFT_VERSION, PROTOCOL_VERSION};

use crate::connect::HandshakeData;
use crate::packet_io::PacketIo;
use crate::session_core::{
    bungeecord_profile, compression_decision, login_protocol_decision, offline_profile,
    CompressionDecision, LoginProtocolDecision,
};
use crate::{CleanupOnDrop, ConnectionMode, NewClientInfo, SharedNetworkState};

const ONLINE_VERIFY_TOKEN_BYTES: usize = 16;
const VELOCITY_MIN_SUPPORTED_VERSION: u8 = 1;
const VELOCITY_MODERN_FORWARDING_WITH_KEY_V2: i32 = 3;
const VELOCITY_MESSAGE_ID: i32 = 0;
const VELOCITY_SIGNATURE_BYTES: usize = 32;

/// Handle the login process and return the new client's data if successful.
pub(crate) async fn handle_login(
    shared: &SharedNetworkState,
    io: &mut PacketIo,
    remote_addr: SocketAddr,
    handshake: HandshakeData,
) -> anyhow::Result<Option<(NewClientInfo, CleanupOnDrop)>> {
    match login_protocol_decision(
        handshake.protocol_version,
        PROTOCOL_VERSION,
        MINECRAFT_VERSION,
    ) {
        LoginProtocolDecision::Accept => {}
        LoginProtocolDecision::DisconnectMismatchedVersion { server_version } => {
            io.send_packet(&LoginDisconnectS2c {
                // TODO: use correct translation key.
                reason: format!("Mismatched Minecraft version (server is on {server_version})")
                    .color(Color::RED)
                    .into(),
            })
            .await?;

            return Ok(None);
        }
    }

    let LoginHelloC2s {
        username,
        .. // TODO: profile_id
    } = io.recv_packet().await?;

    let username = username.0.to_owned();

    let info = match shared.connection_mode() {
        ConnectionMode::Online { .. } => login_online(shared, io, remote_addr, username).await?,
        ConnectionMode::Offline => offline_profile(remote_addr, username)?,
        ConnectionMode::BungeeCord => {
            bungeecord_profile(remote_addr, &handshake.server_address, username)?
        }
        ConnectionMode::Velocity { secret } => login_velocity(io, username, secret).await?,
    };

    match compression_decision(shared.0.threshold) {
        CompressionDecision::Enable(threshold) => {
            io.send_packet(&LoginCompressionS2c {
                threshold: threshold.0.into(),
            })
            .await?;

            io.set_compression(threshold);
        }
        CompressionDecision::Disabled => {}
    }

    let cleanup = match shared.0.callbacks.inner.login(shared, &info).await {
        Ok(f) => CleanupOnDrop(Some(f)),
        Err(reason) => {
            tracing::info!("disconnect at login: \"{reason}\"");
            io.send_packet(&LoginDisconnectS2c {
                reason: reason.into(),
            })
            .await?;
            return Ok(None);
        }
    };

    io.send_packet(&LoginSuccessS2c {
        uuid: info.uuid,
        username: info.username.as_str().into(),
        properties: Default::default(),
    })
    .await?;

    Ok(Some((info, cleanup)))
}

/// Login procedure for online mode.
async fn login_online(
    shared: &SharedNetworkState,
    io: &mut PacketIo,
    remote_addr: SocketAddr,
    username: String,
) -> anyhow::Result<NewClientInfo> {
    let my_verify_token: [u8; ONLINE_VERIFY_TOKEN_BYTES] = rand::random();

    io.send_packet(&LoginHelloS2c {
        server_id: "".into(), // Always empty
        public_key: &shared.0.public_key_der,
        verify_token: &my_verify_token,
    })
    .await?;

    let LoginKeyC2s {
        shared_secret,
        verify_token: encrypted_verify_token,
    } = io.recv_packet().await?;

    let shared_secret = shared
        .0
        .rsa_key
        .decrypt(Pkcs1v15Encrypt, shared_secret)
        .context("failed to decrypt shared secret")?;

    let verify_token = shared
        .0
        .rsa_key
        .decrypt(Pkcs1v15Encrypt, encrypted_verify_token)
        .context("failed to decrypt verify token")?;

    ensure!(
        my_verify_token.as_slice() == verify_token,
        "verify tokens do not match"
    );

    let crypt_key: [u8; ONLINE_VERIFY_TOKEN_BYTES] = shared_secret
        .as_slice()
        .try_into()
        .context("shared secret has the wrong length")?;

    io.enable_encryption(&crypt_key);

    let hash = Sha1::new()
        .chain(&shared_secret)
        .chain(&shared.0.public_key_der)
        .finalize();

    let url = shared
        .0
        .callbacks
        .inner
        .session_server(
            shared,
            username.as_str(),
            &auth_digest(&hash),
            &remote_addr.ip(),
        )
        .await;

    let resp = shared.0.http_client.get(url).send().await?;

    match resp.status() {
        StatusCode::OK => {}
        StatusCode::NO_CONTENT => {
            let reason = Text::translate(keys::MULTIPLAYER_DISCONNECT_UNVERIFIED_USERNAME, []);
            io.send_packet(&LoginDisconnectS2c {
                reason: reason.into(),
            })
            .await?;
            bail!("session server could not verify username");
        }
        status => {
            bail!("session server GET request failed (status code {status})");
        }
    }

    #[derive(Deserialize)]
    struct GameProfile {
        id: Uuid,
        name: String,
        properties: Vec<Property>,
    }

    let profile: GameProfile = resp.json().await.context("parsing game profile")?;

    ensure!(profile.name == username, "usernames do not match");

    Ok(NewClientInfo {
        uuid: profile.id,
        username,
        ip: remote_addr.ip(),
        properties: Properties(profile.properties),
    })
}

fn auth_digest(bytes: &[u8]) -> String {
    BigInt::from_signed_bytes_be(bytes).to_str_radix(16)
}

/// Login procedure for Velocity.
async fn login_velocity(
    io: &mut PacketIo,
    username: String,
    velocity_secret: &str,
) -> anyhow::Result<NewClientInfo> {
    // Send Player Info Request into the Plugin Channel.
    io.send_packet(&LoginQueryRequestS2c {
        message_id: VarInt(VELOCITY_MESSAGE_ID),
        channel: ident!("velocity:player_info").into(),
        data: RawBytes(&[VELOCITY_MIN_SUPPORTED_VERSION]).into(),
    })
    .await?;

    let plugin_response: LoginQueryResponseC2s = io.recv_packet().await?;

    ensure!(
        plugin_response.message_id.0 == VELOCITY_MESSAGE_ID,
        "mismatched plugin response ID (got {}, expected {VELOCITY_MESSAGE_ID})",
        plugin_response.message_id.0,
    );

    let data = plugin_response
        .data
        .context("missing plugin response data")?
        .0;

    ensure!(
        data.len() >= VELOCITY_SIGNATURE_BYTES,
        "invalid plugin response data length"
    );
    let (signature, mut data_without_signature) = data.split_at(VELOCITY_SIGNATURE_BYTES);

    let mut mac = Hmac::<Sha256>::new_from_slice(velocity_secret.as_bytes())?;
    Mac::update(&mut mac, data_without_signature);
    mac.verify_slice(signature)?;

    let version = VarInt::decode(&mut data_without_signature)
        .context("failed to decode velocity version")?
        .0;

    let remote_addr = String::decode(&mut data_without_signature)?.parse()?;
    let uuid = Uuid::decode(&mut data_without_signature)?;

    ensure!(
        username == <&str>::decode(&mut data_without_signature)?,
        "mismatched usernames"
    );

    let properties = Vec::<Property>::decode(&mut data_without_signature)
        .context("decoding velocity game profile properties")?;

    if version >= VELOCITY_MODERN_FORWARDING_WITH_KEY_V2 {
        // TODO
    }

    Ok(NewClientInfo {
        uuid,
        username,
        properties: Properties(properties),
        ip: remote_addr,
    })
}

#[cfg(test)]
mod tests {
    use sha1::Digest;

    use super::*;

    #[test]
    fn auth_digest_usernames() {
        assert_eq!(
            auth_digest(&Sha1::digest("Notch")),
            "4ed1f46bbe04bc756bcb17c0c7ce3e4632f06a48"
        );
        assert_eq!(
            auth_digest(&Sha1::digest("jeb_")),
            "-7c9d5b0044c130109a5d7b5fb5c317c02b4e28c1"
        );
        assert_eq!(
            auth_digest(&Sha1::digest("simon")),
            "88e16a1019277b15d58faf0541e11910eb756f6"
        );
    }
}
