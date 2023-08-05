import { Exact, Match, Rule, Whitespace } from './base.ts'
import { Ast } from './deps.ts'
import { any, format, repeat, seq, token } from './rules.ts'
import { MatchString } from './string.ts'

type ExpressionType = Ast.Expression['$']

export interface ExpressionOptions {
	omit?: ExpressionType[]
}

export function Expression(options: ExpressionOptions = {}): Rule<Ast.Expression> {
	const omit = options.omit || []
	const canParse = (expression: ExpressionType) => !omit.includes(expression)

	const rules: Rule<Ast.Expression>[] = []

	if (canParse('AdditionExpression') && canParse('SubtractionExpression')) rules.push(AdditiveExpression())

	if (canParse('StringLiteral')) rules.push(StringLiteral())
	if (canParse('BooleanLiteral')) rules.push(BooleanLiteral())
	if (canParse('NumberLiteral')) rules.push(NumberLiteral())

	return any(rules)
}

export function NumberLiteral(): Rule<Ast.NumberLiteral> {
	const rule = token('constant.numeric', Match(/\.\d+|\d+\.?\d*/))

	return format(rule, ({ node }): Ast.NumberLiteral => {
		const int = parseFloat(node)

		return { $: 'NumberLiteral', content: int }
	})
}

export function BooleanLiteral(): Rule<Ast.BooleanLiteral> {
	const rule = token('constant.language', any([Exact('true'), Exact('false')]))

	return format(rule, ({ node }): Ast.BooleanLiteral => {
		const bool = node === 'true' ? true : false

		return { $: 'BooleanLiteral', content: bool }
	})
}

export function StringLiteral(): Rule<Ast.StringLiteral> {
	const rule = token('strings.quoted', MatchString())

	return format(rule, ({ node }): Ast.StringLiteral => {
		return { $: 'StringLiteral', content: node }
	})
}

export type AdditiveExpression = Ast.AdditionExpression | Ast.SubtractionExpression

export function AdditiveExpression(): Rule<AdditiveExpression> {
	const getExpression = () => Expression({ omit: ['AdditionExpression', 'SubtractionExpression'] })

	const rule = seq([
		getExpression(),
		repeat(
			seq([
				Whitespace(),
				token('keyword.operator', any([Exact('+'), Exact('-')])),
				Whitespace(),
				getExpression(),
			]),
		),
	])

	return format(rule, ({ node }): AdditiveExpression => {
		const [firstExpression, repeats] = node

		const [_, initialOperator, __, secondExpression] = repeats[0]

		let additive: AdditiveExpression = {
			$: initialOperator === '+' ? 'AdditionExpression' : 'SubtractionExpression',
			left: firstExpression,
			right: secondExpression,
		}

		for (const [_, newOperator, __, newExpression] of repeats.slice(1)) {
			additive = {
				$: newOperator === '+' ? 'AdditionExpression' : 'SubtractionExpression',
				left: additive,
				right: newExpression,
			}
		}

		return additive
	})
}
