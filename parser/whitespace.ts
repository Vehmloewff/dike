import { Match, Rule } from './base.ts'

export function Whitespace(): Rule<string> {
	return Match(/\s+/)
}

export function InlineWhitespace(): Rule<string> {
	return Match(/[ \t]+/)
}
