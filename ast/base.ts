export interface Span {
	start: number
	end: number
}

export interface AstNode {
	span: Span
}
