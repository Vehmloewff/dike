// use std::collections::HashMap;

// use super::instruction::{Instruction, InstructionAction};
// use super::memory::Memory;
// use super::stack::Stack;
// use super::value::Value;

// pub enum ScopeTick {
//     Running,
//     Finished(Value),
// }

// pub struct Scope<'a> {
//     memory: &'a Memory,
//     instruction_sets: &'a Vec<Vec<Instruction>>,
//     stack: Stack,
//     index: usize,
//     instructions: &'a Vec<Instruction>,
//     child_scope: Option<Box<Scope<'a>>>,
// }

// impl Scope<'_> {
//     pub fn from<'a>(
//         memory: &Memory,
//         instruction_sets: &'a HashMap<usize, Vec<Instruction>>,
//         address: usize,
//     ) -> Scope<'a> {
//         let instructions = instruction_sets.get(&address).unwrap();

//         Scope {
//             memory,
//             instruction_sets,
//             stack: Vec::new(),
//             index: 0,
//             instructions,
//             child_scope: None,
//         }
//     }

//     // fn new<'a>(
//     // 	memory
//     //     instruction_sets: &'a HashMap<usize, Vec<Instruction>>,
//     //     instructions: &'a Vec<Instruction>,
//     // ) -> Scope<'a> {
//     //     Scope {
//     //         memory,
//     //         instruction_sets,
//     //         stack: Vec::new(),
//     //         index: 0,
//     //         instructions,
//     //         child_scope: None,
//     //     }
//     // }

//     pub fn tick(&self) -> ScopeTick {
//         if self.child_scope.is_some() {
//             let child = self.child_scope.as_mut().unwrap();

//             return child.tick();
//         }

//         let instruction = self.instructions.get(self.index);

//         if instruction.is_none() {
//             return ScopeTick::Finished(self.stack.pop().unwrap());
//         }

//         let action = instruction
//             .unwrap()
//             .execute(&mut self.memory, &mut self.stack);

//         match action {
//             InstructionAction::Continue => {
//                 self.index += 1;

//                 ScopeTick::Running
//             }
//             InstructionAction::Halt => ScopeTick::Finished(self.stack.pop().unwrap()),
//             InstructionAction::GoTo(address) => {
//                 self.step_into(address);

//                 ScopeTick::Running
//             }
//         }
//     }

//     fn step_into(&mut self, address: usize) {
//         let child = Scope::from(self.memory, self.instruction_sets, address);
//         self.child_scope = Some(Box::new(child));
//     }
// }
