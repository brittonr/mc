use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Event;
use petgraph::graph::NodeIndex;

use crate::ModifierValue;

use super::packet_adapter::CommandExecutionPacketEvent;

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

pub(super) fn plan_command_execution_event(
    packet_event: &CommandExecutionPacketEvent,
) -> CommandExecutionEvent {
    CommandExecutionEvent {
        command: packet_event.command.clone(),
        executor: packet_event.client,
    }
}

pub(super) fn plan_processed_events(
    command: String,
    executor: Entity,
    modifiers: &HashMap<ModifierValue, ModifierValue>,
    executable_nodes: &[NodeIndex],
) -> Vec<CommandProcessedEvent> {
    executable_nodes
        .iter()
        .map(|node| CommandProcessedEvent {
            command: command.clone(),
            executor,
            modifiers: modifiers.clone(),
            node: *node,
        })
        .collect()
}
