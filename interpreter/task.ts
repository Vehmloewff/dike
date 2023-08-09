import { Instruction } from './instructions.ts'

export interface Task {
	memory: Map<string, number>
	instructions: Instruction[]
}

export interface MemoryDescriptor {
}

// A task with upper-scope dependencies runs
// -> When the task is queued at runtime, we:
//    -> Get a list of those dependencies
//
