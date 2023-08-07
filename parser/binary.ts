import { Exact, Rule } from './base.ts'
import { Ast } from './deps.ts'
import { Expression } from './expression.ts'
import { any, format, optional, repeat, seq, token } from './rules.ts'
import { InlineWhitespace } from './whitespace.ts'

export interface BuildBinaryExpressionParams<T extends Ast.BinaryExpression> {
	types: T['$'][]
	operators: string[]
}

export function buildBinaryExpression<T extends Ast.BinaryExpression>(params: BuildBinaryExpressionParams<T>): Rule<T> {
	const getExpression = () => Expression({ omit: params.types })

	const rule = seq([
		getExpression(),
		repeat(
			seq([
				optional(InlineWhitespace()),
				token('keyword.operator', any(params.operators.map((operator) => Exact(operator)))),
				optional(InlineWhitespace()),
				optional(getExpression()),
			]),
		),
	])

	const getTypeFromOperator = (operator: string): T['$'] => {
		const operatorIndex = params.operators.indexOf(operator)
		return params.types[operatorIndex]
	}

	return format(rule, ({ node, addDiagnostic }): T => {
		const [firstExpression, repeats] = node

		const [beforeOperatorWhitespace, initialOperator, afterOperatorWhitespace, secondExpression] = repeats[0]

		const closestIndex = firstExpression.span.end + (beforeOperatorWhitespace?.length || 0) + initialOperator.length +
			(afterOperatorWhitespace?.length || 0)

		const filledSecondExpression: Ast.Expression = secondExpression ||
			{ $: 'NullLiteral', span: { start: closestIndex, end: closestIndex } }

		let additive: T = {
			$: getTypeFromOperator(initialOperator),
			left: firstExpression,
			right: filledSecondExpression,
			span: Ast.mergeSpans(firstExpression.span, filledSecondExpression.span),
		} as T

		if (!secondExpression) addDiagnostic({ message: `Missing second term in ${additive.$}`, span: additive.span })

		for (const [_, newOperator, __, newExpression] of repeats.slice(1)) {
			const newClosestIndex = firstExpression.span.end + (beforeOperatorWhitespace?.length || 0) + initialOperator.length +
				(afterOperatorWhitespace?.length || 0)

			const filledNewExpression: Ast.Expression = newExpression ||
				{ $: 'NullLiteral', span: { start: newClosestIndex, end: newClosestIndex } }

			additive = {
				$: getTypeFromOperator(newOperator),
				left: additive,
				right: filledNewExpression,
				span: Ast.mergeSpans(additive.span, filledNewExpression.span),
			} as T

			if (!newExpression) addDiagnostic({ message: `Missing second term in ${additive.$}`, span: additive.span })
		}

		return additive
	})
}
