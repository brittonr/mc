use std::borrow::Cow;
use std::time::Instant;

use super::*;
use valence_server::event_loop::PacketEvent;

/// A decoded `UpdateSelectedSlotC2s` packet from a live inventory client.
///
/// The adapter emits this event during [`EventLoopSet::TypedAdapters`] in
/// [`valence_server::event_loop::EventLoopPreUpdate`]. Consumers own hotbar
/// validation and mutation. Raw [`PacketEvent`] values remain available for
/// low-level packet users.
#[derive(Event, Copy, Clone, Debug)]
pub struct UpdateSelectedSlotPacketEvent {
    /// The live inventory client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded hotbar slot requested by the client.
    pub slot: u16,
}

/// A decoded `ClickSlotC2s` packet from a live inventory client.
///
/// The adapter emits this event during [`EventLoopSet::TypedAdapters`] in
/// [`valence_server::event_loop::EventLoopPreUpdate`]. Inventory validation,
/// resynchronization, and mutation remain owned by the inventory domain
/// consumer. Raw [`PacketEvent`] values remain available for low-level packet
/// users.
#[derive(Event, Clone, Debug)]
pub struct ClickSlotPacketEvent {
    /// The live inventory client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded click packet body with owned slot-change storage.
    pub packet: ClickSlotC2s<'static>,
}

/// A decoded `CreativeInventoryActionC2s` packet from a live inventory client.
///
/// The adapter emits this event during [`EventLoopSet::TypedAdapters`] in
/// [`valence_server::event_loop::EventLoopPreUpdate`]. Creative-mode policy and
/// inventory mutation remain owned by the inventory domain consumer. Raw
/// [`PacketEvent`] values remain available for low-level packet users.
#[derive(Event, Clone, Debug)]
pub struct CreativeInventoryActionPacketEvent {
    /// The live inventory client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded slot index carried by the packet.
    pub slot: i16,
    /// The decoded item stack carried by the packet.
    pub clicked_item: ItemStack,
}

/// A decoded `CloseHandledScreenC2s` packet from a live inventory client.
///
/// The adapter emits this event during [`EventLoopSet::TypedAdapters`] in
/// [`valence_server::event_loop::EventLoopPreUpdate`]. Inventory and fixture
/// consumers own cleanup of their own open-container components. Raw
/// [`PacketEvent`] values remain available for low-level packet users.
#[derive(Event, Copy, Clone, Debug)]
pub struct CloseHandledScreenEvent {
    /// The live inventory client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded window ID carried by the packet.
    pub window_id: i8,
}

pub(super) fn emit_update_selected_slot_packet_events(
    mut packets: EventReader<PacketEvent>,
    live_inventory_clients: Query<(), With<ClientInventoryState>>,
    mut events: EventWriter<UpdateSelectedSlotPacketEvent>,
) {
    for packet in packets.read() {
        if !live_inventory_clients.contains(packet.client) {
            continue;
        }
        if let Some(event) = update_selected_slot_packet_event_from_packet(packet) {
            events.send(event);
        }
    }
}

pub(super) fn emit_click_slot_packet_events(
    mut packets: EventReader<PacketEvent>,
    live_inventory_clients: Query<(), With<ClientInventoryState>>,
    mut events: EventWriter<ClickSlotPacketEvent>,
) {
    for packet in packets.read() {
        if !live_inventory_clients.contains(packet.client) {
            continue;
        }
        if let Some(event) = click_slot_packet_event_from_packet(packet) {
            events.send(event);
        }
    }
}

pub(super) fn emit_creative_inventory_action_packet_events(
    mut packets: EventReader<PacketEvent>,
    live_inventory_clients: Query<(), With<ClientInventoryState>>,
    mut events: EventWriter<CreativeInventoryActionPacketEvent>,
) {
    for packet in packets.read() {
        if !live_inventory_clients.contains(packet.client) {
            continue;
        }
        if let Some(event) = creative_inventory_action_packet_event_from_packet(packet) {
            events.send(event);
        }
    }
}

pub(super) fn emit_close_handled_screen_events(
    mut packets: EventReader<PacketEvent>,
    live_inventory_clients: Query<(), With<ClientInventoryState>>,
    mut events: EventWriter<CloseHandledScreenEvent>,
) {
    for packet in packets.read() {
        if !live_inventory_clients.contains(packet.client) {
            continue;
        }
        if let Some(event) = close_handled_screen_event_from_packet(packet) {
            events.send(event);
        }
    }
}

fn update_selected_slot_packet_event_from_packet(
    packet: &PacketEvent,
) -> Option<UpdateSelectedSlotPacketEvent> {
    let pkt = packet.decode::<UpdateSelectedSlotC2s>()?;
    Some(UpdateSelectedSlotPacketEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        slot: pkt.slot,
    })
}

fn click_slot_packet_event_from_packet(packet: &PacketEvent) -> Option<ClickSlotPacketEvent> {
    let pkt = packet.decode::<ClickSlotC2s>()?;
    Some(ClickSlotPacketEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        packet: own_click_slot_packet(pkt),
    })
}

fn creative_inventory_action_packet_event_from_packet(
    packet: &PacketEvent,
) -> Option<CreativeInventoryActionPacketEvent> {
    let pkt = packet.decode::<CreativeInventoryActionC2s>()?;
    Some(CreativeInventoryActionPacketEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        slot: pkt.slot,
        clicked_item: pkt.clicked_item,
    })
}

fn close_handled_screen_event_from_packet(packet: &PacketEvent) -> Option<CloseHandledScreenEvent> {
    let pkt = packet.decode::<CloseHandledScreenC2s>()?;
    Some(CloseHandledScreenEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        window_id: pkt.window_id,
    })
}

fn own_click_slot_packet(packet: ClickSlotC2s<'_>) -> ClickSlotC2s<'static> {
    ClickSlotC2s {
        window_id: packet.window_id,
        state_id: packet.state_id,
        slot_idx: packet.slot_idx,
        button: packet.button,
        mode: packet.mode,
        slot_changes: Cow::Owned(packet.slot_changes.into_owned()),
        carried_item: packet.carried_item,
    }
}

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use bevy_ecs::event::Events;
    use bevy_ecs::prelude::Resource;
    use valence_server::event_loop::{EventLoopPlugin, EventLoopPreUpdate, EventLoopSet};
    use valence_server::protocol::{Encode, Packet, VarInt};

    use super::*;

    const INITIAL_HOTBAR_INDEX: u8 = 0;
    const SELECTED_HOTBAR_INDEX: u8 = 2;
    const TEST_WINDOW_ID_U8: u8 = 1;
    const TEST_WINDOW_ID_I8: i8 = 1;
    const TEST_STATE_ID: i32 = 3;
    const TEST_CLICK_SLOT_ID: i16 = 4;
    const TEST_CLICK_BUTTON: i8 = 0;
    const INVALID_CLICK_MODE_TAG: i32 = 99;
    const WRONG_PACKET_ID_OFFSET: i32 = 1;
    const PARTIAL_DECODE_TRUNCATED_BYTES: usize = 1;
    const EXPECTED_SINGLE_EVENT_COUNT: usize = 1;

    #[derive(Resource, Default)]
    struct RawPacketObservation {
        count: usize,
    }

    #[test]
    fn adapter_emits_update_selected_slot_event_for_live_valid_packet() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(
                client,
                timestamp,
                UpdateSelectedSlotC2s::ID,
                valid_update_selected_slot_body(),
            ),
        );
        app.update();

        let packet_events = update_selected_slot_packet_events(&app);
        assert_eq!(packet_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let packet_event = packet_events[0];
        assert_eq!(packet_event.client, client);
        assert_eq!(packet_event.timestamp, timestamp);
        assert_eq!(packet_event.slot, u16::from(SELECTED_HOTBAR_INDEX));

        let public_events = update_selected_slot_events(&app);
        assert_eq!(public_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        assert_eq!(public_events[0].client, client);
        assert_eq!(public_events[0].slot, SELECTED_HOTBAR_INDEX);
        assert_eq!(held_hotbar_index(&app, client), SELECTED_HOTBAR_INDEX);
    }

    #[test]
    fn adapter_emits_close_event_and_domain_removes_open_inventory() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(
                client,
                timestamp,
                CloseHandledScreenC2s::ID,
                valid_close_handled_screen_body(),
            ),
        );
        app.update();

        let close_events = close_handled_screen_events(&app);
        assert_eq!(close_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        assert_eq!(close_events[0].client, client);
        assert_eq!(close_events[0].timestamp, timestamp);
        assert_eq!(close_events[0].window_id, TEST_WINDOW_ID_I8);
        assert!(app.world().entity(client).get::<OpenInventory>().is_none());
    }

    #[test]
    fn adapter_emits_click_slot_event_with_owned_packet_body() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(client, timestamp, ClickSlotC2s::ID, valid_click_slot_body()),
        );
        app.update();

        let click_events = click_slot_packet_events(&app);
        assert_eq!(click_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let event = &click_events[0];
        assert_eq!(event.client, client);
        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.packet.window_id, TEST_WINDOW_ID_U8);
        assert_eq!(event.packet.state_id.0, TEST_STATE_ID);
        assert_eq!(event.packet.slot_idx, TEST_CLICK_SLOT_ID);
        assert!(event.packet.slot_changes.is_empty());
    }

    #[test]
    fn adapter_emits_creative_inventory_action_event() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(
                client,
                timestamp,
                CreativeInventoryActionC2s::ID,
                valid_creative_inventory_action_body(),
            ),
        );
        app.update();

        let creative_events = creative_inventory_action_packet_events(&app);
        assert_eq!(creative_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let event = &creative_events[0];
        assert_eq!(event.client, client);
        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.slot, TEST_CLICK_SLOT_ID);
        assert_eq!(event.clicked_item, ItemStack::EMPTY);
    }

    #[test]
    fn raw_packet_observer_reads_before_inventory_typed_adapter() {
        let mut app = inventory_app();
        app.init_resource::<RawPacketObservation>().add_systems(
            EventLoopPreUpdate,
            count_raw_packets.in_set(EventLoopSet::RawPacketObservers),
        );
        let client = spawn_live_inventory_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                UpdateSelectedSlotC2s::ID,
                valid_update_selected_slot_body(),
            ),
        );
        app.update();

        assert_eq!(
            raw_packet_observation_count(&app),
            EXPECTED_SINGLE_EVENT_COUNT
        );
        assert_eq!(
            update_selected_slot_packet_events(&app).len(),
            EXPECTED_SINGLE_EVENT_COUNT
        );
    }

    #[test]
    fn duplicate_inventory_packet_events_are_detected() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                UpdateSelectedSlotC2s::ID,
                valid_update_selected_slot_body(),
            ),
        );
        app.update();

        let events = update_selected_slot_packet_events(&app);
        assert!(!has_duplicate_update_selected_slot_packet_event(&events));
        assert!(has_duplicate_update_selected_slot_packet_event(&[
            events[0], events[0]
        ]));
    }

    #[test]
    fn adapter_rejects_wrong_packet_id() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                UpdateSelectedSlotC2s::ID + WRONG_PACKET_ID_OFFSET,
                valid_update_selected_slot_body(),
            ),
        );
        app.update();

        assert_no_inventory_packet_events(&app);
        assert_eq!(held_hotbar_index(&app, client), INITIAL_HOTBAR_INDEX);
    }

    #[test]
    fn adapter_rejects_partial_decode() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                UpdateSelectedSlotC2s::ID,
                partial_update_selected_slot_body(),
            ),
        );
        app.update();

        assert_no_inventory_packet_events(&app);
        assert_eq!(held_hotbar_index(&app, client), INITIAL_HOTBAR_INDEX);
    }

    #[test]
    fn adapter_rejects_malformed_payload() {
        let mut app = inventory_app();
        let client = spawn_live_inventory_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                ClickSlotC2s::ID,
                malformed_click_slot_body(),
            ),
        );
        app.update();

        assert_no_inventory_packet_events(&app);
    }

    #[test]
    fn adapter_rejects_stale_client() {
        let mut app = inventory_app();
        let stale_client = spawn_live_inventory_client(&mut app);
        app.world_mut().despawn(stale_client);

        send_packet_event(
            &mut app,
            packet_event(
                stale_client,
                Instant::now(),
                UpdateSelectedSlotC2s::ID,
                valid_update_selected_slot_body(),
            ),
        );
        app.update();

        assert_no_inventory_packet_events(&app);
    }

    fn inventory_app() -> App {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin)
            .add_event::<InteractBlockEvent>()
            .add_plugins(InventoryPlugin);
        app
    }

    fn spawn_live_inventory_client(app: &mut App) -> Entity {
        let open_inventory = app
            .world_mut()
            .spawn(Inventory::new(InventoryKind::Generic9x3))
            .id();
        app.world_mut()
            .spawn((
                ClientInventoryState {
                    window_id: TEST_WINDOW_ID_U8,
                    state_id: Wrapping(TEST_STATE_ID),
                    slots_changed: 0,
                    client_updated_cursor_item: None,
                },
                CursorItem(ItemStack::EMPTY),
                HeldItem {
                    held_item_slot: PlayerInventory::hotbar_to_slot(INITIAL_HOTBAR_INDEX),
                },
                Inventory::new(InventoryKind::Player),
                OpenInventory::new(open_inventory),
            ))
            .id()
    }

    fn send_packet_event(app: &mut App, event: PacketEvent) {
        app.world_mut()
            .resource_mut::<Events<PacketEvent>>()
            .send(event);
    }

    fn packet_event(client: Entity, timestamp: Instant, id: i32, data: Vec<u8>) -> PacketEvent {
        PacketEvent {
            client,
            timestamp,
            id,
            data: data.into(),
        }
    }

    fn valid_update_selected_slot_body() -> Vec<u8> {
        encoded_body(&UpdateSelectedSlotC2s {
            slot: u16::from(SELECTED_HOTBAR_INDEX),
        })
    }

    fn partial_update_selected_slot_body() -> Vec<u8> {
        let mut body = valid_update_selected_slot_body();
        let remaining_len = body.len() - PARTIAL_DECODE_TRUNCATED_BYTES;
        body.truncate(remaining_len);
        body
    }

    fn valid_close_handled_screen_body() -> Vec<u8> {
        encoded_body(&CloseHandledScreenC2s {
            window_id: TEST_WINDOW_ID_I8,
        })
    }

    fn valid_click_slot_body() -> Vec<u8> {
        encoded_body(&ClickSlotC2s {
            window_id: TEST_WINDOW_ID_U8,
            state_id: VarInt(TEST_STATE_ID),
            slot_idx: TEST_CLICK_SLOT_ID,
            button: TEST_CLICK_BUTTON,
            mode: ClickMode::Click,
            slot_changes: Cow::Borrowed(&[]),
            carried_item: ItemStack::EMPTY,
        })
    }

    fn valid_creative_inventory_action_body() -> Vec<u8> {
        encoded_body(&CreativeInventoryActionC2s {
            slot: TEST_CLICK_SLOT_ID,
            clicked_item: ItemStack::EMPTY,
        })
    }

    fn malformed_click_slot_body() -> Vec<u8> {
        let mut body = Vec::new();
        TEST_WINDOW_ID_U8.encode(&mut body).unwrap();
        VarInt(TEST_STATE_ID).encode(&mut body).unwrap();
        TEST_CLICK_SLOT_ID.encode(&mut body).unwrap();
        TEST_CLICK_BUTTON.encode(&mut body).unwrap();
        VarInt(INVALID_CLICK_MODE_TAG).encode(&mut body).unwrap();
        body
    }

    fn encoded_body<P>(packet: &P) -> Vec<u8>
    where
        P: Encode,
    {
        let mut body = Vec::new();
        packet.encode(&mut body).unwrap();
        body
    }

    fn count_raw_packets(
        mut packets: EventReader<PacketEvent>,
        mut observation: ResMut<RawPacketObservation>,
    ) {
        observation.count += packets.read().count();
    }

    fn raw_packet_observation_count(app: &App) -> usize {
        app.world().resource::<RawPacketObservation>().count
    }

    fn update_selected_slot_packet_events(app: &App) -> Vec<UpdateSelectedSlotPacketEvent> {
        app.world()
            .resource::<Events<UpdateSelectedSlotPacketEvent>>()
            .iter_current_update_events()
            .copied()
            .collect()
    }

    fn click_slot_packet_events(app: &App) -> Vec<ClickSlotPacketEvent> {
        app.world()
            .resource::<Events<ClickSlotPacketEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    fn creative_inventory_action_packet_events(
        app: &App,
    ) -> Vec<CreativeInventoryActionPacketEvent> {
        app.world()
            .resource::<Events<CreativeInventoryActionPacketEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    fn close_handled_screen_events(app: &App) -> Vec<CloseHandledScreenEvent> {
        app.world()
            .resource::<Events<CloseHandledScreenEvent>>()
            .iter_current_update_events()
            .copied()
            .collect()
    }

    fn update_selected_slot_events(app: &App) -> Vec<UpdateSelectedSlotEvent> {
        app.world()
            .resource::<Events<UpdateSelectedSlotEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    fn assert_no_inventory_packet_events(app: &App) {
        assert!(update_selected_slot_packet_events(app).is_empty());
        assert!(click_slot_packet_events(app).is_empty());
        assert!(creative_inventory_action_packet_events(app).is_empty());
        assert!(close_handled_screen_events(app).is_empty());
    }

    fn has_duplicate_update_selected_slot_packet_event(
        events: &[UpdateSelectedSlotPacketEvent],
    ) -> bool {
        let mut unique_events = Vec::new();
        for event in events {
            if unique_events
                .iter()
                .any(|candidate: &UpdateSelectedSlotPacketEvent| {
                    candidate.client == event.client
                        && candidate.timestamp == event.timestamp
                        && candidate.slot == event.slot
                })
            {
                return true;
            }
            unique_events.push(*event);
        }

        false
    }

    fn held_hotbar_index(app: &App, client: Entity) -> u8 {
        app.world()
            .entity(client)
            .get::<HeldItem>()
            .unwrap()
            .hotbar_idx()
    }
}
