type ClickSlotC2s<'a> = super::super::ClickSlotC2s<'a>;
type CursorItem = super::super::CursorItem;
type InventoryWindow<'a> = super::super::InventoryWindow<'a>;
type Result<T> = super::super::Result<T>;

pub(super) fn check(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    debug_assert!(matches!(packet.mode, super::super::ClickMode::DropKey));
    debug_assert!(packet.carried_item.is_empty());
    if packet.slot_changes.is_empty() {
        return super::ensure_zero_delta(packet, window, cursor_item);
    }
    valence_server::protocol::anyhow::ensure!(
        packet.slot_changes.len() == super::super::SLOT_CHANGES_ONE,
        "drop key must modify exactly one slot"
    );
    valence_server::protocol::anyhow::ensure!(
        packet.slot_idx
            == packet
                .slot_changes
                .first()
                .map_or(super::super::INVALID_SLOT_FALLBACK, |slot| slot.idx),
        "slot index does not match modified slot"
    );
    valence_server::protocol::anyhow::ensure!(
        !transmutes(packet, window)?,
        "transmuting items is not allowed"
    );
    count_delta_matches(packet, window, cursor_item)
}

fn transmutes(packet: &ClickSlotC2s, window: &InventoryWindow) -> Result<bool> {
    let old_slot = window.slot(super::super::slot_index(packet.slot_idx)?);
    let new_slot = &super::super::changed_slot(packet, super::super::SLOT_CHANGE_FIRST)?.stack;
    Ok(match (!old_slot.is_empty(), !new_slot.is_empty()) {
        (true, true) => old_slot.item != new_slot.item,
        (_, false) => false,
        (false, true) => true,
    })
}

fn count_delta_matches(
    packet: &ClickSlotC2s,
    window: &InventoryWindow,
    cursor_item: &CursorItem,
) -> Result<()> {
    let old_slot = window.slot(super::super::slot_index(packet.slot_idx)?);
    let expected = match packet.button {
        super::super::LEFT_BUTTON => super::super::SINGLE_DROP_DELTA,
        super::super::RIGHT_BUTTON if !old_slot.is_empty() => {
            super::super::delta::negate_count(old_slot.count)
        }
        super::super::RIGHT_BUTTON => super::super::ITEM_DELTA_ZERO,
        _ => unreachable!(),
    };
    super::ensure_delta(super::DeltaCheck {
        actual: super::super::delta::net_item_change(packet, window, cursor_item),
        expected,
    })
}
