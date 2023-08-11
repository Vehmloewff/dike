use crate::{
    instruction::Instruction, instruction_graph::InstructionGraph,
    instruction_sweep::InstructionSweep, value::Value, VirtualMachine,
};

#[test]
fn simple_addition() {
    let graph = InstructionGraph::new().add_sweep(
        InstructionSweep::new(0)
            .add(Instruction::LoadInt(2))
            .add(Instruction::LoadInt(2))
            .add(Instruction::Add),
    );

    let result = VirtualMachine::new(graph).get_last_value(0);

    match result {
        Value::Int(int) => assert_eq!(int, 4),
        _ => panic!("Expected an integer result"),
    }
}

#[test]
fn variables_work() {
    let graph = InstructionGraph::new().add_sweep(
        InstructionSweep::new(1)
            .add(Instruction::LoadInt(10))
            .add(Instruction::Write(0))
            .add(Instruction::Use(0)),
    );

    let vm = VirtualMachine::new(graph);
    let result = vm.get_last_value(0);

    match result {
        Value::Ref(address) => match *vm.memory.get(address) {
            Value::Int(int) => assert_eq!(int, 10),
            _ => panic!("Expected an int"),
        },
        _ => panic!("Expected an variable reference"),
    }
}
