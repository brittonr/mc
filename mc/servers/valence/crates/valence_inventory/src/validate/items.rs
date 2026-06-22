use super::*;

mod drop;

pub(super) fn check_conservation(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    match packet.mode {
        ClickMode::Click => click(packet, window, cursor_item),
        ClickMode::ShiftClick => shift_click(packet, window, cursor_item),
        ClickMode::Hotbar => hotbar(packet, window, cursor_item),
        ClickMode::CreativeMiddleClick => Ok(()),
        ClickMode::DropKey => drop::check(packet, window, cursor_item),
        ClickMode::Drag => drag(packet, window, cursor_item),
        ClickMode::DoubleClick => ensure_zero_delta(packet, window, cursor_item),
    }
}

fn click(packet: &ClickSlotC2s, window: &InventoryWindow, cursor_item: &CursorItem) -> Result<()> {
    match packet.slot_idx {
        MARGIN_SLOT => empty_change_delta(packet, window, cursor_item),
        OUTSIDE_WINDOW_SLOT => outside_click_delta_matches(packet, window, cursor_item),
        _ if packet.slot_changes.is_empty() => ensure_zero_delta(packet, window, cursor_item),
        _ => modified_click_slot(packet, window, cursor_item),
    }
}

fn empty_change_delta(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.is_empty(),
        "slot modifications must be empty"
    );
    ensure_zero_delta(packet, window, cursor_item)
}

fn outside_click_delta_matches(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.is_empty(),
        "slot modifications must be empty"
    );
    let expected_delta = outside_click_delta(packet.button, cursor_item);
    ensure_delta(DeltaCheck {
        actual: super::delta::net_item_change(packet, window, cursor_item),
        expected: expected_delta,
    })
}

fn modified_click_slot(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    debug_assert!(matches!(packet.mode, ClickMode::Click));
    debug_assert!(!packet.slot_changes.is_empty());
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.len() == SLOT_CHANGES_ONE,
        "click must modify one slot, got {}",
        packet.slot_changes.len()
    );
    let old_slot = window.slot(slot_index(changed_slot(packet, SLOT_CHANGE_FIRST)?.idx)?);
    if should_swap(packet, old_slot, cursor_item) {
        valence_server::protocol::anyhow::ensure!(
            click_swap_matches(packet, old_slot, cursor_item)?,
            "swapped items must match"
        );
    } else {
        ensure_zero_delta_with_message(
            packet,
            window,
            cursor_item,
            "invalid item delta for stack merge",
        )?;
    }
    Ok(())
}

fn should_swap(packet: &ClickSlotC2s, old_slot: &ItemStack, cursor_item: &CursorItem) -> bool {
    packet.button == LEFT_BUTTON
        && match (!old_slot.is_empty(), !cursor_item.is_empty()) {
            (true, true) => old_slot.item != cursor_item.item,
            (true, false) => true,
            (false, true) => cursor_item.count <= cursor_item.item.max_stack(),
            (false, false) => false,
        }
}

fn click_swap_matches(
    packet: &ClickSlotC2s,
    old_slot: &ItemStack,
    cursor_item: &CursorItem,
) -> Result<bool> {
    let changed_stack = &changed_slot(packet, SLOT_CHANGE_FIRST)?.stack;
    Ok(old_slot.item == packet.carried_item.item
        && old_slot.count == packet.carried_item.count
        && cursor_item.0 == *changed_stack)
}

fn shift_click(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    debug_assert!(matches!(packet.mode, ClickMode::ShiftClick));
    debug_assert!(packet.carried_item.is_empty());
    if packet.slot_changes.is_empty() {
        return ensure_zero_delta(packet, window, cursor_item);
    }
    valence_server::protocol::anyhow::ensure!(
        (SLOT_CHANGES_TWO..=SLOT_CHANGES_THREE).contains(&packet.slot_changes.len()),
        "shift click must modify 2 or 3 slots, got {}",
        packet.slot_changes.len()
    );
    ensure_zero_delta(packet, window, cursor_item)?;
    let item_kind = moved_item_kind(packet)?;
    let old_slot_kind = window.slot(slot_index(packet.slot_idx)?).item;
    valence_server::protocol::anyhow::ensure!(
        old_slot_kind == item_kind,
        "shift click must move the same item kind as modified slots"
    );
    valence_server::protocol::anyhow::ensure!(
        all_non_empty_changes_match(packet, item_kind),
        "shift click must move the same item kind"
    );
    Ok(())
}

fn moved_item_kind(packet: &ClickSlotC2s) -> Result<ItemKind> {
    let Some(item_kind) = packet
        .slot_changes
        .iter()
        .find(|slot| !slot.stack.is_empty())
        .map(|slot| slot.stack.item)
    else {
        valence_server::protocol::anyhow::bail!("shift click must move an item")
    };
    Ok(item_kind)
}

fn all_non_empty_changes_match(packet: &ClickSlotC2s, item_kind: ItemKind) -> bool {
    packet
        .slot_changes
        .iter()
        .filter(|slot| !slot.stack.is_empty())
        .all(|slot| slot.stack.item == item_kind)
}

fn hotbar(packet: &ClickSlotC2s, window: &InventoryWindow, cursor_item: &CursorItem) -> Result<()> {
    if packet.slot_changes.is_empty() {
        return ensure_zero_delta(packet, window, cursor_item);
    }
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.len() == SLOT_CHANGES_TWO,
        "hotbar swap must modify two slots, got {}",
        packet.slot_changes.len()
    );
    ensure_zero_delta(packet, window, cursor_item)?;
    valence_server::protocol::anyhow::ensure!(
        hotbar_swap_matches(packet, window)?,
        "swapped items must match"
    );
    Ok(())
}

fn hotbar_swap_matches(packet: &ClickSlotC2s, window: &InventoryWindow) -> Result<bool> {
    let first_change = changed_slot(packet, SLOT_CHANGE_FIRST)?;
    let second_change = changed_slot(packet, SLOT_CHANGE_SECOND)?;
    let old_slots = [
        window.slot(slot_index(first_change.idx)?),
        window.slot(slot_index(second_change.idx)?),
    ];
    Ok(old_slots
        .iter()
        .any(|slot| slot_matches(slot, &first_change.stack))
        && old_slots
            .iter()
            .any(|slot| slot_matches(slot, &second_change.stack)))
}

fn slot_matches(old_slot: &ItemStack, new_slot: &ItemStack) -> bool {
    old_slot.item == new_slot.item && old_slot.count == new_slot.count
}

fn drag(packet: &ClickSlotC2s, window: &InventoryWindow, cursor_item: &CursorItem) -> Result<()> {
    if is_drag_distribution_button(packet.button) {
        return ensure_zero_delta(packet, window, cursor_item);
    }
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.is_empty() && packet.carried_item == cursor_item.0,
        "drag start/end must not modify slots"
    );
    Ok(())
}

fn is_drag_distribution_button(button: i8) -> bool {
    button == CLICK_DRAG_START_BUTTON_MAX
        || button == CLICK_DRAG_MIDDLE_BUTTON_MAX
        || button == CLICK_DRAG_END_BUTTON_MAX
}

fn outside_click_delta(button: i8, cursor_item: &CursorItem) -> i32 {
    match button {
        RIGHT_BUTTON => SINGLE_DROP_DELTA,
        LEFT_BUTTON if !cursor_item.is_empty() => super::delta::negate_count(cursor_item.0.count),
        LEFT_BUTTON => ITEM_DELTA_ZERO,
        _ => unreachable!(),
    }
}

fn ensure_zero_delta(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    ensure_delta(DeltaCheck {
        actual: super::delta::net_item_change(packet, window, cursor_item),
        expected: ITEM_DELTA_ZERO,
    })
}

fn ensure_zero_delta_with_message(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
    message: &str,
) -> Result<()> {
    let count_delta = super::delta::net_item_change(packet, window, cursor_item);
    valence_server::protocol::anyhow::ensure!(
        count_delta == ITEM_DELTA_ZERO,
        "{message}: {count_delta}"
    );
    Ok(())
}

pub(super) struct DeltaCheck {
    actual: i32,
    expected: i32,
}

pub(super) fn ensure_delta(delta: DeltaCheck) -> Result<()> {
    valence_server::protocol::anyhow::ensure!(
        delta.actual == delta.expected,
        "invalid item delta: expected {}, got {}",
        delta.expected,
        delta.actual
    );
    Ok(())
}
