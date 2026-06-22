use super::*;

pub(super) fn window_matches(
    packet: &ClickSlotC2s,
    open_inventory: Option<&Inventory>,
) -> Result<()> {
    let is_player_window = packet.window_id == PLAYER_WINDOW_ID;
    valence_server::protocol::anyhow::ensure!(
        is_player_window == open_inventory.is_none(),
        "window id and open inventory mismatch: window_id: {} open_inventory: {}",
        packet.window_id,
        open_inventory.is_some()
    );
    Ok(())
}

pub(super) fn accessible_slot_count(
    player_inventory: &Inventory,
    open_inventory: Option<&Inventory>,
) -> Result<u16> {
    let Some(open_inv) = open_inventory else {
        return Ok(player_inventory.slot_count());
    };
    let Some(count) = PlayerInventory::MAIN_SIZE.checked_add(open_inv.slot_count()) else {
        valence_server::protocol::anyhow::bail!("accessible slot count overflowed")
    };
    Ok(count)
}

pub(super) fn check_packet(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    slot_changes(packet, max_slot)?;
    carried_item(packet)?;
    mode(packet, max_slot)
}

fn slot_changes(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(
        packet
            .slot_changes
            .iter()
            .all(|slot| is_valid_slot_change(slot, max_slot)),
        "invalid slot ids or item counts"
    );
    Ok(())
}

fn is_valid_slot_change(slot: &SlotChange, max_slot: u16) -> bool {
    slot_index_in_window(slot.idx, max_slot) && is_valid_stack_count(&slot.stack)
}

fn is_valid_stack_count(stack: &ItemStack) -> bool {
    if stack.is_empty() {
        return true;
    }
    let max_stack_count = stack.item.max_stack().max(stack.count);
    (MIN_ITEM_COUNT..=max_stack_count).contains(&stack.count)
}

fn carried_item(packet: &ClickSlotC2s) -> Result<()> {
    if packet.carried_item.is_empty() {
        return Ok(());
    }
    valence_server::protocol::anyhow::ensure!(
        is_valid_stack_count(&packet.carried_item),
        "invalid carried item count"
    );
    Ok(())
}

fn mode(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    match packet.mode {
        ClickMode::Click => click(packet, max_slot),
        ClickMode::ShiftClick => shift(packet, max_slot),
        ClickMode::Hotbar => hotbar(packet),
        ClickMode::CreativeMiddleClick => creative_middle(packet, max_slot),
        ClickMode::DropKey => drop_key(packet, max_slot),
        ClickMode::Drag => drag(packet, max_slot),
        ClickMode::DoubleClick => double_click(packet),
    }
}

fn click(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(is_primary_button(packet.button), "invalid button");
    valence_server::protocol::anyhow::ensure!(
        slot_index_in_window(packet.slot_idx, max_slot)
            || packet.slot_idx == OUTSIDE_WINDOW_SLOT
            || packet.slot_idx == MARGIN_SLOT,
        "invalid slot index"
    );
    Ok(())
}

fn shift(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(is_primary_button(packet.button), "invalid button");
    valence_server::protocol::anyhow::ensure!(
        packet.carried_item.is_empty(),
        "carried item must be empty for a hotbar swap"
    );
    valence_server::protocol::anyhow::ensure!(
        slot_index_in_window(packet.slot_idx, max_slot),
        "invalid slot index"
    );
    Ok(())
}

fn hotbar(packet: &ClickSlotC2s) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(is_hotbar_button(packet.button), "invalid button");
    valence_server::protocol::anyhow::ensure!(
        packet.carried_item.is_empty(),
        "carried item must be empty for a hotbar swap"
    );
    Ok(())
}

fn creative_middle(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(
        packet.button == CLICK_DRAG_START_BUTTON_MAX,
        "invalid button"
    );
    valence_server::protocol::anyhow::ensure!(
        slot_index_in_window(packet.slot_idx, max_slot),
        "invalid slot index"
    );
    Ok(())
}

fn drop_key(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(is_primary_button(packet.button), "invalid button");
    valence_server::protocol::anyhow::ensure!(
        packet.carried_item.is_empty(),
        "carried item must be empty for an item drop"
    );
    valence_server::protocol::anyhow::ensure!(
        slot_index_in_window(packet.slot_idx, max_slot) || packet.slot_idx == OUTSIDE_WINDOW_SLOT,
        "invalid slot index"
    );
    Ok(())
}

fn drag(packet: &ClickSlotC2s, max_slot: u16) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(is_drag_button(packet.button), "invalid button");
    valence_server::protocol::anyhow::ensure!(
        slot_index_in_window(packet.slot_idx, max_slot) || packet.slot_idx == OUTSIDE_WINDOW_SLOT,
        "invalid slot index"
    );
    Ok(())
}

fn double_click(packet: &ClickSlotC2s) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(packet.button == LEFT_BUTTON, "invalid button");
    Ok(())
}

fn is_primary_button(button: i8) -> bool {
    (LEFT_BUTTON..=RIGHT_BUTTON).contains(&button)
}

fn is_hotbar_button(button: i8) -> bool {
    (LEFT_BUTTON..=HOTBAR_BUTTON_MAX).contains(&button) || button == HOTBAR_OFFHAND_BUTTON
}

fn is_drag_button(button: i8) -> bool {
    (CLICK_DRAG_START_BUTTON_MIN..=CLICK_DRAG_START_BUTTON_MAX).contains(&button)
        || (CLICK_DRAG_MIDDLE_BUTTON_MIN..=CLICK_DRAG_MIDDLE_BUTTON_MAX).contains(&button)
        || (CLICK_DRAG_END_BUTTON_MIN..=CLICK_DRAG_END_BUTTON_MAX).contains(&button)
}

fn slot_index_in_window(slot_idx: i16, max_slot: u16) -> bool {
    let Ok(slot_idx) = u16::try_from(slot_idx) else {
        return false;
    };
    (PLAYER_WINDOW_ID.into()..=max_slot).contains(&slot_idx)
}
