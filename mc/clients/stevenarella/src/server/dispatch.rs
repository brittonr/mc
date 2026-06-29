use crate::protocol::{self, packet};
use log::warn;

use super::Server;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ServerPacketFamily {
    SessionLifecycle,
    PluginMessage,
    WorldDimension,
    Chunk,
    Entity,
    InventoryWindow,
    ChatPlayerList,
}

pub(crate) fn packet_family(packet: &packet::Packet) -> Option<ServerPacketFamily> {
    match packet {
        packet::Packet::KeepAliveClientbound_i64(_)
        | packet::Packet::KeepAliveClientbound_VarInt(_)
        | packet::Packet::KeepAliveClientbound_i32(_)
        | packet::Packet::TeleportPlayer_WithDismount(_)
        | packet::Packet::TeleportPlayer_WithConfirm(_)
        | packet::Packet::TeleportPlayer_NoConfirm(_)
        | packet::Packet::TeleportPlayer_OnGround(_)
        | packet::Packet::Disconnect(_) => Some(ServerPacketFamily::SessionLifecycle),

        packet::Packet::PluginMessageClientbound_i16(_)
        | packet::Packet::PluginMessageClientbound(_) => Some(ServerPacketFamily::PluginMessage),

        packet::Packet::JoinGame_WorldNames_IsHard_SimDist_LastDeath_PortalCooldown(_)
        | packet::Packet::JoinGame_WorldNames_IsHard_SimDist(_)
        | packet::Packet::JoinGame_WorldNames_IsHard(_)
        | packet::Packet::JoinGame_WorldNames(_)
        | packet::Packet::JoinGame_HashedSeed_Respawn(_)
        | packet::Packet::JoinGame_i32_ViewDistance(_)
        | packet::Packet::JoinGame_i32(_)
        | packet::Packet::JoinGame_i8(_)
        | packet::Packet::JoinGame_i8_NoDebug(_)
        | packet::Packet::Respawn_Gamemode(_)
        | packet::Packet::Respawn_HashedSeed(_)
        | packet::Packet::Respawn_WorldName(_)
        | packet::Packet::Respawn_WorldNames_LastDeath_PortalCooldown(_)
        | packet::Packet::Respawn_NBT(_)
        | packet::Packet::TimeUpdate(_)
        | packet::Packet::ChangeGameState(_)
        | packet::Packet::UpdateHealth(_)
        | packet::Packet::UpdateBlockEntity_VarInt(_)
        | packet::Packet::UpdateBlockEntity_u8(_)
        | packet::Packet::UpdateBlockEntity_Data(_)
        | packet::Packet::UpdateLight_Arrays(_)
        | packet::Packet::SignEditorOpen(_)
        | packet::Packet::SignEditorOpen_i32(_)
        | packet::Packet::UpdateSign(_)
        | packet::Packet::UpdateSign_u16(_) => Some(ServerPacketFamily::WorldDimension),

        packet::Packet::ChunkData_AndLight(_)
        | packet::Packet::ChunkData_AndLight_NoTrustEdges(_)
        | packet::Packet::ChunkData_Biomes3D_Bitmasks(_)
        | packet::Packet::ChunkData_Biomes3D_VarInt(_)
        | packet::Packet::ChunkData_Biomes3D_bool(_)
        | packet::Packet::ChunkData(_)
        | packet::Packet::ChunkData_Biomes3D(_)
        | packet::Packet::ChunkData_HeightMap(_)
        | packet::Packet::ChunkData_NoEntities(_)
        | packet::Packet::ChunkData_NoEntities_u16(_)
        | packet::Packet::ChunkData_17(_)
        | packet::Packet::ChunkDataBulk(_)
        | packet::Packet::ChunkDataBulk_17(_)
        | packet::Packet::ChunkUnload(_)
        | packet::Packet::BlockChange_VarInt(_)
        | packet::Packet::BlockChange_u8(_)
        | packet::Packet::MultiBlockChange_Packed(_)
        | packet::Packet::MultiBlockChange_VarInt(_)
        | packet::Packet::MultiBlockChange_u16(_) => Some(ServerPacketFamily::Chunk),

        packet::Packet::WindowOpen_VarInt(_)
        | packet::Packet::WindowItems_StateCarry(_)
        | packet::Packet::WindowSetSlot_State(_)
        | packet::Packet::CollectItem(_)
        | packet::Packet::SetCurrentHotbarSlot(_) => Some(ServerPacketFamily::InventoryWindow),

        packet::Packet::DeathMessage_VarInt(_)
        | packet::Packet::PlayerRemove_UUIDs(_)
        | packet::Packet::PlayerInfo(_)
        | packet::Packet::PlayerInfo_BitSet(_)
        | packet::Packet::PlayerInfo_String(_)
        | packet::Packet::ServerMessage_NoPosition(_)
        | packet::Packet::ServerMessage_Position(_)
        | packet::Packet::ServerMessage_Sender(_) => Some(ServerPacketFamily::ChatPlayerList),

        packet::Packet::EntityDestroy(_)
        | packet::Packet::EntityDestroy_u8(_)
        | packet::Packet::SpawnObject_VarInt_HeadYaw(_)
        | packet::Packet::SpawnObject_VarInt(_)
        | packet::Packet::SpawnMob_NoMeta(_)
        | packet::Packet::SpawnPlayer_f64_NoMeta(_)
        | packet::Packet::SpawnPlayer_f64(_)
        | packet::Packet::SpawnPlayer_i32(_)
        | packet::Packet::SpawnPlayer_i32_HeldItem(_)
        | packet::Packet::SpawnPlayer_i32_HeldItem_String(_)
        | packet::Packet::EntityEquipment_Array(_)
        | packet::Packet::EntityVelocity(_)
        | packet::Packet::EntityVelocity_i32(_)
        | packet::Packet::EntityTeleport_f64(_)
        | packet::Packet::EntityTeleport_i32(_)
        | packet::Packet::EntityTeleport_i32_i32_NoGround(_)
        | packet::Packet::EntityMove_i16(_)
        | packet::Packet::EntityMove_i8(_)
        | packet::Packet::EntityMove_i8_i32_NoGround(_)
        | packet::Packet::EntityLook_VarInt(_)
        | packet::Packet::EntityLook_i32_NoGround(_)
        | packet::Packet::EntityLookAndMove_i16(_)
        | packet::Packet::EntityLookAndMove_i8(_)
        | packet::Packet::EntityLookAndMove_i8_i32_NoGround(_) => Some(ServerPacketFamily::Entity),

        _ => None,
    }
}

macro_rules! dispatch_packet {
    ($server:ident $packet:ident {
        $($packet_variant:ident => $handler:ident,)*
    }) => {
        match $packet {
        $(
            protocol::packet::Packet::$packet_variant(value) => $server.$handler(value),
        )*
            _ => {},
        }
    };
}

pub(crate) fn drain_read_queue(server: &mut Server) {
    if let Some(rx) = server.read_queue.take() {
        while let Ok(packet) = rx.try_recv() {
            match packet {
                Ok(packet) => {
                    let _family = packet_family(&packet);
                    dispatch_packet! {
                    server packet {
                        PluginMessageClientbound_i16 => on_plugin_message_clientbound_i16,
                        PluginMessageClientbound => on_plugin_message_clientbound_1,
                        JoinGame_WorldNames_IsHard_SimDist_LastDeath_PortalCooldown => on_game_join_worldnames_ishard_simdist_lastdeath_portal,
                        JoinGame_WorldNames_IsHard_SimDist => on_game_join_worldnames_ishard_simdist,
                        JoinGame_WorldNames_IsHard => on_game_join_worldnames_ishard,
                        JoinGame_WorldNames => on_game_join_worldnames,
                        JoinGame_HashedSeed_Respawn => on_game_join_hashedseed_respawn,
                        JoinGame_i32_ViewDistance => on_game_join_i32_viewdistance,
                        JoinGame_i32 => on_game_join_i32,
                        JoinGame_i8 => on_game_join_i8,
                        JoinGame_i8_NoDebug => on_game_join_i8_nodebug,
                        Respawn_Gamemode => on_respawn_gamemode,
                        Respawn_HashedSeed => on_respawn_hashedseed,
                        Respawn_WorldName => on_respawn_worldname,
                        Respawn_WorldNames_LastDeath_PortalCooldown => on_respawn_worldnames_lastdeath_portal,
                        Respawn_NBT => on_respawn_nbt,
                        KeepAliveClientbound_i64 => on_keep_alive_i64,
                        KeepAliveClientbound_VarInt => on_keep_alive_varint,
                        KeepAliveClientbound_i32 => on_keep_alive_i32,
                        ChunkData_AndLight => on_chunk_data_and_light,
                        ChunkData_AndLight_NoTrustEdges => on_chunk_data_and_light_no_trust_edges,
                        ChunkData_Biomes3D_Bitmasks => on_chunk_data_biomes3d_bitmasks,
                        ChunkData_Biomes3D_VarInt => on_chunk_data_biomes3d_varint,
                        ChunkData_Biomes3D_bool => on_chunk_data_biomes3d_bool,
                        ChunkData => on_chunk_data,
                        ChunkData_Biomes3D => on_chunk_data_biomes3d,
                        ChunkData_HeightMap => on_chunk_data_heightmap,
                        ChunkData_NoEntities => on_chunk_data_no_entities,
                        ChunkData_NoEntities_u16 => on_chunk_data_no_entities_u16,
                        ChunkData_17 => on_chunk_data_17,
                        ChunkDataBulk => on_chunk_data_bulk,
                        ChunkDataBulk_17 => on_chunk_data_bulk_17,
                        ChunkUnload => on_chunk_unload,
                        BlockChange_VarInt => on_block_change_varint,
                        BlockChange_u8 => on_block_change_u8,
                        MultiBlockChange_Packed => on_multi_block_change_packed,
                        MultiBlockChange_VarInt => on_multi_block_change_varint,
                        MultiBlockChange_u16 => on_multi_block_change_u16,
                        TeleportPlayer_WithDismount => on_teleport_player_withdismount,
                        TeleportPlayer_WithConfirm => on_teleport_player_withconfirm,
                        TeleportPlayer_NoConfirm => on_teleport_player_noconfirm,
                        TeleportPlayer_OnGround => on_teleport_player_onground,
                        TimeUpdate => on_time_update,
                        ChangeGameState => on_game_state_change,
                        UpdateHealth => on_update_health,
                        WindowOpen_VarInt => on_window_open_varint,
                        WindowItems_StateCarry => on_window_items_state_carry,
                        WindowSetSlot_State => on_window_set_slot_state,
                        CollectItem => on_collect_item,
                        SetCurrentHotbarSlot => on_set_current_hotbar_slot,
                        DeathMessage_VarInt => on_death_message_varint,
                        PlayerRemove_UUIDs => on_player_remove_uuids,
                        UpdateBlockEntity_VarInt => on_block_entity_update_varint,
                        UpdateBlockEntity_u8 => on_block_entity_update_u8,
                        UpdateBlockEntity_Data => on_block_entity_update_data,
                        UpdateLight_Arrays => on_update_light_arrays,
                        SignEditorOpen => on_sign_editor_open,
                        SignEditorOpen_i32 => on_sign_editor_open_i32,
                        UpdateSign => on_sign_update,
                        UpdateSign_u16 => on_sign_update_u16,
                        PlayerInfo => on_player_info,
                        PlayerInfo_BitSet => on_player_info_bit_set,
                        PlayerInfo_String => on_player_info_string,
                        ServerMessage_NoPosition => on_servermessage_noposition,
                        ServerMessage_Position => on_servermessage_position,
                        ServerMessage_Sender => on_servermessage_sender,
                        Disconnect => on_disconnect,
                        EntityDestroy => on_entity_destroy,
                        EntityDestroy_u8 => on_entity_destroy_u8,
                        SpawnObject_VarInt_HeadYaw => on_spawn_object_varint_head_yaw,
                        SpawnObject_VarInt => on_spawn_object_varint,
                        SpawnMob_NoMeta => on_spawn_mob_no_meta,
                        SpawnPlayer_f64_NoMeta => on_player_spawn_f64_nometa,
                        SpawnPlayer_f64 => on_player_spawn_f64,
                        SpawnPlayer_i32 => on_player_spawn_i32,
                        SpawnPlayer_i32_HeldItem => on_player_spawn_i32_helditem,
                        SpawnPlayer_i32_HeldItem_String => on_player_spawn_i32_helditem_string,
                        EntityEquipment_Array => on_entity_equipment_array,
                        EntityVelocity => on_entity_velocity,
                        EntityVelocity_i32 => on_entity_velocity_i32,
                        EntityTeleport_f64 => on_entity_teleport_f64,
                        EntityTeleport_i32 => on_entity_teleport_i32,
                        EntityTeleport_i32_i32_NoGround => on_entity_teleport_i32_i32_noground,
                        EntityMove_i16 => on_entity_move_i16,
                        EntityMove_i8 => on_entity_move_i8,
                        EntityMove_i8_i32_NoGround => on_entity_move_i8_i32_noground,
                        EntityLook_VarInt => on_entity_look_varint,
                        EntityLook_i32_NoGround => on_entity_look_i32_noground,
                        EntityLookAndMove_i16 => on_entity_look_and_move_i16,
                        EntityLookAndMove_i8 => on_entity_look_and_move_i8,
                        EntityLookAndMove_i8_i32_NoGround => on_entity_look_and_move_i8_i32_noground,
                    }
                    }
                }
                Err(err) => {
                    if std::env::var("MC_COMPAT_IGNORE_DECODE_ERRORS")
                        .map(|value| value != "0")
                        .unwrap_or(false)
                    {
                        warn!("MC-COMPAT-NONFATAL packet_parse_ignored");
                        continue;
                    }
                    panic!("Err: {:?}", err)
                }
            }

            if server.conn.read().unwrap().is_none() {
                break;
            }
        }

        if server.conn.read().unwrap().is_some() {
            server.read_queue = Some(rx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format;

    const TEST_KEEP_ALIVE_ID: i64 = 42;
    const TEST_CHUNK_X: i32 = 3;
    const TEST_CHUNK_Z: i32 = -2;
    const TEST_ENTITY_ID: i32 = 7;
    const TEST_WINDOW_ID: i32 = 5;
    const TEST_WINDOW_TYPE: i32 = 1;
    const TEST_WORLD_AGE: i64 = 1200;
    const TEST_TIME_OF_DAY: i64 = 6000;
    const TEST_STATUS_PING: i64 = 99;

    fn text_component(raw: &str) -> format::Component {
        format::Component::from_string(raw)
    }

    #[test]
    fn classifies_supported_packet_families() {
        let plugin = packet::Packet::PluginMessageClientbound(
            packet::play::clientbound::PluginMessageClientbound {
                channel: "minecraft:brand".to_string(),
                data: Vec::new(),
            },
        );
        let session = packet::Packet::KeepAliveClientbound_i64(
            packet::play::clientbound::KeepAliveClientbound_i64 {
                id: TEST_KEEP_ALIVE_ID,
            },
        );
        let world = packet::Packet::TimeUpdate(packet::play::clientbound::TimeUpdate {
            world_age: TEST_WORLD_AGE,
            time_of_day: TEST_TIME_OF_DAY,
        });
        let chunk = packet::Packet::ChunkUnload(packet::play::clientbound::ChunkUnload {
            x: TEST_CHUNK_X,
            z: TEST_CHUNK_Z,
        });
        let entity =
            packet::Packet::EntityVelocity_i32(packet::play::clientbound::EntityVelocity_i32 {
                entity_id: TEST_ENTITY_ID,
                velocity_x: 0,
                velocity_y: 0,
                velocity_z: 0,
            });
        let inventory =
            packet::Packet::WindowOpen_VarInt(packet::play::clientbound::WindowOpen_VarInt {
                id: protocol::VarInt(TEST_WINDOW_ID),
                ty: protocol::VarInt(TEST_WINDOW_TYPE),
                title: text_component("inventory"),
            });
        let chat = packet::Packet::ServerMessage_NoPosition(
            packet::play::clientbound::ServerMessage_NoPosition {
                message: text_component("hello"),
            },
        );

        assert_eq!(
            packet_family(&plugin),
            Some(ServerPacketFamily::PluginMessage)
        );
        assert_eq!(
            packet_family(&session),
            Some(ServerPacketFamily::SessionLifecycle)
        );
        assert_eq!(
            packet_family(&world),
            Some(ServerPacketFamily::WorldDimension)
        );
        assert_eq!(packet_family(&chunk), Some(ServerPacketFamily::Chunk));
        assert_eq!(packet_family(&entity), Some(ServerPacketFamily::Entity));
        assert_eq!(
            packet_family(&inventory),
            Some(ServerPacketFamily::InventoryWindow)
        );
        assert_eq!(
            packet_family(&chat),
            Some(ServerPacketFamily::ChatPlayerList)
        );
    }

    #[test]
    fn rejects_packets_not_owned_by_play_session_dispatch() {
        let status = packet::Packet::StatusPong(packet::status::clientbound::StatusPong {
            ping: TEST_STATUS_PING,
        });

        assert_eq!(packet_family(&status), None);
    }
}
