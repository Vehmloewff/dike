use std::mem;

pub struct Array<T>(Vec<T>);

impl<T> Array<T> {
    pub fn new() -> Array<T> {
        Array(Vec::new())
    }

    /// Push an item onto the end of an array
    pub fn push(mut self, value: T) {
        self.0.push(value);
    }

    /// Pop an item off the end of an array
    pub fn pop(mut self) -> T {
        self.0.pop().unwrap()
    }

    /// Remove an item from an array, shifting all elements to the left
    pub fn remove(mut self, index: usize) -> T {
        self.0.remove(index)
    }

    /// Put an item into an array, returning whatever is there
    pub fn put(mut self, index: usize, value: T) -> T {
        mem::replace(&mut self.0[index], value)
    }
}
