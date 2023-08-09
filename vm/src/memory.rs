use std::cell::{Ref, RefCell};

use crate::value::Value;

pub struct Memory {
    free_slots: RefCell<Vec<usize>>,
    reference_counts: RefCell<Vec<usize>>,
    map: RefCell<Vec<Value>>,
}

impl Memory {
    /// Allocate space on the heap. Returns a memory address with a single tracked reference
    pub fn allocate(&self) -> usize {
        let mem = self.map.borrow_mut();
        let free_address = self.free_slots.borrow_mut().pop();

        let address = match free_address {
            Some(num) => num,
            None => mem.len(),
        };

        self.set(address, Value::Null);
        self.track_ref(address);

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
