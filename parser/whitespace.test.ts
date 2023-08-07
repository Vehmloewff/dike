import { asserts } from './deps.ts'
import { OmitManager } from './omits.ts'
import { Whitespace } from './whitespace.ts'

Deno.test('Whitespace captures whitespace', () => {
	const rule = Whitespace()
	const res = rule(' \t\n44', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: ' \t\n',
		comments: [],
		consumed: 3,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace\'s params.omitNewline works', () => {
	const rule = Whitespace({ omitNewline: true })
	const res = rule(' \t\n4', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: ' \t',
		comments: [],
		consumed: 2,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace consumes comments', () => {
	const rule = Whitespace()
	const res = rule(' // comment here', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: ' // comment here',
		comments: [{ text: ' comment here', span: { start: 3, end: 16 } }],
		consumed: 16,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace consumes newlines after comments when omitNewline is not set', () => {
	const rule = Whitespace()
	const res = rule(' // comment here\n   \n   abc', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: ' // comment here\n   \n   ',
		comments: [{ text: ' comment here', span: { start: 3, end: 16 } }],
		consumed: 24,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace does not consume newlines after comments when omitNewlines is set', () => {
	const rule = Whitespace({ omitNewline: true })
	const res = rule(' // comment here\n  foo', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: ' // comment here',
		comments: [{ text: ' comment here', span: { start: 3, end: 16 } }],
		consumed: 16,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace consumes comments even if that\'s all there is', () => {
	const rule = Whitespace()
	const res = rule('//com', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: '//com',
		comments: [{ text: 'com', span: { start: 2, end: 5 } }],
		consumed: 5,
		diagnostics: [],
		tokens: [],
	})
})

Deno.test('Whitespace registers comments even when there is not comment content', () => {
	const rule = Whitespace()
	const res = rule('\t//\nd', 0, new OmitManager())

	asserts.assertEquals(res, {
		node: '\t//\n',
		comments: [{ text: '', span: { start: 3, end: 3 } }],
		consumed: 4,
		diagnostics: [],
		tokens: [],
	})
})
