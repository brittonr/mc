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
    mut packets: EventReader<PacketEvent>,
    mut clients: Query<&mut HeldItem>,
    mut events: EventWriter<UpdateSelectedSlotEvent>,
) {
    for packet in packets.read() {
        let Some(pkt) = packet.decode::<UpdateSelectedSlotC2s>() else {
            continue;
        };
        let Ok(mut mut_held) = clients.get_mut(packet.client) else {
            continue;
        };
        let held = mut_held.bypass_change_detection();
        if pkt.slot > PlayerInventory::HOTBAR_INDEX_MAX {
            continue;
        }

        let Ok(slot) = u8::try_from(pkt.slot) else {
            continue;
        };
        held.set_hotbar_idx(slot);
        events.send(UpdateSelectedSlotEvent {
            client: packet.client,
            slot,
        });
    }
}
