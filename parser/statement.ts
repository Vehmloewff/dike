import { Match, Rule } from './base.ts'
import { Ast } from './deps.ts'
import { Expression } from './expression.ts'
import { any, format, optional, repeat, seq } from './rules.ts'
import { InlineWhitespace } from './whitespace.ts'

export function Statements(): Rule<Ast.Statement[]> {
	// NOTE: due to the way things work, we have to be real careful with this here.
	// Because of the way that Expression prevents infinite recursion by ignoring certain rules at certain indexes,
	// If we let Statement() succeed, but then ask for it to succeed again, we won't get any binary expressions
	// This is because they have all been already filtered out at that start index
	const rule = repeat(
		seq([
			Statement(),
			optional(InlineWhitespace()),
			Match(/$|\n/),
		]),
	)

	return format(rule, ({ node }) => {
		const statements: Ast.Statement[] = []

		for (const [statement] of node) statements.push(statement)

		return statements
	})
}

export function Statement(): Rule<Ast.Statement> {
	return any<Ast.Statement>([ExpressionStatement(), EmptyStatement()])
}

export function ExpressionStatement(): Rule<Ast.ExpressionStatement> {
	return format(Expression(), ({ node, span }): Ast.ExpressionStatement => {
		return { $: 'ExpressionStatement', expression: node, span }
	})
}

export function EmptyStatement(): Rule<Ast.EmptyStatement> {
	return format(optional(InlineWhitespace()), ({ span }): Ast.EmptyStatement => {
		return { $: 'EmptyStatement', span }
	})
}
