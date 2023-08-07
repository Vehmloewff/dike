import { Span } from '../ast/base.ts'
import { NullLiteral, StringLiteral } from '../ast/expression.ts'
import { ResultOk, Rule } from './base.ts'

type StringType = 'double' | 'single' | 'backtick'

function getQuoteType(char: string): StringType | null {
	if (char === '"') return 'double'
	if (char === '\'') return 'single'
	if (char === '`') return 'backtick'

	return null
}

export function MatchString(): Rule<StringLiteral | NullLiteral> {
	return (text, start) => {
		if (!text.length) return null

		const stringType = getQuoteType(text[0])
		if (!stringType) return null

		let consumed = 1
		let stringSoFar = ''
		let isEscaped = false

		for (const char of text.slice(1)) {
			consumed++

			if (isEscaped) {
				stringSoFar += char
				isEscaped = false
				continue
			}

			if (char === '\\') {
				isEscaped = true
				continue
			}

			if (getQuoteType(char) === stringType) return validString(start, consumed, stringSoFar)
			if (char === '\n') {
				consumed-- // We don't want to consume the newline here
				return unterminatedString(start, consumed, stringSoFar)
			}

			stringSoFar += char
		}

		return unterminatedString(start, consumed, stringSoFar)
	}
}

function validString(start: number, consumed: number, content: string): ResultOk<StringLiteral> {
	const span: Span = { start, end: start + consumed }

	return {
		consumed,
		diagnostics: [],
		node: { $: 'StringLiteral', content, span },
		tokens: [{ name: 'string.quoted', span }],
	}
}

function unterminatedString(start: number, consumed: number, content: string): ResultOk<StringLiteral> {
	const span: Span = { start, end: start + consumed }

	return {
		consumed,
		diagnostics: [{ span, message: 'Unterminated string literal' }],
		node: { $: 'StringLiteral', content, span },
		tokens: [{ name: 'string.quoted', span }],
	}
}
