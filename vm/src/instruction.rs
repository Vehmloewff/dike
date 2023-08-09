use super::math::add;
use super::memory::Memory;
use super::stack::Stack;
use super::value::Value;

pub enum InstructionAction {
    GoTo(usize),
    Halt,
    Continue,
}

pub enum Instruction {
    Load(Value),
    Read(String),
    Write(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    Pop,
    GoTo,
    If,
    HaltIf,
}

impl Instruction {
    pub fn execute(&self, memory: &Memory, stack: &Stack) -> InstructionAction {
        match self {
            Self::Add => self.execute_add(memory, stack),
            _ => panic!("Instruction not implemented"),
        }
    }

    pub fn execute_add(&self, _memory: &Memory, stack: &Stack) -> InstructionAction {
        let left = stack.consume();
        let right = stack.consume();

        stack.push(add(left, right));

        InstructionAction::Continue
    }
}
