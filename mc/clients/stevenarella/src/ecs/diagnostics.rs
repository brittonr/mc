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

pub(super) const MISSING_ENTITY_MESSAGE: &str = "Missing entity";
pub(super) const DOUBLE_CHANGE_MESSAGE: &str = "Double change within a single tick";
pub(super) const DUPLICATE_ADD_MESSAGE: &str = "Duplicate add";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ComponentAddDecision {
    Insert,
    MissingEntity,
    DoubleChangeWithinSingleTick,
    DuplicateAdd,
}

pub(super) fn classify_component_add(
    entity_slot_present: bool,
    generation_matches: bool,
    entity_state_present: bool,
    changed_this_tick: bool,
    component_present: bool,
) -> ComponentAddDecision {
    if !entity_slot_present || !generation_matches || !entity_state_present {
        ComponentAddDecision::MissingEntity
    } else if changed_this_tick {
        ComponentAddDecision::DoubleChangeWithinSingleTick
    } else if component_present {
        ComponentAddDecision::DuplicateAdd
    } else {
        ComponentAddDecision::Insert
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ComponentRemoveDecision {
    Remove,
    NoComponent,
    MissingEntity,
    DoubleChangeWithinSingleTick,
}

pub(super) fn classify_component_remove(
    component_storage_present: bool,
    entity_slot_present: bool,
    generation_matches: bool,
    entity_state_present: bool,
    changed_this_tick: bool,
    component_present: bool,
) -> ComponentRemoveDecision {
    if !component_storage_present {
        ComponentRemoveDecision::NoComponent
    } else if !entity_slot_present || !generation_matches || !entity_state_present {
        ComponentRemoveDecision::MissingEntity
    } else if changed_this_tick {
        ComponentRemoveDecision::DoubleChangeWithinSingleTick
    } else if component_present {
        ComponentRemoveDecision::Remove
    } else {
        ComponentRemoveDecision::NoComponent
    }
}

pub(super) fn add_decision_message(decision: ComponentAddDecision) -> Option<&'static str> {
    match decision {
        ComponentAddDecision::Insert => None,
        ComponentAddDecision::MissingEntity => Some(MISSING_ENTITY_MESSAGE),
        ComponentAddDecision::DoubleChangeWithinSingleTick => Some(DOUBLE_CHANGE_MESSAGE),
        ComponentAddDecision::DuplicateAdd => Some(DUPLICATE_ADD_MESSAGE),
    }
}

pub(super) fn remove_decision_message(decision: ComponentRemoveDecision) -> Option<&'static str> {
    match decision {
        ComponentRemoveDecision::Remove | ComponentRemoveDecision::NoComponent => None,
        ComponentRemoveDecision::MissingEntity => Some(MISSING_ENTITY_MESSAGE),
        ComponentRemoveDecision::DoubleChangeWithinSingleTick => Some(DOUBLE_CHANGE_MESSAGE),
    }
}
