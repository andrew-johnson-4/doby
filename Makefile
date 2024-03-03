
bench:
	cargo install --path .
	doby bench benchmarks
	mv benchmarks/fib.svg /mnt/c/Users/andre/OneDrive/Desktop/
