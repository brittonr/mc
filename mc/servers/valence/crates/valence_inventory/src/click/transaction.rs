use super::*;

pub(super) struct ClickTransactionInput<'a, 'p> {
    pub(super) packet: &'a ClickSlotC2s<'p>,
    pub(super) client: ClickTransactionClient<'a>,
}

pub(super) enum ClickTransactionClient<'a> {
    Missing,
    Present {
        client_inventory: &'a Inventory,
        open_window: OpenWindowSummary<'a>,
        cursor_item: &'a CursorItem,
    },
}

#[derive(Clone, Copy, Debug)]
pub(super) enum OpenWindowSummary<'a> {
    Closed,
    Open(&'a Inventory),
    Missing,
}

#[derive(Clone, Debug)]
pub(super) enum ClickTransactionDecision {
    Ignore(ClickIgnoreReason),
    ResyncInvalid(ResyncInvalidPlan),
    DropCursor(DropCursorPlan),
    DropKey(DropKeyPlan),
    Regular(RegularClickPlan),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ClickIgnoreReason {
    MissingClient,
    MissingOpenInventory,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ResyncInvalidPlan {
    pub(super) reason: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct DropCursorPlan {
    pub(super) stack: ItemStack,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DropKeyPlan;

#[derive(Clone, Debug)]
pub(super) struct RegularClickPlan {
    pub(super) event: ClickSlotEventPlan,
}

#[derive(Clone, Debug)]
pub(super) struct ClickSlotEventPlan {
    pub(super) window_id: u8,
    pub(super) state_id: i32,
    pub(super) slot_id: i16,
    pub(super) button: i8,
    pub(super) mode: ClickMode,
    pub(super) slot_changes: Vec<SlotChange>,
    pub(super) carried_item: ItemStack,
}

pub(super) fn plan_click_transaction(
    input: ClickTransactionInput<'_, '_>,
) -> ClickTransactionDecision {
    let ClickTransactionInput { packet, client } = input;
    let ClickTransactionClient::Present {
        client_inventory,
        open_window,
        cursor_item,
    } = client
    else {
        return ClickTransactionDecision::Ignore(ClickIgnoreReason::MissingClient);
    };

    if let Err(error) = validate::check_packet(
        packet,
        client_inventory,
        open_window.validation_inventory(),
        cursor_item,
    ) {
        return ClickTransactionDecision::ResyncInvalid(ResyncInvalidPlan {
            reason: error.to_string(),
        });
    }

    if packet.slot_idx == OUTSIDE_WINDOW_SLOT && packet.mode == ClickMode::Click {
        return ClickTransactionDecision::DropCursor(DropCursorPlan {
            stack: cursor_item.0.clone(),
        });
    }

    if open_window.is_missing() {
        return ClickTransactionDecision::Ignore(ClickIgnoreReason::MissingOpenInventory);
    }

    if packet.mode == ClickMode::DropKey {
        return ClickTransactionDecision::DropKey(DropKeyPlan);
    }

    ClickTransactionDecision::Regular(RegularClickPlan {
        event: click_slot_event_plan(packet),
    })
}

impl<'a> OpenWindowSummary<'a> {
    fn validation_inventory(self) -> Option<&'a Inventory> {
        match self {
            OpenWindowSummary::Closed | OpenWindowSummary::Missing => None,
            OpenWindowSummary::Open(inventory) => Some(inventory),
        }
    }

    fn is_missing(self) -> bool {
        matches!(self, OpenWindowSummary::Missing)
    }
}

fn click_slot_event_plan(packet: &ClickSlotC2s<'_>) -> ClickSlotEventPlan {
    ClickSlotEventPlan {
        window_id: packet.window_id,
        state_id: packet.state_id.0,
        slot_id: packet.slot_idx,
        button: packet.button,
        mode: packet.mode,
        slot_changes: packet.slot_changes.iter().cloned().collect(),
        carried_item: packet.carried_item.clone(),
    }
}

#[cfg(test)]
mod tests {
    use valence_server::protocol::VarInt;

    use super::*;

    const CLOSED_WINDOW_ID: u8 = 0;
    const OPEN_WINDOW_ID: u8 = 1;
    const CURRENT_STATE_ID: i32 = 2;
    const LEFT_BUTTON: i8 = 0;
    const REGULAR_SLOT_IDX: i16 = 9;
    const REGULAR_SLOT_ID: u16 = 9;
    const OPEN_SLOT_IDX: i16 = 0;
    const OPEN_SLOT_ID: u16 = 0;
    const DROP_KEY_SLOT_IDX: i16 = 9;
    const DROP_KEY_SLOT_ID: u16 = 9;
    const UNSAFE_SLOT_IDX: i16 = 200;
    const FIRST_COUNT: i8 = 1;
    const SECOND_COUNT: i8 = 2;
    const THIRD_COUNT: i8 = 3;

    #[test]
    fn regular_player_click_plans_event_and_cursor_update() {
        let mut inventory = Inventory::new(InventoryKind::Player);
        let stack = stack(ItemKind::Diamond, THIRD_COUNT);
        inventory.set_slot(REGULAR_SLOT_ID, stack.clone());
        let cursor_item = CursorItem::default();
        let packet = regular_player_pickup_packet(stack.clone());

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let event = assert_regular(decision);
        assert_eq!(event.window_id, CLOSED_WINDOW_ID);
        assert_eq!(event.state_id, CURRENT_STATE_ID);
        assert_eq!(event.slot_id, REGULAR_SLOT_IDX);
        assert_eq!(event.button, LEFT_BUTTON);
        assert_eq!(event.mode, ClickMode::Click);
        assert_eq!(event.carried_item, stack);
        assert_eq!(event.slot_changes.len(), usize::from(FIRST_COUNT as u8));
    }

    #[test]
    fn outside_window_click_plans_cursor_drop() {
        let inventory = Inventory::new(InventoryKind::Player);
        let cursor_stack = stack(ItemKind::Diamond, THIRD_COUNT);
        let cursor_item = CursorItem(cursor_stack.clone());
        let packet = outside_window_drop_packet();

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let plan = assert_drop_cursor(decision);
        assert_eq!(plan.stack, cursor_stack);
    }

    #[test]
    fn drop_key_click_plans_drop_key_shell_path() {
        let mut inventory = Inventory::new(InventoryKind::Player);
        inventory.set_slot(DROP_KEY_SLOT_ID, stack(ItemKind::Diamond, SECOND_COUNT));
        let cursor_item = CursorItem::default();
        let packet = drop_key_packet();

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        assert!(matches!(
            decision,
            ClickTransactionDecision::DropKey(DropKeyPlan)
        ));
    }

    #[test]
    fn open_inventory_click_plans_open_window_event() {
        let client_inventory = Inventory::new(InventoryKind::Player);
        let mut open_inventory = Inventory::new(InventoryKind::Generic9x1);
        let stack = stack(ItemKind::Emerald, SECOND_COUNT);
        open_inventory.set_slot(OPEN_SLOT_ID, stack.clone());
        let cursor_item = CursorItem::default();
        let packet = open_inventory_pickup_packet(stack.clone());

        let decision = plan_for_present_client(
            &packet,
            &client_inventory,
            OpenWindowSummary::Open(&open_inventory),
            &cursor_item,
        );

        let event = assert_regular(decision);
        assert_eq!(event.window_id, OPEN_WINDOW_ID);
        assert_eq!(event.slot_id, OPEN_SLOT_IDX);
        assert_eq!(event.carried_item, stack);
        assert_eq!(
            event.slot_changes[usize::from(OPEN_SLOT_ID)].stack,
            ItemStack::EMPTY
        );
    }

    #[test]
    fn invalid_packet_plans_resync() {
        let inventory = Inventory::new(InventoryKind::Player);
        let cursor_item = CursorItem::default();
        let mut packet = empty_click_packet();
        packet.window_id = OPEN_WINDOW_ID;

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let plan = assert_resync_invalid(decision);
        assert!(plan.reason.contains("window id"));
    }

    #[test]
    fn unsafe_slot_indices_plan_resync() {
        let inventory = Inventory::new(InventoryKind::Player);
        let cursor_item = CursorItem::default();
        let mut packet = empty_click_packet();
        packet.slot_idx = UNSAFE_SLOT_IDX;

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let plan = assert_resync_invalid(decision);
        assert!(plan.reason.contains("slot"));
    }

    #[test]
    fn malformed_slot_changes_plan_resync() {
        let mut inventory = Inventory::new(InventoryKind::Player);
        inventory.set_slot(REGULAR_SLOT_ID, stack(ItemKind::Diamond, THIRD_COUNT));
        let cursor_item = CursorItem::default();
        let packet = malformed_regular_packet();

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let plan = assert_resync_invalid(decision);
        assert!(plan.reason.contains("click must modify one slot"));
    }

    #[test]
    fn missing_client_is_ignored() {
        let packet = empty_click_packet();

        let decision = plan_click_transaction(ClickTransactionInput {
            packet: &packet,
            client: ClickTransactionClient::Missing,
        });

        assert!(matches!(
            decision,
            ClickTransactionDecision::Ignore(ClickIgnoreReason::MissingClient)
        ));
    }

    #[test]
    fn missing_open_inventory_is_ignored_after_validation() {
        let mut inventory = Inventory::new(InventoryKind::Player);
        inventory.set_slot(REGULAR_SLOT_ID, stack(ItemKind::Diamond, THIRD_COUNT));
        let cursor_item = CursorItem::default();
        let packet = regular_player_pickup_packet(stack(ItemKind::Diamond, THIRD_COUNT));

        let decision = plan_for_present_client(
            &packet,
            &inventory,
            OpenWindowSummary::Missing,
            &cursor_item,
        );

        assert!(matches!(
            decision,
            ClickTransactionDecision::Ignore(ClickIgnoreReason::MissingOpenInventory)
        ));
    }

    #[test]
    fn invalid_cursor_state_plans_resync() {
        let inventory = Inventory::new(InventoryKind::Player);
        let cursor_item = CursorItem(stack(ItemKind::Diamond, SECOND_COUNT));
        let packet = impossible_cursor_growth_packet();

        let decision =
            plan_for_present_client(&packet, &inventory, OpenWindowSummary::Closed, &cursor_item);

        let plan = assert_resync_invalid(decision);
        assert!(plan.reason.contains("swapped items"));
    }

    fn plan_for_present_client(
        packet: &ClickSlotC2s<'_>,
        client_inventory: &Inventory,
        open_window: OpenWindowSummary<'_>,
        cursor_item: &CursorItem,
    ) -> ClickTransactionDecision {
        plan_click_transaction(ClickTransactionInput {
            packet,
            client: ClickTransactionClient::Present {
                client_inventory,
                open_window,
                cursor_item,
            },
        })
    }

    fn assert_regular(decision: ClickTransactionDecision) -> ClickSlotEventPlan {
        let ClickTransactionDecision::Regular(plan) = decision else {
            panic!("expected regular click plan, got {decision:?}");
        };
        plan.event
    }

    fn assert_drop_cursor(decision: ClickTransactionDecision) -> DropCursorPlan {
        let ClickTransactionDecision::DropCursor(plan) = decision else {
            panic!("expected cursor-drop plan, got {decision:?}");
        };
        plan
    }

    fn assert_resync_invalid(decision: ClickTransactionDecision) -> ResyncInvalidPlan {
        let ClickTransactionDecision::ResyncInvalid(plan) = decision else {
            panic!("expected invalid-resync plan, got {decision:?}");
        };
        plan
    }

    fn regular_player_pickup_packet(stack: ItemStack) -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: REGULAR_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: vec![SlotChange {
                idx: REGULAR_SLOT_IDX,
                stack: ItemStack::EMPTY,
            }]
            .into(),
            carried_item: stack,
        }
    }

    fn open_inventory_pickup_packet(stack: ItemStack) -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: OPEN_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: OPEN_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: vec![SlotChange {
                idx: OPEN_SLOT_IDX,
                stack: ItemStack::EMPTY,
            }]
            .into(),
            carried_item: stack,
        }
    }

    fn outside_window_drop_packet() -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: OUTSIDE_WINDOW_SLOT,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: Vec::new().into(),
            carried_item: ItemStack::EMPTY,
        }
    }

    fn drop_key_packet() -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: DROP_KEY_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::DropKey,
            slot_changes: vec![SlotChange {
                idx: DROP_KEY_SLOT_IDX,
                stack: stack(ItemKind::Diamond, FIRST_COUNT),
            }]
            .into(),
            carried_item: ItemStack::EMPTY,
        }
    }

    fn empty_click_packet() -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: REGULAR_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: Vec::new().into(),
            carried_item: ItemStack::EMPTY,
        }
    }

    fn malformed_regular_packet() -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: REGULAR_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: vec![
                SlotChange {
                    idx: REGULAR_SLOT_IDX,
                    stack: ItemStack::EMPTY,
                },
                SlotChange {
                    idx: REGULAR_SLOT_IDX + 1,
                    stack: ItemStack::EMPTY,
                },
            ]
            .into(),
            carried_item: stack(ItemKind::Diamond, THIRD_COUNT),
        }
    }

    fn impossible_cursor_growth_packet() -> ClickSlotC2s<'static> {
        ClickSlotC2s {
            window_id: CLOSED_WINDOW_ID,
            state_id: VarInt(CURRENT_STATE_ID),
            slot_idx: REGULAR_SLOT_IDX,
            button: LEFT_BUTTON,
            mode: ClickMode::Click,
            slot_changes: vec![SlotChange {
                idx: REGULAR_SLOT_IDX,
                stack: stack(ItemKind::Diamond, THIRD_COUNT),
            }]
            .into(),
            carried_item: ItemStack::EMPTY,
        }
    }

    fn stack(item: ItemKind, count: i8) -> ItemStack {
        ItemStack::new(item, count, None)
    }
}
