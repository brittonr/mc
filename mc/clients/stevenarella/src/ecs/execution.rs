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

use super::query::Filter;
use crate::types::bit::Set as BSet;

pub(super) fn ordered_system_indices(system_count: usize) -> Vec<usize> {
    (0..system_count).collect()
}

pub(super) fn should_trigger_add(old_set: &BSet, new_set: &BSet, filter: &Filter) -> bool {
    new_set.includes_set(filter.bits()) && !old_set.includes_set(filter.bits())
}

pub(super) fn should_trigger_remove(old_set: &BSet, new_set: &BSet, filter: &Filter) -> bool {
    !new_set.includes_set(filter.bits()) && old_set.includes_set(filter.bits())
}
