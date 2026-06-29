use std::collections::HashMap;
use std::time::Instant;

use bevy_app::App;
use bevy_ecs::event::Events;
use bevy_ecs::prelude::{EventReader, IntoSystemConfigs, ResMut, Resource};
use valence_server::event_loop::{EventLoopPlugin, EventLoopPreUpdate};
use valence_server::protocol::{Bounded, Encode, FixedBitSet, Packet, VarInt};

use super::event_plan::{plan_processed_events, CommandExecutionEvent, CommandProcessedEvent};
use super::packet_adapter::plan_command_packet_event;
use super::parse_core::{parse_command_request, CommandParseRequest};
use super::tree_sync::{
    command_tree_update_decision, visible_command_tree_plan, CommandTreePlan,
    CommandTreeUpdateDecision,
};
use super::*;
use crate::graph::CommandGraphBuilder;
use crate::scopes::CommandScopes;
use crate::{CommandRegistry, CommandScopeRegistry, ModifierValue};

const TEST_COMMAND: &str = "say typed events";
const TEST_PROTOCOL_TIMESTAMP: u64 = 1_700_000_001;
const TEST_COMMAND_SALT: u64 = 99;
const TEST_MESSAGE_COUNT: i32 = 0;
const INVALID_STRING_BYTE_LEN: i32 = 1;
const INVALID_UTF8_BYTE: u8 = 0xff;
const WRONG_PACKET_ID_OFFSET: i32 = 1;
const PARTIAL_DECODE_TRUNCATED_BYTES: usize = 1;
const EXPECTED_SINGLE_EVENT_COUNT: usize = 1;
const TEST_LITERAL: &str = "math";
const TEST_ARGUMENT_NAME: &str = "value";
const TEST_ARGUMENT: &str = "42";
const INVALID_ARGUMENT: &str = "not-an-integer";
const UNKNOWN_COMMAND: &str = "unknown 42";
const REQUIRED_SCOPE: &str = "command.math";
const TEST_MODIFIER_KEY: &str = "argument";

#[derive(Resource, Default)]
struct RawPacketObservation {
    count: usize,
}

struct TestCommand;

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
            valence_server::protocol::packets::play::CommandExecutionC2s::ID,
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
        count_raw_packets.in_set(valence_server::event_loop::EventLoopSet::RawPacketObservers),
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
            valence_server::protocol::packets::play::CommandExecutionC2s::ID
                + WRONG_PACKET_ID_OFFSET,
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
            valence_server::protocol::packets::play::CommandExecutionC2s::ID,
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
            valence_server::protocol::packets::play::CommandExecutionC2s::ID,
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

#[test]
fn adapter_ignores_disabled_client_without_command_scopes() {
    let mut app = command_app();
    let disabled_client = app.world_mut().spawn_empty().id();

    send_valid_command_packet(&mut app, disabled_client);
    app.update();

    assert_no_command_events(&app);
}

#[test]
fn tree_update_decision_tracks_registry_change() {
    assert_eq!(
        command_tree_update_decision(true),
        CommandTreeUpdateDecision::Update
    );
    assert_eq!(
        command_tree_update_decision(false),
        CommandTreeUpdateDecision::Skip
    );
}

#[test]
fn visible_command_tree_plan_sends_allowed_tree() {
    let (registry, scope_registry, _executable_node) = command_registry_with_parse_fixture(None);
    let client_scopes = CommandScopes::new();

    let CommandTreePlan::Send(packet) =
        visible_command_tree_plan(&registry.graph, &scope_registry, &client_scopes)
    else {
        panic!("expected visible command tree packet");
    };

    assert!(!packet.commands.is_empty());
}

#[test]
fn stale_command_tree_registry_does_not_update_without_change() {
    assert_eq!(
        command_tree_update_decision(false),
        CommandTreeUpdateDecision::Skip
    );
}

#[test]
fn command_tree_plan_rejects_missing_root_scope() {
    let mut registry = CommandRegistry::default();
    registry.graph.graph[registry.graph.root].scopes = vec![REQUIRED_SCOPE.to_owned()];
    let mut scope_registry = CommandScopeRegistry::new();
    scope_registry.add_scope(REQUIRED_SCOPE);
    let client_scopes = CommandScopes::new();

    assert!(matches!(
        visible_command_tree_plan(&registry.graph, &scope_registry, &client_scopes),
        CommandTreePlan::NoVisibleRoot
    ));
}

#[test]
fn valid_command_parse_produces_argument_and_modifier_plan() {
    let (registry, scope_registry, executable_node) = command_registry_with_parse_fixture(None);
    let command = format!("{TEST_LITERAL} {TEST_ARGUMENT}");

    let plan = parse_command_request(CommandParseRequest {
        command: &command,
        registry: &registry,
        scope_registry: &scope_registry,
        executor_scopes: None,
    });

    assert_eq!(plan.command_args, vec![TEST_ARGUMENT.to_owned()]);
    assert_eq!(plan.parsed_command(), TEST_ARGUMENT);
    assert_eq!(plan.modifier_plans.len(), EXPECTED_SINGLE_EVENT_COUNT);
    assert_eq!(plan.modifier_plans[0].node, executable_node);
    assert_eq!(plan.modifier_plans[0].value, TEST_ARGUMENT);
    assert_eq!(plan.executable_nodes, vec![executable_node]);
}

#[test]
fn unknown_command_parse_has_no_executable_plan() {
    let (registry, scope_registry, _executable_node) = command_registry_with_parse_fixture(None);

    let plan = parse_command_request(CommandParseRequest {
        command: UNKNOWN_COMMAND,
        registry: &registry,
        scope_registry: &scope_registry,
        executor_scopes: None,
    });

    assert!(plan.command_args.is_empty());
    assert!(plan.modifier_plans.is_empty());
    assert!(plan.executable_nodes.is_empty());
}

#[test]
fn invalid_argument_parse_has_no_executable_plan() {
    let (registry, scope_registry, _executable_node) = command_registry_with_parse_fixture(None);
    let command = format!("{TEST_LITERAL} {INVALID_ARGUMENT}");

    let plan = parse_command_request(CommandParseRequest {
        command: &command,
        registry: &registry,
        scope_registry: &scope_registry,
        executor_scopes: None,
    });

    assert!(plan.command_args.is_empty());
    assert!(plan.modifier_plans.is_empty());
    assert!(plan.executable_nodes.is_empty());
}

#[test]
fn missing_scope_parse_has_no_executable_plan() {
    let (registry, scope_registry, _executable_node) =
        command_registry_with_parse_fixture(Some(REQUIRED_SCOPE));
    let command = format!("{TEST_LITERAL} {TEST_ARGUMENT}");
    let client_scopes = CommandScopes::new();

    let plan = parse_command_request(CommandParseRequest {
        command: &command,
        registry: &registry,
        scope_registry: &scope_registry,
        executor_scopes: Some(&client_scopes),
    });

    assert!(plan.command_args.is_empty());
    assert!(plan.modifier_plans.is_empty());
    assert!(plan.executable_nodes.is_empty());
}

#[test]
fn processed_event_plan_clones_command_executor_and_modifiers() {
    let (registry, scope_registry, executable_node) = command_registry_with_parse_fixture(None);
    let command = format!("{TEST_LITERAL} {TEST_ARGUMENT}");
    let parse_plan = parse_command_request(CommandParseRequest {
        command: &command,
        registry: &registry,
        scope_registry: &scope_registry,
        executor_scopes: None,
    });
    let mut modifiers = HashMap::new();
    modifiers.insert(TEST_MODIFIER_KEY.into(), TEST_ARGUMENT.into());
    let mut app = App::new();
    let executor = app.world_mut().spawn_empty().id();

    let events = plan_processed_events(
        parse_plan.parsed_command(),
        executor,
        &modifiers,
        &parse_plan.executable_nodes,
    );

    assert_eq!(events.len(), EXPECTED_SINGLE_EVENT_COUNT);
    assert_eq!(events[0].command, TEST_ARGUMENT);
    assert_eq!(events[0].executor, executor);
    assert_eq!(events[0].modifiers, modifiers);
    assert_eq!(events[0].node, executable_node);
}

#[test]
fn command_plugin_registers_events_registry_and_empty_core_state() {
    let app = command_app();

    assert!(app
        .world()
        .contains_resource::<Events<CommandExecutionPacketEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<CommandExecutionEvent>>());
    assert!(app
        .world()
        .contains_resource::<Events<CommandProcessedEvent>>());
    assert!(app.world().contains_resource::<CommandRegistry>());

    let registry = app.world().resource::<CommandRegistry>();
    assert!(registry.parsers.is_empty());
    assert!(registry.modifiers.is_empty());
    assert!(registry.executables.is_empty());
}

#[test]
fn packet_event_plan_copies_decoded_packet_fields() {
    let mut app = App::new();
    let client = app.world_mut().spawn_empty().id();
    let timestamp = Instant::now();

    let event = plan_command_packet_event(client, timestamp, TEST_COMMAND.to_owned());

    assert_eq!(event.client, client);
    assert_eq!(event.timestamp, timestamp);
    assert_eq!(event.command, TEST_COMMAND);
}

fn command_app() -> App {
    let mut app = App::new();
    app.add_plugins(EventLoopPlugin).add_plugins(CommandPlugin);
    app
}

fn spawn_live_command_client(app: &mut App) -> bevy_ecs::entity::Entity {
    app.world_mut().spawn(CommandScopes::new()).id()
}

fn send_valid_command_packet(app: &mut App, client: bevy_ecs::entity::Entity) {
    send_packet_event(
        app,
        packet_event(
            client,
            Instant::now(),
            valence_server::protocol::packets::play::CommandExecutionC2s::ID,
            valid_command_execution_body(),
        ),
    );
}

fn send_packet_event(app: &mut App, event: valence_server::event_loop::PacketEvent) {
    app.world_mut()
        .resource_mut::<Events<valence_server::event_loop::PacketEvent>>()
        .send(event);
}

fn packet_event(
    client: bevy_ecs::entity::Entity,
    timestamp: Instant,
    id: i32,
    data: Vec<u8>,
) -> valence_server::event_loop::PacketEvent {
    valence_server::event_loop::PacketEvent {
        client,
        timestamp,
        id,
        data: data.into(),
    }
}

fn valid_command_execution_body() -> Vec<u8> {
    encoded_body(
        &valence_server::protocol::packets::play::CommandExecutionC2s {
            command: Bounded(TEST_COMMAND),
            timestamp: TEST_PROTOCOL_TIMESTAMP,
            salt: TEST_COMMAND_SALT,
            argument_signatures: Vec::new(),
            message_count: VarInt(TEST_MESSAGE_COUNT),
            acknowledgement: FixedBitSet::default(),
        },
    )
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
    mut packets: EventReader<valence_server::event_loop::PacketEvent>,
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

fn command_registry_with_parse_fixture(
    required_scope: Option<&str>,
) -> (
    CommandRegistry,
    CommandScopeRegistry,
    petgraph::graph::NodeIndex,
) {
    let mut registry = CommandRegistry::default();
    let mut scope_registry = CommandScopeRegistry::new();
    let mut executables = HashMap::new();
    let mut parsers = HashMap::new();
    let mut modifiers = HashMap::new();
    let builder = &mut CommandGraphBuilder::<TestCommand>::new(
        &mut registry,
        &mut executables,
        &mut parsers,
        &mut modifiers,
    );

    builder.root().literal(TEST_LITERAL);
    if let Some(scope) = required_scope {
        builder.with_scopes(vec![scope]);
    }
    let executable_node = builder
        .argument(TEST_ARGUMENT_NAME)
        .with_parser::<i32>()
        .with_modifier(record_argument_modifier)
        .with_executable(|_| TestCommand)
        .id();
    builder.apply_scopes(&mut scope_registry);

    registry.parsers.extend(parsers);
    registry.modifiers.extend(modifiers);
    registry.executables.extend(executables.keys().copied());

    (registry, scope_registry, executable_node)
}

fn record_argument_modifier(
    argument: String,
    modifiers: &mut HashMap<ModifierValue, ModifierValue>,
) {
    modifiers.insert(TEST_MODIFIER_KEY.into(), argument.into());
}
