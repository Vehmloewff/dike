import { Comment, Rule } from './base.ts'

export interface WhitespaceParams {
	omitNewline?: boolean
}

export function Whitespace(params: WhitespaceParams = {}): Rule<string> {
	return (text, start) => {
		const whitespaceChars = [' ', '\t']
		const comments: Comment[] = []

		let consumed = 0
		let currentComment: Comment | null = null

		if (!params.omitNewline) whitespaceChars.push('\n')

		const reduceComment = () => {
			if (!currentComment) throw new Error('Cannot reduce comment. There is no currentComment')

			comments.push(currentComment)
			currentComment = null
		}

		while (consumed < text.length) {
			consumed++

			const textIndex = consumed - 1
			const globalIndex = start + consumed

			const char = text[textIndex]
			const nextChar = text[textIndex + 1]

			if (currentComment) {
				currentComment.text += char
				currentComment.span.end++

				if (nextChar === '\n') reduceComment()

				continue
			}

			// If we encounter a slash, we will only continue if the next char is a slash. This means that we are starting a comment
			if (char === '/' && nextChar === '/') {
				currentComment = {
					text: '',
					span: { start: globalIndex + 1, end: globalIndex + 1 }, // We have to use globalIndex + 1 because we need to step over the second slash
				}

				consumed++ // We will consume an extra char so that we skip the next slash

				// Real quick here, we need to make sure that the next char is not a newline. If it is, we need to reduce the comment right away
				if (text[textIndex + 2] === '\n') {
					reduceComment()
					continue
				}

				continue
			}

			// We will continue if the current character is a whitespace char
			if (whitespaceChars.includes(char)) continue

			// We have passed all valid cases, so we only have invalid cases left.
			// Thus, because we don't want to consume an invalid case, we will backtrack once
			consumed--

			break
		}

		if (currentComment) reduceComment()

		return { comments, consumed, diagnostics: [], node: text.slice(0, consumed), tokens: [] }
	}
}

export function InlineWhitespace(): Rule<string> {
	return Whitespace({ omitNewline: true })
}
