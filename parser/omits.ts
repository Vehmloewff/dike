import { Ast } from './deps.ts'

export type ExpressionType = Ast.Expression['$']

export class OmitManager {
	#omitsAtIndex = new Map<number, ExpressionType[]>()

	getOmitsAt(index: number): string[] {
		return this.#omitsAtIndex.get(index) || []
	}

	makeCanParseExpression(start: number, omits: ExpressionType[]): (type: ExpressionType) => boolean {
		const previousOmits = this.#omitsAtIndex.get(start) || []
		const currentOmits = omits || []
		const omit = [...previousOmits, ...currentOmits]

		if (currentOmits.length) this.#omitsAtIndex.set(start, omit)

		return (type) => !omit.includes(type)
	}
}
