type ClickMode = valence_server::protocol::packets::play::click_slot_c2s::ClickMode;
type ClickSlotC2s<'a> = valence_server::protocol::packets::play::ClickSlotC2s<'a>;
type CursorItem = super::CursorItem;
type Inventory = super::Inventory;
type InventoryWindow<'a> = super::InventoryWindow<'a>;
type ItemKind = valence_server::ItemKind;
type ItemStack = valence_server::ItemStack;
type PlayerInventory = crate::player_inventory::PlayerInventory;
type Result<T> = valence_server::protocol::anyhow::Result<T>;
type SlotChange = valence_server::protocol::packets::play::click_slot_c2s::SlotChange;

mod delta;
mod fields;
mod items;

#[cfg(test)]
mod tests;

const CLICK_DRAG_END_BUTTON_MAX: i8 = 10;
const CLICK_DRAG_END_BUTTON_MIN: i8 = 8;
const CLICK_DRAG_MIDDLE_BUTTON_MAX: i8 = 6;
const CLICK_DRAG_MIDDLE_BUTTON_MIN: i8 = 4;
const CLICK_DRAG_START_BUTTON_MAX: i8 = 2;
const CLICK_DRAG_START_BUTTON_MIN: i8 = 0;
const HOTBAR_BUTTON_MAX: i8 = 8;
const HOTBAR_OFFHAND_BUTTON: i8 = 40;
const INVALID_SLOT_FALLBACK: i16 = -2;
const ITEM_DELTA_ZERO: i32 = 0;
const LEFT_BUTTON: i8 = 0;
const MARGIN_SLOT: i16 = -1;
const MIN_ITEM_COUNT: i8 = 1;
const OUTSIDE_WINDOW_SLOT: i16 = -999;
const PLAYER_WINDOW_ID: u8 = 0;
const RIGHT_BUTTON: i8 = 1;
const SINGLE_DROP_DELTA: i32 = -1;
const SLOT_CHANGE_FIRST: usize = 0;
const SLOT_CHANGE_SECOND: usize = 1;
const SLOT_CHANGES_ONE: usize = 1;
const SLOT_CHANGES_THREE: usize = 3;
const SLOT_CHANGES_TWO: usize = 2;

#[cfg(test)]
use delta::net_item_change as calculate_net_item_delta;

/// Validates a click slot packet enforcing that all fields are valid.
pub(super) fn check_packet(
    packet: &ClickSlotC2s,
    player_inventory: &Inventory,
    open_inventory: Option<&Inventory>,
    cursor_item: &CursorItem,
) -> Result<()> {
    fields::window_matches(packet, open_inventory)?;
    let window = InventoryWindow {
        player_inventory,
        open_inventory,
    };
    let max_slot = fields::accessible_slot_count(player_inventory, open_inventory)?;

    fields::check_packet(packet, max_slot)?;
    items::check_conservation(packet, &window, cursor_item)
}

fn slot_index(slot_idx: i16) -> Result<u16> {
    let Ok(slot_idx) = u16::try_from(slot_idx) else {
        valence_server::protocol::anyhow::bail!("invalid slot index")
    };
    Ok(slot_idx)
}

fn changed_slot<'a>(packet: &'a ClickSlotC2s<'_>, index: usize) -> Result<&'a SlotChange> {
    let Some(slot) = packet.slot_changes.get(index) else {
        valence_server::protocol::anyhow::bail!("missing slot change")
    };
    Ok(slot)
}
