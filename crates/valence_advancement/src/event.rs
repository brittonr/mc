use bevy_ecs::prelude::*;

type Ident<T> = valence_server::Ident<T>;

/// This event sends when the client changes or closes advancement's tab.
// API compatibility: keep the established public advancement tab event name.
#[allow(unknown_lints)]
#[allow(path_segment_repetition)]
#[derive(Event, Clone, PartialEq, Eq, Debug)]
pub struct AdvancementTabChangeEvent {
    pub client: Entity,
    /// If None then the client has closed advancement's tabs.
    pub opened_tab: Option<Ident<String>>,
}

pub(crate) fn handle_advancement_tab_change(
    mut packets: EventReader<valence_server::event_loop::PacketEvent>,
    mut advancement_tab_change_events: EventWriter<AdvancementTabChangeEvent>,
) {
    for packet in packets.read() {
        if let Some(pkt) =
            packet.decode::<valence_server::protocol::packets::play::AdvancementTabC2s>()
        {
            advancement_tab_change_events.send(AdvancementTabChangeEvent {
                client: packet.client,
                opened_tab: match pkt {
                    valence_server::protocol::packets::play::AdvancementTabC2s::ClosedScreen => {
                        None
                    }
                    valence_server::protocol::packets::play::AdvancementTabC2s::OpenedTab {
                        tab_id,
                    } => Some(tab_id.into()),
                },
            });
        }
    }
}
