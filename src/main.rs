use vm::{
    instruction::Instruction, instruction_graph::InstructionGraph,
    instruction_sweep::InstructionSweep, VirtualMachine,
};

fn main() {
    let graph = InstructionGraph {
        graph: vec![InstructionSweep {
            local_variables_count: 1,
            instructions: vec![
                Instruction::LoadInt(10),
                Instruction::Write(0),
                Instruction::LoadInt(10),
                Instruction::Use(0),
                Instruction::Add,
            ],
        }],
    };

    VirtualMachine::new(graph).run();
}
