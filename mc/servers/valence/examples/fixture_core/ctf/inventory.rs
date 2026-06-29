#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryProbeItem {
    ExpectedStackItem,
    Other,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryClickMode {
    Click,
    Drag,
    Other,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryItemStack {
    pub item: InventoryProbeItem,
    pub count: i8,
}

impl InventoryItemStack {
    pub fn empty(empty_count: i8) -> Self {
        Self {
            item: InventoryProbeItem::Empty,
            count: empty_count,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventorySlotChange {
    pub slot: i16,
    pub stack: InventoryItemStack,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InventoryClickSnapshot {
    pub actor_matches: bool,
    pub window_id: u8,
    pub slot_id: i16,
    pub button: i8,
    pub mode: InventoryClickMode,
    pub carried_item: InventoryItemStack,
    pub slot_changes: Vec<InventorySlotChange>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryStackContract {
    pub window_id: u8,
    pub source_slot: i16,
    pub destination_slot: i16,
    pub full_count: i8,
    pub half_count: i8,
    pub empty_count: i8,
    pub left_button: i8,
    pub right_button: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InventoryStackState {
    pub split_pickup_seen: bool,
    pub split_place_seen: bool,
    pub merge_pickup_seen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryStackAction {
    SplitPickup,
    SplitPlace,
    MergePickup,
    MergePlace,
}

pub fn classify_inventory_stack_split_merge_event(
    event: &InventoryClickSnapshot,
    state: InventoryStackState,
    contract: InventoryStackContract,
) -> Option<InventoryStackAction> {
    if !event.actor_matches || event.window_id != contract.window_id {
        return None;
    }
    if event.mode != InventoryClickMode::Click {
        return None;
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.source_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.half_count,
    ) && event.slot_id == contract.source_slot
        && event.button == contract.right_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.half_count
        && !state.split_pickup_seen
    {
        return Some(InventoryStackAction::SplitPickup);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.destination_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.half_count,
    ) && event.slot_id == contract.destination_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && state.split_pickup_seen
        && !state.split_place_seen
    {
        return Some(InventoryStackAction::SplitPlace);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.destination_slot,
        InventoryProbeItem::Empty,
        contract.empty_count,
    ) && event.slot_id == contract.destination_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.half_count
        && state.split_place_seen
        && !state.merge_pickup_seen
    {
        return Some(InventoryStackAction::MergePickup);
    }
    if inventory_stack_slot_change_matches(
        event,
        contract.source_slot,
        InventoryProbeItem::ExpectedStackItem,
        contract.full_count,
    ) && event.slot_id == contract.source_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && state.merge_pickup_seen
    {
        return Some(InventoryStackAction::MergePlace);
    }
    None
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryDragContract {
    pub window_id: u8,
    pub source_slot: i16,
    pub target_slot_a: i16,
    pub target_slot_b: i16,
    pub outside_slot: i16,
    pub full_count: i8,
    pub half_count: i8,
    pub empty_count: i8,
    pub left_button: i8,
    pub drag_start_button: i8,
    pub drag_add_slot_button: i8,
    pub drag_end_button: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InventoryDragState {
    pub pickup_seen: bool,
    pub drag_start_seen: bool,
    pub target_a_seen: bool,
    pub target_b_seen: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InventoryDragAction {
    PickupSource,
    DragStart,
    AddTargetA,
    AddTargetB,
    DragEnd,
}

pub fn classify_inventory_drag_transactions_event(
    event: &InventoryClickSnapshot,
    state: InventoryDragState,
    contract: InventoryDragContract,
) -> Option<InventoryDragAction> {
    if !event.actor_matches || event.window_id != contract.window_id {
        return None;
    }
    if event.mode == InventoryClickMode::Click
        && event.slot_id == contract.source_slot
        && event.button == contract.left_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && inventory_stack_slot_change_matches(
            event,
            contract.source_slot,
            InventoryProbeItem::Empty,
            contract.empty_count,
        )
        && !state.pickup_seen
    {
        return Some(InventoryDragAction::PickupSource);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.outside_slot
        && event.button == contract.drag_start_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.pickup_seen
        && !state.drag_start_seen
    {
        return Some(InventoryDragAction::DragStart);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.target_slot_a
        && event.button == contract.drag_add_slot_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.drag_start_seen
        && !state.target_a_seen
    {
        return Some(InventoryDragAction::AddTargetA);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.target_slot_b
        && event.button == contract.drag_add_slot_button
        && event.carried_item.item == InventoryProbeItem::ExpectedStackItem
        && event.carried_item.count == contract.full_count
        && event.slot_changes.is_empty()
        && state.target_a_seen
        && !state.target_b_seen
    {
        return Some(InventoryDragAction::AddTargetB);
    }
    if event.mode == InventoryClickMode::Drag
        && event.slot_id == contract.outside_slot
        && event.button == contract.drag_end_button
        && event.carried_item.item == InventoryProbeItem::Empty
        && event.carried_item.count == contract.empty_count
        && inventory_stack_slot_change_matches(
            event,
            contract.target_slot_a,
            InventoryProbeItem::ExpectedStackItem,
            contract.half_count,
        )
        && inventory_stack_slot_change_matches(
            event,
            contract.target_slot_b,
            InventoryProbeItem::ExpectedStackItem,
            contract.half_count,
        )
        && state.target_b_seen
    {
        return Some(InventoryDragAction::DragEnd);
    }
    None
}

fn inventory_stack_slot_change_matches(
    event: &InventoryClickSnapshot,
    slot: i16,
    item: InventoryProbeItem,
    count: i8,
) -> bool {
    event.slot_changes.iter().any(|change| {
        change.slot == slot && change.stack.item == item && change.stack.count == count
    })
}
