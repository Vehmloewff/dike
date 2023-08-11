import { astTranslator, colors, dtils, parser, pathUtils } from './deps.ts'

export async function addParseTest(args: string[]): Promise<void> {
	const [name, content] = args
	if (!name) throw new Error('Expected a text name as the first argument')

	const path = pathUtils.join('parser', 'test', name, 'code.dk')

	if (await dtils.exists(path)) {
		if (!confirm(`Parser test ${name} already exists. Are you sure you want to overwrite it?`)) return
	}

	await dtils.writeText(path, content)
	await fillParseTest([name])
}

export async function fillParseTests(): Promise<void> {
	if (!confirm(`Are you sure you want to re-fill the expected AST of all the parser tests?`)) return

	for await (const entry of Deno.readDir('parser/test')) await fillParseTest([entry.name])
}

export async function fillParseTest(args: string[]): Promise<void> {
	const [name] = args
	if (!name) throw new Error('Expected a test name as the first argument')

	const basePath = pathUtils.join('parser', 'test', name)
	const codePath = pathUtils.join(basePath, 'code.dk')

	if (!await dtils.exists(codePath)) throw new Error()

	const code = await dtils.readText(codePath)
	const { diagnostics, tokens, statements } = parser.parse(code)

	await dtils.writeJson(pathUtils.join(basePath, 'statements.json'), statements, { separator: '\t' })
	await dtils.writeJson(pathUtils.join(basePath, 'tokens.json'), tokens, { separator: '\t' })
	await dtils.writeJson(pathUtils.join(basePath, 'diagnostics.json'), diagnostics, { separator: '\t' })

	console.log(colors.green('Filled'), basePath)
}

export async function test(modules: string[]): Promise<void> {
	const hasModule = (module: string) => !modules.length || modules.includes(module)

	const denoTestFiles: string[] = []
	const cargoDirectories: string[] = []

	if (hasModule('parser')) denoTestFiles.push('parser/test/mod.ts', 'parser/*.test.ts')
	if (hasModule('vm')) cargoDirectories.push('vm')

	const getDenoCommands = () => {
		if (!denoTestFiles.length) return []

		const files = denoTestFiles.join(' ')

		if (dtils.getEnv() !== 'dev') return [`deno test -A ${files}`]
		return [`deno test -A --watch ${files}`]
	}

	const getCargoCommands = () => {
		return cargoDirectories.map((dir) => {
			const command = `test --manifest-path=${dir}/Cargo.toml --color=always -- --color always`

			if (dtils.getEnv() === 'dev') return `cargo watch -x "${command}"`
			return `cargo ${command}`
		})
	}

	const commands = [...getDenoCommands(), ...getCargoCommands()]

	await Promise.all(commands.map((command) => dtils.sh(command, { env: { COLOR_FORCE: '1' } })))
}

export async function translateAst(): Promise<void> {
	const newTypes = await astTranslator.translateAst({ tsEntry: 'ast/mod.ts' })

	const divider = '\n\n// ==== Impl ====\n\n'
	const libRs = await dtils.readText('rust_ast/src/lib.rs')
	const [_, impl] = libRs.split(divider)

	const newLibRs = `${newTypes}${divider}${impl || ''}`
	dtils.writeText('rust_ast/src/lib.rs', newLibRs)
}
