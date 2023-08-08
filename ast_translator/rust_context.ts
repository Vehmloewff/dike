export interface EnumItem {
	name: string
	child: string | null
}

interface Enum {
	name: string
	items: EnumItem[]
}

export interface StructProperty {
	name: string
	type: string
}

interface Struct {
	name: string
	properties: StructProperty[]
}

export class RustContext {
	#enums: Enum[] = []
	#structs: Struct[] = []

	addEnum(name: string, items: EnumItem[]): void {
		this.#enums.push({ name, items })
	}

	addStruct(name: string, properties: StructProperty[]): void {
		this.#structs.push({ name, properties })
	}

	#codifyEnum(enumDef: Enum): string {
		const entries = enumDef.items
			.map((item) => {
				if (!item.child) return item.name

				return `${item.name}(${item.child})`
			})
			.map((line) => `\t${line},`)
			.join('\n')

		return `pub enum ${enumDef.name} {\n${entries}\n}`
	}

	#codifyStruct(structDef: Struct): string {
		const entries = structDef.properties
			.map((item) => `${item.name}: ${item.type}`)
			.map((line) => `\tpub ${line},`)
			.join('\n')

		return `pub struct ${structDef.name} {\n${entries}\n}`
	}

	getCode(): string {
		const enums = this.#enums
			.map((def) => this.#codifyEnum(def))
			.join('\n\n')

		const structs = this.#structs
			.map((def) => this.#codifyStruct(def))
			.join('\n\n')

		return `${enums}\n\n${structs}`
	}
}
