import { denoDoc } from './deps.ts'
import { RustContext } from './rust_context.ts'
import { translateInterfaceProperties, translateTypeAlias } from './translate.ts'

export interface TranslateAstParams {
	tsEntry: string
}

export async function translateAst(params: TranslateAstParams): Promise<string> {
	const tsUrl = new URL(params.tsEntry, `file://${Deno.cwd()}/`)
	const meta = await denoDoc.doc(tsUrl.toString())

	const context = new RustContext()

	for (const node of meta) {
		if (node.kind === 'interface') context.addStruct(node.name, translateInterfaceProperties(node.interfaceDef))
		if (node.kind === 'typeAlias') context.addEnum(node.name, translateTypeAlias(node.typeAliasDef))
	}

	return context.getCode()
}
