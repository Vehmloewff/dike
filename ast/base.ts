export interface Span {
	start: number
	end: number
}

export interface AstNode {
	span: Span
}

export function getSpan(start: number, end: number): Span {
	return { start, end }
}

export function mergeSpans(start: Span, end: Span): Span {
	return { start: start.start, end: end.end }
}
