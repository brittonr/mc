use std::collections::HashMap;
use std::time::Instant;

use bevy_ecs::prelude::{
    Added, Changed, Commands, DetectChanges, EventReader, EventWriter, Mut, Or, Query, Res, With,
};
use tracing::{debug, info, trace, warn};
use valence_server::client::Client;
use valence_server::event_loop::PacketEvent;
use valence_server::protocol::WritePacket;

use crate::scopes::CommandScopes;
use crate::{CommandRegistry, CommandScopeRegistry, ModifierValue};

use super::event_plan::{
    plan_command_execution_event, plan_processed_events, CommandExecutionEvent,
    CommandProcessedEvent,
};
use super::packet_adapter::{
    command_execution_packet_event_from_packet, CommandExecutionPacketEvent,
};
use super::parse_core::{parse_command_request, CommandParsePlan, CommandParseRequest};
use super::tree_sync::{
    command_tree_update_decision, visible_command_tree_plan, CommandTreePlan,
    CommandTreeUpdateDecision,
};

pub(super) fn insert_scope_component(
    mut clients: Query<bevy_ecs::entity::Entity, Added<Client>>,
    mut commands: Commands,
) {
    for client in &mut clients {
        commands.entity(client).insert(CommandScopes::new());
    }
}

pub(super) fn emit_command_execution_packet_events(
    mut packets: EventReader<PacketEvent>,
    live_command_clients: Query<(), With<CommandScopes>>,
    mut event_writer: EventWriter<CommandExecutionPacketEvent>,
) {
    for packet in packets.read() {
        if !live_command_clients.contains(packet.client) {
            continue;
        }
        if let Some(event) = command_execution_packet_event_from_packet(packet) {
            event_writer.send(event);
        }
    }
}

pub(super) fn emit_command_execution_events(
    mut packet_events: EventReader<CommandExecutionPacketEvent>,
    mut event_writer: EventWriter<CommandExecutionEvent>,
) {
    for packet_event in packet_events.read() {
        event_writer.send(plan_command_execution_event(packet_event));
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn command_tree_update_with_client(
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    mut updated_clients: Query<
        (&mut Client, &CommandScopes),
        Or<(Added<Client>, Changed<CommandScopes>)>,
    >,
) {
    let mut updated_clients = updated_clients.iter_mut().collect::<Vec<_>>();
    update_client_command_tree(
        command_registry.as_ref(),
        scope_registry.as_ref(),
        &mut updated_clients,
    );
}

pub(super) fn update_command_tree(
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    mut clients: Query<(&mut Client, &CommandScopes)>,
) {
    if command_tree_update_decision(command_registry.is_changed())
        == CommandTreeUpdateDecision::Skip
    {
        return;
    }

    let mut clients = clients.iter_mut().collect::<Vec<_>>();
    update_client_command_tree(
        command_registry.as_ref(),
        scope_registry.as_ref(),
        &mut clients,
    );
}

fn update_client_command_tree(
    command_registry: &CommandRegistry,
    scope_registry: &CommandScopeRegistry,
    updated_clients: &mut [(Mut<Client>, &CommandScopes)],
) {
    for (ref mut client, client_scopes) in updated_clients {
        let time = Instant::now();

        match visible_command_tree_plan(&command_registry.graph, scope_registry, client_scopes) {
            CommandTreePlan::Send(packet) => {
                client.write_packet(&packet);
            }
            CommandTreePlan::NoVisibleRoot => {
                warn!(
                    "Client has no permissions to execute any commands so we sent them nothing. \
                     It is generally a bad idea to scope the root node of the command graph as it \
                     can cause undefined behavior. For example, if the player has permission to \
                     execute a command before you change the scope of the root node, the packet \
                     will not be sent to the client and so the client will still think they can \
                     execute the command."
                )
            }
        }

        debug!("command tree update took {:?}", time.elapsed());
    }
}

pub(super) fn parse_incoming_commands(
    mut event_reader: EventReader<CommandExecutionEvent>,
    mut event_writer: EventWriter<CommandProcessedEvent>,
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    entity_scopes: Query<&CommandScopes>,
) {
    for command_event in event_reader.read() {
        let executor = command_event.executor;
        let parse_plan = parse_command_request(CommandParseRequest {
            command: &command_event.command,
            registry: command_registry.as_ref(),
            scope_registry: scope_registry.as_ref(),
            executor_scopes: entity_scopes.get(executor).ok(),
        });
        let modifiers = apply_modifier_plans(&parse_plan, command_registry.as_ref());
        let processed_events = plan_processed_events(
            parse_plan.parsed_command(),
            executor,
            &modifiers,
            &parse_plan.executable_nodes,
        );

        for event in processed_events {
            trace!("executing node: {:?}", event.node);
            event_writer.send(event);
        }

        info!(
            "Command dispatched: /{} (debug logs for more data)",
            command_event.command
        );
        debug!("Command modifiers: {:?}", modifiers);
    }
}

fn apply_modifier_plans(
    parse_plan: &CommandParsePlan,
    command_registry: &CommandRegistry,
) -> HashMap<ModifierValue, ModifierValue> {
    let mut modifiers = HashMap::new();

    for modifier_plan in &parse_plan.modifier_plans {
        let modifier = command_registry
            .modifiers
            .get(&modifier_plan.node)
            .expect("modifier plan node must resolve to a registered modifier");
        modifier(modifier_plan.value.clone(), &mut modifiers);
    }

    modifiers
}
