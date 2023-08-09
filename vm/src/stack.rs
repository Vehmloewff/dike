use std::cell::RefCell;

use crate::{memory::Memory, value::Value};

pub struct Stack<'a> {
    pub memory: &'a Memory,
    stack: RefCell<Vec<Value>>,
}

impl Stack<'_> {
    /// Consume an item from the stack. If the item contains a reference, it will be dereferenced.
    pub fn consume(&self) -> Value {
        let value = self.dangerous_pop();
        deref_value(&value, self.memory);

        value
    }

    /// Pop an item off the stack. It is dangerous because ff the value is a reference, it will NOT be dereferenced,
    /// which could lead to memory leaks.
    pub fn dangerous_pop(&self) -> Value {
        self.stack.borrow_mut().pop().unwrap()
    }

    /// Push a new value onto the stack
    pub fn push(&self, value: Value) {
        self.stack.borrow_mut().push(value)
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
