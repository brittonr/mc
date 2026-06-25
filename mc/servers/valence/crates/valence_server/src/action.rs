use std::time::Instant;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use derive_more::Deref;
use valence_protocol::packets::play::player_action_c2s::PlayerAction;
use valence_protocol::packets::play::{PlayerActionC2s, PlayerActionResponseS2c};
use valence_protocol::{BlockPos, Direction, VarInt, WritePacket};

use crate::client::{Client, UpdateClientsSet};
use crate::event_loop::{EventLoopPreUpdate, PacketEvent};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerActionEvent>()
            .add_event::<DiggingEvent>()
            .add_systems(
                EventLoopPreUpdate,
                (emit_player_action_events, handle_player_action).chain(),
            )
            .add_systems(
                PostUpdate,
                acknowledge_player_actions.in_set(UpdateClientsSet),
            );
    }
}

/// A validated `PlayerActionC2s` packet promoted into the event loop.
///
/// This event is emitted during [`EventLoopPreUpdate`] after the raw packet ID,
/// full packet body, and source client have been validated. Raw [`PacketEvent`]
/// values remain available for low-level systems that need unsupported packet
/// access.
#[derive(Event, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlayerActionEvent {
    /// The live client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded action requested by the client.
    pub action: PlayerAction,
    /// The block position carried by the packet.
    pub position: BlockPos,
    /// The face direction carried by the packet.
    pub direction: Direction,
    /// The synchronization sequence carried by the packet.
    pub sequence: i32,
}

#[derive(Event, Copy, Clone, Debug)]
pub struct DiggingEvent {
    pub client: Entity,
    pub position: BlockPos,
    pub direction: Direction,
    pub state: DiggingState,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DiggingState {
    Start,
    Abort,
    Stop,
}

#[derive(Component, Copy, Clone, PartialEq, Eq, Default, Debug, Deref)]
pub struct ActionSequence(i32);

impl ActionSequence {
    pub fn update(&mut self, val: i32) {
        self.0 = self.0.max(val);
    }

    pub fn get(&self) -> i32 {
        self.0
    }
}

fn emit_player_action_events(
    mut packets: EventReader<PacketEvent>,
    live_clients: Query<(), With<ActionSequence>>,
    mut player_action_events: EventWriter<PlayerActionEvent>,
) {
    for packet in packets.read() {
        if !live_clients.contains(packet.client) {
            continue;
        }

        if let Some(event) = player_action_event_from_packet(packet) {
            player_action_events.send(event);
        }
    }
}

fn handle_player_action(
    mut clients: Query<&mut ActionSequence>,
    mut player_action_events: EventReader<PlayerActionEvent>,
    mut digging_events: EventWriter<DiggingEvent>,
) {
    for event in player_action_events.read() {
        if let Ok(mut seq) = clients.get_mut(event.client) {
            seq.update(event.sequence);
        }

        // TODO: check that digging is happening within configurable distance to client.
        // TODO: check that blocks are being broken at the appropriate speeds.

        if let Some(digging_event) = digging_event_from_player_action(*event) {
            digging_events.send(digging_event);
        }
    }
}

fn player_action_event_from_packet(packet: &PacketEvent) -> Option<PlayerActionEvent> {
    let pkt = packet.decode::<PlayerActionC2s>()?;
    Some(PlayerActionEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        action: pkt.action,
        position: pkt.position,
        direction: pkt.direction,
        sequence: pkt.sequence.0,
    })
}

fn digging_event_from_player_action(event: PlayerActionEvent) -> Option<DiggingEvent> {
    let state = match event.action {
        PlayerAction::StartDestroyBlock => DiggingState::Start,
        PlayerAction::AbortDestroyBlock => DiggingState::Abort,
        PlayerAction::StopDestroyBlock => DiggingState::Stop,
        PlayerAction::DropAllItems
        | PlayerAction::DropItem
        | PlayerAction::ReleaseUseItem
        | PlayerAction::SwapItemWithOffhand => return None,
    };

    Some(DiggingEvent {
        client: event.client,
        position: event.position,
        direction: event.direction,
        state,
    })
}

fn acknowledge_player_actions(
    mut clients: Query<(&mut Client, &mut ActionSequence), Changed<ActionSequence>>,
) {
    for (mut client, mut action_seq) in &mut clients {
        if action_seq.0 != 0 {
            client.write_packet(&PlayerActionResponseS2c {
                sequence: VarInt(action_seq.0),
            });

            action_seq.0 = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bevy_ecs::event::Events;
    use bytes::Bytes;
    use valence_protocol::{Encode, Packet};

    use super::*;
    use crate::event_loop::EventLoopPlugin;

    const TEST_BLOCK_X: i32 = 11;
    const TEST_BLOCK_Y: i32 = 64;
    const TEST_BLOCK_Z: i32 = -5;
    const TEST_SEQUENCE: i32 = 7;
    const INVALID_PLAYER_ACTION_TAG: i32 = 99;
    const WRONG_PACKET_ID_OFFSET: i32 = 1;
    const PARTIAL_DECODE_TRUNCATED_BYTES: usize = 1;
    const EXPECTED_SINGLE_EVENT_COUNT: usize = 1;

    #[test]
    fn adapter_emits_player_action_event_for_live_valid_packet() {
        let mut app = action_app();
        let client = spawn_live_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(
                client,
                timestamp,
                PlayerActionC2s::ID,
                valid_player_action_body(),
            ),
        );
        app.update();

        let action_events = player_action_events(&app);
        assert_eq!(action_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let event = action_events[0];
        assert_eq!(event.client, client);
        assert_eq!(event.timestamp, timestamp);
        assert_eq!(event.action, PlayerAction::StartDestroyBlock);
        assert_eq!(event.position, test_block_position());
        assert_eq!(event.direction, Direction::Up);
        assert_eq!(event.sequence, TEST_SEQUENCE);
    }

    #[test]
    fn gameplay_consumes_typed_action_event_without_duplicate_digging() {
        let mut app = action_app();
        let client = spawn_live_client(&mut app);

        send_valid_player_action_packet(&mut app, client);
        app.update();

        let digging_events = digging_events(&app);
        assert_eq!(digging_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let event = digging_events[0];
        assert_eq!(event.client, client);
        assert_eq!(event.position, test_block_position());
        assert_eq!(event.direction, Direction::Up);
        assert_eq!(event.state, DiggingState::Start);
        assert_eq!(action_sequence(&app, client), TEST_SEQUENCE);
    }

    #[test]
    fn adapter_rejects_wrong_packet_id() {
        let mut app = action_app();
        let client = spawn_live_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                PlayerActionC2s::ID + WRONG_PACKET_ID_OFFSET,
                valid_player_action_body(),
            ),
        );
        app.update();

        assert_no_action_or_digging_events(&app);
        assert_eq!(action_sequence(&app, client), 0);
    }

    #[test]
    fn adapter_rejects_partial_decode() {
        let mut app = action_app();
        let client = spawn_live_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(client, Instant::now(), PlayerActionC2s::ID, partial_body()),
        );
        app.update();

        assert_no_action_or_digging_events(&app);
        assert_eq!(action_sequence(&app, client), 0);
    }

    #[test]
    fn adapter_rejects_malformed_payload() {
        let mut app = action_app();
        let client = spawn_live_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                PlayerActionC2s::ID,
                malformed_player_action_body(),
            ),
        );
        app.update();

        assert_no_action_or_digging_events(&app);
        assert_eq!(action_sequence(&app, client), 0);
    }

    #[test]
    fn adapter_rejects_stale_client() {
        let mut app = action_app();
        let stale_client = spawn_live_client(&mut app);
        app.world_mut().despawn(stale_client);

        send_valid_player_action_packet(&mut app, stale_client);
        app.update();

        assert_no_action_or_digging_events(&app);
    }

    fn action_app() -> App {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin).add_plugins(ActionPlugin);
        app
    }

    fn spawn_live_client(app: &mut App) -> Entity {
        app.world_mut().spawn((ActionSequence::default(),)).id()
    }

    fn send_valid_player_action_packet(app: &mut App, client: Entity) {
        send_packet_event(
            app,
            packet_event(
                client,
                Instant::now(),
                PlayerActionC2s::ID,
                valid_player_action_body(),
            ),
        );
    }

    fn send_packet_event(app: &mut App, event: PacketEvent) {
        app.world_mut()
            .resource_mut::<Events<PacketEvent>>()
            .send(event);
    }

    fn packet_event(client: Entity, timestamp: Instant, id: i32, data: Bytes) -> PacketEvent {
        PacketEvent {
            client,
            timestamp,
            id,
            data,
        }
    }

    fn valid_player_action_body() -> Bytes {
        encoded_body(&PlayerActionC2s {
            action: PlayerAction::StartDestroyBlock,
            position: test_block_position(),
            direction: Direction::Up,
            sequence: VarInt(TEST_SEQUENCE),
        })
    }

    fn partial_body() -> Bytes {
        let mut body = Vec::from(valid_player_action_body().as_ref());
        let remaining_len = body.len() - PARTIAL_DECODE_TRUNCATED_BYTES;
        body.truncate(remaining_len);
        Bytes::from(body)
    }

    fn malformed_player_action_body() -> Bytes {
        let mut body = Vec::new();
        VarInt(INVALID_PLAYER_ACTION_TAG).encode(&mut body).unwrap();
        test_block_position().encode(&mut body).unwrap();
        Direction::Up.encode(&mut body).unwrap();
        VarInt(TEST_SEQUENCE).encode(&mut body).unwrap();
        Bytes::from(body)
    }

    fn encoded_body(packet: &PlayerActionC2s) -> Bytes {
        let mut body = Vec::new();
        packet.encode(&mut body).unwrap();
        Bytes::from(body)
    }

    fn test_block_position() -> BlockPos {
        BlockPos::new(TEST_BLOCK_X, TEST_BLOCK_Y, TEST_BLOCK_Z)
    }

    fn player_action_events(app: &App) -> Vec<PlayerActionEvent> {
        app.world()
            .resource::<Events<PlayerActionEvent>>()
            .iter_current_update_events()
            .copied()
            .collect()
    }

    fn digging_events(app: &App) -> Vec<DiggingEvent> {
        app.world()
            .resource::<Events<DiggingEvent>>()
            .iter_current_update_events()
            .copied()
            .collect()
    }

    fn assert_no_action_or_digging_events(app: &App) {
        assert!(player_action_events(app).is_empty());
        assert!(digging_events(app).is_empty());
    }

    fn action_sequence(app: &App, client: Entity) -> i32 {
        app.world()
            .entity(client)
            .get::<ActionSequence>()
            .unwrap()
            .get()
    }
}
