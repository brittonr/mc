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

mod components;
mod diagnostics;
mod entity;
mod execution;
mod query;
mod registration;

pub use components::Key;
pub use entity::Entity;
pub use query::Filter;
pub use registration::System;

use crate::render;
use crate::types::hash::FNVHash;
use crate::world;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;

const SYSTEMS_PRESENT_MESSAGE: &str = "systems are present outside system execution";
const RENDER_SYSTEMS_PRESENT_MESSAGE: &str = "render systems are present outside render execution";
const COMPONENT_STORAGE_PRESENT_MESSAGE: &str =
    "component storage exists after component storage setup";
const ENTITY_STATE_PRESENT_MESSAGE: &str =
    "entity state exists after component mutation classification";
const REUSED_ENTITY_PRESENT_MESSAGE: &str =
    "entity allocator selected an available reusable entity";

/// Stores and manages a collection of entities.
#[derive(Default)]
pub struct Manager {
    num_components: usize,
    entities: Vec<(Option<entity::EntityState>, u32)>,
    free_entities: Vec<usize>,
    components: Vec<Option<components::ComponentMem>>,

    component_ids: RefCell<HashMap<TypeId, usize, BuildHasherDefault<FNVHash>>>,

    systems: Option<registration::SystemList>,
    render_systems: Option<registration::SystemList>,

    changed_entity_components: HashSet<Entity, BuildHasherDefault<FNVHash>>,
}

impl Manager {
    /// Creates a new manager.
    pub fn new() -> Manager {
        Manager {
            num_components: 0,
            entities: vec![(
                Some(entity::EntityState::new(0)),
                entity::WORLD_ENTITY_GENERATION,
            )], // Has the world entity pre-defined
            free_entities: vec![],
            components: vec![],

            component_ids: RefCell::new(HashMap::with_hasher(BuildHasherDefault::default())),
            systems: Some(registration::empty_systems()),
            render_systems: Some(registration::empty_systems()),

            changed_entity_components: empty_changed_entities(),
        }
    }

    /// Returns the world entity. This should never be removed.
    pub fn get_world(&self) -> Entity {
        entity::world_entity()
    }

    /// Adds a system which will be called every tick
    pub fn add_system<S: System + Send + 'static>(&mut self, s: S) {
        registration::register_system(
            self.systems.as_mut().expect(SYSTEMS_PRESENT_MESSAGE),
            Box::new(s),
        );
    }

    /// Adds a system which will be called every frame
    pub fn add_render_system<S: System + Send + 'static>(&mut self, s: S) {
        registration::register_system(
            self.render_systems
                .as_mut()
                .expect(RENDER_SYSTEMS_PRESENT_MESSAGE),
            Box::new(s),
        );
    }

    /// Ticks all tick systems
    pub fn tick(&mut self, world: &mut world::World, renderer: &mut render::Renderer) {
        self.process_entity_changes(world, renderer);
        let mut systems = self.systems.take().expect(SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            systems[system_index].update(self, world, renderer);
        }
        self.systems = Some(systems);
        self.process_entity_changes(world, renderer);
    }

    /// Ticks all render systems
    pub fn render_tick(&mut self, world: &mut world::World, renderer: &mut render::Renderer) {
        self.process_entity_changes(world, renderer);
        let mut systems = self
            .render_systems
            .take()
            .expect(RENDER_SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            systems[system_index].update(self, world, renderer);
        }
        self.render_systems = Some(systems);
        self.process_entity_changes(world, renderer);
    }

    fn process_entity_changes(
        &mut self,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let changes = std::mem::replace(
            &mut self.changed_entity_components,
            empty_changed_entities(),
        );
        for entity in changes {
            let (current_components, original_state) = {
                let state = self.entities[entity.id]
                    .0
                    .as_mut()
                    .expect(ENTITY_STATE_PRESENT_MESSAGE);
                let current_components = state.components.clone();
                let original_state = state.clone();
                state.components.or(&state.last_components);
                (current_components, original_state)
            };
            self.trigger_add_for_systems(
                entity,
                &original_state.last_components,
                &original_state.components,
                world,
                renderer,
            );
            self.trigger_add_for_render_systems(
                entity,
                &original_state.last_components,
                &original_state.components,
                world,
                renderer,
            );
            self.trigger_remove_for_systems(
                entity,
                &original_state.last_components,
                &original_state.components,
                world,
                renderer,
            );
            self.trigger_remove_for_render_systems(
                entity,
                &original_state.last_components,
                &original_state.components,
                world,
                renderer,
            );
            for component_id in 0..self.components.len() {
                if !original_state.components.get(component_id)
                    && original_state.last_components.get(component_id)
                {
                    let components = self
                        .components
                        .get_mut(component_id)
                        .and_then(|v| v.as_mut())
                        .expect(COMPONENT_STORAGE_PRESENT_MESSAGE);
                    components.remove(entity.id);
                }
            }

            {
                let state = self.entities[entity.id]
                    .0
                    .as_mut()
                    .expect(ENTITY_STATE_PRESENT_MESSAGE);
                state.components = current_components;
                state.last_components = state.components.clone();
            }
            if original_state.removed {
                self.free_entities.push(entity.id);
                self.entities[entity.id].0 = None;
            }
        }
    }

    /// Returns all entities matching the filter
    pub fn find(&self, filter: &Filter) -> Vec<Entity> {
        let mut ret = vec![];
        // Skip the world entity.
        for (offset, &(ref state, generation)) in self.entities[entity::FIRST_ALLOCATED_ENTITY_ID..]
            .iter()
            .enumerate()
        {
            if let Some(state) = state.as_ref() {
                if query::entity_matches_filter(state, filter) {
                    ret.push(Entity::from_parts(
                        offset + entity::FIRST_ALLOCATED_ENTITY_ID,
                        generation,
                    ));
                }
            }
        }
        ret
    }

    /// Allocates a new entity without any components.
    pub fn create_entity(&mut self) -> Entity {
        match entity::allocation_decision(&self.free_entities, self.entities.len()) {
            entity::EntityAllocation::Reuse { entity_id } => {
                let reused_entity_id = self
                    .free_entities
                    .pop()
                    .expect(REUSED_ENTITY_PRESENT_MESSAGE);
                assert_eq!(reused_entity_id, entity_id);
                let entity = &mut self.entities[entity_id];
                entity.0 = Some(entity::EntityState::new(self.num_components));
                entity.1 += 1;
                Entity::from_parts(entity_id, entity.1)
            }
            entity::EntityAllocation::Append { entity_id } => {
                self.entities
                    .push((Some(entity::EntityState::new(self.num_components)), 0));
                Entity::from_parts(entity_id, 0)
            }
        }
    }

    /// Deallocates an entity and frees its components
    pub fn remove_entity(&mut self, e: Entity) {
        if let Some(state) = self.entities[e.id].0.as_mut() {
            state.components = crate::types::bit::Set::new(self.components.len());
            state.removed = true;
            self.changed_entity_components.insert(e);
        }
    }

    /// Deallocates all entities/components excluding the world entity
    pub fn remove_all_entities(
        &mut self,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        for (offset, e) in self.entities[entity::FIRST_ALLOCATED_ENTITY_ID..]
            .iter_mut()
            .enumerate()
        {
            if let Some(state) = e.0.as_mut() {
                state.components = crate::types::bit::Set::new(self.components.len());
                state.removed = true;
                self.changed_entity_components.insert(Entity::from_parts(
                    offset + entity::FIRST_ALLOCATED_ENTITY_ID,
                    e.1,
                ));
            }
        }
        self.process_entity_changes(world, renderer);
    }

    /// Returns whether an entity reference is valid.
    pub fn is_entity_valid(&self, e: Entity) -> bool {
        match self.entities.get(e.id) {
            Some(val) => entity::generation_matches(val.1, e) && val.0.is_some(),
            None => false,
        }
    }

    /// Gets a key for the component type. Creates one
    /// if the component has never been referenced before.
    pub fn get_key<T: Any>(&self) -> Key<T> {
        let mut ids = self.component_ids.borrow_mut();
        let next_id = ids.len();
        let id = ids.entry(TypeId::of::<T>()).or_insert(next_id);
        Key::from_id(*id)
    }

    /// Adds the component to the target entity
    /// # Panics
    /// Panics when the target entity doesn't exist
    pub fn add_component<T>(&mut self, entity: Entity, key: Key<T>, val: T) {
        self.ensure_component_storage::<T>(key);
        match self.classify_component_add(entity, key.id) {
            diagnostics::ComponentAddDecision::Insert => {}
            decision => panic!(
                "{}",
                diagnostics::add_decision_message(decision).expect(ENTITY_STATE_PRESENT_MESSAGE)
            ),
        }
        let state = self
            .entity_state_mut(entity)
            .expect(ENTITY_STATE_PRESENT_MESSAGE);
        state.components.set(key.id, true);
        self.changed_entity_components.insert(entity);
        let components = self
            .components
            .get_mut(key.id)
            .and_then(|v| v.as_mut())
            .expect(COMPONENT_STORAGE_PRESENT_MESSAGE);
        components.add(entity.id, val);
    }

    fn trigger_add_for_systems(
        &mut self,
        e: Entity,
        old_set: &crate::types::bit::Set,
        new_set: &crate::types::bit::Set,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.systems.take().expect(SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            let sys = &mut systems[system_index];
            if execution::should_trigger_add(old_set, new_set, sys.filter()) {
                sys.entity_added(self, e, world, renderer);
            }
        }
        self.systems = Some(systems);
    }

    fn trigger_add_for_render_systems(
        &mut self,
        e: Entity,
        old_set: &crate::types::bit::Set,
        new_set: &crate::types::bit::Set,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self
            .render_systems
            .take()
            .expect(RENDER_SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            let sys = &mut systems[system_index];
            if execution::should_trigger_add(old_set, new_set, sys.filter()) {
                sys.entity_added(self, e, world, renderer);
            }
        }
        self.render_systems = Some(systems);
    }

    /// Same as `add_component` but doesn't require a key. Using a key
    /// is better for frequent lookups.
    pub fn add_component_direct<T: Any>(&mut self, entity: Entity, val: T) {
        let key = self.get_key();
        self.add_component(entity, key, val);
    }

    /// Removes the component to the target entity. Returns whether anything
    /// was removed.
    /// # Panics
    /// Panics when the target entity doesn't exist
    pub fn remove_component<T>(&mut self, entity: Entity, key: Key<T>) -> bool {
        let component_storage_present = self
            .components
            .get(key.id)
            .and_then(|v| v.as_ref())
            .is_some();
        match self.classify_component_remove(entity, key.id, component_storage_present) {
            diagnostics::ComponentRemoveDecision::Remove => {}
            diagnostics::ComponentRemoveDecision::NoComponent => return false,
            decision => panic!(
                "{}",
                diagnostics::remove_decision_message(decision).expect(ENTITY_STATE_PRESENT_MESSAGE)
            ),
        }
        let state = self
            .entity_state_mut(entity)
            .expect(ENTITY_STATE_PRESENT_MESSAGE);
        state.components.set(key.id, false);
        self.changed_entity_components.insert(entity);
        // Actual removal is delayed until ticking finishes
        true
    }

    fn trigger_remove_for_systems(
        &mut self,
        e: Entity,
        old_set: &crate::types::bit::Set,
        new_set: &crate::types::bit::Set,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.systems.take().expect(SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            let sys = &mut systems[system_index];
            if execution::should_trigger_remove(old_set, new_set, sys.filter()) {
                sys.entity_removed(self, e, world, renderer);
            }
        }
        self.systems = Some(systems);
    }

    fn trigger_remove_for_render_systems(
        &mut self,
        e: Entity,
        old_set: &crate::types::bit::Set,
        new_set: &crate::types::bit::Set,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self
            .render_systems
            .take()
            .expect(RENDER_SYSTEMS_PRESENT_MESSAGE);
        for system_index in execution::ordered_system_indices(systems.len()) {
            let sys = &mut systems[system_index];
            if execution::should_trigger_remove(old_set, new_set, sys.filter()) {
                sys.entity_removed(self, e, world, renderer);
            }
        }
        self.render_systems = Some(systems);
    }

    /// Same as `remove_component` but doesn't require a key. Using a key
    /// is better for frequent lookups.
    pub fn remove_component_direct<T: Any>(&mut self, entity: Entity) -> bool {
        let key = self.get_key();
        self.remove_component::<T>(entity, key)
    }

    /// Returns the given component that the key points to if it exists.
    pub fn get_component<T>(&self, entity: Entity, key: Key<T>) -> Option<&T> {
        let components = self.components.get(key.id).and_then(|v| v.as_ref())?;
        let state = self.entity_state(entity)?;
        if !state.has_component(key.id) {
            return None;
        }

        components.get(entity.id)
    }

    /// Same as `get_component` but doesn't require a key. Using a key
    /// is better for frequent lookups.
    pub fn get_component_direct<T: Any>(&self, entity: Entity) -> Option<&T> {
        let key = self.get_key();
        self.get_component(entity, key)
    }

    /// Returns the given component that the key points to if it exists.
    pub fn get_component_mut<T>(&mut self, entity: Entity, key: Key<T>) -> Option<&mut T> {
        if !self.entity_has_components(entity, &[key.id]) {
            return None;
        }
        let components = self.components.get_mut(key.id).and_then(|v| v.as_mut())?;

        components.get_mut(entity.id)
    }

    /// Same as `get_component_mut` but doesn't require a key. Using a key
    /// is better for frequent lookups.
    pub fn get_component_mut_direct<T: Any>(&mut self, entity: Entity) -> Option<&mut T> {
        let key = self.get_key();
        self.get_component_mut(entity, key)
    }

    pub fn get_two_components_mut<A, B>(
        &mut self,
        entity: Entity,
        key_a: Key<A>,
        key_b: Key<B>,
    ) -> Option<(&mut A, &mut B)> {
        let component_ids = [key_a.id, key_b.id];
        if !query::component_ids_are_unique(&component_ids)
            || !self.entity_has_components(entity, &component_ids)
        {
            return None;
        }
        let component_a = self.component_mem_mut_ptr(key_a.id)?;
        let component_b = self.component_mem_mut_ptr(key_b.id)?;
        unsafe {
            Some((
                (*component_a).get_mut(entity.id)?,
                (*component_b).get_mut(entity.id)?,
            ))
        }
    }

    pub fn get_three_components_mut<A, B, C>(
        &mut self,
        entity: Entity,
        key_a: Key<A>,
        key_b: Key<B>,
        key_c: Key<C>,
    ) -> Option<(&mut A, &mut B, &mut C)> {
        let component_ids = [key_a.id, key_b.id, key_c.id];
        if !query::component_ids_are_unique(&component_ids)
            || !self.entity_has_components(entity, &component_ids)
        {
            return None;
        }
        let component_a = self.component_mem_mut_ptr(key_a.id)?;
        let component_b = self.component_mem_mut_ptr(key_b.id)?;
        let component_c = self.component_mem_mut_ptr(key_c.id)?;
        unsafe {
            Some((
                (*component_a).get_mut(entity.id)?,
                (*component_b).get_mut(entity.id)?,
                (*component_c).get_mut(entity.id)?,
            ))
        }
    }

    pub fn get_four_components_mut<A, B, C, D>(
        &mut self,
        entity: Entity,
        key_a: Key<A>,
        key_b: Key<B>,
        key_c: Key<C>,
        key_d: Key<D>,
    ) -> Option<(&mut A, &mut B, &mut C, &mut D)> {
        let component_ids = [key_a.id, key_b.id, key_c.id, key_d.id];
        if !query::component_ids_are_unique(&component_ids)
            || !self.entity_has_components(entity, &component_ids)
        {
            return None;
        }
        let component_a = self.component_mem_mut_ptr(key_a.id)?;
        let component_b = self.component_mem_mut_ptr(key_b.id)?;
        let component_c = self.component_mem_mut_ptr(key_c.id)?;
        let component_d = self.component_mem_mut_ptr(key_d.id)?;
        unsafe {
            Some((
                (*component_a).get_mut(entity.id)?,
                (*component_b).get_mut(entity.id)?,
                (*component_c).get_mut(entity.id)?,
                (*component_d).get_mut(entity.id)?,
            ))
        }
    }

    fn entity_has_components(&self, entity: Entity, component_ids: &[usize]) -> bool {
        let Some(state) = self.entity_state(entity) else {
            return false;
        };
        component_ids.iter().all(|id| state.has_component(*id))
    }

    fn component_mem_mut_ptr(
        &mut self,
        component_id: usize,
    ) -> Option<*mut components::ComponentMem> {
        self.components
            .get_mut(component_id)
            .and_then(|v| v.as_mut())
            .map(|component| component as *mut components::ComponentMem)
    }

    fn ensure_component_storage<T>(&mut self, key: Key<T>) {
        while self.components.len() <= key.id {
            self.components.push(None);
        }
        if self.components[key.id].is_none() {
            self.components[key.id] = Some(components::ComponentMem::new::<T>());
            self.num_components += 1;
            for &mut (ref mut state, _) in &mut self.entities {
                if let Some(state) = state.as_mut() {
                    state.last_components.resize(self.num_components);
                    state.components.resize(self.num_components);
                }
            }
        }
    }

    fn entity_state(&self, entity: Entity) -> Option<&entity::EntityState> {
        match self.entities.get(entity.id) {
            Some(val) if entity::generation_matches(val.1, entity) => val.0.as_ref(),
            _ => None,
        }
    }

    fn entity_state_mut(&mut self, entity: Entity) -> Option<&mut entity::EntityState> {
        match self.entities.get_mut(entity.id) {
            Some(val) if entity::generation_matches(val.1, entity) => val.0.as_mut(),
            _ => None,
        }
    }

    fn classify_component_add(
        &self,
        entity: Entity,
        component_id: usize,
    ) -> diagnostics::ComponentAddDecision {
        match self.entities.get(entity.id) {
            Some((state, generation)) => diagnostics::classify_component_add(
                true,
                entity::generation_matches(*generation, entity),
                state.is_some(),
                state.as_ref().map_or(false, |state| {
                    state.component_changed_this_tick(component_id)
                }),
                state
                    .as_ref()
                    .map_or(false, |state| state.has_component(component_id)),
            ),
            None => diagnostics::classify_component_add(false, false, false, false, false),
        }
    }

    fn classify_component_remove(
        &self,
        entity: Entity,
        component_id: usize,
        component_storage_present: bool,
    ) -> diagnostics::ComponentRemoveDecision {
        if !component_storage_present {
            return diagnostics::classify_component_remove(
                false, false, false, false, false, false,
            );
        }
        match self.entities.get(entity.id) {
            Some((state, generation)) => diagnostics::classify_component_remove(
                component_storage_present,
                true,
                entity::generation_matches(*generation, entity),
                state.is_some(),
                state.as_ref().map_or(false, |state| {
                    state.component_changed_this_tick(component_id)
                }),
                state
                    .as_ref()
                    .map_or(false, |state| state.has_component(component_id)),
            ),
            None => diagnostics::classify_component_remove(
                component_storage_present,
                false,
                false,
                false,
                false,
                false,
            ),
        }
    }
}

fn empty_changed_entities() -> HashSet<Entity, BuildHasherDefault<FNVHash>> {
    HashSet::with_hasher(BuildHasherDefault::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    const TEST_COMPONENT_VALUE: i32 = 7;
    const UPDATED_COMPONENT_VALUE: i32 = 11;
    const STALE_GENERATION_OFFSET: u32 = 1;
    const TEST_COMPONENT_INDEX: usize = components::COMPONENTS_PER_BLOCK + 1;
    const REUSABLE_ENTITY_A: usize = 4;
    const REUSABLE_ENTITY_B: usize = 5;
    const NEXT_ENTITY_ID: usize = 9;
    const TEST_SYSTEM_COUNT: usize = 3;
    const FIRST_SYSTEM_INDEX: usize = 0;
    const SECOND_SYSTEM_INDEX: usize = 1;
    const THIRD_SYSTEM_INDEX: usize = 2;
    const EMPTY_SYSTEM_COUNT: usize = 0;
    const QUERY_COMPONENT_ID_A: usize = 6;
    const QUERY_COMPONENT_ID_B: usize = 8;

    #[derive(Debug, PartialEq, Eq)]
    struct TestComponent {
        value: i32,
    }

    struct OtherComponent {
        value: i32,
    }

    struct ThirdComponent {
        value: i32,
    }

    struct FourthComponent {
        value: i32,
    }

    struct DropCounter {
        drops: Arc<AtomicUsize>,
    }

    impl Drop for DropCounter {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    struct NoopSystem {
        filter: Filter,
    }

    impl System for NoopSystem {
        fn filter(&self) -> &Filter {
            &self.filter
        }

        fn update(
            &mut self,
            _m: &mut Manager,
            _world: &mut world::World,
            _renderer: &mut render::Renderer,
        ) {
        }
    }

    #[test]
    fn ecs_entity_allocation_core_reuses_last_free_entity() {
        let free_entities = [REUSABLE_ENTITY_A, REUSABLE_ENTITY_B];

        assert_eq!(
            entity::allocation_decision(&free_entities, NEXT_ENTITY_ID),
            entity::EntityAllocation::Reuse {
                entity_id: REUSABLE_ENTITY_B,
            }
        );
    }

    #[test]
    fn ecs_entity_allocation_core_appends_without_free_entity() {
        let free_entities = [];

        assert_eq!(
            entity::allocation_decision(&free_entities, NEXT_ENTITY_ID),
            entity::EntityAllocation::Append {
                entity_id: NEXT_ENTITY_ID,
            }
        );
    }

    #[test]
    fn ecs_component_access_reads_and_mutates_valid_entity() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();

        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            TEST_COMPONENT_VALUE
        );
        manager.get_component_mut(entity, key).unwrap().value = UPDATED_COMPONENT_VALUE;

        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
    }

    #[test]
    fn ecs_component_insert_remove_get_round_trip() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            TEST_COMPONENT_VALUE
        );
        let state = manager.entities[entity.id]
            .0
            .as_mut()
            .expect(ENTITY_STATE_PRESENT_MESSAGE);
        state.last_components.set(key.id, true);

        assert!(manager.remove_component(entity, key));
        assert!(manager.get_component(entity, key).is_none());
    }

    #[test]
    fn ecs_component_access_returns_none_for_absent_component() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        let absent_key = manager.get_key::<OtherComponent>();

        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        assert!(manager.get_component(entity, absent_key).is_none());
        assert!(manager.get_component_mut(entity, absent_key).is_none());
    }

    #[test]
    fn ecs_query_matching_finds_entity_with_component() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        let filter = Filter::new().with(key);

        assert_eq!(manager.find(&filter), vec![entity]);
    }

    #[test]
    fn ecs_multi_component_access_mutates_distinct_components() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        let other_key = manager.get_key::<OtherComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            other_key,
            OtherComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        let (first, second) = manager
            .get_two_components_mut(entity, key, other_key)
            .unwrap();
        first.value = UPDATED_COMPONENT_VALUE;
        second.value = UPDATED_COMPONENT_VALUE;

        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, other_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
    }

    #[test]
    fn ecs_three_component_access_mutates_distinct_components() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        let other_key = manager.get_key::<OtherComponent>();
        let third_key = manager.get_key::<ThirdComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            other_key,
            OtherComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            third_key,
            ThirdComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        let (first, second, third) = manager
            .get_three_components_mut(entity, key, other_key, third_key)
            .unwrap();
        first.value = UPDATED_COMPONENT_VALUE;
        second.value = UPDATED_COMPONENT_VALUE;
        third.value = UPDATED_COMPONENT_VALUE;

        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, other_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, third_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
    }

    #[test]
    fn ecs_four_component_access_mutates_distinct_components() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        let other_key = manager.get_key::<OtherComponent>();
        let third_key = manager.get_key::<ThirdComponent>();
        let fourth_key = manager.get_key::<FourthComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            other_key,
            OtherComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            third_key,
            ThirdComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        manager.add_component(
            entity,
            fourth_key,
            FourthComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        let (first, second, third, fourth) = manager
            .get_four_components_mut(entity, key, other_key, third_key, fourth_key)
            .unwrap();
        first.value = UPDATED_COMPONENT_VALUE;
        second.value = UPDATED_COMPONENT_VALUE;
        third.value = UPDATED_COMPONENT_VALUE;
        fourth.value = UPDATED_COMPONENT_VALUE;

        assert_eq!(
            manager.get_component(entity, key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, other_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, third_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
        assert_eq!(
            manager.get_component(entity, fourth_key).unwrap().value,
            UPDATED_COMPONENT_VALUE
        );
    }

    #[test]
    fn ecs_multi_component_access_rejects_duplicate_keys() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        assert!(manager.get_two_components_mut(entity, key, key).is_none());
    }

    #[test]
    fn ecs_component_access_returns_none_for_stale_generation() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        let stale = Entity::from_parts(entity.id, entity.generation + STALE_GENERATION_OFFSET);

        assert!(manager.get_component(stale, key).is_none());
        assert!(manager.get_component_mut(stale, key).is_none());
    }

    #[test]
    fn ecs_component_access_returns_none_after_entity_removal_request() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        manager.remove_entity(entity);

        assert!(manager.get_component(entity, key).is_none());
        assert!(manager.get_component_mut(entity, key).is_none());
    }

    #[test]
    fn ecs_duplicate_entity_generation_is_rejected() {
        let manager = Manager::new();
        let world_entity = manager.get_world();
        let duplicate_generation = Entity::from_parts(
            world_entity.id,
            world_entity.generation + STALE_GENERATION_OFFSET,
        );

        assert!(!manager.is_entity_valid(duplicate_generation));
    }

    #[test]
    fn ecs_remove_component_returns_false_for_missing_component() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();

        assert!(!manager.remove_component(entity, key));
    }

    #[test]
    #[should_panic(expected = "Double change within a single tick")]
    fn ecs_add_component_reports_double_change_before_duplicate_add() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );

        manager.add_component(
            entity,
            key,
            TestComponent {
                value: UPDATED_COMPONENT_VALUE,
            },
        );
    }

    #[test]
    #[should_panic(expected = "Duplicate add")]
    fn ecs_add_component_reports_duplicate_add_after_stable_component() {
        let mut manager = Manager::new();
        let entity = manager.create_entity();
        let key = manager.get_key::<TestComponent>();
        manager.add_component(
            entity,
            key,
            TestComponent {
                value: TEST_COMPONENT_VALUE,
            },
        );
        let state = manager.entities[entity.id]
            .0
            .as_mut()
            .expect(ENTITY_STATE_PRESENT_MESSAGE);
        state.last_components.set(key.id, true);

        manager.add_component(
            entity,
            key,
            TestComponent {
                value: UPDATED_COMPONENT_VALUE,
            },
        );
    }

    #[test]
    fn ecs_component_storage_drops_removed_component_once() {
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let mut storage = components::ComponentMem::new::<DropCounter>();
            storage.add(
                TEST_COMPONENT_INDEX,
                DropCounter {
                    drops: drops.clone(),
                },
            );

            storage.remove(TEST_COMPONENT_INDEX);
            storage.remove(TEST_COMPONENT_INDEX);
        }

        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ecs_component_storage_returns_none_for_missing_slot() {
        let mut storage = components::ComponentMem::new::<TestComponent>();

        assert!(storage.get::<TestComponent>(TEST_COMPONENT_INDEX).is_none());
        assert!(storage
            .get_mut::<TestComponent>(TEST_COMPONENT_INDEX)
            .is_none());
    }

    #[test]
    fn ecs_system_registration_records_system_count() {
        let mut systems = registration::empty_systems();

        registration::register_system(
            &mut systems,
            Box::new(NoopSystem {
                filter: Filter::new(),
            }),
        );

        assert_eq!(registration::registered_system_count(&systems), 1);
    }

    #[test]
    fn ecs_system_ordering_core_preserves_registration_order() {
        assert_eq!(
            execution::ordered_system_indices(TEST_SYSTEM_COUNT),
            vec![FIRST_SYSTEM_INDEX, SECOND_SYSTEM_INDEX, THIRD_SYSTEM_INDEX]
        );
    }

    #[test]
    fn ecs_empty_system_order_has_no_entries() {
        assert!(execution::ordered_system_indices(EMPTY_SYSTEM_COUNT).is_empty());
    }

    #[test]
    fn ecs_query_shape_accepts_unique_component_ids() {
        let component_ids = [QUERY_COMPONENT_ID_A, QUERY_COMPONENT_ID_B];

        assert!(query::component_ids_are_unique(&component_ids));
    }

    #[test]
    fn ecs_invalid_query_shape_rejects_duplicate_component_ids() {
        let component_ids = [QUERY_COMPONENT_ID_A, QUERY_COMPONENT_ID_A];

        assert!(!query::component_ids_are_unique(&component_ids));
    }

    #[test]
    fn ecs_diagnostics_report_insertable_component_add() {
        assert_eq!(
            diagnostics::classify_component_add(true, true, true, false, false),
            diagnostics::ComponentAddDecision::Insert
        );
    }

    #[test]
    fn ecs_diagnostics_report_missing_component_remove() {
        assert_eq!(
            diagnostics::classify_component_remove(true, true, true, true, false, false),
            diagnostics::ComponentRemoveDecision::NoComponent
        );
    }
}
