import { Exact, Match, Rule, Whitespace } from './base.ts'
import { Ast } from './deps.ts'
import { any, format, repeat, seq } from './rules.ts'

type ExpressionType = Ast.Expression['$']

export interface ExpressionOptions {
	omit?: ExpressionType[]
}

export function Expression(options: ExpressionOptions = {}): Rule<Ast.Expression> {
	const omit = options.omit || []
	const canParse = (expression: ExpressionType) => !omit.includes(expression)

	if (canParse('AdditionExpression') && canParse('SubtractionExpression')) return AdditiveExpression()
	if (canParse('NumberLiteral')) return NumberLiteral()

	throw new Error('Unknown Expression')
}

export function NumberLiteral(): Rule<Ast.NumberLiteral> {
	format(Match(/\d+/), ({ node }): Ast.NumberLiteral => {
		const int = parseInt(node)

		return { $: 'NumberLiteral', content: int }
	})
}

export type AdditiveExpression = Ast.AdditionExpression | Ast.SubtractionExpression

export function AdditiveExpression(): Rule<AdditiveExpression> {
	const getExpression = () => Expression({ omit: ['AdditionExpression', 'SubtractionExpression'] })

	const rule = seq([
		getExpression(),
		Whitespace(),
		repeat(
			seq([
				any([Exact('+'), Exact('-')]),
				Whitespace(),
				getExpression(),
			]),
		),
	])

	format(rule, ({ node }): AdditiveExpression => {
		const [firstExpression, _, repeats] = node

		for (const [operator, _, newExpression] of repeats) {
		}
	})
}
