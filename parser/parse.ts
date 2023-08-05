import { Expression } from '../ast/expression.ts'
import { Diagnostic, Rule, Token } from './base.ts'
import { Ast } from './deps.ts'
import * as expr from './expression.ts'
import { OmitManager } from './omits.ts'

export interface ParseParams<T> {
	rule?: Rule<T>
}

export interface ParseResult<T = Expression> {
	tokens: Token[]
	diagnostics: Diagnostic[]
	statements: T
}

export function parse<T = Ast.Expression>(code: string, params: ParseParams<T>): ParseResult<T> {
	const expression = params.rule || expr.Expression()
	const res = expression(code, 0, new OmitManager())

	if (!res) throw new Error('Failed to parse')
	if (res.consumed !== code.length) {
		throw new Error(`Only consumed ${res.consumed}/${code.length} characters. Ast: ${Deno.inspect(res.node, { depth: Infinity })}`)
	}

	return { tokens: res.tokens, diagnostics: res.diagnostics, statements: res.node as T }
}
