use crate::{
    instruction::ExecuteResult, instruction_graph::InstructionGraph,
    instruction_sweep::InstructionSweep, memory::Memory, stack::Stack,
};

pub enum SweepExecutorResult {
    Done,
    Next,
}

pub struct SweepExecutor<'a> {
    graph: &'a InstructionGraph,
    memory: &'a Memory,
    sweep_index: usize,
    sweep: &'a InstructionSweep,
    memory_addresses: Vec<usize>,
    child_executor: Option<Box<SweepExecutor<'a>>>,
}

impl SweepExecutor<'_> {
    pub fn setup<'a>(
        memory: &'a Memory,
        graph: &'a InstructionGraph,
        sweep_pointer: usize,
    ) -> SweepExecutor<'a> {
        let sweep = graph.get_sweep(sweep_pointer);

        let mut executor = SweepExecutor {
            memory,
            graph,
            sweep_index: 0,
            sweep,
            memory_addresses: Vec::new(),
            child_executor: None,
        };

        executor.start();
        executor
    }

    pub fn start(&mut self) {
        for _ in 0..self.sweep.local_variables_count {
            self.memory_addresses.push(self.memory.allocate())
        }
    }

    pub fn finish(&mut self) -> SweepExecutorResult {
        while self.memory_addresses.len() > 0 {
            self.memory.deref(self.memory_addresses.pop().unwrap())
        }

        SweepExecutorResult::Done
    }

    pub fn tick(&mut self, stack: &Stack) -> SweepExecutorResult {
        let child = self.child_executor.as_deref_mut();

        if child.is_some() {
            return child.unwrap().tick(stack);
        }

        let instruction = self.sweep.get_instruction(self.sweep_index);
        let result = instruction.execute(self.memory, stack);

        match result {
            ExecuteResult::Done => self.finish(),
            ExecuteResult::GoTo(sweep_pointer) => {
                self.queue_child(sweep_pointer);

                SweepExecutorResult::Next
            }
            ExecuteResult::Next => self.next_instruction(1, stack),
            ExecuteResult::Skip(skip) => self.next_instruction(skip, stack),
        }
    }

    fn queue_child(&mut self, sweep_pointer: usize) {
        self.child_executor = Some(Box::new(SweepExecutor::setup(
            self.memory,
            self.graph,
            sweep_pointer,
        )));
    }

    fn next_instruction(&mut self, skip: usize, stack: &Stack) -> SweepExecutorResult {
        self.sweep_index += skip;

        if self.sweep.get_length() > self.sweep_index {
            self.tick(stack)
        } else {
            self.finish()
        }
    }
}
