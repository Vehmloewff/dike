use crate::{
    instruction::Instruction, instruction_graph::InstructionGraph,
    instruction_sweep::InstructionSweep, number::Number, VirtualMachine,
};

#[test]
fn simple_addition() {
    let graph = InstructionGraph::new().add_sweep(
        InstructionSweep::new(0)
            .add(Instruction::LoadNum(Number::Int(2)))
            .add(Instruction::LoadNum(Number::Int(2)))
            .add(Instruction::Add),
    );

    let vm = VirtualMachine::new(graph);
    let value = vm.get_last_value(0);

    assert_eq!(value.get_num(&vm.memory).get_int(), 4);
}

#[test]
fn variables_work() {
    let graph = InstructionGraph::new().add_sweep(
        InstructionSweep::new(1)
            .add(Instruction::LoadNum(Number::Int(10)))
            .add(Instruction::Write(0))
            .add(Instruction::Use(0))
            .add(Instruction::Use(0))
            .add(Instruction::Add),
    );

    let vm = VirtualMachine::new(graph);
    let value = vm.get_last_value(0);

    assert_eq!(value.get_num(&vm.memory).get_int(), 20);
}
