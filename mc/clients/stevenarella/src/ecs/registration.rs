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

use super::entity::Entity;
use super::query::Filter;
use super::Manager;
use crate::render;
use crate::world;

/// A system processes entities
pub trait System {
    fn filter(&self) -> &Filter;
    fn update(
        &mut self,
        m: &mut Manager,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    );

    fn entity_added(
        &mut self,
        _m: &mut Manager,
        _e: Entity,
        _world: &mut world::World,
        _renderer: &mut render::Renderer,
    ) {
    }

    fn entity_removed(
        &mut self,
        _m: &mut Manager,
        _e: Entity,
        _world: &mut world::World,
        _renderer: &mut render::Renderer,
    ) {
    }
}

pub(super) type SystemList = Vec<Box<dyn System + Send>>;

pub(super) fn empty_systems() -> SystemList {
    vec![]
}

pub(super) fn register_system(systems: &mut SystemList, system: Box<dyn System + Send>) {
    systems.push(system);
}

#[cfg(test)]
pub(super) fn registered_system_count(systems: &SystemList) -> usize {
    systems.len()
}
