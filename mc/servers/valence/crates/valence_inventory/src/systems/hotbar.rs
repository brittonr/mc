use super::*;

#[derive(Event, Clone, Debug)]
pub struct UpdateSelectedSlotEvent {
    pub client: Entity,
    pub slot: u8,
}

/// Handles the `HeldItem` component being changed on a client entity, which
/// indicates that the server has changed the selected hotbar slot.
pub(super) fn update_player_selected_slot(
    mut clients: Query<(&mut Client, &HeldItem), Changed<HeldItem>>,
) {
    for (mut client, held_item) in &mut clients {
        client.write_packet(&UpdateSelectedSlotS2c {
            slot: held_item.hotbar_idx(),
        });
    }
}

/// Client to Server `HeldItem` Slot
pub(super) fn handle_update_selected_slot(
    mut packet_events: EventReader<UpdateSelectedSlotPacketEvent>,
    mut clients: Query<&mut HeldItem>,
    mut events: EventWriter<UpdateSelectedSlotEvent>,
) {
    for packet_event in packet_events.read() {
        let Ok(mut mut_held) = clients.get_mut(packet_event.client) else {
            continue;
        };
        let held = mut_held.bypass_change_detection();
        if packet_event.slot > PlayerInventory::HOTBAR_INDEX_MAX {
            continue;
        }

        let Ok(slot) = u8::try_from(packet_event.slot) else {
            continue;
        };
        held.set_hotbar_idx(slot);
        events.send(UpdateSelectedSlotEvent {
            client: packet_event.client,
            slot,
        });
    }
}
