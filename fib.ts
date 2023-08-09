function getSeq(length: number) {
	function fibonacci(n: number): number {
		if (n <= 0) {
			return 0
		} else if (n === 1) {
			return 1
		} else {
			return fibonacci(n - 1) + fibonacci(n - 2)
		}
	}

	const seq: number[] = []

	for (let i = 0; i < length; i++) seq.push(fibonacci(i))

	return seq
}

console.log(getSeq(20))
