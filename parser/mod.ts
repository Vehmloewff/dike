import { Expression } from './expression.ts'

export function parse(text: string): void {
	console.log(Expression()(text, 0))
}
