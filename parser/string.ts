import { Rule } from './base.ts'

type StringType = 'double' | 'single' | 'backtick'

function getQuoteType(char: string): StringType | null {
	if (char === '"') return 'double'
	if (char === '\'') return 'single'
	if (char === '`') return 'backtick'

	return null
}

export function MatchString(): Rule<string> {
	return (text) => {
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

			if (getQuoteType(char) === stringType) return { consumed, diagnostics: [], node: stringSoFar, tokens: [] }

			stringSoFar += char
		}

		return null
	}
}
