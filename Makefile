WORKDIR=$(shell pwd)
LAST_TAG_VERSION=$(shell git describe --abbrev=0 --tags | sed "s/^v//")
BIN_PATH=$(shell which autocorrect)

bench:
	cargo bench
run1:
	cargo run -- --lint --config /Users/jason/work/translated-content/.autocorrectrc  /Users/jason/work/translated-content/files/zh-cn/webassembly
run:
	cargo run -- --lint --config $(WORKDIR)/.autocorrectrc.template
run\:json:
	cargo run -- --lint --format json
build:
	cargo build --release --target aarch64-apple-darwin
	ls -lha target/aarch64-apple-darwin/release/autocorrect
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect $(BIN_PATH)
test:
	@cargo test --manifest-path autocorrect/Cargo.toml 
	@cargo test
test\:lint:
	@cargo run -q -- --lint tests/fixtures/*.fixed.*
test\:init:
	tests/test_init_config.sh
test\:lint-json:
	tests/test_lint_json.sh
test\:node:
	make node
	node tests/node.test.js
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
node:
	wasm-pack build --target nodejs --release --scope huacnlee -d $(WORKDIR)/node-pkg --out-name autocorrect autocorrect-wasm
	sed -ie "s/autocorrect\-wasm/autocorrect-node/" $(WORKDIR)/node-pkg/package.json
	wasm-opt -Os -o node-pkg/autocorrect_bg.wasm node-pkg/autocorrect_bg.wasm
node\:publish:
	make node
	@echo "\n\nWill release version: $(LAST_TAG_VERSION)\n\n"
	cd node-pkg && yarn publish --access public --new-version $(LAST_TAG_VERSION)
crate\:publish:
	cargo release --manifest-path autocorrect/Cargo.toml --config autocorrect/release.toml $(LAST_TAG_VERSION)
tauri\:release:
	cd autocorrect-tauri; yarn tauri build --target universal-apple-darwin 