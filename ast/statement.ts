import { AstNode } from './base.ts'
import { Expression } from './expression.ts'

export type Statement = ExpressionStatement | EmptyStatement

export interface EmptyStatement extends AstNode {
	$: 'EmptyStatement'
}

export interface ExpressionStatement extends AstNode {
	$: 'ExpressionStatement'
	expression: Expression
}
