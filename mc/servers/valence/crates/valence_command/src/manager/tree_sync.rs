use std::collections::{HashMap, HashSet};

use petgraph::prelude::EdgeRef;
use petgraph::{Direction, Graph};
use valence_server::protocol::packets::play::CommandTreeS2c;

use crate::admin_permissions::evaluate_command_scopes;
use crate::graph::CommandGraph;
use crate::scopes::{CommandScopeRegistry, CommandScopes};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CommandTreeUpdateDecision {
    Skip,
    Update,
}

#[derive(Debug)]
pub(super) enum CommandTreePlan {
    Send(CommandTreeS2c),
    NoVisibleRoot,
}

pub(super) fn command_tree_update_decision(
    command_registry_changed: bool,
) -> CommandTreeUpdateDecision {
    if command_registry_changed {
        CommandTreeUpdateDecision::Update
    } else {
        CommandTreeUpdateDecision::Skip
    }
}

pub(super) fn visible_command_tree_plan(
    command_graph: &CommandGraph,
    scope_registry: &CommandScopeRegistry,
    client_scopes: &CommandScopes,
) -> CommandTreePlan {
    match visible_command_graph(command_graph, scope_registry, client_scopes) {
        Some(command_graph) => CommandTreePlan::Send(command_graph.into()),
        None => CommandTreePlan::NoVisibleRoot,
    }
}

fn visible_command_graph(
    old_graph: &CommandGraph,
    scope_registry: &CommandScopeRegistry,
    client_scopes: &CommandScopes,
) -> Option<CommandGraph> {
    let mut new_graph = Graph::new();
    let root = old_graph.root;
    let mut to_visit = vec![(None, root)];
    let mut already_visited = HashSet::new();
    let mut old_to_new = HashMap::new();
    let mut new_root = None;

    while let Some((parent, node)) = to_visit.pop() {
        if already_visited.contains(&(parent.map(|(node_id, _)| node_id), node)) {
            continue;
        }
        already_visited.insert((parent.map(|(node_id, _)| node_id), node));

        let node_scopes = &old_graph.graph[node].scopes;
        if evaluate_command_scopes(scope_registry, &client_scopes.0, node_scopes).is_denied() {
            continue;
        }

        let new_node = *old_to_new
            .entry(node)
            .or_insert_with(|| new_graph.add_node(old_graph.graph[node].clone()));

        for neighbor in old_graph.graph.edges_directed(node, Direction::Outgoing) {
            to_visit.push((Some((new_node, *neighbor.weight())), neighbor.target()));
        }

        if let Some((parent_node, edge_type)) = parent {
            new_graph.add_edge(parent_node, new_node, edge_type);
        } else {
            new_root = Some(new_node);
        }
    }

    new_root.map(|root| CommandGraph {
        graph: new_graph,
        root,
    })
}
