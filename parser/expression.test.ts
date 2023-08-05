import { asserts, Ast } from './deps.ts'
import { Expression } from './expression.ts'
import * as parser from './parse.ts'

const parse = (code: string) => {
	return parser.parse(code, { rule: Expression() }).statements
}

Deno.test('NumberLiteral parses', () => {
	const code = '99'

	asserts.assertEquals(parse(code), { $: 'NumberLiteral', content: 99, span: Ast.getSpan(0, 2) })
})

Deno.test('NumberLiteral parses floats', () => {
	asserts.assertEquals(parse('99.101'), { $: 'NumberLiteral', content: 99.101, span: Ast.getSpan(0, 6) })
	asserts.assertEquals(parse('.101'), { $: 'NumberLiteral', content: 0.101, span: Ast.getSpan(0, 4) })
	asserts.assertEquals(parse('99.'), { $: 'NumberLiteral', content: 99, span: Ast.getSpan(0, 3) })
})

Deno.test('BooleanLiteral parses', () => {
	asserts.assertEquals(parse('true'), { $: 'BooleanLiteral', content: true, span: Ast.getSpan(0, 4) })
	asserts.assertEquals(parse('false'), { $: 'BooleanLiteral', content: false, span: Ast.getSpan(0, 5) })
})

Deno.test('StringLiteral parses', () => {
	asserts.assertEquals(parse(`'this is a string'`), { $: 'StringLiteral', content: 'this is a string', span: Ast.getSpan(0, 18) })
	asserts.assertThrows(() => parse(`'not a string`))
	asserts.assertThrows(() => parse(`'not one either"`))
	asserts.assertEquals(parse(`"one o' here"`), { $: 'StringLiteral', content: 'one o\' here', span: Ast.getSpan(0, 13) })
	asserts.assertEquals(parse(`"escaped \\""`), { $: 'StringLiteral', content: 'escaped "', span: Ast.getSpan(0, 12) })
	asserts.assertEquals(parse(`"escaped escape\\\\"`), { $: 'StringLiteral', content: 'escaped escape\\', span: Ast.getSpan(0, 18) })
	asserts.assertThrows(() => parse(`'not a string\\'`))
})

Deno.test('AdditiveExpression parses', () => {
	const code = '3 + 4'

	asserts.assertEquals(parse(code), {
		$: 'AdditionExpression',
		left: { $: 'NumberLiteral', content: 3, span: Ast.getSpan(0, 1) },
		right: { $: 'NumberLiteral', content: 4, span: Ast.getSpan(4, 5) },
		span: Ast.getSpan(0, 5),
	})
})

Deno.test('AdditiveExpression parses multiple additions together', () => {
	const code = '3 + 4 + 5 - 6 + 10'

	asserts.assertEquals(parse(code), {
		'$': 'AdditionExpression',
		left: {
			'$': 'SubtractionExpression',
			left: {
				'$': 'AdditionExpression',
				left: {
					'$': 'AdditionExpression',
					left: {
						'$': 'NumberLiteral',
						content: 3,
						span: { start: 0, end: 1 },
					},
					right: {
						'$': 'NumberLiteral',
						content: 4,
						span: { start: 4, end: 5 },
					},
					span: { start: 0, end: 5 },
				},
				right: { '$': 'NumberLiteral', content: 5, span: { start: 8, end: 9 } },
				span: { start: 0, end: 9 },
			},
			right: { '$': 'NumberLiteral', content: 6, span: { start: 12, end: 13 } },
			span: { start: 0, end: 13 },
		},
		right: { '$': 'NumberLiteral', content: 10, span: { start: 16, end: 18 } },
		span: { start: 0, end: 18 },
	})
})
