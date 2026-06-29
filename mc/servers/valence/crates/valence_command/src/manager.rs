//! Command manager plugin, event types, pure command decisions, and Bevy shells.
//!
//! The manager keeps packet decoding, tree synchronization, parsing, event
//! planning, plugin wiring, and Bevy systems in separate modules. Public event
//! types stay available from `valence_command::manager`.

mod event_plan;
mod packet_adapter;
mod parse_core;
mod plugin;
mod systems;
mod tree_sync;

#[cfg(test)]
mod tests;

pub use event_plan::{CommandExecutionEvent, CommandProcessedEvent};
pub use packet_adapter::CommandExecutionPacketEvent;
pub use plugin::CommandPlugin;
