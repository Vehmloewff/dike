use std::cell::RefCell;

use crate::{memory::Memory, value::Value};

pub struct Stack<'a> {
    pub memory: &'a Memory,
    stack: RefCell<Vec<Value>>,
}

impl Stack<'_> {
    pub fn new(memory: &Memory) -> Stack<'_> {
        Stack {
            memory,
            stack: RefCell::new(Vec::new()),
        }
    }

    /// Consume an item from the stack. If the item contains a reference, it will be dereferenced.
    pub fn consume(&self) -> Value {
        let value = self.dangerous_pop();
        deref_value(&value, self.memory);

        value
    }

    /// Pop an item off the stack. It is dangerous because if the value is a reference, it will NOT be dereferenced,
    /// which could lead to memory leaks.
    pub fn dangerous_pop(&self) -> Value {
        self.stack.borrow_mut().pop().unwrap()
    }

    /// Push a new value onto the stack
    pub fn push(&self, value: Value) {
        self.stack.borrow_mut().push(value)
    }

    /// Remove an item from the stack at `index`, shifting all elements after it to the left. It
    /// is dangerous because, if the value is a reference, it will NOT be dereferenced, which could
    /// load to a potential memory leak
    pub fn dangerous_remove(&self, index: usize) -> Value {
        self.stack.borrow_mut().remove(index)
    }

    /// Gets the length of the stack
    pub fn get_length(&self) -> usize {
        self.stack.borrow().len()
    }
}

fn deref_value(value: &Value, memory: &Memory) {
    match value {
        Value::Ref(address) => memory.deref(*address),
        Value::Array(array) => {
            for child in array.iter() {
                deref_value(child, memory)
            }
        }
        _ => (),
    };
}
