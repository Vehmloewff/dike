import { Exact, InlineWhitespace, Rule } from './base.ts'
import { Ast } from './deps.ts'
import { Expression } from './expression.ts'
import { any, format, optional, repeat, seq } from './rules.ts'

export function Statements(): Rule<Ast.Statement[]> {
	// TODO allow for unnecessary indentation and spacing
	const rule = seq([
		optional(repeat(
			seq([
				Statement(),
				Exact('\n'),
			]),
		)),
		optional(Statement()),
	])

	return format(rule, ({ node }) => {
		const statements: Ast.Statement[] = []
		const [repeating, lastStatement] = node

		for (const [statement] of repeating || []) statements.push(statement)
		if (lastStatement) statements.push(lastStatement)

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
