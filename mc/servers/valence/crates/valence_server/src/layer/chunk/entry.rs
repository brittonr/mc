use std::collections::hash_map::{OccupiedEntry, VacantEntry};

use valence_protocol::ChunkPos;

use super::{ChunkLayer, ChunkLayerMessages, LoadedChunk, LocalMsg, UnloadedChunk};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ChunkEntryState {
    Occupied,
    Vacant,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ChunkEntryOperation {
    Insert,
    Remove,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ChunkEntryTransition {
    Load,
    Unload,
    Overwrite,
}

impl ChunkEntryTransition {
    fn message_byte(self) -> u8 {
        match self {
            ChunkEntryTransition::Load => ChunkLayer::LOAD,
            ChunkEntryTransition::Unload => ChunkLayer::UNLOAD,
            ChunkEntryTransition::Overwrite => ChunkLayer::OVERWRITE,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum ChunkStateMessagePlan {
    Noop,
    Load,
    Unload,
    Invalid,
}

fn entry_state_transition(
    state: ChunkEntryState,
    operation: ChunkEntryOperation,
) -> Option<ChunkEntryTransition> {
    match (state, operation) {
        (ChunkEntryState::Occupied, ChunkEntryOperation::Insert) => {
            Some(ChunkEntryTransition::Overwrite)
        }
        (ChunkEntryState::Occupied, ChunkEntryOperation::Remove) => {
            Some(ChunkEntryTransition::Unload)
        }
        (ChunkEntryState::Vacant, ChunkEntryOperation::Insert) => Some(ChunkEntryTransition::Load),
        (ChunkEntryState::Vacant, ChunkEntryOperation::Remove) => None,
    }
}

fn send_transition(
    messages: &mut ChunkLayerMessages,
    pos: ChunkPos,
    transition: ChunkEntryTransition,
) {
    messages.send_local_infallible(LocalMsg::ChangeChunkState { pos }, |b| {
        b.push(transition.message_byte())
    });
}

pub(crate) fn chunk_state_message_plan(bytes: &[u8]) -> ChunkStateMessagePlan {
    match bytes {
        [ChunkLayer::LOAD, .., ChunkLayer::UNLOAD] => ChunkStateMessagePlan::Noop,
        [.., ChunkLayer::LOAD | ChunkLayer::OVERWRITE] => ChunkStateMessagePlan::Load,
        [.., ChunkLayer::UNLOAD] => ChunkStateMessagePlan::Unload,
        _ => ChunkStateMessagePlan::Invalid,
    }
}

#[derive(Debug)]
pub enum ChunkEntry<'a> {
    Occupied(OccupiedChunkEntry<'a>),
    Vacant(VacantChunkEntry<'a>),
}

impl<'a> ChunkEntry<'a> {
    pub fn or_default(self) -> &'a mut LoadedChunk {
        match self {
            ChunkEntry::Occupied(oe) => oe.into_mut(),
            ChunkEntry::Vacant(ve) => ve.insert(UnloadedChunk::new()),
        }
    }
}

#[derive(Debug)]
pub struct OccupiedChunkEntry<'a> {
    pub(super) messages: &'a mut ChunkLayerMessages,
    pub(super) entry: OccupiedEntry<'a, ChunkPos, LoadedChunk>,
}

impl<'a> OccupiedChunkEntry<'a> {
    pub fn get(&self) -> &LoadedChunk {
        self.entry.get()
    }

    pub fn get_mut(&mut self) -> &mut LoadedChunk {
        self.entry.get_mut()
    }

    pub fn insert(&mut self, chunk: UnloadedChunk) -> UnloadedChunk {
        let transition =
            entry_state_transition(ChunkEntryState::Occupied, ChunkEntryOperation::Insert)
                .expect("occupied insert should emit overwrite transition");
        send_transition(self.messages, *self.entry.key(), transition);

        self.entry.get_mut().insert(chunk)
    }

    pub fn into_mut(self) -> &'a mut LoadedChunk {
        self.entry.into_mut()
    }

    pub fn key(&self) -> &ChunkPos {
        self.entry.key()
    }

    pub fn remove(self) -> UnloadedChunk {
        let transition =
            entry_state_transition(ChunkEntryState::Occupied, ChunkEntryOperation::Remove)
                .expect("occupied remove should emit unload transition");
        send_transition(self.messages, *self.entry.key(), transition);

        self.entry.remove().remove()
    }

    pub fn remove_entry(mut self) -> (ChunkPos, UnloadedChunk) {
        let pos = *self.entry.key();
        let chunk = self.entry.get_mut().remove();

        let transition =
            entry_state_transition(ChunkEntryState::Occupied, ChunkEntryOperation::Remove)
                .expect("occupied remove_entry should emit unload transition");
        send_transition(self.messages, pos, transition);

        (pos, chunk)
    }
}

#[derive(Debug)]
pub struct VacantChunkEntry<'a> {
    pub(super) height: u32,
    pub(super) messages: &'a mut ChunkLayerMessages,
    pub(super) entry: VacantEntry<'a, ChunkPos, LoadedChunk>,
}

impl<'a> VacantChunkEntry<'a> {
    pub fn insert(self, chunk: UnloadedChunk) -> &'a mut LoadedChunk {
        let mut loaded = LoadedChunk::new(self.height);
        loaded.insert(chunk);

        let transition =
            entry_state_transition(ChunkEntryState::Vacant, ChunkEntryOperation::Insert)
                .expect("vacant insert should emit load transition");
        send_transition(self.messages, *self.entry.key(), transition);

        self.entry.insert(loaded)
    }

    pub fn into_key(self) -> ChunkPos {
        *self.entry.key()
    }

    pub fn key(&self) -> &ChunkPos {
        self.entry.key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOAD_ONLY_BYTES: &[u8] = &[ChunkLayer::LOAD];
    const UNLOAD_ONLY_BYTES: &[u8] = &[ChunkLayer::UNLOAD];
    const OVERWRITE_ONLY_BYTES: &[u8] = &[ChunkLayer::OVERWRITE];
    const LOAD_THEN_UNLOAD_BYTES: &[u8] = &[ChunkLayer::LOAD, ChunkLayer::UNLOAD];
    const INVALID_STATE_BYTES: &[u8] = &[];

    #[test]
    fn entry_transitions_cover_occupied_and_vacant_writes() {
        assert_eq!(
            entry_state_transition(ChunkEntryState::Vacant, ChunkEntryOperation::Insert),
            Some(ChunkEntryTransition::Load)
        );
        assert_eq!(
            entry_state_transition(ChunkEntryState::Occupied, ChunkEntryOperation::Insert),
            Some(ChunkEntryTransition::Overwrite)
        );
        assert_eq!(
            entry_state_transition(ChunkEntryState::Occupied, ChunkEntryOperation::Remove),
            Some(ChunkEntryTransition::Unload)
        );
    }

    #[test]
    fn missing_or_stale_entries_do_not_emit_state_changes() {
        assert_eq!(
            entry_state_transition(ChunkEntryState::Vacant, ChunkEntryOperation::Remove),
            None
        );
    }

    #[test]
    fn chunk_state_messages_select_load_unload_and_noop_plans() {
        assert_eq!(
            chunk_state_message_plan(LOAD_ONLY_BYTES),
            ChunkStateMessagePlan::Load
        );
        assert_eq!(
            chunk_state_message_plan(OVERWRITE_ONLY_BYTES),
            ChunkStateMessagePlan::Load
        );
        assert_eq!(
            chunk_state_message_plan(UNLOAD_ONLY_BYTES),
            ChunkStateMessagePlan::Unload
        );
        assert_eq!(
            chunk_state_message_plan(LOAD_THEN_UNLOAD_BYTES),
            ChunkStateMessagePlan::Noop
        );
    }

    #[test]
    fn invalid_state_messages_fail_closed() {
        assert_eq!(
            chunk_state_message_plan(INVALID_STATE_BYTES),
            ChunkStateMessagePlan::Invalid
        );
    }
}
