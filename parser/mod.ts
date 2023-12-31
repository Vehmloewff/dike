import { Diagnostic, Token } from './base.ts'
import { Ast } from './deps.ts'
import { OmitManager } from './omits.ts'
import { Statements } from './statement.ts'

export interface ParseResult {
	tokens: Token[]
	diagnostics: Diagnostic[]
	statements: Ast.Statement[]
}

export function parse(code: string): ParseResult {
	const res = Statements()(code, 0, new OmitManager())

	if (!res) throw new Error('Failed to parse')
	if (res.consumed !== code.length) {
		throw new Error(`Only consumed ${res.consumed}/${code.length} characters. Ast: ${Deno.inspect(res.node, { depth: Infinity })}`)
	}

	return { tokens: res.tokens, diagnostics: res.diagnostics, statements: res.node }
}
