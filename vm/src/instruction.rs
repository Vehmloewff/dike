// use crate::array::Array;
// use crate::number::Number;
// use crate::string::Str;
// use crate::value::InimitablePrimitive;

use crate::value;

use super::memory::Memory;
use super::stack::Stack;
use super::value::Value;

pub enum ExecuteResult {
	Done,
	Next,
	Skip(usize),
	GoTo(usize),
}

pub enum Instruction {
	// Memory
	Write,    // Write the value at index 0 into the register at index 1
	Allocate, // Allocate a new memory address, write the value at index 0 into it, and output the address

	// Loading
	// LoadNum(Number),
	// LoadStr(String),
	LoadBool(bool),
	LoadNull,
	// LoadArray,
	LoadSweepPointer(usize),

	// Operations
	// Add,
	// Subtract,
	// Multiply,
	// Divide,
	// Concat,
	// ArrayPush,
	// ArrayPop,

	// Control flow
	Focus(usize),
	Consume,
	If(usize),
	Done,
	GoTo,
}

impl Instruction {
	pub fn execute(&self, memory: &Memory, stack: &Stack) -> ExecuteResult {
		match self {
			// Memory
			Instruction::Write => Instruction::execute_write(memory, stack),
			Instruction::Allocate => Instruction::execute_allocate(memory, stack),

			// Load
			// Instruction::LoadNum(num) => Instruction::execute_load_num(stack, num),
			// Instruction::LoadStr(string) => Instruction::execute_load_str(stack, string),
			Instruction::LoadBool(boolean) => Instruction::execute_load_bool(stack, boolean),
			Instruction::LoadNull => Instruction::execute_load_null(stack),
			// Instruction::LoadArray => Instruction::execute_load_array(stack),
			Instruction::LoadSweepPointer(pointer) => {
				Instruction::execute_load_sweep_pointer(stack, pointer)
			}

			// Operations
			// Instruction::Add => Instruction::execute_add(stack, memory),
			// Instruction::Subtract => Instruction::execute_subtract(stack, memory),
			// Instruction::Multiply => Instruction::execute_multiply(stack, memory),
			// Instruction::Divide => Instruction::execute_divide(stack, memory),
			// Instruction::Concat => Instruction::execute_concat(stack, memory),
			// Instruction::ArrayPush => Instruction::execute_array_push(stack, memory),
			// Instruction::ArrayPop => Instruction::execute_array_pop(stack, memory),

			// Control Flow
			Instruction::Focus(index) => Instruction::execute_focus(stack, index),
			Instruction::Consume => Instruction::execute_consume(stack),
			Instruction::If(skip_count) => Instruction::execute_if(stack, skip_count, memory),
			Instruction::Done => ExecuteResult::Done,
			Instruction::GoTo => Instruction::execute_go_to(stack, memory),
		}
	}

	fn execute_write(memory: &Memory, stack: &Stack) -> ExecuteResult {
		let last_value = stack.dangerous_pop();
		let second_last_value = stack.dangerous_pop();

		// let address = match value {
		//     Value::Ref(address) => address,
		//     _ => panic!("Expected a ref"),
		// };

		// memory.set(address.clone(), stack.consume());

		ExecuteResult::Next
	}

	fn execute_allocate(memory: &Memory, stack: &Stack) -> ExecuteResult {
		stack.push(Value::Ref(memory.allocate()));

		ExecuteResult::Next
	}

	// fn execute_load_num(stack: &Stack, num: &Number) -> ExecuteResult {
	//     stack.push(Value::Number(num.clone()));

	//     ExecuteResult::Next
	// }

	// fn execute_load_str(stack: &Stack, string: &String) -> ExecuteResult {
	//     stack.push(Value::String(Str::new(string)));

	//     ExecuteResult::Next
	// }

	fn execute_load_bool(stack: &Stack, boolean: &bool) -> ExecuteResult {
		stack.push(Value::Boolean(boolean.clone()));

		ExecuteResult::Next
	}

	fn execute_load_null(stack: &Stack) -> ExecuteResult {
		stack.push(Value::Null);

		ExecuteResult::Next
	}

	// fn execute_load_array(stack: &Stack) -> ExecuteResult {
	//     stack.push(Value::Array(Array::new()));

	//     ExecuteResult::Next
	// }

	fn execute_load_sweep_pointer(stack: &Stack, pointer: &usize) -> ExecuteResult {
		stack.push(Value::SweepPointer(pointer.clone()));

		ExecuteResult::Next
	}

	// fn execute_add(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     let right = stack.consume().get_num(memory);
	//     let left = stack.consume().get_num(memory);

	//     stack.push(Value::Number(left.add(right)));

	//     ExecuteResult::Next
	// }

	// fn execute_subtract(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     let right = stack.consume().get_num(memory);
	//     let left = stack.consume().get_num(memory);

	//     stack.push(Value::Number(left.subtract(right)));

	//     ExecuteResult::Next
	// }

	// fn execute_multiply(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     let right = stack.consume().get_num(memory);
	//     let left = stack.consume().get_num(memory);

	//     stack.push(Value::Number(left.multiply(right)));

	//     ExecuteResult::Next
	// }

	// fn execute_divide(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     let right = stack.consume().get_num(memory);
	//     let left = stack.consume().get_num(memory);

	//     stack.push(Value::Number(left.divide(right)));

	//     ExecuteResult::Next
	// }

	// fn execute_concat(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     let right = stack.consume().get_string(memory);
	//     let left = stack.consume().get_string(memory);

	//     let combined = match left {
	//         InimitablePrimitive::Raw(left_string) => match right {
	//             InimitablePrimitive::Raw(right_string) => left_string.concat(&right_string),
	//             InimitablePrimitive::Ref(right_string, _address) => {
	//                 left_string.concat(&right_string)
	//             }
	//         },
	//         InimitablePrimitive::Ref(left_string, _address) => match right {
	//             InimitablePrimitive::Raw(right_string) => left_string.concat(&right_string),
	//             InimitablePrimitive::Ref(right_string, _address) => {
	//                 left_string.concat(&right_string)
	//             }
	//         },
	//     };

	//     stack.push(Value::String(combined));

	//     ExecuteResult::Next
	// }

	// fn execute_array_push(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     // We do the dangerous pop here because none of these values are actually going anywhere
	//     // Both of them are going right back on the stack
	//     let last_value = stack.dangerous_pop();
	//     let second_last_value = stack.dangerous_pop().get_array(memory);

	//     let array = match second_last_value {
	//         InimitablePrimitive::Raw(array) => {
	//             array.push(last_value);
	//             Value::Array(array)
	//         }
	//         InimitablePrimitive::Ref(reference, address) => {
	//             reference.push(last_value);
	//             Value::Ref(address)
	//         }
	//     };

	//     stack.push(array);

	//     ExecuteResult::Next
	// }

	// fn execute_array_pop(stack: &Stack, memory: &Memory) -> ExecuteResult {
	//     // We use the dangerous pop here because the array isn't going anywhere
	//     let array = stack.dangerous_pop().get_array(memory);

	//     let (popped, operated_array) = match array {
	//         InimitablePrimitive::Raw(array) => (array.pop(), Value::Array(array)),
	//         InimitablePrimitive::Ref(array, address) => (array.pop(), Value::Ref(address)),
	//     };

	//     stack.push(operated_array);
	//     stack.push(popped);

	//     ExecuteResult::Next
	// }

	fn execute_consume(stack: &Stack) -> ExecuteResult {
		stack.consume();

		ExecuteResult::Next
	}

	fn execute_focus(stack: &Stack, index: &usize) -> ExecuteResult {
		// The index that is passed in is the index from the end, but dangerous_remove requires the index from start
		let index_from_start = stack.get_length() - 1 - index;
		// We use dangerous remove because the value is not going anywhere. Off the stack then...
		let value = stack.dangerous_remove(index_from_start);

		// ... right back on
		stack.push(value);

		ExecuteResult::Next
	}

	fn execute_if(stack: &Stack, skip_count: &usize, memory: &Memory) -> ExecuteResult {
		let boolean = stack.consume().get_boolean(memory);

		match boolean {
			true => ExecuteResult::Next,
			false => ExecuteResult::Skip(skip_count.clone()),
		}
	}

	fn execute_go_to(stack: &Stack, memory: &Memory) -> ExecuteResult {
		let sweep_pointer = stack.consume().get_sweep_pointer(memory);

		ExecuteResult::GoTo(sweep_pointer)
	}
}
