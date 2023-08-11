use crate::instruction_sweep::InstructionSweep;

pub struct InstructionGraph {
    pub graph: Vec<InstructionSweep>,
}

impl InstructionGraph {
    pub fn new() -> InstructionGraph {
        InstructionGraph { graph: Vec::new() }
    }

    /// Get a sweep by it's numeric address
    pub fn get_sweep(&self, address: usize) -> &InstructionSweep {
        match self.graph.get(address) {
            Some(instructions) => instructions,
            _ => panic!("No instruction list exists at address {}", address),
        }
    }

    pub fn add_sweep(mut self, sweep: InstructionSweep) -> InstructionGraph {
        self.graph.push(sweep);

        self
    }
}
