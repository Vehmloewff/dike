use std::{cell::RefCell, collections::HashMap, mem};

// use super::value::Value;
// use crate::memory::Memory;

struct Map<K, V>(HashMap<K, V>);

struct Value {}

// IS VALUE
pub struct Array<T>(pub RefCell<Vec<T>>);

impl<T> Array<T> {
    pub fn new() -> Array<T> {
        Array(RefCell::new(Vec::new()))
    }

    /// Push an item onto the end of an array
    pub fn push(&self, value: T) {
        self.0.borrow_mut().push(value);
    }

    /// Pop an item off the end of an array
    pub fn pop(&self) -> T {
        self.0.borrow_mut().pop().unwrap()
    }

    /// Remove an item from an array, shifting all elements to the left
    pub fn remove(&self, index: usize) -> T {
        self.0.borrow_mut().remove(index)
    }

    /// Yank an item out of an array, replacing it with Null
    // pub fn yank(&self, index: usize) -> T {
    //     self.put(index, None)
    // }

    /// Put an item into an array, returning whatever is there
    pub fn put(&self, index: usize, value: T) -> T {
        mem::replace(&mut self.0.borrow_mut()[index], value)
        // let mut array = self.0.borrow_mut();
        // let length = array.len();

        // array.push(value);

        // if length > 1 && index != length {
        //     array.swap(index, length)
        // }
    }

    // Get an item from an array as a memory reference. Under the hood, the item is converted to a reference
    // And it's address is copied into the reference returned. This not the fastest array method out there
    // pub fn get_item(&self, memory: &Memory, index: usize) -> Value {
    //     let current = self.yank(index);

    //     match current {
    //         Value::Array(array) => {
    //             let address = memory.allocate();

    //             memory.set(address, Value::Array(array));
    //             self.put(index, Value::Ref(address));

    //             memory.track_ref(address);
    //             Value::Ref(address)
    //         }
    //         Value::String(string) => {
    //             let address = memory.allocate();

    //             memory.set(address, Value::String(string));
    //             self.put(index, Value::Ref(address));

    //             memory.track_ref(address);
    //             Value::Ref(address)
    //         }
    //         Value::Ref(address) => Value::Ref(address),
    //         Value::Boolean(boolean) => Value::Boolean(boolean),
    //         Value::Null => Value::Null,
    //         Value::SweepPointer(pointer) => Value::SweepPointer(pointer),
    //         Value::Number(number) => Value::Number(number),
    //     }
    // }
}
