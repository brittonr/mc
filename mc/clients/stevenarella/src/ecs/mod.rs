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
use crate::types::hash::FNVHash;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use crate::render;
use crate::world;

/// Used to reference an entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Entity {
    id: usize,
    generation: u32,
}

/// Used to access compoents on an entity in an efficient
/// way.
pub struct Key<T> {
    id: usize,
    _t: PhantomData<T>,
}
impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Key {
            id: self.id,
            _t: PhantomData,
        }
    }
}
impl<T> Copy for Key<T> {}

/// Used to search for entities with the requested components.
pub struct Filter {
    bits: BSet,
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
}

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

#[derive(Clone)]
struct EntityState {
    last_components: BSet,
    components: BSet,
    removed: bool,
}

/// Stores and manages a collection of entities.
#[derive(Default)]
pub struct Manager {
    num_components: usize,
    entities: Vec<(Option<EntityState>, u32)>,
    free_entities: Vec<usize>,
    components: Vec<Option<ComponentMem>>,

    component_ids: RefCell<HashMap<TypeId, usize, BuildHasherDefault<FNVHash>>>,

    systems: Option<Vec<Box<dyn System + Send>>>,
    render_systems: Option<Vec<Box<dyn System + Send>>>,

    changed_entity_components: HashSet<Entity, BuildHasherDefault<FNVHash>>,
}

impl Manager {
    /// Creates a new manager.
    pub fn new() -> Manager {
        Manager {
            num_components: 0,
            entities: vec![(
                Some(EntityState {
                    last_components: BSet::new(0),
                    components: BSet::new(0),
                    removed: false,
                }),
                0,
            )], // Has the world entity pre-defined
            free_entities: vec![],
            components: vec![],

            component_ids: RefCell::new(HashMap::with_hasher(BuildHasherDefault::default())),
            systems: Some(vec![]),
            render_systems: Some(vec![]),

            changed_entity_components: HashSet::with_hasher(BuildHasherDefault::default()),
        }
    }

    /// Returns the world entity. This should never be removed.
    pub fn get_world(&self) -> Entity {
        Entity {
            id: 0,
            generation: 0,
        }
    }

    /// Adds a system which will be called every tick
    pub fn add_system<S: System + Send + 'static>(&mut self, s: S) {
        self.systems.as_mut().unwrap().push(Box::new(s));
    }

    /// Adds a system which will be called every frame
    pub fn add_render_system<S: System + Send + 'static>(&mut self, s: S) {
        self.render_systems.as_mut().unwrap().push(Box::new(s));
    }

    /// Ticks all tick systems
    pub fn tick(&mut self, world: &mut world::World, renderer: &mut render::Renderer) {
        self.process_entity_changes(world, renderer);
        let mut systems = self.systems.take().unwrap();
        for sys in &mut systems {
            sys.update(self, world, renderer);
        }
        self.systems = Some(systems);
        self.process_entity_changes(world, renderer);
    }

    /// Ticks all render systems
    pub fn render_tick(&mut self, world: &mut world::World, renderer: &mut render::Renderer) {
        self.process_entity_changes(world, renderer);
        let mut systems = self.render_systems.take().unwrap();
        for sys in &mut systems {
            sys.update(self, world, renderer);
        }
        self.render_systems = Some(systems);
        self.process_entity_changes(world, renderer);
    }

    fn process_entity_changes(
        &mut self,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let changes = self.changed_entity_components.clone();
        self.changed_entity_components = HashSet::with_hasher(BuildHasherDefault::default());
        for entity in changes {
            let (cur, state) = {
                let state = self.entities[entity.id].0.as_mut().unwrap();
                let cur = state.components.clone();
                let orig = state.clone();
                state.components.or(&state.last_components);
                (cur, orig)
            };
            self.trigger_add_for_systems(
                entity,
                &state.last_components,
                &state.components,
                world,
                renderer,
            );
            self.trigger_add_for_render_systems(
                entity,
                &state.last_components,
                &state.components,
                world,
                renderer,
            );
            self.trigger_remove_for_systems(
                entity,
                &state.last_components,
                &state.components,
                world,
                renderer,
            );
            self.trigger_remove_for_render_systems(
                entity,
                &state.last_components,
                &state.components,
                world,
                renderer,
            );
            for i in 0..self.components.len() {
                if !state.components.get(i) && state.last_components.get(i) {
                    let components = self.components.get_mut(i).and_then(|v| v.as_mut()).unwrap();
                    components.remove(entity.id);
                }
            }

            {
                let state = self.entities[entity.id].0.as_mut().unwrap();
                state.components = cur;
                state.last_components = state.components.clone();
            }
            if state.removed {
                self.free_entities.push(entity.id);
                self.entities[entity.id].0 = None;
            }
        }
    }

    /// Returns all entities matching the filter
    pub fn find(&self, filter: &Filter) -> Vec<Entity> {
        let mut ret = vec![];
        // Skip the world entity.
        for (i, &(ref set, gen)) in self.entities[1..].iter().enumerate() {
            if let Some(set) = set.as_ref() {
                if !set.removed && set.components.includes_set(&filter.bits) {
                    ret.push(Entity {
                        id: i + 1,
                        generation: gen,
                    });
                }
            }
        }
        ret
    }

    /// Allocates a new entity without any components.
    pub fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.free_entities.pop() {
            let entity = &mut self.entities[id];
            entity.0 = Some(EntityState {
                last_components: BSet::new(self.num_components),
                components: BSet::new(self.num_components),
                removed: false,
            });
            entity.1 += 1;
            return Entity {
                id,
                generation: entity.1,
            };
        }
        let id = self.entities.len();
        self.entities.push((
            Some(EntityState {
                last_components: BSet::new(self.num_components),
                components: BSet::new(self.num_components),
                removed: false,
            }),
            0,
        ));
        Entity { id, generation: 0 }
    }

    /// Deallocates an entity and frees its components
    pub fn remove_entity(&mut self, e: Entity) {
        if let Some(set) = self.entities[e.id].0.as_mut() {
            set.components = BSet::new(self.components.len());
            set.removed = true;
            self.changed_entity_components.insert(e);
        }
    }

    /// Deallocates all entities/components excluding the world entity
    pub fn remove_all_entities(
        &mut self,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        for (id, e) in self.entities[1..].iter_mut().enumerate() {
            if let Some(set) = e.0.as_mut() {
                set.components = BSet::new(self.components.len());
                set.removed = true;
                self.changed_entity_components.insert(Entity {
                    id: id + 1,
                    generation: e.1,
                });
            }
        }
        self.process_entity_changes(world, renderer);
    }

    /// Returns whether an entity reference is valid.
    pub fn is_entity_valid(&self, e: Entity) -> bool {
        match self.entities.get(e.id) {
            Some(val) => val.1 == e.generation && val.0.is_some(),
            None => false,
        }
    }

    /// Gets a key for the component type. Creates one
    /// if the component has never been referenced before.
    pub fn get_key<T: Any>(&self) -> Key<T> {
        let mut ids = self.component_ids.borrow_mut();
        let next_id = ids.len();
        let id = ids.entry(TypeId::of::<T>()).or_insert(next_id);
        Key {
            id: *id,
            _t: PhantomData,
        }
    }

    /// Adds the component to the target entity
    /// # Panics
    /// Panics when the target entity doesn't exist
    pub fn add_component<T>(&mut self, entity: Entity, key: Key<T>, val: T) {
        if self.components.len() <= key.id {
            while self.components.len() <= key.id {
                self.components.push(None);
            }
        }
        if self.components[key.id].is_none() {
            self.components[key.id] = Some(ComponentMem::new::<T>());
            self.num_components += 1;
            for &mut (ref mut set, _) in &mut self.entities {
                if let Some(set) = set.as_mut() {
                    set.last_components.resize(self.num_components);
                    set.components.resize(self.num_components);
                }
            }
        }
        let mut e = self.entities.get_mut(entity.id);
        let set = match e {
            Some(ref mut val) => {
                if val.1 == entity.generation {
                    &mut val.0
                } else {
                    panic!("Missing entity")
                }
            }
            None => panic!("Missing entity"),
        };
        let set = match set.as_mut() {
            Some(val) => val,
            None => panic!("Missing entity"),
        };
        if set.components.get(key.id) != set.last_components.get(key.id) {
            panic!("Double change within a single tick");
        }
        if set.components.get(key.id) {
            panic!("Duplicate add");
        }
        set.components.set(key.id, true);
        self.changed_entity_components.insert(entity);
        let components = self
            .components
            .get_mut(key.id)
            .and_then(|v| v.as_mut())
            .unwrap();
        components.add(entity.id, val);
    }

    fn trigger_add_for_systems(
        &mut self,
        e: Entity,
        old_set: &BSet,
        new_set: &BSet,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.systems.take().unwrap();
        for sys in &mut systems {
            if new_set.includes_set(&sys.filter().bits) && !old_set.includes_set(&sys.filter().bits)
            {
                sys.entity_added(self, e, world, renderer);
            }
        }
        self.systems = Some(systems);
    }

    fn trigger_add_for_render_systems(
        &mut self,
        e: Entity,
        old_set: &BSet,
        new_set: &BSet,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.render_systems.take().unwrap();
        for sys in &mut systems {
            if new_set.includes_set(&sys.filter().bits) && !old_set.includes_set(&sys.filter().bits)
            {
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
        if self.components.len() <= key.id {
            return false;
        }
        if self.components[key.id].is_none() {
            return false;
        }
        let mut e = self.entities.get_mut(entity.id);
        let set = match e {
            Some(ref mut val) => {
                if val.1 == entity.generation {
                    &mut val.0
                } else {
                    panic!("Missing entity")
                }
            }
            None => panic!("Missing entity"),
        };
        let set = match set.as_mut() {
            Some(val) => val,
            None => panic!("Missing entity"),
        };
        if set.components.get(key.id) != set.last_components.get(key.id) {
            panic!("Double change within a single tick");
        }
        if !set.components.get(key.id) {
            return false;
        }
        set.components.set(key.id, false);
        self.changed_entity_components.insert(entity);
        // Actual removal is delayed until ticking finishes
        true
    }

    fn trigger_remove_for_systems(
        &mut self,
        e: Entity,
        old_set: &BSet,
        new_set: &BSet,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.systems.take().unwrap();
        for sys in &mut systems {
            if !new_set.includes_set(&sys.filter().bits) && old_set.includes_set(&sys.filter().bits)
            {
                sys.entity_removed(self, e, world, renderer);
            }
        }
        self.systems = Some(systems);
    }

    fn trigger_remove_for_render_systems(
        &mut self,
        e: Entity,
        old_set: &BSet,
        new_set: &BSet,
        world: &mut world::World,
        renderer: &mut render::Renderer,
    ) {
        let mut systems = self.render_systems.take().unwrap();
        for sys in &mut systems {
            if !new_set.includes_set(&sys.filter().bits) && old_set.includes_set(&sys.filter().bits)
            {
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
        let set = match self.entities.get(entity.id) {
            Some(val) if val.1 == entity.generation => &val.0,
            _ => return None,
        };
        if !set.as_ref().map_or(false, |v| v.components.get(key.id)) {
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
        let set = match self.entities.get(entity.id) {
            Some(val) if val.1 == entity.generation => &val.0,
            _ => return None,
        };
        if !set.as_ref().map_or(false, |v| v.components.get(key.id)) {
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
        if !unique_component_ids(&component_ids)
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
        if !unique_component_ids(&component_ids)
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
        if !unique_component_ids(&component_ids)
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
        let state = match self.entities.get(entity.id) {
            Some(val) if val.1 == entity.generation => match val.0.as_ref() {
                Some(state) => state,
                None => return false,
            },
            _ => return false,
        };
        component_ids.iter().all(|id| state.components.get(*id))
    }

    fn component_mem_mut_ptr(&mut self, component_id: usize) -> Option<*mut ComponentMem> {
        self.components
            .get_mut(component_id)
            .and_then(|v| v.as_mut())
            .map(|component| component as *mut ComponentMem)
    }
}

fn unique_component_ids(component_ids: &[usize]) -> bool {
    for (position, component_id) in component_ids.iter().enumerate() {
        if component_ids[position + 1..].contains(component_id) {
            return false;
        }
    }
    true
}

const COMPONENTS_PER_BLOCK: usize = 64;

struct ComponentMem {
    data: Vec<Option<(Vec<u8>, BSet, usize)>>,
    component_size: usize,
    drop_func: Box<dyn Fn(*mut u8) + Send>,
}

impl ComponentMem {
    fn new<T>() -> ComponentMem {
        ComponentMem {
            data: vec![],
            component_size: mem::size_of::<T>(),
            drop_func: Box::new(|data| unsafe {
                let mut val = mem::MaybeUninit::<T>::uninit();
                ptr::copy(data as *mut T, val.as_mut_ptr(), 1);
                val.assume_init_drop();
            }),
        }
    }

    fn add<T>(&mut self, index: usize, val: T) {
        while self.data.len() < (index / COMPONENTS_PER_BLOCK) + 1 {
            self.data.push(None);
        }
        let idx = index / COMPONENTS_PER_BLOCK;
        let rem = index % COMPONENTS_PER_BLOCK;
        if self.data[idx].is_none() {
            self.data[idx] = Some((
                vec![0; self.component_size * COMPONENTS_PER_BLOCK],
                BSet::new(COMPONENTS_PER_BLOCK),
                0,
            ));
        }
        let data = self.data[idx].as_mut().unwrap();
        let start = rem * self.component_size;
        data.2 += 1;
        data.1.set(rem, true);
        unsafe {
            ptr::write(data.0.as_mut_ptr().add(start) as *mut T, val);
        }
    }

    fn remove(&mut self, index: usize) {
        let idx = index / COMPONENTS_PER_BLOCK;
        let rem = index % COMPONENTS_PER_BLOCK;

        let count = {
            let Some(data) = self.data.get_mut(idx).and_then(|v| v.as_mut()) else {
                return;
            };
            if !data.1.get(rem) {
                return;
            }
            let start = rem * self.component_size;
            data.1.set(rem, false);
            // We don't have access to the actual type in this method so
            // we use the drop_func which stores the type in its closure
            // to handle the dropping for us.
            unsafe {
                (self.drop_func)(data.0.as_mut_ptr().add(start));
            }
            data.2 -= 1;
            data.2
        };
        if count == 0 {
            self.data[idx] = None;
        }
    }

    fn get<T>(&self, index: usize) -> Option<&T> {
        let idx = index / COMPONENTS_PER_BLOCK;
        let rem = index % COMPONENTS_PER_BLOCK;
        let data = self.data.get(idx).and_then(|v| v.as_ref())?;
        if !data.1.get(rem) {
            return None;
        }
        let start = rem * self.component_size;
        unsafe { Some(&*(data.0.as_ptr().add(start) as *const T)) }
    }

    fn get_mut<T>(&mut self, index: usize) -> Option<&mut T> {
        let idx = index / COMPONENTS_PER_BLOCK;
        let rem = index % COMPONENTS_PER_BLOCK;
        let data = self.data.get_mut(idx).and_then(|v| v.as_mut())?;
        if !data.1.get(rem) {
            return None;
        }
        let start = rem * self.component_size;
        unsafe { Some(&mut *(data.0.as_mut_ptr().add(start) as *mut T)) }
    }
}

impl Drop for ComponentMem {
    fn drop(&mut self) {
        for data in &mut self.data {
            if let Some(data) = data.as_mut() {
                for i in 0..COMPONENTS_PER_BLOCK {
                    if data.1.get(i) {
                        let start = i * self.component_size;
                        unsafe {
                            (self.drop_func)(data.0.as_mut_ptr().add(start));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    const TEST_COMPONENT_VALUE: i32 = 7;
    const UPDATED_COMPONENT_VALUE: i32 = 11;
    const STALE_GENERATION_OFFSET: u32 = 1;
    const TEST_COMPONENT_INDEX: usize = COMPONENTS_PER_BLOCK + 1;

    #[derive(Debug, PartialEq, Eq)]
    struct TestComponent {
        value: i32,
    }

    struct OtherComponent {
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
        let stale = Entity {
            id: entity.id,
            generation: entity.generation + STALE_GENERATION_OFFSET,
        };

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
    fn ecs_component_storage_drops_removed_component_once() {
        let drops = Arc::new(AtomicUsize::new(0));
        {
            let mut storage = ComponentMem::new::<DropCounter>();
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
        let mut storage = ComponentMem::new::<TestComponent>();

        assert!(storage.get::<TestComponent>(TEST_COMPONENT_INDEX).is_none());
        assert!(storage
            .get_mut::<TestComponent>(TEST_COMPONENT_INDEX)
            .is_none());
    }
}
