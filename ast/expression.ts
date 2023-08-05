export type Expression =
	| AdditionExpression
	| SubtractionExpression
	| MultiplicativeExpression
	| DivisionExpression
	| NumberLiteral
	| StringLiteral
	| BooleanLiteral

export interface AdditionExpression {
	$: 'AdditionExpression'
	left: Expression
	right: Expression
}

export interface SubtractionExpression {
	$: 'SubtractionExpression'
	left: Expression
	right: Expression
}

export interface MultiplicativeExpression {
	$: 'MultiplicationExpression'
	left: Expression
	right: Expression
}

export interface DivisionExpression {
	$: 'DivisionExpression'
	left: Expression
	right: Expression
}

export interface StringLiteral {
	$: 'StringLiteral'
	content: string
}

export interface NumberLiteral {
	$: 'NumberLiteral'
	content: number
}

export interface BooleanLiteral {
	$: 'BooleanLiteral'
	content: boolean
}
