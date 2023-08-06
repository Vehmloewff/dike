import { AstNode } from './base.ts'

export type Expression =
	| BinaryExpression
	| NumberLiteral
	| StringLiteral
	| BooleanLiteral
	| NullLiteral

export type BinaryExpression =
	| AdditionExpression
	| SubtractionExpression
	| MultiplicationExpression
	| DivisionExpression
	| GreaterThanExpression
	| GreaterThanOrEqualToExpression
	| LessThanExpression
	| LessThanOrEqualToExpression
	| AndExpression
	| OrExpression

export interface Identifier extends AstNode {
	name: string
}

//
// Object-related Expressions
//

export interface MemberExpression extends AstNode {
	$: 'MemberExpression'
	base: Expression
	name: Identifier
}

export interface CallArgument extends AstNode {
	name: Identifier
	value: Expression
}

export interface CallExpression extends AstNode {
	$: 'CallExpression'
	base: Expression
	arguments: CallArgument[]
}

//
// Conditional Operators
//

export interface OrExpression extends AstNode {
	$: 'OrExpression'
	left: Expression
	right: Expression
}

export interface AndExpression extends AstNode {
	$: 'AndExpression'
	left: Expression
	right: Expression
}

//
// Comparison Operators
//

export interface GreaterThanExpression extends AstNode {
	$: 'GreaterThanExpression'
	left: Expression
	right: Expression
}

export interface GreaterThanOrEqualToExpression extends AstNode {
	$: 'GreaterThanOrEqualToExpression'
	left: Expression
	right: Expression
}

export interface LessThanExpression extends AstNode {
	$: 'LessThanExpression'
	left: Expression
	right: Expression
}

export interface LessThanOrEqualToExpression extends AstNode {
	$: 'LessThanOrEqualToExpression'
	left: Expression
	right: Expression
}

//
// Math Operators
//

export interface AdditionExpression extends AstNode {
	$: 'AdditionExpression'
	left: Expression
	right: Expression
}

export interface SubtractionExpression extends AstNode {
	$: 'SubtractionExpression'
	left: Expression
	right: Expression
}

export interface MultiplicationExpression extends AstNode {
	$: 'MultiplicationExpression'
	left: Expression
	right: Expression
}

export interface DivisionExpression extends AstNode {
	$: 'DivisionExpression'
	left: Expression
	right: Expression
}

//
// Literals
//

export interface StringLiteral extends AstNode {
	$: 'StringLiteral'
	content: string
}

export interface NumberLiteral extends AstNode {
	$: 'NumberLiteral'
	content: number
}

export interface BooleanLiteral extends AstNode {
	$: 'BooleanLiteral'
	content: boolean
}

export interface NullLiteral extends AstNode {
	$: 'NullLiteral'
}
