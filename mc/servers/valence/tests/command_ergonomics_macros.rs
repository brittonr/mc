use std::collections::HashMap;

use valence::command::graph::CommandGraphBuilder;
use valence::command::parsers::{CommandArg, ParseInput};
use valence::command::{Command, CommandRegistry};
use valence::command_macros::Command as CommandDerive;
use valence::prelude::App;
use valence::protocol::packets::play::command_tree_s2c::{CommandTreeS2c, NodeData, Suggestion};

const COMMAND_ROOT: &str = "ergotest";
const COMMAND_SCOPE: &str = "valence.command.ergotest";
const SPEED_LITERAL: &str = "speed";
const AMOUNT_ARGUMENT: &str = "amount";
const SUGGEST_LITERAL: &str = "suggest";
const TARGET_ARGUMENT: &str = "target";
const VALID_AMOUNT: &str = "42";
const INVALID_AMOUNT: &str = "not_an_integer";

#[derive(Debug, Clone, PartialEq, CommandDerive)]
#[paths("ergotest")]
#[scopes("valence.command.ergotest")]
enum ErgonomicCommand {
    #[paths("speed {amount}")]
    Speed { amount: i32 },
}

#[derive(Debug, Clone, PartialEq)]
enum ManualCommand {
    Speed { amount: i32 },
}

#[derive(Debug, PartialEq)]
struct CommandGraphSnapshot {
    root_index: i32,
    nodes: Vec<NodeSnapshot>,
}

#[derive(Debug, PartialEq)]
struct NodeSnapshot {
    data: NodeData,
    executable: bool,
    children: Vec<i32>,
    redirect_node: Option<i32>,
}

#[test]
fn derive_output_matches_manual_command_graph() {
    let (derived_snapshot, derived_parser_nodes, derived_executable_nodes) =
        derived_graph_snapshot();
    let (manual_snapshot, manual_parser_nodes, manual_executable_nodes) = manual_graph_snapshot();

    assert_eq!(derived_snapshot, manual_snapshot);
    assert_eq!(derived_parser_nodes, manual_parser_nodes);
    assert_eq!(derived_executable_nodes, manual_executable_nodes);
}

#[test]
fn derived_parser_rejects_invalid_argument_input() {
    let mut registry = CommandRegistry::default();
    let mut executable_map = HashMap::new();
    let mut parser_map = HashMap::new();
    let mut modifier_map = HashMap::new();
    let mut builder = CommandGraphBuilder::<ErgonomicCommand>::new(
        &mut registry,
        &mut executable_map,
        &mut parser_map,
        &mut modifier_map,
    );

    ErgonomicCommand::assemble_graph(&mut builder);
    let snapshot = snapshot(&registry);
    let amount_node = argument_node_index(&snapshot, AMOUNT_ARGUMENT);
    let amount_parser = parser_map
        .iter()
        .find_map(|(node, parser)| (node.index() == amount_node).then_some(parser))
        .unwrap();

    let mut valid_amount = ParseInput::new(VALID_AMOUNT);
    let mut invalid_amount = ParseInput::new(INVALID_AMOUNT);

    assert!(amount_parser(&mut valid_amount));
    assert!(!amount_parser(&mut invalid_amount));
}

#[test]
fn manual_builder_records_argument_suggestion_metadata() {
    let mut registry = CommandRegistry::default();
    let mut executable_map = HashMap::new();
    let mut parser_map = HashMap::new();
    let mut modifier_map = HashMap::new();
    let mut builder = CommandGraphBuilder::<ManualCommand>::new(
        &mut registry,
        &mut executable_map,
        &mut parser_map,
        &mut modifier_map,
    );

    builder
        .root()
        .literal(SUGGEST_LITERAL)
        .argument(TARGET_ARGUMENT)
        .with_suggestion(Suggestion::AskServer);

    let snapshot = snapshot(&registry);
    let target_node = argument_node(&snapshot, TARGET_ARGUMENT);

    assert_eq!(
        target_node.data,
        NodeData::Argument {
            name: TARGET_ARGUMENT.to_owned(),
            parser: valence::protocol::packets::play::command_tree_s2c::Parser::String(
                valence::protocol::packets::play::command_tree_s2c::StringArg::SingleWord,
            ),
            suggestion: Some(Suggestion::AskServer),
        }
    );
    assert!(!target_node.executable);
}

#[test]
fn plugin_disabled_does_not_register_command_graph_resources() {
    let mut app = App::empty();

    app.update();

    assert!(app.world().get_resource::<CommandRegistry>().is_none());
}

fn derived_graph_snapshot() -> (CommandGraphSnapshot, Vec<usize>, Vec<usize>) {
    let mut registry = CommandRegistry::default();
    let mut executable_map = HashMap::new();
    let mut parser_map = HashMap::new();
    let mut modifier_map = HashMap::new();
    let mut builder = CommandGraphBuilder::<ErgonomicCommand>::new(
        &mut registry,
        &mut executable_map,
        &mut parser_map,
        &mut modifier_map,
    );

    ErgonomicCommand::assemble_graph(&mut builder);

    (
        snapshot(&registry),
        sorted_node_indexes(parser_map.keys().map(|node| node.index())),
        sorted_node_indexes(executable_map.keys().map(|node| node.index())),
    )
}

fn manual_graph_snapshot() -> (CommandGraphSnapshot, Vec<usize>, Vec<usize>) {
    let mut registry = CommandRegistry::default();
    let mut executable_map = HashMap::new();
    let mut parser_map = HashMap::new();
    let mut modifier_map = HashMap::new();
    let mut builder = CommandGraphBuilder::<ManualCommand>::new(
        &mut registry,
        &mut executable_map,
        &mut parser_map,
        &mut modifier_map,
    );

    builder
        .root()
        .literal(COMMAND_ROOT)
        .with_scopes(vec![COMMAND_SCOPE])
        .literal(SPEED_LITERAL)
        .with_scopes(vec![COMMAND_SCOPE])
        .argument(AMOUNT_ARGUMENT)
        .with_parser::<i32>()
        .with_executable(|args| ManualCommand::Speed {
            amount: i32::parse_arg(args).unwrap(),
        });

    (
        snapshot(&registry),
        sorted_node_indexes(parser_map.keys().map(|node| node.index())),
        sorted_node_indexes(executable_map.keys().map(|node| node.index())),
    )
}

fn snapshot(registry: &CommandRegistry) -> CommandGraphSnapshot {
    let tree = CommandTreeS2c::from(registry.graph.clone());
    let nodes = tree
        .commands
        .into_iter()
        .map(|node| {
            let mut children = node
                .children
                .into_iter()
                .map(|child| child.0)
                .collect::<Vec<_>>();
            children.sort_unstable();
            NodeSnapshot {
                data: node.data,
                executable: node.executable,
                children,
                redirect_node: node.redirect_node.map(|redirect| redirect.0),
            }
        })
        .collect();

    CommandGraphSnapshot {
        root_index: tree.root_index.0,
        nodes,
    }
}

fn argument_node_index(snapshot: &CommandGraphSnapshot, name: &str) -> usize {
    snapshot
        .nodes
        .iter()
        .position(|node| {
            matches!(
                &node.data,
                NodeData::Argument {
                    name: argument_name,
                    ..
                } if argument_name == name
            )
        })
        .unwrap()
}

fn argument_node<'a>(snapshot: &'a CommandGraphSnapshot, name: &str) -> &'a NodeSnapshot {
    &snapshot.nodes[argument_node_index(snapshot, name)]
}

fn sorted_node_indexes(indexes: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut indexes = indexes.collect::<Vec<_>>();
    indexes.sort_unstable();
    indexes
}
