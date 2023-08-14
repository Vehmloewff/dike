export interface CrateManifest {
	name: string
	values: CrateValue[]
	functions: CrateFunction[]
}

export interface CrateValue {
	name: string
	params: string[]
	methods: CrateFunction[]
}

export type CrateType = CrateTypeBuiltin | CrateTypeExternal | CrateTypeInternal | CrateTypeParam

export interface CrateTypeBuiltin {
	type: 'builtin'
	name: string
}

export interface CrateTypeExternal {
	type: 'external'
	crate: string
	name: string
}

export interface CrateTypeInternal {
	type: 'internal'
	name: string
}

export interface CrateTypeParam {
	type: 'param'
	name: string
}

export interface CrateFunction {
	name: string
	arguments: CrateType[]
	returnType: CrateType
}
