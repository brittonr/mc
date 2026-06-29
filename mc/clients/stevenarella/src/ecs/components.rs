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
use std::marker::PhantomData;
use std::mem;
use std::ptr;

/// Used to access compoents on an entity in an efficient
/// way.
pub struct Key<T> {
    pub(super) id: usize,
    _t: PhantomData<T>,
}

impl<T> Key<T> {
    pub(super) fn from_id(id: usize) -> Key<T> {
        Key {
            id,
            _t: PhantomData,
        }
    }
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

pub(super) const COMPONENTS_PER_BLOCK: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ComponentSlot {
    block_index: usize,
    block_offset: usize,
}

pub(super) fn component_slot(index: usize) -> ComponentSlot {
    ComponentSlot {
        block_index: index / COMPONENTS_PER_BLOCK,
        block_offset: index % COMPONENTS_PER_BLOCK,
    }
}

struct ComponentBlock {
    bytes: Vec<u8>,
    occupied: BSet,
    occupied_count: usize,
}

impl ComponentBlock {
    fn new(component_size: usize) -> ComponentBlock {
        ComponentBlock {
            bytes: vec![0; component_size * COMPONENTS_PER_BLOCK],
            occupied: BSet::new(COMPONENTS_PER_BLOCK),
            occupied_count: 0,
        }
    }

    fn byte_offset(&self, block_offset: usize, component_size: usize) -> usize {
        debug_assert!(block_offset < COMPONENTS_PER_BLOCK);
        block_offset * component_size
    }

    fn is_empty(&self) -> bool {
        self.occupied_count == 0
    }
}

pub(super) struct ComponentMem {
    data: Vec<Option<ComponentBlock>>,
    component_size: usize,
    drop_func: Box<dyn Fn(*mut u8) + Send>,
}

impl ComponentMem {
    pub(super) fn new<T>() -> ComponentMem {
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

    pub(super) fn add<T>(&mut self, index: usize, val: T) {
        let slot = component_slot(index);
        while self.data.len() <= slot.block_index {
            self.data.push(None);
        }
        if self.data[slot.block_index].is_none() {
            self.data[slot.block_index] = Some(ComponentBlock::new(self.component_size));
        }
        let block = self.data[slot.block_index].as_mut().unwrap();
        let start = block.byte_offset(slot.block_offset, self.component_size);
        block.occupied_count += 1;
        block.occupied.set(slot.block_offset, true);
        unsafe {
            ptr::write(block.bytes.as_mut_ptr().add(start) as *mut T, val);
        }
    }

    pub(super) fn remove(&mut self, index: usize) {
        let slot = component_slot(index);

        let Some(block) = self.data.get_mut(slot.block_index).and_then(|v| v.as_mut()) else {
            return;
        };
        if !block.occupied.get(slot.block_offset) {
            return;
        }
        let start = block.byte_offset(slot.block_offset, self.component_size);
        block.occupied.set(slot.block_offset, false);
        // We don't have access to the actual type in this method so
        // we use the drop_func which stores the type in its closure
        // to handle the dropping for us.
        unsafe {
            (self.drop_func)(block.bytes.as_mut_ptr().add(start));
        }
        block.occupied_count -= 1;
        if block.is_empty() {
            self.data[slot.block_index] = None;
        }
    }

    pub(super) fn get<T>(&self, index: usize) -> Option<&T> {
        let slot = component_slot(index);
        let block = self.data.get(slot.block_index).and_then(|v| v.as_ref())?;
        if !block.occupied.get(slot.block_offset) {
            return None;
        }
        let start = block.byte_offset(slot.block_offset, self.component_size);
        unsafe { Some(&*(block.bytes.as_ptr().add(start) as *const T)) }
    }

    pub(super) fn get_mut<T>(&mut self, index: usize) -> Option<&mut T> {
        let slot = component_slot(index);
        let block = self
            .data
            .get_mut(slot.block_index)
            .and_then(|v| v.as_mut())?;
        if !block.occupied.get(slot.block_offset) {
            return None;
        }
        let start = block.byte_offset(slot.block_offset, self.component_size);
        unsafe { Some(&mut *(block.bytes.as_mut_ptr().add(start) as *mut T)) }
    }
}

impl Drop for ComponentMem {
    fn drop(&mut self) {
        for block in &mut self.data {
            if let Some(block) = block.as_mut() {
                for slot_offset in 0..COMPONENTS_PER_BLOCK {
                    if block.occupied.get(slot_offset) {
                        let start = block.byte_offset(slot_offset, self.component_size);
                        unsafe {
                            (self.drop_func)(block.bytes.as_mut_ptr().add(start));
                        }
                    }
                }
            }
        }
    }
}
