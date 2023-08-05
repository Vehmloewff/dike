import { Diagnostic, Rule, Token } from './base.ts'
import { Ast } from './deps.ts'

export function seq<A>(rules: [Rule<A>]): Rule<[A]>
export function seq<A, B>(rules: [Rule<A>, Rule<B>]): Rule<[A, B]>
export function seq<A, B, C>(rules: [Rule<A>, Rule<B>, Rule<C>]): Rule<[A, B, C]>
export function seq<A, B, C, D>(rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>]): Rule<[A, B, C, D]>
export function seq<A, B, C, D, E>(rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>]): Rule<[A, B, C, D, E]>
export function seq<A, B, C, D, E, F>(rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>, Rule<F>]): Rule<[A, B, C, D, E, F]>
export function seq<A, B, C, D, E, F, G>(
	rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>, Rule<F>, Rule<G>],
): Rule<[A, B, C, D, E, F, G]>
export function seq<A, B, C, D, E, F, G, H>(
	rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>, Rule<F>, Rule<G>, Rule<H>],
): Rule<[A, B, C, D, E, F, G, H]>
export function seq<A, B, C, D, E, F, G, H, I>(
	rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>, Rule<F>, Rule<G>, Rule<H>, Rule<I>],
): Rule<[A, B, C, D, E, F, G, H, I]>
export function seq<A, B, C, D, E, F, G, H, I, J>(
	rules: [Rule<A>, Rule<B>, Rule<C>, Rule<D>, Rule<E>, Rule<F>, Rule<G>, Rule<H>, Rule<I>, Rule<J>],
): Rule<[A, B, C, D, E, F, G, H, I, J]>

export function seq(rules: Rule<unknown>[]): Rule<unknown[]> {
	return (text: string, start: number) => {
		const diagnostics: Diagnostic[] = []
		const tokens: Token[] = []
		const nodes: unknown[] = []

		let consumed = 0

		for (const rule of rules) {
			const res = rule(text, start)
			if (!res) return null

			consumed += res.consumed
			tokens.push(...res.tokens)
			diagnostics.push(...res.diagnostics)
			text = text.slice(res.consumed)
			start += consumed
		}

		return { consumed, diagnostics, tokens, node: nodes }
	}
}

export function repeat<T>(rule: Rule<T>): Rule<T[]> {
	return (text: string, start: number) => {
		const diagnostics: Diagnostic[] = []
		const tokens: Token[] = []
		const nodes: T[] = []

		let consumed = 0

		while (text.length) {
			const res = rule(text, start)
			if (!res) break

			consumed += res.consumed
			tokens.push(...res.tokens)
			diagnostics.push(...res.diagnostics)
			text = text.slice(res.consumed)
			start += consumed
		}

		return { consumed, diagnostics, node: nodes, tokens }
	}
}

export function any<T>(rules: Rule<T>[]): Rule<T> {
	return (text, start) => {
		for (const rule of rules) {
			const res = rule(text, start)
			if (!res) return null

			return res
		}

		return null
	}
}

export type Formatter<T, NT> = (params: FormatterParams<T>) => NT

export interface FormatterParams<T> {
	node: T
	addDiagnostic(diagnostic: Diagnostic): void
	getSpan(): Ast.Span
}

export function format<T, NT>(rule: Rule<T>, formatter: Formatter<T, NT>): Rule<NT> {
	return (text, start) => {
		const res = rule(text, start)
		if (!res) return null

		const diagnostics: Diagnostic[] = []

		const newNode = formatter({
			node: res.node,
			addDiagnostic: (diagnostic) => diagnostics.push(diagnostic),
			getSpan: () => ({ start, end: start + res.consumed }),
		})

		return { node: newNode, consumed: res.consumed, diagnostics, tokens: res.tokens }
	}
}
