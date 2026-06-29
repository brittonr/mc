use super::*;

mod flow;
mod press;
mod transaction;

const OUTSIDE_WINDOW_SLOT: i16 = -999;

pub(super) fn handle_packets(
    mut packet_events: EventReader<ClickSlotPacketEvent>,
    mut clients: Query<(
        &mut Client,
        &mut Inventory,
        &mut ClientInventoryState,
        Option<&mut OpenInventory>,
        &mut CursorItem,
    )>,
    mut inventories: Query<&mut Inventory, Without<Client>>,
    mut drop_item_stack_events: EventWriter<DropItemStackEvent>,
    mut slot_events: EventWriter<ClickSlotEvent>,
) {
    for packet_event in packet_events.read() {
        handle_packet(
            packet_event.client,
            packet_event.packet.clone(),
            (
                &mut clients,
                &mut inventories,
                &mut drop_item_stack_events,
                &mut slot_events,
            ),
        );
    }
}

fn handle_packet(
    client_entity: Entity,
    pkt: ClickSlotC2s,
    state: (
        &mut Query<(
            &mut Client,
            &mut Inventory,
            &mut ClientInventoryState,
            Option<&mut OpenInventory>,
            &mut CursorItem,
        )>,
        &mut Query<&mut Inventory, Without<Client>>,
        &mut EventWriter<DropItemStackEvent>,
        &mut EventWriter<ClickSlotEvent>,
    ),
) {
    debug_assert!(pkt.slot_idx >= OUTSIDE_WINDOW_SLOT);
    debug_assert!(pkt.slot_changes.len() <= usize::from(u16::MAX));

    let (clients, inventories, drop_item_stack_events, slot_events) = state;
    let Ok((mut client, mut client_inv, mut inv_state, open_inventory, mut cursor_item)) =
        clients.get_mut(client_entity)
    else {
        let decision = transaction::plan_click_transaction(transaction::ClickTransactionInput {
            packet: &pkt,
            client: transaction::ClickTransactionClient::Missing,
        });
        debug_assert!(matches!(
            decision,
            transaction::ClickTransactionDecision::Ignore(
                transaction::ClickIgnoreReason::MissingClient
            )
        ));
        return;
    };

    let decision = {
        let open_inv = open_inventory
            .as_ref()
            .and_then(|open| inventories.get_mut(open.entity).ok());
        let open_window = match (open_inventory.is_some(), open_inv.as_ref()) {
            (false, _) => transaction::OpenWindowSummary::Closed,
            (true, Some(open_inv)) => transaction::OpenWindowSummary::Open(open_inv),
            (true, None) => transaction::OpenWindowSummary::Missing,
        };
        let decision = transaction::plan_click_transaction(transaction::ClickTransactionInput {
            packet: &pkt,
            client: transaction::ClickTransactionClient::Present {
                client_inventory: &client_inv,
                open_window,
                cursor_item: &cursor_item,
            },
        });
        if let transaction::ClickTransactionDecision::ResyncInvalid(plan) = decision {
            resync_invalid((
                &mut client,
                &inv_state,
                &cursor_item,
                client_entity,
                &pkt,
                open_inv,
                client_inv,
                plan.reason,
            ));
            return;
        }
        decision
    };

    match decision {
        transaction::ClickTransactionDecision::Ignore(_) => {}
        transaction::ClickTransactionDecision::ResyncInvalid(_) => unreachable!(),
        transaction::ClickTransactionDecision::DropCursor(plan) => {
            apply_drop_cursor_plan(
                &mut cursor_item,
                drop_item_stack_events,
                client_entity,
                plan,
            );
        }
        transaction::ClickTransactionDecision::DropKey(_) => {
            press::handle_key(
                &pkt,
                (
                    client_entity,
                    &mut client,
                    &mut client_inv,
                    &mut inv_state,
                    open_inventory,
                    inventories,
                    drop_item_stack_events,
                    &cursor_item,
                ),
            );
        }
        transaction::ClickTransactionDecision::Regular(plan) => {
            if flow::handle_regular(
                &pkt,
                flow::Ctx {
                    client: &mut client,
                    client_inv: &mut client_inv,
                    inv_state: &mut inv_state,
                    open_inventory,
                    cursor_item: &mut cursor_item,
                },
                inventories,
            ) {
                slot_events.send(event_from_plan(client_entity, plan.event));
            }
        }
    }
}

fn resync_invalid<E: std::fmt::Display>(
    input: (
        &mut Client,
        &ClientInventoryState,
        &CursorItem,
        Entity,
        &ClickSlotC2s,
        Option<bevy_ecs::change_detection::Mut<Inventory>>,
        bevy_ecs::change_detection::Mut<Inventory>,
        E,
    ),
) {
    let (client, inv_state, cursor_item, client_entity, pkt, open_inv, client_inv, error) = input;
    tracing::debug!(
        "failed to validate click slot packet for client {client_entity:#?}: \"{error:#}\" \
         {pkt:#?}"
    );
    let is_open_inventory = open_inv.is_some();
    client.write_packet(&InventoryS2c {
        window_id: if is_open_inventory {
            inv_state.window_id
        } else {
            0
        },
        state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
        slots: std::borrow::Cow::Borrowed(open_inv.unwrap_or(client_inv).slot_slice()),
        carried_item: std::borrow::Cow::Borrowed(&cursor_item.0),
    });
}

fn apply_drop_cursor_plan(
    cursor_item: &mut CursorItem,
    drop_item_stack_events: &mut EventWriter<DropItemStackEvent>,
    client: Entity,
    plan: transaction::DropCursorPlan,
) {
    let stack = std::mem::take(&mut cursor_item.0);
    debug_assert_eq!(stack, plan.stack);
    if stack.is_empty() {
        return;
    }
    drop_item_stack_events.send(DropItemStackEvent {
        client,
        from_slot: None,
        stack,
    });
}

fn resync_state_mismatch(
    client: &mut Client,
    inv_state: &mut ClientInventoryState,
    inv: &Inventory,
    expected_state_id: i32,
    cursor_item: &CursorItem,
) -> bool {
    if inv_state.state_id.0 == expected_state_id {
        return false;
    }

    tracing::debug!("Client state id mismatch, resyncing");
    inv_state.state_id += 1;
    write_inventory_packet(client, inv_state.window_id, inv_state, inv, cursor_item);
    true
}

fn resync_readonly_inventory(
    client: &mut Client,
    inv: &Inventory,
    inv_state: &ClientInventoryState,
    window_id: u8,
    cursor_item: &CursorItem,
) -> bool {
    if !inv.readonly {
        return false;
    }

    write_inventory_packet(client, window_id, inv_state, inv, cursor_item);
    true
}

fn write_inventory_packet(
    client: &mut Client,
    window_id: u8,
    inv_state: &ClientInventoryState,
    inv: &Inventory,
    cursor_item: &CursorItem,
) {
    client.write_packet(&InventoryS2c {
        window_id,
        state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
        slots: std::borrow::Cow::Borrowed(inv.slot_slice()),
        carried_item: std::borrow::Cow::Borrowed(&cursor_item.0),
    });
}

fn remove_clicked_stack(inv: &mut Inventory, slot_id: u16, is_entire_stack: bool) -> ItemStack {
    let stack = inv.slot(slot_id);
    if stack.is_empty() {
        return ItemStack::EMPTY;
    }
    if is_entire_stack || stack.count == 1 {
        return inv.replace_slot(slot_id, ItemStack::EMPTY);
    }

    let Some(new_count) = stack.count.checked_sub(1) else {
        unreachable!();
    };
    let stack = stack.clone().with_count(new_count);
    let mut old_slot = inv.replace_slot(slot_id, stack);
    old_slot.count = 1;
    old_slot
}

fn mark_changed_slot(changed: &mut u64, slot_id: u16) {
    let Some(mask) = 1_u64.checked_shl(u32::from(slot_id)) else {
        unreachable!();
    };
    *changed |= mask;
}

fn event_from_plan(client: Entity, plan: transaction::ClickSlotEventPlan) -> ClickSlotEvent {
    ClickSlotEvent {
        client,
        window_id: plan.window_id,
        state_id: plan.state_id,
        slot_id: plan.slot_id,
        button: plan.button,
        mode: plan.mode,
        slot_changes: plan.slot_changes,
        carried_item: plan.carried_item,
    }
}
