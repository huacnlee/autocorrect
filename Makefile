bench:
	rustup run nightly cargo bench
build:
	cargo build --features="bin"
run:
	cargo run --features="bin"
