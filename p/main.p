fn item(n) {
	let denominator = 2*n+1;
	let sign = 1;
	for (let i = 0; i < n; i = i+1) {
		sign = sign * -1;
	}
	return sign / denominator;
}

fn leibniz(k) {
	let quarter = 0;
	for (let i = 0; i < k; i = i+1) {
		quarter = quarter + item(i);
	}

	return 4*quarter;
}
print leibniz(200);