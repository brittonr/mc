#![doc = include_str!("../README.md")]

pub mod abilities;
pub mod action;
pub mod brand;
mod chunk_view;
pub mod client;
#[path = "client/command.rs"]
pub mod client_command;
#[path = "client/settings.rs"]
pub mod client_settings;
#[path = "custom/payload.rs"]
pub mod custom_payload;
#[path = "event/loop.rs"]
pub mod event_loop;
#[path = "hand/swing.rs"]
pub mod hand_swing;
#[path = "interact/block.rs"]
pub mod interact_block;
#[path = "interact/entity.rs"]
pub mod interact_entity;
#[path = "interact/item.rs"]
pub mod interact_item;
pub mod keepalive;
pub mod layer;
pub mod message;
pub mod movement;
#[path = "op/level.rs"]
pub mod op_level;
#[path = "resource/pack.rs"]
pub mod resource_pack;
pub mod spawn;
pub mod status;
#[path = "status/effect.rs"]
pub mod status_effect;
pub mod teleport;
pub mod title;

pub use bevy_app as app;
pub use bevy_ecs as ecs;
pub use chunk_view::ChunkView;
pub use event_loop::{EventLoopPostUpdate, EventLoopPreUpdate, EventLoopUpdate};
pub use layer::{ChunkLayer, EntityLayer, Layer, LayerBundle};
pub use rand;
pub use valence_entity as entity;
pub use valence_nbt as nbt;
pub use valence_protocol as protocol;
pub use valence_protocol::{
    block, ident, item, math, text, uuid, BiomePos, BlockPos, BlockState, ChunkPos,
    CompressionThreshold, Difficulty, Direction, GameMode, Hand, Ident, ItemKind, ItemStack, Text,
    MINECRAFT_VERSION, PROTOCOL_VERSION,
};
pub use valence_registry as registry;
pub use valence_server_common::*;
