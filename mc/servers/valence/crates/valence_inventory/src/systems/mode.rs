use super::*;

const CREATIVE_DROP_SLOT: i16 = -1;

#[derive(Event, Clone, Debug)]
pub struct CreativeInventoryActionEvent {
    pub client: Entity,
    pub slot: i16,
    pub clicked_item: ItemStack,
}

pub(super) fn handle_creative_inventory_action(
    mut packet_events: EventReader<CreativeInventoryActionPacketEvent>,
    mut clients: Query<(
        &mut Client,
        &mut Inventory,
        &mut ClientInventoryState,
        &valence_server::GameMode,
    )>,
    mut inv_action_events: EventWriter<CreativeInventoryActionEvent>,
    mut drop_item_stack_events: EventWriter<DropItemStackEvent>,
) {
    for packet_event in packet_events.read() {
        let Ok((mut client, mut inventory, mut inv_state, game_mode)) =
            clients.get_mut(packet_event.client)
        else {
            continue;
        };

        if *game_mode != valence_server::GameMode::Creative {
            continue;
        }
        if packet_event.slot == CREATIVE_DROP_SLOT {
            drop_creative_stack(
                packet_event.client,
                &packet_event.clicked_item,
                &mut drop_item_stack_events,
            );
            continue;
        }

        let Some(slot_id) = slot_idx_to_u16(packet_event.slot) else {
            continue;
        };
        if slot_id >= inventory.slot_count() {
            continue;
        }

        inventory.slots[usize::from(slot_id)] = packet_event.clicked_item.clone();
        inv_state.state_id += 1;
        client.write_packet(&ScreenHandlerSlotUpdateS2c {
            window_id: 0,
            state_id: valence_server::protocol::VarInt(inv_state.state_id.0),
            slot_idx: packet_event.slot,
            slot_data: std::borrow::Cow::Borrowed(&packet_event.clicked_item),
        });
        inv_action_events.send(CreativeInventoryActionEvent {
            client: packet_event.client,
            slot: packet_event.slot,
            clicked_item: packet_event.clicked_item.clone(),
        });
    }
}

fn drop_creative_stack(
    client: Entity,
    clicked_item: &ItemStack,
    drop_item_stack_events: &mut EventWriter<DropItemStackEvent>,
) {
    let stack = clicked_item.clone();
    if stack.is_empty() {
        return;
    }
    drop_item_stack_events.send(DropItemStackEvent {
        client,
        from_slot: None,
        stack,
    });
}
