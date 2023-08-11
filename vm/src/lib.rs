use instruction_graph::InstructionGraph;
use memory::Memory;
use stack::Stack;
use sweep_executor::{SweepExecutor, SweepExecutorResult};

use crate::value::Value;

pub mod arithmetic;
pub mod instruction;
pub mod instruction_graph;
pub mod instruction_sweep;
pub mod memory;
pub mod number;
pub mod stack;
pub mod sweep_executor;
pub mod value;

pub struct VirtualMachine {
    memory: Memory,
    graph: InstructionGraph,
}

impl VirtualMachine {
    pub fn new(graph: InstructionGraph) -> VirtualMachine {
        VirtualMachine {
            memory: Memory::new(),
            graph,
        }
    }

    pub fn run(&self) {
        let mut executor = SweepExecutor::setup(&self.memory, &self.graph, 0);
        let stack = Stack::new(&self.memory);

        loop {
            match executor.tick(&stack) {
                SweepExecutorResult::Next => continue,
                SweepExecutorResult::Done => break,
            }
        }

        let leftover = stack.consume();

        match leftover {
            Value::Int(num) => println!("{}", num),
            _ => panic!("Expected int"),
        };

        // println!("{}", .get_human_type());
    }
}
