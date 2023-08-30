use crate::{
	interop_gen::{RustyImitable, RustyInimitable},
	memory::Memory,
};

pub enum Value {
	Null,
	Undefined,
	Boolean(bool),
	SweepPointer(usize),
	Ref(usize),
	RustyImitable(RustyImitable),
	RustyInimitable(RustyInimitable),
}

impl Value {
	pub fn get_human_type(&self) -> String {
		match self {
			Value::Null => String::from("null"),
			Value::Undefined => String::from("undefined"),
			Value::Boolean(_) => String::from("bool"),
			Value::Ref(address) => format!("ref *{}", address),
			Value::SweepPointer(pointer) => format!("sweep *{}", pointer),
			Value::RustyImitable(_) => format!("rusty"),
			Value::RustyInimitable(_) => format!("mut rusty"),
		}
	}

	/// Gets value as a boolean, by copying if it is a reference
	pub fn get_boolean(self, memory: &Memory) -> bool {
		match self {
			Value::Boolean(boolean) => boolean,
			Value::Ref(address) => match *memory.get(address) {
				Value::Boolean(boolean) => boolean,
				_ => panic!("Expected a boolean"),
			},
			_ => panic!("Expected a boolean"),
		}
	}

	/// Gets value as a sweep pointer, by coping, if it is a reference
	pub fn get_sweep_pointer(self, memory: &Memory) -> usize {
		match self {
			Value::SweepPointer(pointer) => pointer,
			Value::Ref(address) => match *memory.get(address) {
				Value::SweepPointer(pointer) => pointer,
				_ => panic!("Expected a sweep pointer"),
			},
			_ => panic!("Expected a sweep pointer"),
		}
	}

	// / Grabs a rusty value, removing it from memory if is is a reference. To put the value back in the memory that it once rested, call `RustyValueReviewer::revive`
	// pub fn grab_rusty_value(self, memory: &Memory) -> (RustyValue, RustyValueReviver) {
	// 	match self {
	// 		Value::RustyValue(value) => (value, RustyValueReviver(None)),
	// 		Value::Ref(address) => (
	// 			match memory.grab(address) {
	// 				Value::RustyValue(value) => value,
	// 				_ => panic!("Not a rusty value"),
	// 			},
	// 			RustyValueReviver(Some(address)),
	// 		),
	// 		_ => panic!("Expected a rusty value"),
	// 	}
	// }
}
