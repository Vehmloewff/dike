import { Ast } from './deps.ts'

export type ExpressionType = Ast.Expression['$']

export class OmitManager {
	#omitsAtIndex = new Map<number, ExpressionType[]>()

	makeCanParseExpression(start: number, omits: ExpressionType[]): (type: ExpressionType) => boolean {
		const previousOmits = this.#omitsAtIndex.get(start) || []
		const currentOmits = omits || []
		const omit = [...previousOmits, ...currentOmits]

		if (currentOmits.length) this.#omitsAtIndex.set(start, omit)

		return (type) => !omit.includes(type)
	}
}
