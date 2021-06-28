bench:
	rustup run nightly cargo bench
release:
	cargo release --features="bin"
run:
	cargo run --features="bin"
release\:arm:
	cargo +stable build --release --features bin --target aarch64-apple-darwin
	cd target/aarch64-apple-darwin/release; tar czvf autocorrect-aarch64-apple-darwin.tar.gz autocorrect; mv autocorrect-aarch64-apple-darwin.tar.gz ~/Downloads/
	ls -lh ~/Downloads/autocorrect*.tar.gz