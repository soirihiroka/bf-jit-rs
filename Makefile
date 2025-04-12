hanoi:
	cargo run --release -- -i bf/hanoi.bf

hello:
	cargo run --release -- -i  bf/hello.bf

hello2:
	cargo run --release -- -i  bf/hello2.bf
	
bench:
	cargo run --release -- -i  bf/bench.bf

mandelbrot:
	cargo run --release -- -i  bf/mandelbrot.bf

test:
	cargo test -- --test-threads=1

small_bf:
	gcc -m32 -o bf.exe bf.c -Wno-error=incompatible-pointer-types -Wno-error=int-conversion -Wno-error=implicit-function-declaration -Wno-error=implicit-int