import { asserts } from './deps.ts'
import { Expression } from './expression.ts'

const parse = (code: string) => {
	const res = Expression()(code, 0)
	if (!res) throw new Error('Couldn\'t parse code')

	if (res.consumed !== code.length) throw new Error(`Couldn\'t parse entire file. Got: ${Deno.inspect(res.node, { depth: Infinity })}`)

	return res.node
}

Deno.test('NumberLiteral parses', () => {
	const code = '99'

	asserts.assertEquals(parse(code), { $: 'NumberLiteral', content: 99 })
})

Deno.test('NumberLiteral parses floats', () => {
	asserts.assertEquals(parse('99.101'), { $: 'NumberLiteral', content: 99.101 })
	asserts.assertEquals(parse('.101'), { $: 'NumberLiteral', content: 0.101 })
	asserts.assertEquals(parse('99.'), { $: 'NumberLiteral', content: 99 })
})

Deno.test('BooleanLiteral parses', () => {
	asserts.assertEquals(parse('true'), { $: 'BooleanLiteral', content: true })
	asserts.assertEquals(parse('false'), { $: 'BooleanLiteral', content: false })
})

Deno.test('StringLiteral parses', () => {
	asserts.assertEquals(parse(`'this is a string'`), { $: 'StringLiteral', content: 'this is a string' })
	asserts.assertThrows(() => parse(`'not a string`))
	asserts.assertThrows(() => parse(`'not one either"`))
	asserts.assertEquals(parse(`"one o' here"`), { $: 'StringLiteral', content: 'one o\' here' })
	asserts.assertEquals(parse(`"escaped \\""`), { $: 'StringLiteral', content: 'escaped "' })
	asserts.assertEquals(parse(`"escaped escape\\\\"`), { $: 'StringLiteral', content: 'escaped escape\\' })
	asserts.assertThrows(() => parse(`'not a string\\'`))
})

Deno.test('AdditiveExpression parses', () => {
	const code = '3 + 4'

	asserts.assertEquals(parse(code), {
		$: 'AdditionExpression',
		left: { $: 'NumberLiteral', content: 3 },
		right: { $: 'NumberLiteral', content: 4 },
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
					left: { '$': 'NumberLiteral', content: 3 },
					right: { '$': 'NumberLiteral', content: 4 },
				},
				right: { '$': 'NumberLiteral', content: 5 },
			},
			right: { '$': 'NumberLiteral', content: 6 },
		},
		right: { '$': 'NumberLiteral', content: 10 },
	})
})
