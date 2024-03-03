
bench:
	cargo install --path .
	doby bench benchmarks
	mv benchmarks/fibonacci.svg /mnt/c/Users/andre/OneDrive/Desktop/
