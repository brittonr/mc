use std::collections::{HashMap, HashSet};
use std::time::Instant;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Added, Changed, Commands, DetectChanges, Event, EventReader, EventWriter, IntoSystemConfigs,
    Mut, Or, Query, Res, With,
};
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::{Direction, Graph};
use tracing::{debug, info, trace, warn};
use valence_server::client::{Client, SpawnClientsSet};
use valence_server::event_loop::{EventLoopSet, PacketEvent};
use valence_server::protocol::packets::play::command_tree_s2c::NodeData;
use valence_server::protocol::packets::play::{CommandExecutionC2s, CommandTreeS2c};
use valence_server::protocol::WritePacket;
use valence_server::EventLoopPreUpdate;

use crate::admin_permissions::{evaluate_command_scopes, AdminPermissionDecision};
use crate::graph::{CommandEdgeType, CommandGraph, CommandNode};
use crate::parsers::ParseInput;
use crate::scopes::{CommandScopePlugin, CommandScopes};
use crate::{
    CommandRegistry, CommandScopeRegistry, CommandSystemSet, CommandTreeSet, ModifierValue,
};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommandScopePlugin)
            .add_event::<CommandExecutionPacketEvent>()
            .add_event::<CommandExecutionEvent>()
            .add_event::<CommandProcessedEvent>()
            .add_systems(PreUpdate, insert_scope_component.after(SpawnClientsSet))
            .add_systems(
                EventLoopPreUpdate,
                (
                    update_command_tree.in_set(CommandTreeSet),
                    command_tree_update_with_client.in_set(CommandTreeSet),
                    emit_command_execution_packet_events.in_set(EventLoopSet::TypedAdapters),
                    emit_command_execution_events
                        .in_set(EventLoopSet::DomainConsumers)
                        .before(CommandSystemSet),
                    parse_incoming_commands
                        .in_set(EventLoopSet::DomainConsumers)
                        .in_set(CommandSystemSet),
                ),
            );

        let graph: CommandGraph = CommandGraph::new();
        let modifiers = HashMap::new();
        let parsers = HashMap::new();
        let executables = HashSet::new();

        app.insert_resource(CommandRegistry {
            graph,
            parsers,
            modifiers,
            executables,
        });
    }
}

/// A decoded `CommandExecutionC2s` packet from a live command client.
///
/// This event is emitted from [`EventLoopSet::TypedAdapters`] during
/// [`EventLoopPreUpdate`] after the raw packet body and source command client
/// have been validated. Raw [`PacketEvent`] values remain available for
/// low-level packet users.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct CommandExecutionPacketEvent {
    /// The live command client that sent the packet.
    pub client: Entity,
    /// The packet arrival timestamp copied from the raw packet boundary.
    pub timestamp: Instant,
    /// The decoded command string supplied by the client.
    pub command: String,
}

/// This event is sent when a command is sent (you can send this with any
/// entity).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct CommandExecutionEvent {
    /// the command that was executed eg. "teleport @p 0 ~ 0"
    pub command: String,
    /// usually the Client entity but it could be a command block or something
    /// (whatever the library user wants)
    pub executor: Entity,
}

/// This will only be sent if the command was successfully parsed and an
/// executable was found
#[derive(Debug, Clone, PartialEq, Eq, Event)]
pub struct CommandProcessedEvent {
    /// the command that was executed eg. "teleport @p 0 ~ 0"
    pub command: String,
    /// usually the Client entity but it could be a command block or something
    /// (whatever the library user wants)
    pub executor: Entity,
    /// the modifiers that were applied to the command
    pub modifiers: HashMap<ModifierValue, ModifierValue>,
    /// the node that was executed
    pub node: NodeIndex,
}

fn insert_scope_component(mut clients: Query<Entity, Added<Client>>, mut commands: Commands) {
    for client in &mut clients {
        commands.entity(client).insert(CommandScopes::new());
    }
}

fn emit_command_execution_packet_events(
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

fn emit_command_execution_events(
    mut packet_events: EventReader<CommandExecutionPacketEvent>,
    mut event_writer: EventWriter<CommandExecutionEvent>,
) {
    for packet_event in packet_events.read() {
        event_writer.send(CommandExecutionEvent {
            command: packet_event.command.clone(),
            executor: packet_event.client,
        });
    }
}

fn command_execution_packet_event_from_packet(
    packet: &PacketEvent,
) -> Option<CommandExecutionPacketEvent> {
    let pkt = packet.decode::<CommandExecutionC2s>()?;
    Some(CommandExecutionPacketEvent {
        client: packet.client,
        timestamp: packet.timestamp,
        command: pkt.command.to_string(),
    })
}

#[allow(clippy::type_complexity)]
fn command_tree_update_with_client(
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    mut updated_clients: Query<
        (&mut Client, &CommandScopes),
        Or<(Added<Client>, Changed<CommandScopes>)>,
    >,
) {
    update_client_command_tree(
        &command_registry,
        scope_registry,
        &mut updated_clients.iter_mut().collect(),
    );
}

fn update_command_tree(
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    mut clients: Query<(&mut Client, &CommandScopes)>,
) {
    if command_registry.is_changed() {
        update_client_command_tree(
            &command_registry,
            scope_registry,
            &mut clients.iter_mut().collect(),
        );
    }
}

fn update_client_command_tree(
    command_registry: &Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    updated_clients: &mut Vec<(Mut<Client>, &CommandScopes)>,
) {
    for (ref mut client, client_scopes) in updated_clients {
        let time = std::time::Instant::now();

        let old_graph = &command_registry.graph;
        let mut new_graph = Graph::new();

        // collect a new graph into only nodes that are allowed to be executed
        let root = old_graph.root;

        let mut to_visit = vec![(None, root)];
        let mut already_visited = HashSet::new(); // prevent recursion
        let mut old_to_new = HashMap::new();
        let mut new_root = None;

        while let Some((parent, node)) = to_visit.pop() {
            if already_visited.contains(&(parent.map(|(node_id, _)| node_id), node)) {
                continue;
            }
            already_visited.insert((parent.map(|(node_id, _)| node_id), node));
            let node_scopes = &old_graph.graph[node].scopes;
            if evaluate_command_scopes(scope_registry.as_ref(), &client_scopes.0, node_scopes)
                .is_denied()
            {
                continue;
            }

            let new_node = *old_to_new
                .entry(node)
                .or_insert_with(|| new_graph.add_node(old_graph.graph[node].clone()));

            for neighbor in old_graph.graph.edges_directed(node, Direction::Outgoing) {
                to_visit.push((Some((new_node, neighbor.weight())), neighbor.target()));
            }

            if let Some(parent) = parent {
                new_graph.add_edge(parent.0, new_node, *parent.1);
            } else {
                new_root = Some(new_node);
            }
        }

        match new_root {
            Some(new_root) => {
                let command_graph = CommandGraph {
                    graph: new_graph,
                    root: new_root,
                };
                let packet: CommandTreeS2c = command_graph.into();

                client.write_packet(&packet);
            }
            None => {
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

fn parse_incoming_commands(
    mut event_reader: EventReader<CommandExecutionEvent>,
    mut event_writer: EventWriter<CommandProcessedEvent>,
    command_registry: Res<CommandRegistry>,
    scope_registry: Res<CommandScopeRegistry>,
    entity_scopes: Query<&CommandScopes>,
) {
    for command_event in event_reader.read() {
        let executor = command_event.executor;
        // these are the leafs of the graph that are executable under this command
        // group
        let executable_leafs = command_registry
            .executables
            .iter()
            .collect::<Vec<&NodeIndex>>();
        let root = command_registry.graph.root;

        let command_input = &*command_event.command;
        let graph = &command_registry.graph.graph;
        let input = ParseInput::new(command_input);

        let mut to_be_executed = Vec::new();

        let mut args = Vec::new();
        let mut modifiers_to_be_executed = Vec::new();

        parse_command_args(
            &mut args,
            &mut modifiers_to_be_executed,
            input,
            graph,
            &executable_leafs,
            command_registry.as_ref(),
            &mut to_be_executed,
            root,
            executor,
            &entity_scopes,
            scope_registry.as_ref(),
            false,
        );

        let mut modifiers = HashMap::new();
        for (node, modifier) in modifiers_to_be_executed {
            command_registry.modifiers[&node](modifier, &mut modifiers);
        }

        for node in to_be_executed {
            trace!("executing node: {node:?}");
            event_writer.send(CommandProcessedEvent {
                command: args.join(" "),
                executor,
                modifiers: modifiers.clone(),
                node,
            });
        }
        info!(
            "Command dispatched: /{} (debug logs for more data)",
            command_event.command
        );
        debug!("Command modifiers: {:?}", modifiers);
    }
}

#[allow(clippy::too_many_arguments)]
/// recursively parse the command args.
fn parse_command_args(
    command_args: &mut Vec<String>,
    modifiers_to_be_executed: &mut Vec<(NodeIndex, String)>,
    mut input: ParseInput,
    graph: &Graph<CommandNode, CommandEdgeType>,
    executable_leafs: &[&NodeIndex],
    command_registry: &CommandRegistry,
    to_be_executed: &mut Vec<NodeIndex>,
    current_node: NodeIndex,
    executor: Entity,
    scopes: &Query<&CommandScopes>,
    scope_registry: &CommandScopeRegistry,
    coming_from_redirect: bool,
) -> bool {
    let node_scopes = &graph[current_node].scopes;
    let default_scopes = CommandScopes::new();
    let client_scopes = scopes.get(executor).unwrap_or(&default_scopes);
    // if empty, we assume the node is global
    if let AdminPermissionDecision::Denied(denial) =
        evaluate_command_scopes(scope_registry, &client_scopes.0, node_scopes)
    {
        debug!(
            "command denied for executor {:?}: {}",
            executor,
            denial.diagnostic(&graph[current_node].to_string())
        );
        return false;
    }

    if !coming_from_redirect {
        // we want to skip whitespace before matching the node
        input.skip_whitespace();
        match &graph[current_node].data {
            // no real need to check for root node
            NodeData::Root => {
                if command_registry.modifiers.contains_key(&current_node) {
                    modifiers_to_be_executed.push((current_node, String::new()));
                }
            }
            // if the node is a literal, we want to match the name of the literal
            // to the input
            NodeData::Literal { name } => {
                if input.match_next(name) {
                    if !input.match_next(" ") && !input.is_done() {
                        return false;
                    } // we want to pop the whitespace after the literal
                    if command_registry.modifiers.contains_key(&current_node) {
                        modifiers_to_be_executed.push((current_node, String::new()));
                    }
                } else {
                    return false;
                }
            }
            // if the node is an argument, we want to parse the argument
            NodeData::Argument { .. } => {
                let Some(parser) = command_registry.parsers.get(&current_node) else {
                    return false;
                };

                // we want to save the input before and after parsing
                // this is so we can save the argument to the command args
                let pre_input = input.clone().into_inner();
                let valid = parser(&mut input);
                if valid {
                    // If input.len() > pre_input.len() the parser replaced the input
                    let Some(arg) = pre_input
                        .get(..pre_input.len().wrapping_sub(input.len()))
                        .map(|s| s.to_owned())
                    else {
                        panic!(
                            "Parser replaced input with another string. This is not allowed. \
                             Attempting to parse: {}",
                            input.into_inner()
                        );
                    };

                    if command_registry.modifiers.contains_key(&current_node) {
                        modifiers_to_be_executed.push((current_node, arg.clone()));
                    }
                    command_args.push(arg);
                } else {
                    return false;
                }
            }
        }
    } else {
        command_args.clear();
    }

    input.skip_whitespace();
    if input.is_done() && executable_leafs.contains(&&current_node) {
        to_be_executed.push(current_node);
        return true;
    }

    let mut all_invalid = true;
    for neighbor in graph.neighbors(current_node) {
        let pre_input = input.clone();
        let mut args = command_args.clone();
        let mut modifiers = modifiers_to_be_executed.clone();
        let valid = parse_command_args(
            &mut args,
            &mut modifiers,
            input.clone(),
            graph,
            executable_leafs,
            command_registry,
            to_be_executed,
            neighbor,
            executor,
            scopes,
            scope_registry,
            {
                let edge = graph.find_edge(current_node, neighbor).unwrap();
                matches!(&graph[edge], CommandEdgeType::Redirect)
            },
        );
        if valid {
            *command_args = args;
            *modifiers_to_be_executed = modifiers;
            all_invalid = false;
        } else {
            input = pre_input;
        }
    }
    if all_invalid {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::event::Events;
    use bevy_ecs::prelude::{ResMut, Resource};
    use valence_server::event_loop::{EventLoopPlugin, EventLoopPreUpdate};
    use valence_server::protocol::{Bounded, Encode, FixedBitSet, Packet, VarInt};

    use super::*;

    const TEST_COMMAND: &str = "say typed events";
    const TEST_PROTOCOL_TIMESTAMP: u64 = 1_700_000_001;
    const TEST_COMMAND_SALT: u64 = 99;
    const TEST_MESSAGE_COUNT: i32 = 0;
    const INVALID_STRING_BYTE_LEN: i32 = 1;
    const INVALID_UTF8_BYTE: u8 = 0xff;
    const WRONG_PACKET_ID_OFFSET: i32 = 1;
    const PARTIAL_DECODE_TRUNCATED_BYTES: usize = 1;
    const EXPECTED_SINGLE_EVENT_COUNT: usize = 1;

    #[derive(Resource, Default)]
    struct RawPacketObservation {
        count: usize,
    }

    #[test]
    fn adapter_emits_command_packet_event_for_live_valid_packet() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);
        let timestamp = Instant::now();

        send_packet_event(
            &mut app,
            packet_event(
                client,
                timestamp,
                CommandExecutionC2s::ID,
                valid_command_execution_body(),
            ),
        );
        app.update();

        let packet_events = command_execution_packet_events(&app);
        assert_eq!(packet_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        let packet_event = &packet_events[0];
        assert_eq!(packet_event.client, client);
        assert_eq!(packet_event.timestamp, timestamp);
        assert_eq!(packet_event.command, TEST_COMMAND);
    }

    #[test]
    fn command_execution_event_consumes_typed_packet_event_without_duplicate_public_event() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);

        send_valid_command_packet(&mut app, client);
        app.update();

        let packet_events = command_execution_packet_events(&app);
        assert!(!has_duplicate_command_packet_event(&packet_events));

        let public_events = command_execution_events(&app);
        assert_eq!(public_events.len(), EXPECTED_SINGLE_EVENT_COUNT);
        assert_eq!(public_events[0].executor, client);
        assert_eq!(public_events[0].command, TEST_COMMAND);
    }

    #[test]
    fn raw_packet_observer_reads_before_command_typed_adapter() {
        let mut app = command_app();
        app.init_resource::<RawPacketObservation>().add_systems(
            EventLoopPreUpdate,
            count_raw_packets.in_set(EventLoopSet::RawPacketObservers),
        );
        let client = spawn_live_command_client(&mut app);

        send_valid_command_packet(&mut app, client);
        app.update();

        assert_eq!(
            raw_packet_observation_count(&app),
            EXPECTED_SINGLE_EVENT_COUNT
        );
        assert_eq!(
            command_execution_packet_events(&app).len(),
            EXPECTED_SINGLE_EVENT_COUNT
        );
    }

    #[test]
    fn duplicate_command_packet_events_are_detected() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);

        send_valid_command_packet(&mut app, client);
        app.update();

        let events = command_execution_packet_events(&app);
        assert!(!has_duplicate_command_packet_event(&events));
        assert!(has_duplicate_command_packet_event(&[
            events[0].clone(),
            events[0].clone(),
        ]));
    }

    #[test]
    fn adapter_rejects_wrong_command_packet_id() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                CommandExecutionC2s::ID + WRONG_PACKET_ID_OFFSET,
                valid_command_execution_body(),
            ),
        );
        app.update();

        assert_no_command_events(&app);
    }

    #[test]
    fn adapter_rejects_partial_command_decode() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                CommandExecutionC2s::ID,
                partial_command_execution_body(),
            ),
        );
        app.update();

        assert_no_command_events(&app);
    }

    #[test]
    fn adapter_rejects_malformed_command_payload() {
        let mut app = command_app();
        let client = spawn_live_command_client(&mut app);

        send_packet_event(
            &mut app,
            packet_event(
                client,
                Instant::now(),
                CommandExecutionC2s::ID,
                malformed_command_execution_body(),
            ),
        );
        app.update();

        assert_no_command_events(&app);
    }

    #[test]
    fn adapter_rejects_stale_command_client() {
        let mut app = command_app();
        let stale_client = spawn_live_command_client(&mut app);
        app.world_mut().despawn(stale_client);

        send_valid_command_packet(&mut app, stale_client);
        app.update();

        assert_no_command_events(&app);
    }

    fn command_app() -> App {
        let mut app = App::new();
        app.add_plugins(EventLoopPlugin).add_plugins(CommandPlugin);
        app
    }

    fn spawn_live_command_client(app: &mut App) -> Entity {
        app.world_mut().spawn(CommandScopes::new()).id()
    }

    fn send_valid_command_packet(app: &mut App, client: Entity) {
        send_packet_event(
            app,
            packet_event(
                client,
                Instant::now(),
                CommandExecutionC2s::ID,
                valid_command_execution_body(),
            ),
        );
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

    fn valid_command_execution_body() -> Vec<u8> {
        encoded_body(&CommandExecutionC2s {
            command: Bounded(TEST_COMMAND),
            timestamp: TEST_PROTOCOL_TIMESTAMP,
            salt: TEST_COMMAND_SALT,
            argument_signatures: Vec::new(),
            message_count: VarInt(TEST_MESSAGE_COUNT),
            acknowledgement: FixedBitSet::default(),
        })
    }

    fn partial_command_execution_body() -> Vec<u8> {
        let mut body = valid_command_execution_body();
        let remaining_len = body.len() - PARTIAL_DECODE_TRUNCATED_BYTES;
        body.truncate(remaining_len);
        body
    }

    fn malformed_command_execution_body() -> Vec<u8> {
        let mut body = Vec::new();
        VarInt(INVALID_STRING_BYTE_LEN).encode(&mut body).unwrap();
        body.push(INVALID_UTF8_BYTE);
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

    fn command_execution_packet_events(app: &App) -> Vec<CommandExecutionPacketEvent> {
        app.world()
            .resource::<Events<CommandExecutionPacketEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    fn command_execution_events(app: &App) -> Vec<CommandExecutionEvent> {
        app.world()
            .resource::<Events<CommandExecutionEvent>>()
            .iter_current_update_events()
            .cloned()
            .collect()
    }

    fn assert_no_command_events(app: &App) {
        assert!(command_execution_packet_events(app).is_empty());
        assert!(command_execution_events(app).is_empty());
    }

    fn has_duplicate_command_packet_event(events: &[CommandExecutionPacketEvent]) -> bool {
        let mut unique_events = Vec::new();
        for event in events {
            if unique_events
                .iter()
                .any(|candidate: &CommandExecutionPacketEvent| {
                    candidate.client == event.client
                        && candidate.timestamp == event.timestamp
                        && candidate.command == event.command
                })
            {
                return true;
            }
            unique_events.push(event.clone());
        }

        false
    }
}
