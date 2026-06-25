pub mod admin_permissions;
pub mod graph;
pub mod handler;
pub mod manager;
mod modifier_value;
pub mod parsers;
pub mod scopes;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use bevy_app::App;
use bevy_ecs::prelude::{Resource, SystemSet};
pub use manager::{CommandExecutionEvent, CommandProcessedEvent};
pub use modifier_value::ModifierValue;
use petgraph::prelude::NodeIndex;
pub use scopes::CommandScopeRegistry;

use crate::graph::{CommandGraph, CommandGraphBuilder};
use crate::handler::CommandHandlerPlugin;
use crate::parsers::ParseInput;

/// The [`SystemSet`] in [`valence_server::EventLoopPreUpdate`] where command
/// trees are synchronized to clients after command graph or scope changes.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CommandTreeSet;

/// The [`SystemSet`] in [`valence_server::EventLoopPreUpdate`] where incoming
/// command events are parsed and dispatched to registered command handlers.
///
/// Packet input remains an internal ordering point before this set, while
/// typed command handlers run after it.
#[derive(SystemSet, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CommandSystemSet;

#[derive(Resource, Default)]
#[allow(clippy::type_complexity)]
pub struct CommandRegistry {
    pub graph: CommandGraph,
    pub parsers: HashMap<NodeIndex, fn(&mut ParseInput) -> bool>,
    pub modifiers: HashMap<NodeIndex, fn(String, &mut HashMap<ModifierValue, ModifierValue>)>,
    pub executables: HashSet<NodeIndex>,
}

pub trait Command {
    fn assemble_graph(graph: &mut CommandGraphBuilder<Self>)
    where
        Self: Sized;
}

pub trait AddCommand {
    fn add_command<T: Command + Send + Sync + 'static>(&mut self) -> &mut Self;
}

impl AddCommand for App {
    fn add_command<T: Command + Send + Sync + 'static>(&mut self) -> &mut Self {
        self.add_plugins(CommandHandlerPlugin::<T>::new())
    }
}
