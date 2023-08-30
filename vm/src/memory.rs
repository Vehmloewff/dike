use std::{
	cell::{Ref, RefCell},
	mem,
};

use crate::value::Value;

/// Memory is, generally speaking, the heap. It needs to support the following operations:
///
/// - Allocate (create a new, empty address)
/// - Write (write a value (which could be another ref) to an address)
///     - If the overwritten value is a reference, it should be dereferenced
/// - Grab (given an address, return a value and a reviver)
///     - The returned value may not be a Ref
///     - If the address points to a ref, it should be recursively resolved
///     - The returned value is swapped with undefined in memory
///     - The reviver should take in the value, and swap it back with undefined
/// - Deref address
///     - If the value is dereferenced to 0, it should be cleaned
///     - If the value was cleaned and it was a reference itself, what it pointed to should be dereferenced
///     - The above statement should be applied recursively
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

	/// Allocate space on the heap as undefined. Returns a memory address with a single tracked reference
	pub fn allocate(&self) -> usize {
		let mut mem = self.map.borrow_mut();
		let mut counts = self.reference_counts.borrow_mut();
		let free_address = self.free_slots.borrow_mut().pop();

		let address = match free_address {
			// If it is a free_address, it will already be undefined
			// The reference count, however, will be 0
			Some(existing_address) => {
				*counts.get_mut(existing_address).unwrap() = 1;

				existing_address
			}
			// But if it isn't we need to set it to undefined
			// We also need to set the counts to 1
			None => {
				let length = mem.len();
				mem.push(Value::Undefined);
				counts.push(1);

				length
			}
		};

		address
	}

	/// Sets the value of an item at a memory address
	pub fn write(&self, address: usize, value: Value) {
		let mut binding = self.map.borrow_mut();
		let current_value = binding.get_mut(address).unwrap();

		match current_value {
			Value::Ref(ref address) => self.deref(*address),
			_ => (),
		}

		*current_value = value
	}

	/// Gets the value of an item at a memory address
	pub fn get(&self, address: usize) -> Ref<Value> {
		let value = Ref::map(self.map.borrow(), |map| map.get(address).unwrap());

		match *value {
			Value::Ref(address) => self.get(address),
			_ => value,
		}
	}

	/// Gets the value at an address with undefined, resolving references recursively so that it is guaranteed that a reference is not returned.
	/// After the value is used, MemoryReviver.revive(value) must be called. If not, undefined behavior will occur
	pub fn grab(&self, address: usize) -> (Value, MemoryReviver) {
		let value = self.swap(address, Value::Undefined);

		match value {
			Value::Ref(new_address) => {
				self.swap(address, value);

				self.grab(new_address)
			}
			Value::Undefined => panic!("Encountered undefined when grabbing from memory"),
			Value::Null => (Value::Null, MemoryReviver(self, None)),
			Value::Boolean(boolean) => (Value::Boolean(boolean), MemoryReviver(self, None)),
			Value::SweepPointer(pointer) => {
				(Value::SweepPointer(pointer), MemoryReviver(self, None))
			}
			Value::RustyImitable(value) => (Value::RustyImitable(value), MemoryReviver(self, None)),
			Value::RustyInimitable(value) => (
				Value::RustyInimitable(value),
				MemoryReviver(self, Some(address)),
			),
		}
	}

	fn swap(&self, address: usize, input: Value) -> Value {
		mem::replace(&mut self.map.borrow_mut()[address], input)
	}

	fn revive(&self, address: usize, value: Value) {
		let _ = mem::replace(&mut self.map.borrow_mut()[address], value);
	}

	/// Increments the reference count of a memory address
	///
	/// This is not currently used, but it will be needed later, when we have support for constants. Take the following example:
	///
	/// ```dk
	/// let x = 'Hello'
	/// let y = x
	///
	/// x = 'World'
	/// ```
	///
	/// In this example, x is assigned a ref that points to a str, but y cannot be assigned to the same reference, lest it equal
	/// 'World'. But take the next example.
	///
	/// ```dk
	/// const x = 'Hello'
	/// const y = x
	/// ```
	///
	/// In this example, x and y can safely be pointed at the ref that points to the str because neither will overwrite the other.
	/// That might seem completely impractical, but take the following example, which is much the same.
	///
	/// ```dk
	/// let people = []
	/// const name = '$firstName $lastName'
	///
	/// // ...
	///
	/// people.push(name)
	/// ```
	///
	/// In that example, name should never change, so there is no reason why people cannot directly contain the same reference that
	pub fn track_ref(&self, address: usize) {
		*self.reference_counts.borrow_mut().get_mut(address).unwrap() += 1
	}

	/// Decrements the reference count of a memory address. If it reaches 0, the memory is cleaned.
	pub fn deref(&self, address: usize) {
		*self.reference_counts.borrow_mut().get_mut(address).unwrap() -= 1;

		if self.reference_counts.borrow().get(address).unwrap() <= &(0 as usize) {
			self.free_slots.borrow_mut().push(address);
			self.write(address, Value::Undefined);
		}
	}
}

pub struct MemoryReviver<'a>(&'a Memory, Option<usize>);

impl MemoryReviver<'_> {
	fn revive(self, value: Value) {
		match self.1 {
			Some(address) => self.0.revive(address, value),
			None => (),
		}
	}
}
