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

use super::components::Key;
use super::entity::EntityState;
use crate::types::bit::Set as BSet;

/// Used to search for entities with the requested components.
pub struct Filter {
    pub(super) bits: BSet,
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter {
    /// Creates an empty filter which matches everything
    pub fn new() -> Filter {
        Filter { bits: BSet::new(0) }
    }

    /// Adds the component to the filter.
    pub fn with<T>(mut self, key: Key<T>) -> Self {
        if self.bits.capacity() <= key.id {
            self.bits.resize(key.id + 1);
        }
        self.bits.set(key.id, true);
        self
    }

    pub(super) fn bits(&self) -> &BSet {
        &self.bits
    }
}

pub(super) fn entity_matches_filter(state: &EntityState, filter: &Filter) -> bool {
    !state.removed && state.components.includes_set(filter.bits())
}

pub(super) fn component_ids_are_unique(component_ids: &[usize]) -> bool {
    for (position, component_id) in component_ids.iter().enumerate() {
        if component_ids[position + 1..].contains(component_id) {
            return false;
        }
    }
    true
}
