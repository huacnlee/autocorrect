WORKDIR=$(shell pwd)
LAST_TAG_VERSION=$(shell git describe --abbrev=0 --tags | sed "s/^v//")

bench:
	cargo bench
run:
	cargo run -- --lint --config $(WORKDIR)/.autocorrectrc.template
run\:json:
	cargo run -- --lint --format json
build:
	cargo build --release --target aarch64-apple-darwin
	ls -lha target/aarch64-apple-darwin/release/autocorrect
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect /usr/local/bin/autocorrect
test:
	@cargo test --manifest-path autocorrect/Cargo.toml 
	@cargo test
test\:lint:
	@cargo run -q -- --lint tests/fixtures/*.fixed.*
test\:init:
	tests/test_init_config.sh
test\:lint-json:
	tests/test_lint_json.sh
install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	brew install binaryen
wasm:
	wasm-pack build --release --scope huacnlee -d $(WORKDIR)/pkg --out-name autocorrect autocorrect-wasm
	sed -ie "s/autocorrect\-wasm/autocorrect/" $(WORKDIR)/pkg/package.json
	wasm-opt -Os -o pkg/autocorrect_bg.wasm pkg/autocorrect_bg.wasm
wasm\:publish:
	make wasm
	@echo "\n\nWill release version: $(LAST_TAG_VERSION)\n\n"
	cd pkg && yarn publish --new-version $(LAST_TAG_VERSION)
crate\:publish:
	cargo release --manifest-path autocorrect/Cargo.toml --config autocorrect/release.toml $(LAST_TAG_VERSION)