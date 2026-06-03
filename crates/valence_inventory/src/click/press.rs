use super::*;

struct DroppedStack {
    slot_id: u16,
    stack: ItemStack,
}

pub(super) fn handle_key(
    pkt: &ClickSlotC2s,
    input: (
        Entity,
        &mut Client,
        &mut Inventory,
        &mut ClientInventoryState,
        Option<bevy_ecs::change_detection::Mut<OpenInventory>>,
        &mut Query<&mut Inventory, Without<Client>>,
        &mut EventWriter<DropItemStackEvent>,
        &CursorItem,
    ),
) {
    debug_assert_eq!(pkt.mode, ClickMode::DropKey);
    debug_assert!(pkt.slot_idx >= OUTSIDE_WINDOW_SLOT);

    let (
        client_entity,
        client,
        client_inv,
        inv_state,
        open_inventory,
        inventories,
        drop_item_stack_events,
        cursor_item,
    ) = input;
    let dropped = dropped_from_key(
        pkt,
        (
            client,
            client_inv,
            inv_state,
            open_inventory,
            inventories,
            cursor_item,
        ),
    );
    let Some(dropped) = dropped else {
        return;
    };
    drop_item_stack_events.send(DropItemStackEvent {
        client: client_entity,
        from_slot: Some(dropped.slot_id),
        stack: dropped.stack,
    });
}

fn dropped_from_key(
    pkt: &ClickSlotC2s,
    input: (
        &mut Client,
        &mut Inventory,
        &mut ClientInventoryState,
        Option<bevy_ecs::change_detection::Mut<OpenInventory>>,
        &mut Query<&mut Inventory, Without<Client>>,
        &CursorItem,
    ),
) -> Option<DroppedStack> {
    debug_assert_eq!(pkt.mode, ClickMode::DropKey);
    debug_assert!(pkt.slot_idx >= OUTSIDE_WINDOW_SLOT);

    let (client, client_inv, inv_state, open_inventory, inventories, cursor_item) = input;
    let Some(open_inventory) = open_inventory else {
        return drop_key_without_view(client, client_inv, inv_state, pkt, cursor_item);
    };

    let Ok(mut target_inventory) = inventories.get_mut(open_inventory.entity) else {
        return None;
    };
    if resync_state_mismatch(
        client,
        inv_state,
        &target_inventory,
        pkt.state_id.0,
        cursor_item,
    ) {
        return None;
    }
    if pkt.slot_idx == OUTSIDE_WINDOW_SLOT {
        return None;
    }

    if slot_idx_in_inventory(&target_inventory, pkt.slot_idx) {
        return drop_key_from_target(client, inv_state, &mut target_inventory, pkt, cursor_item);
    }
    drop_key_from_player(
        pkt,
        (
            client,
            client_inv,
            inv_state,
            &target_inventory,
            cursor_item,
        ),
    )
}

fn drop_key_from_target(
    client: &mut Client,
    inv_state: &ClientInventoryState,
    target_inventory: &mut Inventory,
    pkt: &ClickSlotC2s,
    cursor_item: &CursorItem,
) -> Option<DroppedStack> {
    debug_assert_eq!(pkt.mode, ClickMode::DropKey);
    debug_assert!(slot_idx_in_inventory(target_inventory, pkt.slot_idx));

    if resync_readonly_inventory(
        client,
        target_inventory,
        inv_state,
        inv_state.window_id,
        cursor_item,
    ) {
        return None;
    }
    let Some(target_slot_id) = slot_idx_to_u16(pkt.slot_idx) else {
        return None;
    };
    remove_stack(target_inventory, target_slot_id, is_entire_stack(pkt))
}

fn drop_key_from_player(
    pkt: &ClickSlotC2s,
    input: (
        &mut Client,
        &mut Inventory,
        &ClientInventoryState,
        &Inventory,
        &CursorItem,
    ),
) -> Option<DroppedStack> {
    let (client, client_inv, inv_state, target_inventory, cursor_item) = input;
    debug_assert!(!slot_idx_in_inventory(target_inventory, pkt.slot_idx));
    debug_assert_eq!(pkt.mode, ClickMode::DropKey);

    if resync_readonly_inventory(client, client_inv, inv_state, 0, cursor_item) {
        return None;
    }
    let Some(packet_slot_id) = slot_idx_to_u16(pkt.slot_idx) else {
        return None;
    };
    let slot_id = convert_to_player_slot_id(target_inventory.kind, packet_slot_id);
    remove_stack(client_inv, slot_id, is_entire_stack(pkt))
}

fn drop_key_without_view(
    client: &mut Client,
    client_inv: &mut Inventory,
    inv_state: &ClientInventoryState,
    pkt: &ClickSlotC2s,
    cursor_item: &CursorItem,
) -> Option<DroppedStack> {
    if resync_readonly_inventory(client, client_inv, inv_state, 0, cursor_item) {
        return None;
    }
    if pkt.slot_idx == OUTSIDE_WINDOW_SLOT {
        return None;
    }
    let Some(slot_id) = slot_idx_to_u16(pkt.slot_idx) else {
        return None;
    };
    remove_stack(client_inv, slot_id, is_entire_stack(pkt))
}

fn remove_stack(inv: &mut Inventory, slot_id: u16, is_entire_stack: bool) -> Option<DroppedStack> {
    let stack = remove_clicked_stack(inv, slot_id, is_entire_stack);
    if stack.is_empty() {
        return None;
    }
    Some(DroppedStack { slot_id, stack })
}

fn is_entire_stack(pkt: &ClickSlotC2s) -> bool {
    pkt.button == 1
}
