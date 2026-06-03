use super::*;

const FIREWORK_OFFHAND_SLOT: u16 = 45;

/// This component will broadcast item interactions (e.g. drawing a bow, eating
/// food) to other players using `LivingFlags::set_using_item`.
// API compatibility: keep the established public equipment broadcast component name.
#[allow(unknown_lints)]
#[allow(path_segment_repetition)]
#[derive(Debug, Default, Clone, Component)]
pub struct EquipmentInteractionBroadcast;

// Sets flag to true when the client starts interacting with an
// item.
pub(crate) fn start(
    mut clients: Query<
        (
            &valence_inventory::Inventory,
            &valence_inventory::HeldItem,
            &mut valence_server::entity::living::LivingFlags,
        ),
        (With<Client>, With<EquipmentInteractionBroadcast>),
    >,
    mut events: EventReader<valence_server::interact_item::InteractItemEvent>,
) {
    for event in events.read() {
        if let Ok((inv, held_item, mut flags)) = clients.get_mut(event.client) {
            let item = inv.slot(held_item.slot()).item;
            let has_arrows = inv
                .first_slot_with_item(valence_server::ItemKind::Arrow, i8::MAX)
                .is_some();
            if (item == valence_server::ItemKind::Bow && !has_arrows)
                || (item == valence_server::ItemKind::Crossbow
                    && !has_arrows
                    && inv.slot(FIREWORK_OFFHAND_SLOT).item
                        != valence_server::ItemKind::FireworkRocket)
            {
                continue;
            }
            flags.set_using_item(true);
        }
    }
}

// Sets flag to false when the client stops interacting with an
// item.
pub(crate) fn stop(
    mut clients: Query<
        &mut valence_server::entity::living::LivingFlags,
        (With<Client>, With<EquipmentInteractionBroadcast>),
    >,
    mut packets: EventReader<valence_server::event_loop::PacketEvent>,
) {
    for packet in packets.read() {
        if let Some(pkt) =
            packet.decode::<valence_server::protocol::packets::play::PlayerActionC2s>()
        {
            if pkt.action == valence_inventory::PlayerAction::ReleaseUseItem {
                if let Ok(mut flags) = clients.get_mut(packet.client) {
                    flags.set_using_item(false);
                }
            }
        }
    }
}
