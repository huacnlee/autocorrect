bench:
	rustup run nightly cargo bench
release:
	cargo release --features="bin"
run:
	cargo run --features="bin"
build:
	cargo +stable build --release --features bin --target aarch64-apple-darwin
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect /usr/local/bin/autocorrect
test\:lint:
	@cargo run -q --features="bin" -- --lint tests/fixtures/*.fixed.*