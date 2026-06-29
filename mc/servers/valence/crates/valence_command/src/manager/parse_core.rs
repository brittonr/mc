use std::collections::HashSet;

use petgraph::graph::NodeIndex;
use petgraph::Graph;
use valence_server::protocol::packets::play::command_tree_s2c::NodeData;

use crate::admin_permissions::evaluate_command_scopes;
use crate::graph::{CommandEdgeType, CommandNode};
use crate::parsers::ParseInput;
use crate::scopes::{CommandScopeRegistry, CommandScopes};
use crate::CommandRegistry;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ModifierExecutionPlan {
    pub node: NodeIndex,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CommandParsePlan {
    pub command_args: Vec<String>,
    pub modifier_plans: Vec<ModifierExecutionPlan>,
    pub executable_nodes: Vec<NodeIndex>,
}

pub(super) struct CommandParseRequest<'a> {
    pub command: &'a str,
    pub registry: &'a CommandRegistry,
    pub scope_registry: &'a CommandScopeRegistry,
    pub executor_scopes: Option<&'a CommandScopes>,
}

impl CommandParsePlan {
    pub(super) fn parsed_command(&self) -> String {
        self.command_args.join(" ")
    }
}

impl ModifierExecutionPlan {
    fn new(node: NodeIndex, value: String) -> Self {
        Self { node, value }
    }
}

pub(super) fn parse_command_request(request: CommandParseRequest<'_>) -> CommandParsePlan {
    let root = request.registry.graph.root;
    let graph = &request.registry.graph.graph;
    let input = ParseInput::new(request.command);
    let mut command_args = Vec::new();
    let mut modifier_plans = Vec::new();
    let mut executable_nodes = Vec::new();

    parse_command_args(
        &mut command_args,
        &mut modifier_plans,
        input,
        graph,
        &request.registry.executables,
        request.registry,
        &mut executable_nodes,
        root,
        request.executor_scopes,
        request.scope_registry,
        false,
    );

    CommandParsePlan {
        command_args,
        modifier_plans,
        executable_nodes,
    }
}

#[allow(clippy::too_many_arguments)]
fn parse_command_args(
    command_args: &mut Vec<String>,
    modifier_plans: &mut Vec<ModifierExecutionPlan>,
    mut input: ParseInput,
    graph: &Graph<CommandNode, CommandEdgeType>,
    executable_leafs: &HashSet<NodeIndex>,
    command_registry: &CommandRegistry,
    executable_nodes: &mut Vec<NodeIndex>,
    current_node: NodeIndex,
    executor_scopes: Option<&CommandScopes>,
    scope_registry: &CommandScopeRegistry,
    coming_from_redirect: bool,
) -> bool {
    let node_scopes = &graph[current_node].scopes;
    let default_scopes = CommandScopes::new();
    let client_scopes = executor_scopes.unwrap_or(&default_scopes);

    if evaluate_command_scopes(scope_registry, &client_scopes.0, node_scopes).is_denied() {
        return false;
    }

    if !coming_from_redirect {
        input.skip_whitespace();
        match &graph[current_node].data {
            NodeData::Root => {
                if command_registry.modifiers.contains_key(&current_node) {
                    modifier_plans.push(ModifierExecutionPlan::new(current_node, String::new()));
                }
            }
            NodeData::Literal { name } => {
                if input.match_next(name) {
                    if !input.match_next(" ") && !input.is_done() {
                        return false;
                    }
                    if command_registry.modifiers.contains_key(&current_node) {
                        modifier_plans
                            .push(ModifierExecutionPlan::new(current_node, String::new()));
                    }
                } else {
                    return false;
                }
            }
            NodeData::Argument { .. } => {
                let Some(parser) = command_registry.parsers.get(&current_node) else {
                    return false;
                };

                let pre_input = input.clone().into_inner();
                let valid = parser(&mut input);
                if valid {
                    let Some(consumed_len) = pre_input.len().checked_sub(input.len()) else {
                        panic!(
                            "Parser replaced input with another string. This is not allowed. \
                             Attempting to parse: {}",
                            input.into_inner()
                        );
                    };
                    let Some(arg) = pre_input.get(..consumed_len).map(ToOwned::to_owned) else {
                        panic!(
                            "Parser split input at a non-character boundary. This is not allowed. \
                             Attempting to parse: {}",
                            input.into_inner()
                        );
                    };

                    if command_registry.modifiers.contains_key(&current_node) {
                        modifier_plans.push(ModifierExecutionPlan::new(current_node, arg.clone()));
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
    if input.is_done() && executable_leafs.contains(&current_node) {
        executable_nodes.push(current_node);
        return true;
    }

    let mut all_invalid = true;
    for neighbor in graph.neighbors(current_node) {
        let pre_input = input.clone();
        let mut args = command_args.clone();
        let mut modifiers = modifier_plans.clone();
        let valid = parse_command_args(
            &mut args,
            &mut modifiers,
            input.clone(),
            graph,
            executable_leafs,
            command_registry,
            executable_nodes,
            neighbor,
            executor_scopes,
            scope_registry,
            {
                let edge = graph
                    .find_edge(current_node, neighbor)
                    .expect("neighbor edge must exist");
                matches!(&graph[edge], CommandEdgeType::Redirect)
            },
        );
        if valid {
            *command_args = args;
            *modifier_plans = modifiers;
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
