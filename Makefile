WORKDIR=$(shell pwd)
LAST_TAG_VERSION=$(shell git describe --abbrev=0 --tags | sed "s/^v//")

bench:
	rustup run nightly cargo bench --features bench
run:
	cargo run -- --lint
run\:json:
	cargo run -- --lint --format json
build:
	cargo build --release --target aarch64-apple-darwin
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect /usr/local/bin/autocorrect
test:
	@cargo test --manifest-path autocorrect/Cargo.toml 
	@cargo test
test\:lint:
	@cargo run -q -- --lint tests/fixtures/*.fixed.*
test\:bench:
	tests/bench.sh
test\:lint-json:
	tests/test_lint_json.sh
install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	brew install binaryen
wasm:
	wasm-pack build --release --scope huacnlee -d $(WORKDIR)/pkg autocorrect
	wasm-opt -Os -o pkg/autocorrect_bg.wasm pkg/autocorrect_bg.wasm
wasm\:publish:
	make wasm
	@echo "\n\nWill release version: $(LAST_TAG_VERSION)\n\n"
	cd pkg && yarn publish --new-version $(LAST_TAG_VERSION)
crate\:publish:
	cargo release --manifest-path autocorrect/Cargo.toml --config autocorrect/release.toml