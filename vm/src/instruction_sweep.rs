use crate::instruction::Instruction;

pub struct InstructionSweep {
    /// The instructions in this sweep
    pub instructions: Vec<Instruction>,
    /// The number of local variables in this sweep. Because variables are named from 0,
    /// the names of all the local variable names can be inferred from this value
    pub local_variables_count: usize,
    // I don't think this is needed anymore. Inherited variables should have use instructions to push them onto the stack
    // before a sweep that requires them is gone to.
    //
    // The variables that this sweep uses that are inherited. If this sweep uses variables 2 and 4 in sweep 498,
    // this map will look like this { 498: [2, 4] }
    // pub inherited_variables: HashMap<usize, Vec<usize>>,
}

impl InstructionSweep {
    /// Creates a new, empty instruction sweep
    pub fn new(local_variables_count: usize) -> InstructionSweep {
        InstructionSweep {
            instructions: Vec::new(),
            local_variables_count,
        }
    }

    /// Gets an instruction by it's address
    pub fn get_instruction(&self, index: usize) -> &Instruction {
        match self.instructions.get(index) {
            Some(instruction) => instruction,
            _ => panic!("No instruction exists in sweep at index {}", index),
        }
    }

    /// Gets the total length of the instructions
    pub fn get_length(&self) -> usize {
        self.instructions.len()
    }

    /// Adds a new instruction to the sweep
    pub fn add(mut self, instruction: Instruction) -> InstructionSweep {
        self.instructions.push(instruction);

        self
    }
}
