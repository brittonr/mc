use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;

use async_trait::async_trait;
use base64::prelude::*;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;
use valence_protocol::text::IntoText;
use valence_server::protocol::packets::status::{
    QueryPingC2s, QueryPongS2c, QueryRequestC2s, QueryResponseS2c,
};
use valence_server::text::Color;
use valence_server::{Text, MINECRAFT_VERSION, PROTOCOL_VERSION};

use crate::connect::HandshakeData;
use crate::packet_io::PacketIo;
use crate::{NetworkCallbacks, SharedNetworkState};

const PRE_1_16_PROTOCOL_VERSION: i32 = 735;
const FAVICON_DATA_URI_PREFIX: &str = "data:image/png;base64,";

/// Stable, resource-owned data for server-list status responses.
///
/// This is a small compatibility-test seam: applications can install this as a
/// [`NetworkCallbacks`] implementation or use [`Self::to_server_list_ping`]
/// from their own callback to keep MOTD/version/player-sample responses
/// deterministic.
#[derive(Clone, Debug)]
pub struct StatusResponseResource {
    /// Displayed online player count.
    pub online_players: i32,
    /// Displayed maximum player count.
    pub max_players: i32,
    /// Hover sample entries.
    pub player_sample: Vec<PlayerSampleEntry>,
    /// Server description/MOTD.
    pub description: Text,
    /// Server version name.
    pub version_name: String,
    /// Server protocol.
    pub protocol: i32,
    /// Optional static favicon PNG bytes. Empty means no icon.
    pub favicon_png: &'static [u8],
}

impl Default for StatusResponseResource {
    fn default() -> Self {
        Self {
            online_players: 0,
            max_players: 20,
            player_sample: Vec::new(),
            description: "A Valence Server".into_text(),
            version_name: MINECRAFT_VERSION.to_owned(),
            protocol: PROTOCOL_VERSION,
            favicon_png: &[],
        }
    }
}

impl StatusResponseResource {
    /// Convert the resource into the existing status-ping callback response.
    pub fn to_server_list_ping(&self) -> ServerListPing<'static> {
        ServerListPing::Respond {
            online_players: self.online_players,
            max_players: self.max_players,
            player_sample: self.player_sample.clone(),
            description: self.description.clone(),
            favicon_png: self.favicon_png,
            version_name: self.version_name.clone(),
            protocol: self.protocol,
        }
    }
}

#[async_trait]
impl NetworkCallbacks for StatusResponseResource {
    async fn server_list_ping(
        &self,
        _shared: &SharedNetworkState,
        _remote_addr: SocketAddr,
        _handshake_data: &HandshakeData,
    ) -> ServerListPing {
        self.to_server_list_ping()
    }
}

/// The result of the Server List Ping [callback].
///
/// [callback]: NetworkCallbacks::server_list_ping
#[derive(Clone, Default, Debug)]
pub enum ServerListPing<'a> {
    /// Responds to the server list ping with the given information.
    Respond {
        /// Displayed as the number of players on the server.
        online_players: i32,
        /// Displayed as the maximum number of players allowed on the server at
        /// a time.
        max_players: i32,
        /// The list of players visible by hovering over the player count.
        ///
        /// Has no effect if this list is empty.
        player_sample: Vec<PlayerSampleEntry>,
        /// A description of the server.
        description: Text,
        /// The server's icon as the bytes of a PNG image.
        /// The image must be 64x64 pixels.
        ///
        /// No icon is used if the slice is empty.
        favicon_png: &'a [u8],
        /// The version name of the server. Displayed when client is using a
        /// different protocol.
        ///
        /// Can be formatted using `§` and format codes. Or use
        /// [`valence_protocol::text::Text::to_legacy_lossy`].
        version_name: String,
        /// The protocol version of the server.
        protocol: i32,
    },
    /// Ignores the query and disconnects from the client.
    #[default]
    Ignore,
}

/// The result of the Server List Legacy Ping [callback].
///
/// [callback]: NetworkCallbacks::server_list_legacy_ping
#[derive(Clone, Default, Debug)]
pub enum ServerListLegacyPing {
    /// Responds to the server list legacy ping with the given information.
    Respond(crate::legacy_ping::ServerListLegacyPingResponse),
    /// Ignores the query and disconnects from the client.
    #[default]
    Ignore,
}

/// The result of the Broadcast To Lan [callback].
///
/// [callback]: NetworkCallbacks::broadcast_to_lan
#[derive(Clone, Default, Debug)]
pub enum BroadcastToLan<'a> {
    /// Disabled Broadcast To Lan.
    #[default]
    Disabled,
    /// Send packet to broadcast to LAN every 1.5 seconds with specified MOTD.
    Enabled(Cow<'a, str>),
}

/// Represents an individual entry in the player sample.
#[derive(Clone, Debug, Serialize)]
pub struct PlayerSampleEntry {
    /// The name of the player.
    ///
    /// This string can contain
    /// [legacy formatting codes](https://minecraft.wiki/w/Formatting_codes).
    pub name: String,
    /// The player UUID.
    pub id: Uuid,
}

pub(crate) async fn handle_status(
    shared: SharedNetworkState,
    mut io: PacketIo,
    remote_addr: SocketAddr,
    handshake: HandshakeData,
) -> anyhow::Result<()> {
    io.recv_packet::<QueryRequestC2s>().await?;

    let response = shared
        .0
        .callbacks
        .inner
        .server_list_ping(&shared, remote_addr, &handshake)
        .await;

    let Some(json) = status_response_json(response, handshake.protocol_version) else {
        return Ok(());
    };

    io.send_packet(&QueryResponseS2c { json: &json }).await?;

    let QueryPingC2s { payload } = io.recv_packet().await?;

    io.send_packet(&QueryPongS2c { payload }).await?;

    Ok(())
}

pub(crate) fn status_response_json(
    response: ServerListPing<'_>,
    protocol_version: i32,
) -> Option<String> {
    let ServerListPing::Respond {
        online_players,
        max_players,
        player_sample,
        description,
        favicon_png,
        version_name,
        protocol,
    } = response
    else {
        return None;
    };

    let description = status_description_for_protocol(description, protocol_version);

    let mut json = json!({
        "version": {
            "name": version_name,
            "protocol": protocol,
        },
        "players": {
            "online": online_players,
            "max": max_players,
            "sample": player_sample,
        },
        "description": description,
    });

    if !favicon_png.is_empty() {
        let mut buf = FAVICON_DATA_URI_PREFIX.to_owned();
        BASE64_STANDARD.encode_string(favicon_png, &mut buf);
        json["favicon"] = Value::String(buf);
    }

    Some(json.to_string())
}

fn status_description_for_protocol(mut description: Text, protocol_version: i32) -> Text {
    if protocol_version < PRE_1_16_PROTOCOL_VERSION {
        fallback_webcolors(&mut description);
    }

    description
}

fn fallback_webcolors(txt: &mut Text) {
    if let Some(Color::Rgb(color)) = txt.color {
        txt.color = Some(Color::Named(color.to_named_lossy()));
    }

    for child in &mut txt.extra {
        fallback_webcolors(child);
    }
}

pub(crate) fn default_server_list_ping(shared: &SharedNetworkState) -> ServerListPing<'static> {
    ServerListPing::Respond {
        online_players: shared.player_count().load(Ordering::Relaxed) as i32,
        max_players: shared.max_players() as i32,
        player_sample: vec![],
        description: "A Valence Server".into_text(),
        favicon_png: &[],
        version_name: MINECRAFT_VERSION.to_owned(),
        protocol: PROTOCOL_VERSION,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use valence_server::text::IntoText;

    use super::*;

    const CURRENT_PROTOCOL: i32 = PROTOCOL_VERSION;
    const PRE_WEB_COLOR_PROTOCOL: i32 = PRE_1_16_PROTOCOL_VERSION - 1;
    const STATUS_ONLINE_PLAYERS: i32 = 7;
    const STATUS_MAX_PLAYERS: i32 = 20;
    const SAMPLE_UUID: Uuid = Uuid::from_u128(0x11111111222233334444555555555555);
    const FAVICON_BYTES: &[u8] = b"png";
    const RGB_MAX_CHANNEL: u8 = 255;
    const RGB_MIN_CHANNEL: u8 = 0;

    fn status_response(description: Text, favicon_png: &'static [u8]) -> ServerListPing<'static> {
        ServerListPing::Respond {
            online_players: STATUS_ONLINE_PLAYERS,
            max_players: STATUS_MAX_PLAYERS,
            player_sample: vec![PlayerSampleEntry {
                name: "sample".to_owned(),
                id: SAMPLE_UUID,
            }],
            description,
            favicon_png,
            version_name: "Valence".to_owned(),
            protocol: CURRENT_PROTOCOL,
        }
    }

    #[test]
    fn status_response_json_includes_status_facts_sample_and_favicon() {
        let json = status_response_json(
            status_response("hello".into_text(), FAVICON_BYTES),
            CURRENT_PROTOCOL,
        )
        .unwrap();
        let value = serde_json::from_str::<Value>(&json).unwrap();

        assert_eq!(value["version"]["name"], "Valence");
        assert_eq!(value["version"]["protocol"], CURRENT_PROTOCOL);
        assert_eq!(value["players"]["online"], STATUS_ONLINE_PLAYERS);
        assert_eq!(value["players"]["max"], STATUS_MAX_PLAYERS);
        assert_eq!(value["players"]["sample"][0]["name"], "sample");
        assert_eq!(value["favicon"], "data:image/png;base64,cG5n");
    }

    #[test]
    fn status_response_json_omits_empty_favicon() {
        let json =
            status_response_json(status_response("hello".into_text(), &[]), CURRENT_PROTOCOL)
                .unwrap();
        let value = serde_json::from_str::<Value>(&json).unwrap();

        assert!(value.get("favicon").is_none());
    }

    #[test]
    fn ignored_status_response_has_no_json() {
        assert!(status_response_json(ServerListPing::Ignore, CURRENT_PROTOCOL).is_none());
    }

    #[test]
    fn pre_1_16_status_response_falls_back_from_rgb_to_named_color() {
        let json = status_response_json(
            status_response(
                "hello".color(Color::rgb(
                    RGB_MAX_CHANNEL,
                    RGB_MIN_CHANNEL,
                    RGB_MIN_CHANNEL,
                )),
                &[],
            ),
            PRE_WEB_COLOR_PROTOCOL,
        )
        .unwrap();
        let value = serde_json::from_str::<Value>(&json).unwrap();

        assert_eq!(value["description"]["color"], "dark_red");
    }
}
