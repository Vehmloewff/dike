use std::{cell::RefCell, mem};

use crate::memory::Memory;

use super::value::Value;

pub struct Array(pub RefCell<Vec<Value>>);

impl Array {
    pub fn new() -> Array {
        Array(RefCell::new(Vec::new()))
    }

    /// Push an item onto the end of an array
    pub fn push(&self, value: Value) {
        self.0.borrow_mut().push(value);
    }

    /// Pop an item off the end of an array
    pub fn pop(&self) -> Value {
        self.0.borrow_mut().pop().unwrap()
    }

    /// Remove an item from an array, shifting all elements to the left
    pub fn remove(&self, index: usize) -> Value {
        self.0.borrow_mut().remove(index)
    }

    /// Yank an item out of an array, replacing it with Null
    pub fn yank(&self, index: usize) -> Value {
        self.put(index, Value::Null)
    }

    /// Put an item into an array, returning whatever is there
    pub fn put(&self, index: usize, value: Value) -> Value {
        mem::replace(&mut self.0.borrow_mut()[index], value)
        // let mut array = self.0.borrow_mut();
        // let length = array.len();

        // array.push(value);

        // if length > 1 && index != length {
        //     array.swap(index, length)
        // }
    }

    /// Get an item from an array as a memory reference. Under the hood, the item is converted to a reference
    /// And it's address is copied into the reference returned. This not the fastest array method out there
    pub fn get_item(&self, index: usize, memory: &Memory) -> Value {
        let current = self.yank(index);

        match current {
            Value::Array(array) => {
                let address = memory.allocate();

                memory.set(address, Value::Array(array));
                self.put(index, Value::Ref(address));

                memory.track_ref(address);
                Value::Ref(address)
            }
            Value::String(string) => {
                let address = memory.allocate();

                memory.set(address, Value::String(string));
                self.put(index, Value::Ref(address));

                memory.track_ref(address);
                Value::Ref(address)
            }
            Value::Ref(address) => Value::Ref(address),
            Value::Boolean(boolean) => Value::Boolean(boolean),
            Value::Null => Value::Null,
            Value::SweepPointer(pointer) => Value::SweepPointer(pointer),
            Value::Number(number) => Value::Number(number),
        }
    }

    // pub fn get_iter(&self) -> Iter<Value> {
    //     self.0.borrow().iter()
    // }
}
