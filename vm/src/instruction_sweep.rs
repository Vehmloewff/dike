use std::collections::HashMap;

use crate::instruction::Instruction;

pub struct InstructionSweep {
    /// The instructions in this sweep
    pub instructions: Vec<Instruction>,
    /// The number of local variables in this sweep. Because variables are named from 0,
    /// the names of all the local variable names can be inferred from this value
    pub local_variables_count: usize,
    /// The variables that this sweep uses that are inherited. If this sweep uses variables 2 and 4 in sweep 498,
    /// this map will look like this { 498: [2, 4] }
    pub inherited_variables: HashMap<usize, Vec<usize>>,
}

impl InstructionSweep {
    pub fn get_instruction(&self, index: usize) -> &Instruction {
        match self.instructions.get(index) {
            Some(instruction) => instruction,
            _ => panic!("No instruction exists in sweep at index {}", index),
        }
    }
}
