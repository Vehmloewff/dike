export type Value = StrValue

export interface StrValue {
	$: 'str'
	value: string
}

export interface BoolValue {
	$: 'bool'
	value: boolean
}

export interface IntValue {
	$: 'int'
	value: number
}

export interface FloatValue {
	$: 'float'
	value: number
}

export interface BigIntValue {
	$: 'big_int'
	value: number
}

export interface BigFloatValue {
	$: 'big_float'
	value: number
}

export interface ArrayValue {
	$: 'array'
	values: Value[]
}

export interface FunctionPointerValue {
	$: 'function_pointer'
	pointer: number
}
