use super::*;

pub(super) fn handle_player_actions(
    mut packets: EventReader<PacketEvent>,
    mut clients: Query<(
        &mut Inventory,
        &mut ClientInventoryState,
        &HeldItem,
        &mut Client,
    )>,
    mut drop_item_stack_events: EventWriter<DropItemStackEvent>,
) {
    for packet in packets.read() {
        let Some(pkt) = packet.decode::<PlayerActionC2s>() else {
            continue;
        };

        match pkt.action {
            PlayerAction::DropAllItems => {
                drop_all_held_items(&mut clients, &mut drop_item_stack_events, packet.client);
            }
            PlayerAction::DropItem => {
                drop_one_held_item(&mut clients, &mut drop_item_stack_events, packet.client);
            }
            PlayerAction::SwapItemWithOffhand => {
                swap_held_item_with_offhand(&mut clients, packet.client);
            }
            _ => {}
        }
    }
}

fn drop_all_held_items(
    clients: &mut Query<(
        &mut Inventory,
        &mut ClientInventoryState,
        &HeldItem,
        &mut Client,
    )>,
    drop_item_stack_events: &mut EventWriter<DropItemStackEvent>,
    client: Entity,
) {
    let Ok((mut inv, mut inv_state, &held, mut packet_client)) = clients.get_mut(client) else {
        return;
    };

    if resync_readonly_client_inventory(&inv, &inv_state, &mut packet_client) {
        return;
    }

    let stack = inv.replace_slot(held.slot(), ItemStack::EMPTY);
    if stack.is_empty() {
        return;
    }

    mark_slot_changed(&mut inv_state, held.slot());
    drop_item_stack_events.send(DropItemStackEvent {
        client,
        from_slot: Some(held.slot()),
        stack,
    });
}

fn drop_one_held_item(
    clients: &mut Query<(
        &mut Inventory,
        &mut ClientInventoryState,
        &HeldItem,
        &mut Client,
    )>,
    drop_item_stack_events: &mut EventWriter<DropItemStackEvent>,
    client: Entity,
) {
    let Ok((mut inv, mut inv_state, held, mut packet_client)) = clients.get_mut(client) else {
        return;
    };

    if resync_readonly_client_inventory(&inv, &inv_state, &mut packet_client) {
        return;
    }

    let mut stack = inv.replace_slot(held.slot(), ItemStack::EMPTY);
    if stack.is_empty() {
        return;
    }

    keep_remaining_held_items(&mut inv, held, &mut stack);
    mark_slot_changed(&mut inv_state, held.slot());
    drop_item_stack_events.send(DropItemStackEvent {
        client,
        from_slot: Some(held.slot()),
        stack,
    });
}

fn swap_held_item_with_offhand(
    clients: &mut Query<(
        &mut Inventory,
        &mut ClientInventoryState,
        &HeldItem,
        &mut Client,
    )>,
    client: Entity,
) {
    let Ok((mut inv, inv_state, held, mut packet_client)) = clients.get_mut(client) else {
        return;
    };

    if resync_readonly_client_inventory(&inv, &inv_state, &mut packet_client) {
        return;
    }

    inv.swap_slot(held.slot(), PlayerInventory::SLOT_OFFHAND);
}

fn resync_readonly_client_inventory(
    inv: &Inventory,
    inv_state: &ClientInventoryState,
    client: &mut Client,
) -> bool {
    if !inv.readonly {
        return false;
    }

    client.write_packet(&InventoryS2c {
        window_id: 0,
        state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
        slots: std::borrow::Cow::Borrowed(inv.slot_slice()),
        carried_item: std::borrow::Cow::Borrowed(&ItemStack::EMPTY),
    });
    true
}

fn mark_slot_changed(inv_state: &mut ClientInventoryState, slot: u16) {
    let Some(mask) = 1_u64.checked_shl(u32::from(slot)) else {
        unreachable!();
    };
    inv_state.slots_changed |= mask;
}

fn keep_remaining_held_items(inv: &mut Inventory, held: &HeldItem, stack: &mut ItemStack) {
    if stack.count <= 1 {
        return;
    }

    let Some(remaining_count) = stack.count.checked_sub(1) else {
        unreachable!();
    };
    inv.set_slot(held.slot(), stack.clone().with_count(remaining_count));
    stack.count = 1;
}
