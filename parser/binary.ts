import { Exact, InlineWhitespace, Rule } from './base.ts'
import { Ast } from './deps.ts'
import { Expression } from './expression.ts'
import { any, format, optional, repeat, seq, token } from './rules.ts'

export interface BuildBinaryExpressionParams<T extends Ast.BinaryExpression> {
	types: T['$'][]
	operators: string[]
}

export function buildBinaryExpression<T extends Ast.BinaryExpression>(params: BuildBinaryExpressionParams<T>): Rule<T> {
	const getExpression = () =>
		format(
			Expression({ omit: params.types }),
			({ node, span }) => ({ expr: node, span }),
		)

	const rule = seq([
		getExpression(),
		repeat(
			seq([
				optional(InlineWhitespace()),
				token('keyword.operator', any(params.operators.map((operator) => Exact(operator)))),
				optional(InlineWhitespace()),
				getExpression(),
			]),
		),
	])

	const getTypeFromOperator = (operator: string): T['$'] => {
		const operatorIndex = params.operators.indexOf(operator)
		return params.types[operatorIndex]
	}

	return format(rule, ({ node }): T => {
		const [firstExpression, repeats] = node

		const [_, initialOperator, __, secondExpression] = repeats[0]

		let additive: T = {
			$: getTypeFromOperator(initialOperator),
			left: firstExpression.expr,
			right: secondExpression.expr,
			span: Ast.mergeSpans(firstExpression.span, secondExpression.span),
		} as T

		for (const [_, newOperator, __, newExpression] of repeats.slice(1)) {
			additive = {
				$: getTypeFromOperator(newOperator),
				left: additive,
				right: newExpression.expr,
				span: Ast.mergeSpans(additive.span, newExpression.span),
			} as T
		}

		return additive
	})
}
