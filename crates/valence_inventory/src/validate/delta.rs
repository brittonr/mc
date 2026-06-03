use super::*;

struct CountChange {
    new_count: i8,
    old_count: i8,
}

struct RunningChange {
    total: i32,
    next: i32,
}

/// Calculate the total difference in item counts if the changes in this packet
/// were to be applied.
///
/// Returns a positive number if items were added to the window, and a negative
/// number if items were removed from the window.
pub(super) fn net_item_change(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> i32 {
    let slot_change = packet
        .slot_changes
        .iter()
        .fold(ITEM_DELTA_ZERO, |total, slot| {
            add_changes(RunningChange {
                total,
                next: changed_slot_change(window, slot),
            })
        });
    add_changes(RunningChange {
        total: slot_change,
        next: cursor_change(packet, cursor_item),
    })
}

fn changed_slot_change(window: &InventoryWindow, slot: &SlotChange) -> i32 {
    let Ok(slot_idx) = u16::try_from(slot.idx) else {
        return ITEM_DELTA_ZERO;
    };
    item_change(window.slot(slot_idx), &slot.stack)
}

fn cursor_change(packet: &ClickSlotC2s, cursor_item: &CursorItem) -> i32 {
    item_change(&cursor_item.0, &packet.carried_item)
}

fn item_change(old_slot: &ItemStack, new_slot: &ItemStack) -> i32 {
    match (!old_slot.is_empty(), !new_slot.is_empty()) {
        (true, true) => subtract_counts(CountChange {
            new_count: new_slot.count,
            old_count: old_slot.count,
        }),
        (true, false) => negate_count(old_slot.count),
        (false, true) => i32::from(new_slot.count),
        (false, false) => ITEM_DELTA_ZERO,
    }
}

fn subtract_counts(change: CountChange) -> i32 {
    let Some(delta) = i32::from(change.new_count).checked_sub(i32::from(change.old_count)) else {
        unreachable!();
    };
    delta
}

pub(super) fn negate_count(count: i8) -> i32 {
    let Some(delta) = i32::from(count).checked_neg() else {
        unreachable!();
    };
    delta
}

fn add_changes(change: RunningChange) -> i32 {
    match change.total.checked_add(change.next) {
        Some(total) => total,
        None if change.next.is_negative() => i32::MIN,
        None => i32::MAX,
    }
}
