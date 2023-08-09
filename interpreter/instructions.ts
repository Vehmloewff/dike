import { Value } from './value.ts'

export type Instruction = LoadInstruction

/** Consumes 0, outputs 1 */
export interface LoadInstruction {
	$: 'load'
	value: Value
}

/** Consumes 0, outputs 1 */
export interface ReadInstruction {
	$: 'read'
	name: string
}

/** Consumes 1 */
export interface WriteInstruction {
	$: 'write'
	name: string
}

/** Consumes 2, outputs 1 */
export interface AddInstruction {
	$: 'add'
}

/** Consumes 2, outputs 1 */
export interface SubtractInstruction {
	$: 'subtract'
}

/** Consumes 2, outputs 1 */
export interface MultiplyInstruction {
	$: 'multiply'
}

/** Consumes 2, outputs 1 */
export interface DivideInstruction {
	$: 'divide'
}

/** Consumes 2, outputs 1. */
export interface Concat {
	$: 'concat'
}

/** Consumes 1. Just pops a value off the stack */
export interface PopInstruction {
	$: 'pop'
}

/**
 * Consumes 1+, Outputs 0+. Goes to a function pointer (consumed) the function pointer will receive the
 * current stack and the stack will be changed as the function pointer changes it */
export interface GoToInstruction {
	$: 'go_to'
}

/** Halt the current task with whatever is on the stack */
export interface HaltInstruction {
	$: 'give'
}

/**
 * Consumes 2. If the consumed value (E-0) is Bool::true, the consumed function pointer
 * (E-1) will be gone to. */
export interface IfInstruction {
	$: 'if'
}

/** Same thing as IfInstruction, except that after the task at functionPointer runs, halt is called */
export interface HaltIfInstruction {
	$: 'halt_if'
}
