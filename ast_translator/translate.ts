import { docTypes } from './deps.ts'
import { EnumItem, StructProperty } from './rust_context.ts'

export function translateInterfaceProperties(def: docTypes.InterfaceDef): StructProperty[] {
	const properties: StructProperty[] = []

	const extension = def.extends[0]
	if (extension && extension.repr === 'AstNode') properties.push({ name: 'span', type: 'Span' })

	for (const interfaceProperty of def.properties) {
		if (interfaceProperty.name === '$') continue
		if (!interfaceProperty.tsType) throw new Error('Expected a tsType on all interfaces')

		properties.push({ name: interfaceProperty.name, type: translateType(interfaceProperty.tsType) })
	}

	return properties
}

export function translateType(def: docTypes.TsTypeDef): string {
	if (def.kind === 'array') return `Vec<${def.array.repr}>`
	if (def.kind === 'typeRef') return def.repr
	if (def.kind === 'keyword') {
		if (def.repr === 'string') return 'String'
		if (def.repr === 'boolean') return 'bool'
		if (def.repr === 'number') return 'usize'
	}

	throw new Error(`Unknown type def (${def.kind}): ${def.repr}`)
}

export function translateTypeAlias(def: docTypes.TypeAliasDef): EnumItem[] {
	const items: EnumItem[] = []

	if (def.tsType.kind !== 'union') throw new Error('All types must be unions')
	const union = def.tsType.union

	for (const type of union) {
		if (type.kind === 'literal' && type.literal.kind === 'string') items.push({ name: type.literal.string, child: null })
		else if (type.kind === 'typeRef') items.push({ name: type.typeRef.typeName, child: `Box<${type.typeRef.typeName}>` })
	}

	return items
}
