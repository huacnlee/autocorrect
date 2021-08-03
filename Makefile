bench:
	rustup run nightly cargo bench
release:
	cargo release --manifest-path src/lib/Cargo.toml --config src/lib/release.toml
run:
	cargo run -- --debug --lint ./
run\:json:
	cargo run -- --lint --format json ./
build:
	cargo +stable build --release --target aarch64-apple-darwin
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect /usr/local/bin/autocorrect
test:
	@cargo test --manifest-path src/lib/Cargo.toml 
	@cargo test
test\:lint:
	@cargo run -q -- --debug --lint tests/fixtures/*.fixed.*
test\:lint-json:
	tests/test_lint_json.sh