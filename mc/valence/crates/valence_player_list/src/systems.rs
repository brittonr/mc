use bevy_ecs::prelude::*;
use valence_server::protocol::WritePacket;

type Actions = valence_server::protocol::packets::play::PlayerListActions;
type Client = valence_server::client::Client;
type GameMode = valence_server::GameMode;
type HeaderPacket<'a> = valence_server::protocol::packets::play::PlayerListHeaderS2c<'a>;
type PacketEntry<'a> = valence_server::protocol::packets::play::PlayerListEntry<'a>;
type PacketWriter<'a> = valence_server::protocol::encode::PacketWriter<'a>;
type Ping = valence_server::keepalive::Ping;
type Properties = valence_server::client::Properties;
type RemovePacket<'a> = valence_server::protocol::packets::play::PlayerRemoveS2c<'a>;
type Server = valence_server::Server;
type UniqueId = valence_server::UniqueId;
type UpdatePacket<'a> = valence_server::protocol::packets::play::PlayerListS2c<'a>;
type Username = valence_server::client::Username;
type Uuid = valence_server::uuid::Uuid;
type ComponentEntry<'a> = (
    &'a UniqueId,
    &'a Username,
    &'a Properties,
    &'a GameMode,
    &'a Ping,
    &'a super::DisplayName,
    &'a super::Listed,
);
type RefEntry<'a> = (
    &'a Ref<'a, UniqueId>,
    &'a Ref<'a, Username>,
    &'a Ref<'a, Properties>,
    &'a Ref<'a, GameMode>,
    &'a Ref<'a, Ping>,
    &'a Ref<'a, super::DisplayName>,
    &'a Ref<'a, super::Listed>,
);

pub(super) fn update_header_footer(player_list: ResMut<super::PlayerList>, server: Res<Server>) {
    if player_list.changed_header_or_footer {
        let player_list = player_list.into_inner();
        let mut writer = PacketWriter::new(
            &mut player_list.cached_update_packets,
            server.compression_threshold(),
        );

        writer.write_packet(&HeaderPacket {
            header: (&player_list.header).into(),
            footer: (&player_list.footer).into(),
        });

        player_list.changed_header_or_footer = false;
    }
}

pub(super) fn add_new_clients_to_player_list(
    clients: Query<Entity, Added<Client>>,
    player_list: Res<super::PlayerList>,
    mut commands: Commands,
) {
    if player_list.manage_clients {
        for entity in &clients {
            commands.entity(entity).insert((
                super::PlayerListEntry,
                super::DisplayName::default(),
                super::Listed::default(),
            ));
        }
    }
}

pub(super) fn init_player_list_for_clients(
    mut clients: Query<&mut Client, (Added<Client>, Without<valence_server::Despawned>)>,
    player_list: Res<super::PlayerList>,
    entries: Query<
        (
            &UniqueId,
            &Username,
            &Properties,
            &GameMode,
            &Ping,
            &super::DisplayName,
            &super::Listed,
        ),
        With<super::PlayerListEntry>,
    >,
) {
    if player_list.manage_clients {
        for mut client in &mut clients {
            let actions = add_all_actions();
            let entries: Vec<_> = entries
                .iter()
                .map(|entry| packet_entry_from_components(entry))
                .collect();

            if !entries.is_empty() {
                client.write_packet(&UpdatePacket {
                    actions,
                    entries: std::borrow::Cow::Owned(entries),
                });
            }

            if !player_list.header.is_empty() || !player_list.footer.is_empty() {
                client.write_packet(&HeaderPacket {
                    header: std::borrow::Cow::Borrowed(&player_list.header),
                    footer: std::borrow::Cow::Borrowed(&player_list.footer),
                });
            }
        }
    }
}

pub(super) fn remove_despawned_entries(
    entries: Query<
        &UniqueId,
        (
            Added<valence_server::Despawned>,
            With<super::PlayerListEntry>,
        ),
    >,
    player_list: ResMut<super::PlayerList>,
    server: Res<Server>,
    mut removed: Local<Vec<Uuid>>,
) {
    if player_list.manage_clients {
        debug_assert!(removed.is_empty());
        removed.extend(entries.iter().map(|uuid| uuid.0));

        if !removed.is_empty() {
            let player_list = player_list.into_inner();
            let mut writer = PacketWriter::new(
                &mut player_list.cached_update_packets,
                server.compression_threshold(),
            );

            writer.write_packet(&RemovePacket {
                uuids: std::borrow::Cow::Borrowed(&removed),
            });

            removed.clear();
        }
    }
}

pub(super) fn update_entries(
    entries: Query<
        (
            Ref<UniqueId>,
            Ref<Username>,
            Ref<Properties>,
            Ref<GameMode>,
            Ref<Ping>,
            Ref<super::DisplayName>,
            Ref<super::Listed>,
        ),
        (
            With<super::PlayerListEntry>,
            Or<(
                Changed<UniqueId>,
                Changed<Username>,
                Changed<Properties>,
                Changed<GameMode>,
                Changed<Ping>,
                Changed<super::DisplayName>,
                Changed<super::Listed>,
            )>,
        ),
    >,
    server: Res<Server>,
    player_list: ResMut<super::PlayerList>,
) {
    let player_list = player_list.into_inner();
    let mut writer = PacketWriter::new(
        &mut player_list.cached_update_packets,
        server.compression_threshold(),
    );

    for (uuid, username, props, game_mode, ping, display_name, listed) in &entries {
        let ref_entry = (
            &uuid,
            &username,
            &props,
            &game_mode,
            &ping,
            &display_name,
            &listed,
        );
        let actions = changed_actions(ref_entry);
        let entry = packet_entry_from_refs(ref_entry);
        debug_assert_ne!(u8::from(actions), 0);
        debug_assert_eq!(entry.player_uuid, uuid.0);
        writer.write_packet(&UpdatePacket {
            actions,
            entries: std::borrow::Cow::Borrowed(&[entry]),
        });
    }
}

pub(super) fn write_player_list_changes(
    mut player_list: ResMut<super::PlayerList>,
    mut clients: Query<&mut Client, Without<valence_server::Despawned>>,
) {
    if !player_list.cached_update_packets.is_empty() {
        for mut client in &mut clients {
            if !client.is_added() {
                client.write_packet_bytes(&player_list.cached_update_packets);
            }
        }

        player_list.cached_update_packets.clear();
    }
}

fn add_all_actions() -> Actions {
    Actions::new()
        .with_add_player(true)
        .with_update_game_mode(true)
        .with_update_listed(true)
        .with_update_latency(true)
        .with_update_display_name(true)
}

fn changed_actions(entry: RefEntry<'_>) -> Actions {
    let (uuid, username, props, game_mode, ping, display_name, listed) = entry;
    let mut actions = Actions::new();
    if uuid.is_changed() || username.is_changed() || props.is_changed() {
        actions.set_add_player(true);
        actions.set_update_game_mode(**game_mode != GameMode::Survival);
        actions.set_update_latency(ping.0 != 0);
        actions.set_update_display_name(display_name.0.is_some());
        actions.set_update_listed(listed.0);
    } else {
        actions.set_update_game_mode(game_mode.is_changed());
        actions.set_update_latency(ping.is_changed());
        actions.set_update_display_name(display_name.is_changed());
        actions.set_update_listed(listed.is_changed());
        debug_assert_ne!(u8::from(actions), 0);
    }
    actions
}

fn packet_entry_from_components<'a>(entry: ComponentEntry<'a>) -> PacketEntry<'a> {
    let (uuid, username, props, game_mode, ping, display_name, listed) = entry;
    PacketEntry {
        player_uuid: uuid.0,
        username: &username.0,
        properties: std::borrow::Cow::Borrowed(&props.0),
        chat_data: None,
        listed: listed.0,
        ping: ping.0,
        game_mode: *game_mode,
        display_name: display_name.0.as_ref().map(std::borrow::Cow::Borrowed),
    }
}

fn packet_entry_from_refs<'a>(entry: RefEntry<'a>) -> PacketEntry<'a> {
    let (uuid, username, props, game_mode, ping, display_name, listed) = entry;
    PacketEntry {
        player_uuid: uuid.0,
        username: &username.0,
        properties: std::borrow::Cow::Borrowed(&props.0),
        chat_data: None,
        listed: listed.0,
        ping: ping.0,
        game_mode: **game_mode,
        display_name: display_name.0.as_ref().map(|value| value.into()),
    }
}
