use super::*;

pub(super) struct Ctx<'a> {
    pub(super) client: &'a mut Client,
    pub(super) client_inv: &'a mut Inventory,
    pub(super) inv_state: &'a mut ClientInventoryState,
    pub(super) open_inventory: Option<bevy_ecs::change_detection::Mut<'a, OpenInventory>>,
    pub(super) cursor_item: &'a mut CursorItem,
}

pub(super) fn handle_regular(
    pkt: &ClickSlotC2s,
    ctx: Ctx<'_>,
    inventories: &mut Query<&mut Inventory, Without<Client>>,
) -> bool {
    debug_assert_ne!(pkt.mode, ClickMode::DropKey);
    debug_assert!(pkt.slot_idx >= OUTSIDE_WINDOW_SLOT);

    if (pkt.window_id == 0) != ctx.open_inventory.is_none() {
        tracing::warn!(
            "Client sent a click with an invalid window id for current state: window_id = {}, \
             open_inventory present = {}",
            pkt.window_id,
            ctx.open_inventory.is_some()
        );
        return false;
    }

    let Some(open_inventory) = ctx.open_inventory else {
        return handle_player(
            pkt,
            ctx.client,
            ctx.client_inv,
            ctx.inv_state,
            ctx.cursor_item,
        );
    };
    handle_view(
        pkt,
        ViewCtx {
            client: ctx.client,
            client_inv: ctx.client_inv,
            inv_state: ctx.inv_state,
            open_inventory,
            cursor_item: ctx.cursor_item,
        },
        inventories,
    )
}

struct ViewCtx<'a> {
    client: &'a mut Client,
    client_inv: &'a mut Inventory,
    inv_state: &'a mut ClientInventoryState,
    open_inventory: bevy_ecs::change_detection::Mut<'a, OpenInventory>,
    cursor_item: &'a mut CursorItem,
}

fn handle_view(
    pkt: &ClickSlotC2s,
    ctx: ViewCtx<'_>,
    inventories: &mut Query<&mut Inventory, Without<Client>>,
) -> bool {
    debug_assert_ne!(pkt.mode, ClickMode::DropKey);
    debug_assert_ne!(pkt.window_id, 0);

    let mut open_inventory = ctx.open_inventory;
    let Ok(mut target_inventory) = inventories.get_mut(open_inventory.entity) else {
        return false;
    };
    if resync_state_mismatch(
        ctx.client,
        ctx.inv_state,
        &target_inventory,
        pkt.state_id.0,
        ctx.cursor_item,
    ) {
        return false;
    }

    let mut new_cursor = pkt.carried_item.clone();
    for slot in pkt.slot_changes.iter() {
        let mut slot_ctx = SlotCtx {
            new_cursor: &mut new_cursor,
            cursor_item: ctx.cursor_item,
            client_inv: ctx.client_inv,
            target_inventory: &mut target_inventory,
            open_inventory: &mut open_inventory,
            inv_state: ctx.inv_state,
        };
        apply_view_slot_change(slot, pkt, &mut slot_ctx);
    }

    update_cursor_after(ctx.cursor_item, ctx.inv_state, new_cursor);
    resync_readonly_views(
        ctx.client,
        ctx.client_inv,
        ctx.inv_state,
        &target_inventory,
        ctx.cursor_item,
    );
    true
}

struct SlotCtx<'a> {
    new_cursor: &'a mut ItemStack,
    cursor_item: &'a CursorItem,
    client_inv: &'a mut Inventory,
    target_inventory: &'a mut Inventory,
    open_inventory: &'a mut OpenInventory,
    inv_state: &'a mut ClientInventoryState,
}

fn apply_view_slot_change(slot: &SlotChange, pkt: &ClickSlotC2s, ctx: &mut SlotCtx<'_>) {
    debug_assert_ne!(pkt.mode, ClickMode::DropKey);
    debug_assert!(slot.idx >= OUTSIDE_WINDOW_SLOT);

    let is_transfer = (slot_idx_in_inventory(ctx.target_inventory, pkt.slot_idx)
        && pkt.mode == ClickMode::Hotbar)
        || pkt.mode == ClickMode::ShiftClick;

    if slot_idx_in_inventory(ctx.target_inventory, slot.idx) {
        apply_target_slot_change(slot, ctx, is_transfer);
        return;
    }
    apply_client_slot_change(slot, ctx, is_transfer);
}

fn apply_target_slot_change(slot: &SlotChange, ctx: &mut SlotCtx<'_>, is_transfer: bool) {
    if (ctx.client_inv.readonly && is_transfer) || ctx.target_inventory.readonly {
        *ctx.new_cursor = ctx.cursor_item.0.clone();
        return;
    }
    let Some(target_slot_id) = slot_idx_to_u16(slot.idx) else {
        return;
    };
    ctx.target_inventory
        .set_slot(target_slot_id, slot.stack.clone());
    mark_changed_slot(&mut ctx.open_inventory.client_changed, target_slot_id);
}

fn apply_client_slot_change(slot: &SlotChange, ctx: &mut SlotCtx<'_>, is_transfer: bool) {
    if (ctx.target_inventory.readonly && is_transfer) || ctx.client_inv.readonly {
        *ctx.new_cursor = ctx.cursor_item.0.clone();
        return;
    }
    let Some(packet_slot_id) = slot_idx_to_u16(slot.idx) else {
        return;
    };
    let slot_id = convert_to_player_slot_id(ctx.target_inventory.kind, packet_slot_id);
    ctx.client_inv.set_slot(slot_id, slot.stack.clone());
    mark_changed_slot(&mut ctx.inv_state.slots_changed, slot_id);
}

fn handle_player(
    pkt: &ClickSlotC2s,
    client: &mut Client,
    client_inv: &mut Inventory,
    inv_state: &mut ClientInventoryState,
    cursor_item: &mut CursorItem,
) -> bool {
    if resync_state_mismatch(client, inv_state, client_inv, pkt.state_id.0, cursor_item) {
        return false;
    }

    let mut new_cursor = pkt.carried_item.clone();
    for slot in pkt.slot_changes.iter() {
        apply_player_slot_change(slot, &mut new_cursor, cursor_item, client_inv, inv_state);
    }

    update_cursor_after(cursor_item, inv_state, new_cursor);
    resync_readonly_inventory(client, client_inv, inv_state, 0, cursor_item);
    true
}

fn apply_player_slot_change(
    slot: &SlotChange,
    new_cursor: &mut ItemStack,
    cursor_item: &CursorItem,
    client_inv: &mut Inventory,
    inv_state: &mut ClientInventoryState,
) {
    if !slot_idx_in_inventory(client_inv, slot.idx) {
        tracing::warn!(
            "Client attempted to interact with slot {} which does not exist",
            slot.idx
        );
        return;
    }
    if client_inv.readonly {
        *new_cursor = cursor_item.0.clone();
        return;
    }
    let Some(slot_id) = slot_idx_to_u16(slot.idx) else {
        return;
    };
    client_inv.set_slot(slot_id, slot.stack.clone());
    mark_changed_slot(&mut inv_state.slots_changed, slot_id);
}

fn update_cursor_after(
    cursor_item: &mut CursorItem,
    inv_state: &mut ClientInventoryState,
    new_cursor: ItemStack,
) {
    if cursor_item.0 != new_cursor {
        *cursor_item = CursorItem(new_cursor.clone());
    }
    inv_state.client_updated_cursor_item = Some(new_cursor);
}

fn resync_readonly_views(
    client: &mut Client,
    client_inv: &Inventory,
    inv_state: &ClientInventoryState,
    target_inventory: &Inventory,
    cursor_item: &CursorItem,
) {
    if !target_inventory.readonly && !client_inv.readonly {
        return;
    }
    write_inventory_packet(
        client,
        inv_state.window_id,
        inv_state,
        target_inventory,
        cursor_item,
    );
    write_inventory_packet(client, 0, inv_state, client_inv, cursor_item);
}
