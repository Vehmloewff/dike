use std::cell::{Ref, RefCell};

use crate::value::Value;

pub struct Memory {
    free_slots: RefCell<Vec<usize>>,
    reference_counts: RefCell<Vec<usize>>,
    map: RefCell<Vec<Value>>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            free_slots: RefCell::new(Vec::new()),
            reference_counts: RefCell::new(Vec::new()),
            map: RefCell::new(Vec::new()),
        }
    }

    /// Allocate space on the heap. Returns a memory address with a single tracked reference
    pub fn allocate(&self) -> usize {
        let mut mem = self.map.borrow_mut();
        let mut counts = self.reference_counts.borrow_mut();
        let free_address = self.free_slots.borrow_mut().pop();

        let address = match free_address {
            // If it is a free_address, it will already be null
            // The reference count, however, will be 0
            Some(existing_address) => {
                *counts.get_mut(existing_address).unwrap() = 1;

                existing_address
            }
            // But if it isn't we need to set it to null
            // We also need to set the counts to 1
            None => {
                let length = mem.len();
                mem.push(Value::Null);
                counts.push(1);

                length
            }
        };

        address
    }

    /// Sets the value of an item at a memory address
    pub fn set(&self, address: usize, value: Value) {
        *self.map.borrow_mut().get_mut(address).unwrap() = value;
    }

    /// Gets the value of an item at a memory address
    pub fn get(&self, address: usize) -> Ref<Value> {
        Ref::map(self.map.borrow(), |map| map.get(address).unwrap())
    }

    /// Increments the reference count of a memory address
    pub fn track_ref(&self, address: usize) {
        *self.reference_counts.borrow_mut().get_mut(address).unwrap() += 1
    }

    /// Decrements the reference count of a memory address. If it reaches 0, the memory is cleaned.
    pub fn deref(&self, address: usize) {
        *self.reference_counts.borrow_mut().get_mut(address).unwrap() += 1;

        if self.reference_counts.borrow().get(address).unwrap() <= &(0 as usize) {
            self.set(address, Value::Null);
            self.free_slots.borrow_mut().push(address);
        }
    }
}
