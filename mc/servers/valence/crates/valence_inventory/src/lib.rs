#![doc = include_str!("../README.md")]

use std::iter::FusedIterator;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use player_inventory::PlayerInventory;

type Client = valence_server::client::Client;
type PacketEvent = valence_server::event_loop::PacketEvent;
pub use valence_server::interact_block::InteractBlockEvent;
pub use valence_server::protocol::packets::play::click_slot_c2s::{ClickMode, SlotChange};
pub use valence_server::protocol::packets::play::open_screen_s2c::WindowType;
pub use valence_server::protocol::packets::play::player_action_c2s::PlayerAction;
pub use valence_server::protocol::packets::play::{
    ClickSlotC2s, CloseHandledScreenC2s, CloseScreenS2c, CreativeInventoryActionC2s, InventoryS2c,
    OpenScreenS2c, PlayerActionC2s, ScreenHandlerSlotUpdateS2c, UpdateSelectedSlotC2s,
    UpdateSelectedSlotS2c,
};
use valence_server::protocol::WritePacket;
use valence_server::text::IntoText;
pub use valence_server::{ItemKind, ItemStack, Text};

#[path = "model/catalog.rs"]
mod catalog;
mod click;
#[path = "model/components.rs"]
mod components;
#[path = "systems/actions.rs"]
mod control;
pub mod gui;
#[path = "systems/hotbar.rs"]
mod hotbar;
#[path = "systems/mode.rs"]
mod mode;
#[path = "systems/place.rs"]
mod place;
#[path = "slots.rs"]
pub mod player_inventory;
#[path = "model/search.rs"]
mod search;
#[path = "model/storage.rs"]
mod storage;
#[path = "model/view.rs"]
mod view;
#[path = "systems/viewer.rs"]
mod viewer;

mod validate;

pub use catalog::{InventoryKind, InventorySettings};
pub use components::{ClientInventoryState, CursorItem, HeldItem, OpenInventory};
pub use hotbar::UpdateSelectedSlotEvent;
pub use mode::CreativeInventoryActionEvent;
pub use storage::Inventory;
pub use view::{InventoryWindow, InventoryWindowMut};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(
            PreUpdate,
            init_new_client_inventories.after(valence_server::client::SpawnClientsSet),
        )
        .add_systems(
            PostUpdate,
            (
                update_client_on_close_inventory.before(viewer::update_open_inventories),
                hotbar::update_player_selected_slot,
                viewer::update_open_inventories,
                update_player_inventories,
                update_cursor_item,
            )
                .before(valence_server::client::FlushPacketsSet),
        )
        .add_systems(
            valence_server::event_loop::EventLoopPreUpdate,
            (
                hotbar::handle_update_selected_slot,
                click::handle_packets,
                mode::handle_creative_inventory_action,
                handle_close_handled_screen,
                control::handle_player_actions,
                place::resync_readonly_inventory_after_block_interaction,
            ),
        )
        .init_resource::<InventorySettings>()
        .add_event::<ClickSlotEvent>()
        .add_event::<DropItemStackEvent>()
        .add_event::<CreativeInventoryActionEvent>()
        .add_event::<UpdateSelectedSlotEvent>();
    }
}

/// Attach the necessary inventory components to new clients.
fn init_new_client_inventories(clients: Query<Entity, Added<Client>>, mut commands: Commands) {
    for entity in &clients {
        commands.entity(entity).insert((
            Inventory::new(InventoryKind::Player),
            CursorItem(ItemStack::EMPTY),
            ClientInventoryState {
                window_id: 0,
                state_id: std::num::Wrapping(0),
                slots_changed: 0,
                client_updated_cursor_item: None,
            },
            HeldItem {
                // First slot of the hotbar.
                held_item_slot: 36,
            },
        ));
    }
}

/// Send updates for each client's player inventory.
fn update_player_inventories(
    mut query: Query<
        (
            &mut Inventory,
            &mut Client,
            &mut ClientInventoryState,
            &CursorItem,
        ),
        Without<OpenInventory>,
    >,
) {
    for (mut inventory, mut client, mut inv_state, cursor_item) in &mut query {
        if inventory.kind != InventoryKind::Player {
            tracing::warn!("Inventory on client entity is not a player inventory");
        }

        if inventory.changed == u64::MAX {
            // Update the whole inventory.

            inv_state.state_id += 1;

            client.write_packet(&InventoryS2c {
                window_id: 0,
                state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
                slots: std::borrow::Cow::Borrowed(inventory.slot_slice()),
                carried_item: std::borrow::Cow::Borrowed(&cursor_item.0),
            });

            inventory.changed = 0;
            inv_state.slots_changed = 0;
        } else if inventory.changed != 0 {
            // Send the modified slots.

            // The slots that were NOT modified by this client, and they need to be sent
            let changed_filtered = inventory.changed & !inv_state.slots_changed;

            if changed_filtered == 0 {
                inventory.changed = 0;
                inv_state.slots_changed = 0;
                continue;
            }

            inv_state.state_id += 1;

            for (slot_idx, slot) in inventory.slots.iter().enumerate() {
                if ((changed_filtered >> slot_idx) & 1) != 1 {
                    continue;
                }
                let Ok(slot_idx) = i16::try_from(slot_idx) else {
                    continue;
                };
                client.write_packet(&ScreenHandlerSlotUpdateS2c {
                    window_id: 0,
                    state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
                    slot_idx,
                    slot_data: std::borrow::Cow::Borrowed(slot),
                });
            }

            inventory.changed = 0;
            inv_state.slots_changed = 0;
        }
    }
}

fn update_cursor_item(
    mut clients: Query<(&mut Client, &mut ClientInventoryState, &CursorItem), Changed<CursorItem>>,
) {
    for (mut client, inv_state, cursor_item) in &mut clients {
        // The cursor item was not the item the user themselves interacted with
        if inv_state.client_updated_cursor_item.as_ref() != Some(&cursor_item.0) {
            // Contrary to what you might think, we actually don't want to increment the
            // state ID here because the client doesn't actually acknowledge the
            // state_id change for this packet specifically. See #304.
            client.write_packet(&ScreenHandlerSlotUpdateS2c {
                window_id: -1,
                state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
                slot_idx: -1,
                slot_data: std::borrow::Cow::Borrowed(&cursor_item.0),
            });
        }

        inv_state
            .map_unchanged(|f| &mut f.client_updated_cursor_item)
            .set_if_neq(None);
    }
}

/// Handles clients telling the server that they are closing an inventory.
fn handle_close_handled_screen(mut packets: EventReader<PacketEvent>, mut commands: Commands) {
    for packet in packets.read() {
        if packet.decode::<CloseHandledScreenC2s>().is_some() {
            if let Some(mut entity) = commands.get_entity(packet.client) {
                entity.remove::<OpenInventory>();
            }
        }
    }
}

/// Detects when a client's `OpenInventory` component is removed, which
/// indicates that the client is no longer viewing an inventory.
fn update_client_on_close_inventory(
    mut removals: RemovedComponents<OpenInventory>,
    mut clients: Query<(&mut Client, &ClientInventoryState)>,
) {
    for entity in &mut removals.read() {
        if let Ok((mut client, inv_state)) = clients.get_mut(entity) {
            client.write_packet(&CloseScreenS2c {
                window_id: inv_state.window_id,
            })
        }
    }
}

// TODO: make this event user friendly.
#[derive(Event, Clone, Debug)]
pub struct ClickSlotEvent {
    pub client: Entity,
    pub window_id: u8,
    pub state_id: i32,
    pub slot_id: i16,
    pub button: i8,
    pub mode: ClickMode,
    pub slot_changes: Vec<SlotChange>,
    pub carried_item: ItemStack,
}

#[derive(Event, Clone, Debug)]
pub struct DropItemStackEvent {
    pub client: Entity,
    pub from_slot: Option<u16>,
    pub stack: ItemStack,
}

/// Convert a slot that is outside a target inventory's range to a slot that is
/// inside the player's inventory.
fn slot_idx_in_inventory(inventory: &Inventory, slot_idx: i16) -> bool {
    let Some(slot_idx) = slot_idx_to_u16(slot_idx) else {
        return false;
    };
    slot_idx < inventory.slot_count()
}

fn slot_idx_to_u16(slot_idx: i16) -> Option<u16> {
    u16::try_from(slot_idx).ok()
}

pub fn convert_to_player_slot_id(target_kind: InventoryKind, slot_id: u16) -> u16 {
    let target_slot_count = target_kind.slot_count();
    let Some(relative_slot) = slot_id.checked_sub(target_slot_count) else {
        return PlayerInventory::SLOTS_MAIN_START;
    };
    let Some(slot_id) = PlayerInventory::SLOTS_MAIN_START.checked_add(relative_slot) else {
        return PlayerInventory::SLOT_OFFHAND;
    };
    slot_id
}
