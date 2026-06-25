#![doc = include_str!("../README.md")]

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
#[path = "interaction/broadcast.rs"]
mod interaction_broadcast;
pub use interaction_broadcast::EquipmentInteractionBroadcast;
#[path = "inventory/sync.rs"]
mod inventory_sync;
pub use inventory_sync::EquipmentInventorySync;
use valence_server::protocol::WritePacket;
use valence_server::Layer;

type Client = valence_server::client::Client;
type EntityId = valence_server::entity::EntityId;
type LayerEntity = valence_server::EntityLayer;
type LayerId = valence_server::entity::EntityLayerId;
type Living = valence_server::entity::living::LivingEntity;
type LoadEvent = valence_server::client::LoadEntityForClientEvent;
type Packet = valence_server::protocol::packets::play::EntityEquipmentUpdateS2c;
type PacketEntry = valence_server::protocol::packets::play::EquipmentEntry;
type Position = valence_server::entity::Position;
type Stack = valence_server::ItemStack;

pub struct EquipmentPlugin;

/// The [`SystemSet`] in [`PreUpdate`] where missing equipment components are
/// attached to newly initialized living entities.
#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EquipmentInitSet;

/// The [`SystemSet`] in [`PreUpdate`] where equipment reads client input and
/// interaction events.
#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EquipmentInputSet;

/// The [`SystemSet`] in [`PreUpdate`] where player inventory state is
/// synchronized with visible equipment.
#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EquipmentSyncSet;

/// The [`SystemSet`] in [`PostUpdate`] where visible equipment changes are
/// broadcast before packet flushing.
#[derive(SystemSet, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct EquipmentBroadcastSet;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                on_entity_init.in_set(EquipmentInitSet),
                interaction_broadcast::start.in_set(EquipmentInputSet),
                interaction_broadcast::stop.in_set(EquipmentInputSet),
                inventory_sync::on_attach.in_set(EquipmentSyncSet),
                inventory_sync::run.in_set(EquipmentSyncSet),
                inventory_sync::held_item_from_client.in_set(EquipmentInputSet),
            ),
        )
        .add_systems(
            PostUpdate,
            (
                update_equipment.in_set(EquipmentBroadcastSet),
                on_entity_load.in_set(EquipmentBroadcastSet),
            )
                .before(valence_server::client::FlushPacketsSet),
        )
        .add_event::<EquipmentChangeEvent>();
    }
}

/// Contains the visible equipment of a [`LivingEntity`], such as armor and held
/// items. By default this is not synced with a player's
/// [`valence_inventory::Inventory`], so the armor the player has equipped in
/// their inventory, will not be visible by other players. You would have to
/// change the equipment in this component here or attach the
/// [`EquipmentInventorySync`] component to the player entity to sync the
/// equipment with the inventory.
#[derive(Debug, Default, Clone, Component)]
pub struct Equipment {
    equipment: [Stack; Self::SLOT_COUNT],
    /// Contains a set bit for each modified slot in `slots`.
    #[doc(hidden)]
    pub(crate) changed: u8,
}

impl Equipment {
    pub const SLOT_COUNT: usize = 6;
    pub const SLOT_COUNT_U8: u8 = 6;

    pub const MAIN_HAND_IDX: u8 = 0;
    pub const OFF_HAND_IDX: u8 = 1;
    pub const FEET_IDX: u8 = 2;
    pub const LEGS_IDX: u8 = 3;
    pub const CHEST_IDX: u8 = 4;
    pub const HEAD_IDX: u8 = 5;

    // API: preserve the six-slot constructor that mirrors Minecraft equipment
    // slots.
    #[allow(unknown_lints)]
    #[allow(too_many_parameters)]
    pub fn new(
        main_hand: Stack,
        off_hand: Stack,
        boots: Stack,
        leggings: Stack,
        chestplate: Stack,
        helmet: Stack,
    ) -> Self {
        Self {
            equipment: [main_hand, off_hand, boots, leggings, chestplate, helmet],
            changed: 0,
        }
    }

    pub fn slot(&self, idx: u8) -> &Stack {
        &self.equipment[usize::from(idx)]
    }

    pub fn set_slot(&mut self, idx: u8, item: Stack) {
        assert!(
            idx < Self::SLOT_COUNT_U8,
            "slot index of {idx} out of bounds"
        );
        let slot_idx = usize::from(idx);
        if self.equipment[slot_idx] != item {
            self.equipment[slot_idx] = item;
            self.changed |= 1 << idx;
        }
    }

    pub fn main_hand(&self) -> &Stack {
        self.slot(Self::MAIN_HAND_IDX)
    }

    pub fn off_hand(&self) -> &Stack {
        self.slot(Self::OFF_HAND_IDX)
    }

    pub fn feet(&self) -> &Stack {
        self.slot(Self::FEET_IDX)
    }

    pub fn legs(&self) -> &Stack {
        self.slot(Self::LEGS_IDX)
    }

    pub fn chest(&self) -> &Stack {
        self.slot(Self::CHEST_IDX)
    }

    pub fn head(&self) -> &Stack {
        self.slot(Self::HEAD_IDX)
    }

    pub fn set_main_hand(&mut self, item: Stack) {
        self.set_slot(Self::MAIN_HAND_IDX, item);
    }

    pub fn set_off_hand(&mut self, item: Stack) {
        self.set_slot(Self::OFF_HAND_IDX, item);
    }

    pub fn set_feet(&mut self, item: Stack) {
        self.set_slot(Self::FEET_IDX, item);
    }

    pub fn set_legs(&mut self, item: Stack) {
        self.set_slot(Self::LEGS_IDX, item);
    }

    pub fn set_chest(&mut self, item: Stack) {
        self.set_slot(Self::CHEST_IDX, item);
    }

    pub fn set_head(&mut self, item: Stack) {
        self.set_slot(Self::HEAD_IDX, item);
    }

    pub fn clear(&mut self) {
        for slot in 0..Self::SLOT_COUNT_U8 {
            self.set_slot(slot, Stack::EMPTY);
        }
    }

    pub fn is_default(&self) -> bool {
        self.equipment.iter().all(|item| item.is_empty())
    }
}

#[derive(Debug, Clone)]
pub struct EquipmentSlotChange {
    idx: u8,
    stack: Stack,
}

#[derive(Debug, Clone, Event)]
pub struct EquipmentChangeEvent {
    pub client: Entity,
    pub changed: Vec<EquipmentSlotChange>,
}

fn update_equipment(
    mut clients: Query<
        (Entity, &EntityId, &LayerId, &Position, &mut Equipment),
        Changed<Equipment>,
    >,
    mut event_writer: EventWriter<EquipmentChangeEvent>,
    mut entity_layer: Query<&mut LayerEntity>,
) {
    for (entity, entity_id, entity_layer_id, position, mut equipment) in &mut clients {
        let Ok(mut entity_layer) = entity_layer.get_mut(entity_layer_id.0) else {
            continue;
        };

        if equipment.changed != 0 {
            let mut slots_changed: Vec<EquipmentSlotChange> =
                Vec::with_capacity(Equipment::SLOT_COUNT);

            for slot in 0..Equipment::SLOT_COUNT {
                if equipment.changed & (1 << slot) != 0 {
                    let Some(slot) = equipment_slot_idx(slot) else {
                        continue;
                    };
                    slots_changed.push(EquipmentSlotChange {
                        idx: slot,
                        stack: equipment.equipment[usize::from(slot)].clone(),
                    });
                }
            }

            entity_layer
                .view_except_writer(position.0, entity)
                .write_packet(&Packet {
                    entity_id: entity_id.get().into(),
                    equipment: slots_changed
                        .iter()
                        .filter_map(packet_entry_for_change)
                        .collect(),
                });

            event_writer.send(EquipmentChangeEvent {
                client: entity,
                changed: slots_changed,
            });

            equipment.changed = 0;
        }
    }
}

/// Gets called when the player loads an entity, for example
/// when the player gets in range of the entity.
fn on_entity_load(
    mut clients: Query<&mut Client>,
    entities: Query<(&EntityId, &Equipment)>,
    mut events: EventReader<LoadEvent>,
) {
    for event in events.read() {
        let Ok(mut client) = clients.get_mut(event.client) else {
            continue;
        };

        let Ok((entity_id, equipment)) = entities.get(event.entity_loaded) else {
            continue;
        };

        if equipment.is_default() {
            continue;
        }

        let mut entries: Vec<PacketEntry> = Vec::with_capacity(Equipment::SLOT_COUNT);
        for slot in 0..Equipment::SLOT_COUNT_U8 {
            if let Some(packet_slot) = packet_slot_idx(slot) {
                entries.push(PacketEntry {
                    slot: packet_slot,
                    item: equipment.equipment[usize::from(slot)].clone(),
                });
            }
        }

        client.write_packet(&Packet {
            entity_id: entity_id.get().into(),
            equipment: entries,
        });
    }
}

/// Add a default equipment component to all living entities when they are
/// initialized.
fn on_entity_init(
    mut commands: Commands,
    mut entities: Query<Entity, (Added<Living>, Without<Equipment>)>,
) {
    for entity in &mut entities {
        commands.entity(entity).insert(Equipment::default());
    }
}

fn equipment_slot_idx(slot: usize) -> Option<u8> {
    u8::try_from(slot).ok()
}

fn packet_slot_idx(slot: u8) -> Option<i8> {
    i8::try_from(slot).ok()
}

fn packet_entry_for_change(change: &EquipmentSlotChange) -> Option<PacketEntry> {
    Some(PacketEntry {
        slot: packet_slot_idx(change.idx)?,
        item: change.stack.clone(),
    })
}
