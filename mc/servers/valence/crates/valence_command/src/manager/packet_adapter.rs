use std::time::Instant;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Event;
use valence_server::event_loop::PacketEvent;
use valence_server::protocol::packets::play::CommandExecutionC2s;

/// A decoded `CommandExecutionC2s` packet from a live command client.
///
/// This event is emitted from [`valence_server::event_loop::EventLoopSet::TypedAdapters`]
/// during [`valence_server::EventLoopPreUpdate`] after the raw packet body and
/// source command client have been validated. Raw [`PacketEvent`] values remain
/// available for low-level packet users.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct CommandExecutionPacketEvent {
    /// The live command client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded command string supplied by the client.
    pub command: String,
}

pub(super) fn command_execution_packet_event_from_packet(
    packet: &PacketEvent,
) -> Option<CommandExecutionPacketEvent> {
    let pkt = packet.decode::<CommandExecutionC2s>()?;
    Some(plan_command_packet_event(
        packet.client,
        packet.timestamp,
        pkt.command.to_string(),
    ))
}

pub(super) fn plan_command_packet_event(
    client: Entity,
    timestamp: Instant,
    command: String,
) -> CommandExecutionPacketEvent {
    CommandExecutionPacketEvent {
        client,
        timestamp,
        command,
    }
}
