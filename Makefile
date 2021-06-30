bench:
	rustup run nightly cargo bench
release:
	cargo release --features="bin"
run:
	cargo run --features="bin" -- $@
release\:arm:
	cargo +stable build --release --features bin --target aarch64-apple-darwin
	cd target/aarch64-apple-darwin/release; tar czvf autocorrect-darwin-arm64.tar.gz autocorrect; mv autocorrect-darwin-arm64.tar.gz ~/Downloads/
	ls -lh ~/Downloads/autocorrect*.tar.gz
release\:local:
	cargo +stable build --release --features bin --target aarch64-apple-darwin
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect /usr/local/bin/autocorrect