import { asserts } from '../deps.ts'
import { dtils, parser, pathUtils } from './deps.ts'

for await (const entry of Deno.readDir('parser/test')) {
	if (!entry.isDirectory) continue
	const name = entry.name

	const base = pathUtils.join('parser/test', name)
	const code = await dtils.readText(pathUtils.join(base, 'code.dk'))
	if (!code) throw new Error(`Expected ${pathUtils.join(base, 'code.dk')} to be a file`)

	let result: parser.ParseResult

	Deno.test(`${name} parses`, () => {
		result = parser.parse(code)
	})

	Deno.test(`${name} matches ast`, async () => {
		const expectedAst = await dtils.readJson(pathUtils.join(base, 'statements.json'))
		asserts.assertEquals(result.statements, expectedAst)
	})

	Deno.test(`${name} gives expected tokens`, async () => {
		const expectedAst = await dtils.readJson(pathUtils.join(base, 'tokens.json'))
		asserts.assertEquals(result.tokens, expectedAst)
	})

	Deno.test(`${name} throws necessary diagnostics`, async () => {
		const expectedAst = await dtils.readJson(pathUtils.join(base, 'diagnostics.json'))
		asserts.assertEquals(result.diagnostics, expectedAst)
	})
}
