import { Ast } from './deps.ts'
import { OmitManager } from './omits.ts'

export interface Token {
	span: Ast.Span
	name: string
}

export interface Diagnostic {
	span: Ast.Span
	message: string
}

export interface ResultOk<T> {
	node: T
	consumed: number
	tokens: Token[]
	diagnostics: Diagnostic[]
}

export type Result<T> = ResultOk<T> | null

export type Rule<T> = (text: string, start: number, omitManager: OmitManager) => Result<T>

export function Exact(content: string): Rule<string> {
	return (text) => {
		if (!text.startsWith(content)) return null

		return { consumed: content.length, diagnostics: [], tokens: [], node: content }
	}
}

export function Chars(chars: string[]): Rule<string> {
	return (text) => {
		if (!text.length) return null

		const nextChar = text.charAt(0)
		if (!chars.includes(nextChar)) return null

		return { consumed: 1, diagnostics: [], node: nextChar, tokens: [] }
	}
}

export function Match(regex: RegExp): Rule<string> {
	return (text) => {
		const res = text.match(regex)
		if (!res) return null
		if (!res.length) return null

		const match = res[0]
		if (!text.startsWith(match)) return null

		return { consumed: match.length, diagnostics: [], node: match, tokens: [] }
	}
}

export function Whitespace(): Rule<string> {
	return Match(/\s+/)
}

export function InlineWhitespace(): Rule<string> {
	return Match(/[ \t]+/)
}

export function Id(): Rule<string> {
	return Match(/[a-z_$][0-9a-z_$]*/i)
}
