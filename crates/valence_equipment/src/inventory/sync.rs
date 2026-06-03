use super::*;

/// This component will sync a player's [`Equipment`], which is visible to other
/// players, with the player [`valence_inventory::Inventory`].
///
/// API compatibility: keep the established `EquipmentInventorySync` component
/// name for callers that attach equipment sync behavior.
// API compatibility: keep the established public equipment inventory sync component name.
#[allow(unknown_lints)]
#[allow(path_segment_repetition)]
#[derive(Debug, Default, Clone, Component)]
pub struct EquipmentInventorySync;

/// Syncs the player [`Equipment`] with the [`valence_inventory::Inventory`].
/// If a change in the player's inventory and in the equipment occurs in the
/// same tick, the equipment change has priority.
/// Note: This system only handles direct changes to the held item (not actual
/// changes from the client) see [`held_item_from_client`].
pub(crate) fn run(
    mut clients: Query<
        (
            &mut Equipment,
            &mut valence_inventory::Inventory,
            &mut valence_inventory::HeldItem,
        ),
        (
            Or<(
                Changed<Equipment>,
                Changed<valence_inventory::Inventory>,
                Changed<valence_inventory::HeldItem>,
            )>,
            With<EquipmentInventorySync>,
            With<valence_server::entity::player::PlayerEntity>,
        ),
    >,
) {
    for (mut equipment, mut inventory, held_item) in &mut clients {
        let is_inventory_changed = inventory.is_changed();
        main_hand(&mut equipment, &mut inventory, &held_item);
        armor_slots(&mut equipment, &mut inventory, is_inventory_changed);
    }
}

/// Handles the case where the client changes the slot (the bevy change is
/// suppressed for this).
pub(crate) fn held_item_from_client(
    mut clients: Query<
        (
            &valence_inventory::HeldItem,
            &valence_inventory::Inventory,
            &mut Equipment,
        ),
        With<EquipmentInventorySync>,
    >,
    mut events: EventReader<valence_inventory::UpdateSelectedSlotEvent>,
) {
    for event in events.read() {
        let Ok((held_item, inventory, mut equipment)) = clients.get_mut(event.client) else {
            continue;
        };

        let item = inventory.slot(held_item.slot()).clone();
        equipment.set_main_hand(item);
    }
}

pub(crate) fn on_attach(
    entities: Query<
        Option<&valence_server::entity::player::PlayerEntity>,
        (
            Added<EquipmentInventorySync>,
            With<valence_inventory::Inventory>,
        ),
    >,
) {
    for entity in &entities {
        if entity.is_none() {
            tracing::warn!(
                "EquipmentInventorySync attached to non-player entity, this will have no effect"
            );
        }
    }
}

fn main_hand(
    equipment: &mut Equipment,
    inventory: &mut valence_inventory::Inventory,
    held_item: &valence_inventory::HeldItem,
) {
    // Equipment change has priority over held item changes.
    if equipment.changed & (1 << Equipment::MAIN_HAND_IDX) != 0 {
        let item = equipment.main_hand().clone();
        inventory.set_slot(held_item.slot(), item);
    } else {
        let item = inventory.slot(held_item.slot()).clone();
        equipment.set_main_hand(item);
    }
}

fn armor_slots(
    equipment: &mut Equipment,
    inventory: &mut valence_inventory::Inventory,
    is_inventory_changed: bool,
) {
    const ARMOR_SLOT_COUNT: usize = 5;

    let slots = [
        (
            Equipment::OFF_HAND_IDX,
            valence_inventory::player_inventory::PlayerInventory::SLOT_OFFHAND,
        ),
        (
            Equipment::HEAD_IDX,
            valence_inventory::player_inventory::PlayerInventory::SLOT_HEAD,
        ),
        (
            Equipment::CHEST_IDX,
            valence_inventory::player_inventory::PlayerInventory::SLOT_CHEST,
        ),
        (
            Equipment::LEGS_IDX,
            valence_inventory::player_inventory::PlayerInventory::SLOT_LEGS,
        ),
        (
            Equipment::FEET_IDX,
            valence_inventory::player_inventory::PlayerInventory::SLOT_FEET,
        ),
    ];

    debug_assert_eq!(slots.len(), ARMOR_SLOT_COUNT);
    debug_assert!(Equipment::SLOT_COUNT >= slots.len());

    for (equipment_slot, inventory_slot) in slots {
        if equipment.changed & (1 << equipment_slot) != 0 {
            let item = equipment.slot(equipment_slot).clone();
            inventory.set_slot(inventory_slot, item);
        } else if is_inventory_changed {
            let item = inventory.slot(inventory_slot).clone();
            equipment.set_slot(equipment_slot, item);
        }
    }
}
