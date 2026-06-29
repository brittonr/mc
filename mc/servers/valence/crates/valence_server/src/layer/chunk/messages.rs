use bevy_ecs::prelude::Entity;
use valence_protocol::{BlockPos, ChunkPos};

use crate::layer::bvh::GetChunkPos;
use crate::layer::message::Messages;

use super::targeting::{block_pos_within_radius_squared, client_is_not_excluded};

pub(crate) type ChunkLayerMessages = Messages<GlobalMsg, LocalMsg>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum GlobalMsg {
    /// Send packet data to all clients viewing the layer.
    Packet,
    /// Send packet data to all clients viewing the layer, except the client
    /// identified by `except`.
    PacketExcept { except: Entity },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum LocalMsg {
    /// Send packet data to all clients viewing the layer in view of `pos`.
    PacketAt {
        pos: ChunkPos,
    },
    PacketAtExcept {
        pos: ChunkPos,
        except: Entity,
    },
    RadiusAt {
        center: BlockPos,
        radius_squared: u32,
    },
    RadiusAtExcept {
        center: BlockPos,
        radius_squared: u32,
        except: Entity,
    },
    /// Instruct clients to load or unload the chunk at `pos`. Loading and
    /// unloading are combined into a single message so that load/unload order
    /// is not lost when messages are sorted.
    ///
    /// Message content is a single byte indicating load (1) or unload (0).
    ChangeChunkState {
        pos: ChunkPos,
    },
    /// Message content is the data for a single biome in the "change biomes"
    /// packet.
    ChangeBiome {
        pos: ChunkPos,
    },
}

pub(crate) fn global_message_targets_client(msg: GlobalMsg, client: Entity) -> bool {
    match msg {
        GlobalMsg::Packet => true,
        GlobalMsg::PacketExcept { except } => client_is_not_excluded(client, except),
    }
}

pub(crate) fn local_message_targets_client(
    msg: LocalMsg,
    client: Entity,
    client_block_pos: BlockPos,
) -> bool {
    match msg {
        LocalMsg::PacketAt { .. } => true,
        LocalMsg::PacketAtExcept { except, .. } => client_is_not_excluded(client, except),
        LocalMsg::RadiusAt {
            center,
            radius_squared,
        } => block_pos_within_radius_squared(client_block_pos, center, radius_squared),
        LocalMsg::RadiusAtExcept {
            center,
            radius_squared,
            except,
        } => {
            client_is_not_excluded(client, except)
                && block_pos_within_radius_squared(client_block_pos, center, radius_squared)
        }
        LocalMsg::ChangeChunkState { .. } | LocalMsg::ChangeBiome { .. } => true,
    }
}

fn local_message_chunk_pos(msg: LocalMsg) -> ChunkPos {
    match msg {
        LocalMsg::PacketAt { pos } => pos,
        LocalMsg::PacketAtExcept { pos, .. } => pos,
        LocalMsg::RadiusAt { center, .. } => center.into(),
        LocalMsg::RadiusAtExcept { center, .. } => center.into(),
        LocalMsg::ChangeBiome { pos } => pos,
        LocalMsg::ChangeChunkState { pos } => pos,
    }
}

impl GetChunkPos for LocalMsg {
    fn chunk_pos(&self) -> ChunkPos {
        local_message_chunk_pos(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INCLUDED_ENTITY_INDEX: u32 = 1;
    const EXCLUDED_ENTITY_INDEX: u32 = 2;
    const TARGET_CHUNK_X: i32 = 3;
    const TARGET_CHUNK_Z: i32 = -4;
    const TARGET_BLOCK_X: i32 = 48;
    const TARGET_BLOCK_Y: i32 = 64;
    const TARGET_BLOCK_Z: i32 = -64;
    const INSIDE_RADIUS_SQUARED: u32 = 1;
    const OUTSIDE_RADIUS_SQUARED: u32 = 0;

    fn included_entity() -> Entity {
        Entity::from_raw(INCLUDED_ENTITY_INDEX)
    }

    fn excluded_entity() -> Entity {
        Entity::from_raw(EXCLUDED_ENTITY_INDEX)
    }

    fn target_chunk() -> ChunkPos {
        ChunkPos::new(TARGET_CHUNK_X, TARGET_CHUNK_Z)
    }

    fn target_block() -> BlockPos {
        BlockPos::new(TARGET_BLOCK_X, TARGET_BLOCK_Y, TARGET_BLOCK_Z)
    }

    #[test]
    fn global_messages_respect_exception_filters() {
        assert!(global_message_targets_client(
            GlobalMsg::Packet,
            included_entity()
        ));
        assert!(global_message_targets_client(
            GlobalMsg::PacketExcept {
                except: excluded_entity(),
            },
            included_entity(),
        ));
        assert!(!global_message_targets_client(
            GlobalMsg::PacketExcept {
                except: excluded_entity(),
            },
            excluded_entity(),
        ));
    }

    #[test]
    fn local_packet_and_radius_messages_select_target_clients() {
        assert!(local_message_targets_client(
            LocalMsg::PacketAt {
                pos: target_chunk(),
            },
            included_entity(),
            target_block(),
        ));
        assert!(local_message_targets_client(
            LocalMsg::PacketAtExcept {
                pos: target_chunk(),
                except: excluded_entity(),
            },
            included_entity(),
            target_block(),
        ));
        assert!(local_message_targets_client(
            LocalMsg::RadiusAt {
                center: target_block(),
                radius_squared: INSIDE_RADIUS_SQUARED,
            },
            included_entity(),
            target_block(),
        ));
    }

    #[test]
    fn local_messages_fail_closed_for_excluded_or_out_of_radius_clients() {
        assert!(!local_message_targets_client(
            LocalMsg::PacketAtExcept {
                pos: target_chunk(),
                except: excluded_entity(),
            },
            excluded_entity(),
            target_block(),
        ));
        assert!(!local_message_targets_client(
            LocalMsg::RadiusAt {
                center: target_block(),
                radius_squared: OUTSIDE_RADIUS_SQUARED,
            },
            included_entity(),
            BlockPos::new(TARGET_BLOCK_X, TARGET_BLOCK_Y, TARGET_BLOCK_Z + 1),
        ));
        assert!(!local_message_targets_client(
            LocalMsg::RadiusAtExcept {
                center: target_block(),
                radius_squared: INSIDE_RADIUS_SQUARED,
                except: excluded_entity(),
            },
            excluded_entity(),
            target_block(),
        ));
    }

    #[test]
    fn local_messages_report_their_owner_chunk() {
        assert_eq!(
            local_message_chunk_pos(LocalMsg::PacketAt {
                pos: target_chunk(),
            }),
            target_chunk()
        );
        assert_eq!(
            local_message_chunk_pos(LocalMsg::ChangeBiome {
                pos: target_chunk(),
            }),
            target_chunk()
        );
        assert_eq!(
            local_message_chunk_pos(LocalMsg::RadiusAt {
                center: target_block(),
                radius_squared: INSIDE_RADIUS_SQUARED,
            }),
            ChunkPos::from(target_block())
        );
    }
}
