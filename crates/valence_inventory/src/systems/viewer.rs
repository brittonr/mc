use super::*;

const MAX_OPEN_WINDOW_ID: u8 = 100;

/// Handles the `OpenInventory` component being added to a client, which
/// indicates that the client is now viewing an inventory, and sends inventory
/// updates to the client when the inventory is modified.
pub(super) fn update_open_inventories(
    mut clients: Query<(
        Entity,
        &mut Client,
        &mut ClientInventoryState,
        &CursorItem,
        &mut OpenInventory,
    )>,
    mut inventories: Query<&mut Inventory>,
    mut commands: Commands,
) {
    for (client_entity, mut client, mut inv_state, cursor_item, mut open_inventory) in &mut clients
    {
        let is_open_inventory_added = open_inventory.is_added();
        let Ok([inventory, player_inventory]) =
            inventories.get_many_mut([open_inventory.entity, client_entity])
        else {
            close_missing_open_inventory(&mut commands, client_entity, &mut client, &inv_state);
            continue;
        };

        let should_clear_player_changes = sync_open_inventory(
            OpenSync {
                client: &mut client,
                inv_state: &mut inv_state,
                cursor_item,
                open_inventory: &mut open_inventory,
                inventory: &inventory,
                player_inventory: &player_inventory,
            },
            is_open_inventory_added,
        );

        if should_clear_player_changes {
            player_inventory
                .map_unchanged(|f| &mut f.changed)
                .set_if_neq(0);
        }

        open_inventory
            .map_unchanged(|f| &mut f.client_changed)
            .set_if_neq(0);
        inv_state
            .map_unchanged(|f| &mut f.slots_changed)
            .set_if_neq(0);
        inventory.map_unchanged(|f| &mut f.changed).set_if_neq(0);
    }
}

fn close_missing_open_inventory(
    commands: &mut Commands,
    client_entity: Entity,
    client: &mut Client,
    inv_state: &ClientInventoryState,
) {
    commands.entity(client_entity).remove::<OpenInventory>();
    client.write_packet(&CloseScreenS2c {
        window_id: inv_state.window_id,
    });
}

struct OpenSync<'a> {
    client: &'a mut Client,
    inv_state: &'a mut ClientInventoryState,
    cursor_item: &'a CursorItem,
    open_inventory: &'a mut OpenInventory,
    inventory: &'a Inventory,
    player_inventory: &'a Inventory,
}

fn sync_open_inventory(ctx: OpenSync<'_>, is_open_inventory_added: bool) -> bool {
    debug_assert!(ctx.inventory.slot_count() > 0);
    debug_assert!(ctx.player_inventory.slot_count() > 0);

    if is_open_inventory_added {
        send_new_open_inventory(
            ctx.client,
            ctx.inv_state,
            ctx.cursor_item,
            ctx.open_inventory,
            ctx.inventory,
        );
        return false;
    }

    if ctx.inventory.changed == u64::MAX {
        send_full_open_inventory(ctx.client, ctx.inv_state, ctx.cursor_item, ctx.inventory);
        return false;
    }

    send_changed_open_inventory(
        ctx.client,
        ctx.inv_state,
        ctx.open_inventory,
        ctx.inventory,
        ctx.player_inventory,
    )
}

fn send_new_open_inventory(
    client: &mut Client,
    inv_state: &mut ClientInventoryState,
    cursor_item: &CursorItem,
    open_inventory: &mut OpenInventory,
    inventory: &Inventory,
) {
    inv_state.window_id = next_window_id(inv_state.window_id);
    open_inventory.client_changed = 0;

    client.write_packet(&OpenScreenS2c {
        window_id: valence_server::protocol::VarInt(inv_state.window_id.into()),
        window_type: WindowType::from(inventory.kind),
        window_title: std::borrow::Cow::Borrowed(&inventory.title),
    });
    client.write_packet(&InventoryS2c {
        window_id: inv_state.window_id,
        state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
        slots: std::borrow::Cow::Borrowed(inventory.slot_slice()),
        carried_item: std::borrow::Cow::Borrowed(&cursor_item.0),
    });
}

fn send_full_open_inventory(
    client: &mut Client,
    inv_state: &mut ClientInventoryState,
    cursor_item: &CursorItem,
    inventory: &Inventory,
) {
    inv_state.state_id += 1;
    client.write_packet(&InventoryS2c {
        window_id: inv_state.window_id,
        state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
        slots: std::borrow::Cow::Borrowed(inventory.slot_slice()),
        carried_item: std::borrow::Cow::Borrowed(&cursor_item.0),
    });
}

fn send_changed_open_inventory(
    client: &mut Client,
    inv_state: &ClientInventoryState,
    open_inventory: &OpenInventory,
    inventory: &Inventory,
    player_inventory: &Inventory,
) -> bool {
    let changed_filtered = open_inventory_changed_bits(open_inventory, inventory, player_inventory);
    if changed_filtered == 0 {
        return false;
    }

    write_changed_open_slots(
        client,
        inv_state,
        inventory,
        player_inventory,
        changed_filtered,
    );
    true
}

fn open_inventory_changed_bits(
    open_inventory: &OpenInventory,
    inventory: &Inventory,
    player_inventory: &Inventory,
) -> u128 {
    let target_changed = u128::from(inventory.changed & !open_inventory.client_changed);
    let player_changed = shifted_player_inventory_changes(player_inventory, inventory.slot_count());
    target_changed | player_changed
}

fn shifted_player_inventory_changes(player_inventory: &Inventory, target_slot_count: u16) -> u128 {
    let Some(player_changed) = u128::from(player_inventory.changed)
        .checked_shr(u32::from(*PlayerInventory::SLOTS_MAIN.start()))
    else {
        unreachable!();
    };
    let Some(player_changed) = player_changed.checked_shl(u32::from(target_slot_count)) else {
        unreachable!();
    };
    player_changed
}

fn write_changed_open_slots(
    client: &mut Client,
    inv_state: &ClientInventoryState,
    inventory: &Inventory,
    player_inventory: &Inventory,
    changed_filtered: u128,
) {
    for (slot_idx, slot) in inventory
        .slots
        .iter()
        .chain(
            player_inventory
                .slots
                .iter()
                .skip(usize::from(*PlayerInventory::SLOTS_MAIN.start())),
        )
        .enumerate()
    {
        let Ok(shift) = u32::try_from(slot_idx) else {
            continue;
        };
        let Some(mask) = changed_filtered.checked_shr(shift) else {
            continue;
        };
        if mask & 1 != 1 {
            continue;
        }
        let Ok(slot_idx) = i16::try_from(slot_idx) else {
            continue;
        };
        client.write_packet(&ScreenHandlerSlotUpdateS2c {
            window_id: i8::from_ne_bytes([inv_state.window_id]),
            state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
            slot_idx,
            slot_data: std::borrow::Cow::Borrowed(slot),
        });
    }
}

fn next_window_id(window_id: u8) -> u8 {
    let Some(window_id) = window_id.checked_rem(MAX_OPEN_WINDOW_ID) else {
        unreachable!();
    };
    let Some(window_id) = window_id.checked_add(1) else {
        unreachable!();
    };
    window_id
}
