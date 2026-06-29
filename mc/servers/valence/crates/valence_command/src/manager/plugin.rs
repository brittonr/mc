use std::collections::{HashMap, HashSet};

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::prelude::IntoSystemConfigs;
use valence_server::client::SpawnClientsSet;
use valence_server::event_loop::EventLoopSet;
use valence_server::EventLoopPreUpdate;

use crate::graph::CommandGraph;
use crate::scopes::CommandScopePlugin;
use crate::{CommandRegistry, CommandSystemSet, CommandTreeSet};

use super::event_plan::{CommandExecutionEvent, CommandProcessedEvent};
use super::packet_adapter::CommandExecutionPacketEvent;
use super::systems::{
    command_tree_update_with_client, emit_command_execution_events,
    emit_command_execution_packet_events, insert_scope_component, parse_incoming_commands,
    update_command_tree,
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
            )
            .insert_resource(empty_command_registry());
    }
}

pub(super) fn empty_command_registry() -> CommandRegistry {
    CommandRegistry {
        graph: CommandGraph::new(),
        parsers: HashMap::new(),
        modifiers: HashMap::new(),
        executables: HashSet::new(),
    }
}
