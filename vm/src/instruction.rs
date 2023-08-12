use crate::array::Array;
use crate::number::Number;
use crate::value::InimitablePrimitive;

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
    Write(usize),
    Use(usize),

    // Loading
    LoadNum(Number),
    LoadStr(String),
    LoadBool(bool),
    LoadNull,
    LoadArray,

    // Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    ArrayPush,
    ArrayPop,

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
            Instruction::Write(address) => Instruction::execute_write(memory, stack, address),
            Instruction::Use(address) => Instruction::execute_use(memory, stack, address.clone()),

            // Load
            Instruction::LoadNum(num) => Instruction::execute_load_num(stack, num),
            Instruction::LoadStr(string) => Instruction::execute_load_str(stack, string),
            Instruction::LoadBool(boolean) => Instruction::execute_load_bool(stack, boolean),
            Instruction::LoadNull => Instruction::execute_load_null(stack),
            Instruction::LoadArray => Instruction::execute_load_array(stack),

            // Operations
            Instruction::Add => Instruction::execute_add(stack, memory),
            Instruction::Subtract => Instruction::execute_subtract(stack, memory),
            Instruction::Multiply => Instruction::execute_multiply(stack, memory),
            Instruction::Divide => Instruction::execute_divide(stack, memory),
            Instruction::Concat => Instruction::execute_concat(stack, memory),
            Instruction::ArrayPush => Instruction::execute_array_push(stack, memory),
            Instruction::ArrayPop => Instruction::execute_array_pop(stack, memory),

            // Control Flow
            Instruction::Focus(index) => Instruction::execute_focus(stack, index),
            Instruction::Consume => Instruction::execute_consume(stack),
            Instruction::If(skip_count) => Instruction::execute_if(stack, skip_count),
            Instruction::Done => ExecuteResult::Done,
            Instruction::GoTo => Instruction::execute_go_to(stack),
        }
    }

    fn execute_write(memory: &Memory, stack: &Stack, address: &usize) -> ExecuteResult {
        memory.set(address.clone(), stack.consume());

        ExecuteResult::Next
    }

    fn execute_use(memory: &Memory, stack: &Stack, address: usize) -> ExecuteResult {
        memory.track_ref(address);
        stack.push(Value::Ref(address));

        ExecuteResult::Next
    }

    fn execute_load_num(stack: &Stack, num: &Number) -> ExecuteResult {
        stack.push(Value::Number(num.clone()));

        ExecuteResult::Next
    }

    fn execute_load_str(stack: &Stack, string: &String) -> ExecuteResult {
        stack.push(Value::String(string.clone()));

        ExecuteResult::Next
    }

    fn execute_load_bool(stack: &Stack, boolean: &bool) -> ExecuteResult {
        stack.push(Value::Boolean(boolean.clone()));

        ExecuteResult::Next
    }

    fn execute_load_null(stack: &Stack) -> ExecuteResult {
        stack.push(Value::Null);

        ExecuteResult::Next
    }

    fn execute_load_array(stack: &Stack) -> ExecuteResult {
        stack.push(Value::Array(Array::new()));

        ExecuteResult::Next
    }

    fn execute_add(stack: &Stack, memory: &Memory) -> ExecuteResult {
        let right = stack.consume().get_num(memory);
        let left = stack.consume().get_num(memory);

        stack.push(Value::Number(left.add(right)));

        ExecuteResult::Next
    }

    fn execute_subtract(stack: &Stack, memory: &Memory) -> ExecuteResult {
        let right = stack.consume().get_num(memory);
        let left = stack.consume().get_num(memory);

        stack.push(Value::Number(left.subtract(right)));

        ExecuteResult::Next
    }

    fn execute_multiply(stack: &Stack, memory: &Memory) -> ExecuteResult {
        let right = stack.consume().get_num(memory);
        let left = stack.consume().get_num(memory);

        stack.push(Value::Number(left.multiply(right)));

        ExecuteResult::Next
    }

    fn execute_divide(stack: &Stack, memory: &Memory) -> ExecuteResult {
        let right = stack.consume().get_num(memory);
        let left = stack.consume().get_num(memory);

        stack.push(Value::Number(left.divide(right)));

        ExecuteResult::Next
    }

    fn execute_concat(stack: &Stack, memory: &Memory) -> ExecuteResult {
        let right = stack.consume().get_string(memory);
        let left = stack.consume().get_string(memory);

        // let mut left_string = match left {
        //     Value::String(string) => string,
        //     _ => panic!(
        //         "Expected str to concat on left, but found {}",
        //         left.get_human_type()
        //     ),
        // };
        // let right_string = match right {
        //     Value::String(string) => string,
        //     _ => panic!(
        //         "Expected a str to concat on right, but found {}",
        //         right.get_human_type()
        //     ),
        // };

        // We are creating a new string from the two old strings.
        // If one of the strings is stored in memory, we don't want to change it. This is why we clone
        let mut left_string = match left {
            InimitablePrimitive::Raw(string) => string,
            InimitablePrimitive::Ref(reference, _address) => reference.clone(),
        };

        let combined = match right {
            InimitablePrimitive::Raw(string) => {
                left_string.push_str(&string);
                left_string
            }
            InimitablePrimitive::Ref(reference, _address) => {
                left_string.push_str(&*reference);
                left_string
            }
        };

        // left_string.push_str(&right_string);
        stack.push(Value::String(combined));

        ExecuteResult::Next
    }

    fn execute_array_push(stack: &Stack, memory: &Memory) -> ExecuteResult {
        // We do the dangerous pop here because none of these values are actually going anywhere
        // Both of them are going right back on the stack
        let last_value = stack.dangerous_pop();
        let second_last_value = stack.dangerous_pop().get_array(memory);

        let array = match second_last_value {
            InimitablePrimitive::Raw(array) => {
                array.push(last_value);
                Value::Array(array)
            }
            InimitablePrimitive::Ref(reference, address) => {
                reference.push(last_value);
                Value::Ref(address)
            }
        };

        stack.push(array);

        ExecuteResult::Next
    }

    fn execute_array_pop(stack: &Stack, memory: &Memory) -> ExecuteResult {
        // We use the dangerous pop here because the array isn't going anywhere
        let array = stack.dangerous_pop().get_array(memory);

        let (popped, operated_array) = match array {
            InimitablePrimitive::Raw(array) => (array.pop(), Value::Array(array)),
            InimitablePrimitive::Ref(array, address) => (array.pop(), Value::Ref(address)),
        };

        stack.push(operated_array);
        stack.push(popped);

        ExecuteResult::Next
    }

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

    fn execute_if(stack: &Stack, skip_count: &usize) -> ExecuteResult {
        let boolean_value = stack.consume();
        let boolean = match boolean_value {
            Value::Boolean(boolean) => boolean,
            _ => panic!(
                "Expected a bool for if, but found {}",
                boolean_value.get_human_type()
            ),
        };

        match boolean {
            true => ExecuteResult::Next,
            false => ExecuteResult::Skip(skip_count.clone()),
        }
    }

    fn execute_go_to(stack: &Stack) -> ExecuteResult {
        let value = stack.consume();
        let sweep_pointer = match value {
            Value::SweepPointer(pointer) => pointer,
            _ => panic!(
                "Expected a sweep pointer for GoTo, but found {}",
                value.get_human_type()
            ),
        };

        ExecuteResult::GoTo(sweep_pointer)
    }
}
