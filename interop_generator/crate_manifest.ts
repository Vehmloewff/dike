export interface CrateManifest {
	name: string
	values: CrateValue[]
	functions: CrateFunction[]
}

export interface CrateValue {
	name: string
	methods: CrateFunction[]
}

export type CrateType = CrateTypeBuiltin | CrateTypeExternal | CrateTypeInternal

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

export interface CrateFunction {
	name: string
	arguments: CrateType[]
	returnType: CrateType
}
