use super::*;

/// If the player tries to place a block while their inventory is readonly
/// it will be desynced, therefore we set the slot as changed.
pub(super) fn resync_readonly_inventory_after_block_interaction(
    mut clients: Query<(&mut Inventory, &HeldItem)>,
    mut events: EventReader<InteractBlockEvent>,
) {
    for event in events.read() {
        let Ok((mut inventory, held_item)) = clients.get_mut(event.client) else {
            continue;
        };
        if !inventory.readonly {
            continue;
        }

        let slot = interacted_slot(event.hand, held_item);
        if inventory.slot(slot).is_empty() {
            continue;
        }
        mark_inventory_changed(&mut inventory, slot);
    }
}

fn interacted_slot(hand: valence_server::Hand, held_item: &HeldItem) -> u16 {
    if hand == valence_server::Hand::Main {
        return held_item.slot();
    }
    PlayerInventory::SLOT_OFFHAND
}

fn mark_inventory_changed(inventory: &mut Inventory, slot: u16) {
    let Some(mask) = 1_u64.checked_shl(u32::from(slot)) else {
        unreachable!();
    };
    inventory.changed |= mask;
}
