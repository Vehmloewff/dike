use crate::instruction_sweep::InstructionSweep;

pub struct InstructionGraph {
    pub graph: Vec<InstructionSweep>,
}

impl InstructionGraph {
    /// Get a sweep by it's numeric address
    pub fn get_sweep(&self, address: usize) -> &InstructionSweep {
        match self.graph.get(address) {
            Some(instructions) => instructions,
            _ => panic!("No instruction list exists at address {}", address),
        }
    }
}
