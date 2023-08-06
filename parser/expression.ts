import { Exact, Match, Rule } from './base.ts'
import { buildBinaryExpression } from './binary.ts'
import { Ast } from './deps.ts'
import { any, catchState, format, token } from './rules.ts'
import { MatchString } from './string.ts'

type ExpressionType = Ast.Expression['$']

export interface ExpressionOptions {
	omit?: ExpressionType[]
}

export function Expression(options: ExpressionOptions = {}): Rule<Ast.Expression> {
	return catchState((_, start, omitManager) => {
		const canParse = omitManager.makeCanParseExpression(start, options.omit || [])

		const rules: Rule<Ast.Expression>[] = []

		if (
			canParse('AdditionExpression') &&
			canParse('SubtractionExpression')
		) rules.push(AdditiveExpression())

		if (
			canParse('MultiplicationExpression') &&
			canParse('DivisionExpression')
		) rules.push(MultiplicativeExpression())

		if (
			canParse('LessThanExpression') &&
			canParse('GreaterThanExpression') &&
			canParse('LessThanOrEqualToExpression') &&
			canParse('GreaterThanOrEqualToExpression')
		) rules.push(ComparisonExpression())

		if (
			canParse('AndExpression') &&
			canParse('OrExpression')
		) rules.push(ConditionalExpression())

		if (canParse('BooleanLiteral')) rules.push(BooleanLiteral())
		if (canParse('NumberLiteral')) rules.push(NumberLiteral())
		if (canParse('StringLiteral')) rules.push(StringLiteral())
		if (canParse('NullLiteral')) rules.push(NullLiteral())

		return any(rules)
	})
}

export function NumberLiteral(): Rule<Ast.NumberLiteral> {
	const rule = token('constant.numeric', Match(/\.\d+|\d+\.?\d*/))

	return format(rule, ({ node, span }): Ast.NumberLiteral => {
		const int = parseFloat(node)

		return { $: 'NumberLiteral', content: int, span }
	})
}

export function BooleanLiteral(): Rule<Ast.BooleanLiteral> {
	const rule = token('constant.language', any([Exact('true'), Exact('false')]))

	return format(rule, ({ node, span }): Ast.BooleanLiteral => {
		const bool = node === 'true' ? true : false

		return { $: 'BooleanLiteral', content: bool, span }
	})
}

export function StringLiteral(): Rule<Ast.StringLiteral> {
	const rule = token('strings.quoted', MatchString())

	return format(rule, ({ node, span }): Ast.StringLiteral => {
		return { $: 'StringLiteral', content: node, span }
	})
}

export type AdditiveExpression = Ast.AdditionExpression | Ast.SubtractionExpression

export function AdditiveExpression(): Rule<AdditiveExpression> {
	return buildBinaryExpression({
		types: ['AdditionExpression', 'SubtractionExpression'],
		operators: ['+', '-'],
	})
}

export type MultiplicativeExpression = Ast.MultiplicationExpression | Ast.DivisionExpression

export function MultiplicativeExpression(): Rule<MultiplicativeExpression> {
	return buildBinaryExpression({
		types: ['MultiplicationExpression', 'DivisionExpression'],
		operators: ['*', '/'],
	})
}

export type ComparisonExpression =
	| Ast.GreaterThanExpression
	| Ast.LessThanExpression
	| Ast.GreaterThanOrEqualToExpression
	| Ast.LessThanOrEqualToExpression

export function ComparisonExpression(): Rule<ComparisonExpression> {
	return buildBinaryExpression({
		types: ['GreaterThanExpression', 'LessThanExpression', 'GreaterThanOrEqualToExpression', 'LessThanOrEqualToExpression'],
		operators: ['>', '<', '>=', '<='],
	})
}

export type ConditionalExpression = Ast.AndExpression | Ast.OrExpression

export function ConditionalExpression(): Rule<ConditionalExpression> {
	return buildBinaryExpression({
		types: ['AndExpression', 'OrExpression'],
		operators: ['&&', '||'],
	})
}

export function NullLiteral(): Rule<Ast.NullLiteral> {
	return format(token('constant.language', Exact('null')), ({ span }): Ast.NullLiteral => {
		return { $: 'NullLiteral', span }
	})
}
