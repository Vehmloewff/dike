use instruction_graph::InstructionGraph;
use memory::Memory;
use stack::Stack;
use sweep_executor::{SweepExecutor, SweepExecutorResult};

use crate::value::Value;

pub mod array;
pub mod instruction;
pub mod instruction_graph;
pub mod instruction_sweep;
pub mod memory;
pub mod number;
pub mod stack;
pub mod string;
pub mod sweep_executor;
pub mod value;

mod interop_gen;

#[cfg(test)]
mod test;

pub struct VirtualMachine {
    memory: Memory,
    pub graph: InstructionGraph,
}

impl VirtualMachine {
    /// Create a new (empty) instruction graph
    pub fn new(graph: InstructionGraph) -> VirtualMachine {
        VirtualMachine {
            memory: Memory::new(),
            graph,
        }
    }

    /// Run sweep `sweep_pointer` on `stack`, giving the stack up at the end
    pub fn run_sweep_on_stack<'a>(&self, sweep_pointer: usize, stack: Stack<'a>) -> Stack<'a> {
        let mut executor = SweepExecutor::setup(&self.memory, &self.graph, sweep_pointer);

        loop {
            match executor.tick(&stack) {
                SweepExecutorResult::Next => continue,
                SweepExecutorResult::Done => break,
            }
        }

        stack
    }

    /// Run the graph. Creates a new stack and starts executing sweep 0
    pub fn run(&self) {
        self.run_sweep_on_stack(0, Stack::new(&self.memory));
    }

    /// Run the graph, returning the last value on the stack after sweep `sweep_pointer` has executed
    pub fn get_last_value<'a>(&self, sweep_pointer: usize) -> Value {
        let stack = self.run_sweep_on_stack(sweep_pointer, Stack::new(&self.memory));

        stack.consume()
    }
}
