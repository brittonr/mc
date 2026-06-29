// Copyright 2016 Matthew Collins
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::types::bit::Set as BSet;

pub(super) const WORLD_ENTITY_ID: usize = 0;
pub(super) const WORLD_ENTITY_GENERATION: u32 = 0;
pub(super) const FIRST_ALLOCATED_ENTITY_ID: usize = WORLD_ENTITY_ID + 1;

/// Used to reference an entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    pub(super) id: usize,
    pub(super) generation: u32,
}

impl Entity {
    pub(super) fn from_parts(id: usize, generation: u32) -> Entity {
        Entity { id, generation }
    }
}

#[derive(Clone)]
pub(super) struct EntityState {
    pub(super) last_components: BSet,
    pub(super) components: BSet,
    pub(super) removed: bool,
}

impl EntityState {
    pub(super) fn new(component_count: usize) -> EntityState {
        EntityState {
            last_components: BSet::new(component_count),
            components: BSet::new(component_count),
            removed: false,
        }
    }

    pub(super) fn has_component(&self, component_id: usize) -> bool {
        self.components.get(component_id)
    }

    pub(super) fn component_changed_this_tick(&self, component_id: usize) -> bool {
        self.components.get(component_id) != self.last_components.get(component_id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum EntityAllocation {
    Reuse { entity_id: usize },
    Append { entity_id: usize },
}

pub(super) fn allocation_decision(
    free_entities: &[usize],
    next_entity_id: usize,
) -> EntityAllocation {
    if let Some(&entity_id) = free_entities.last() {
        EntityAllocation::Reuse { entity_id }
    } else {
        EntityAllocation::Append {
            entity_id: next_entity_id,
        }
    }
}

pub(super) fn generation_matches(slot_generation: u32, entity: Entity) -> bool {
    slot_generation == entity.generation
}

pub(super) fn world_entity() -> Entity {
    Entity::from_parts(WORLD_ENTITY_ID, WORLD_ENTITY_GENERATION)
}
