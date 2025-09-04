WORKDIR=$(shell pwd)
LAST_TAG_VERSION=$(shell git describe --abbrev=0 --tags | sed "s/^v//")
BIN_PATH=$(shell which autocorrect || echo /usr/local/bin/autocorrect)
VERSION_FILES=autocorrect/Cargo.toml autocorrect-cli/Cargo.toml autocorrect-node/Cargo.toml autocorrect-node/package.json autocorrect-rb/ext/autocorrect/Cargo.toml autocorrect-rb/autocorrect-rb.gemspec autocorrect-py/Cargo.toml autocorrect-wasm/Cargo.toml autocorrect-java/Cargo.toml autocorrect-java/javasrc/pom.xml

bench:
	cargo criterion
run1:
	cargo run -- --lint --config ~/github/translated-content/.autocorrectrc  ~/github/translated-content/files/zh-tw/learn/
run:
	cargo run -- --lint --config $(WORKDIR)/.autocorrectrc.template --no-diff-bg-color
run\:json:
	cargo run -- --lint --format json
server:
	cargo run -- server
build:
	cargo build --manifest-path autocorrect-cli/Cargo.toml --release --target aarch64-apple-darwin
	ls -lha target/aarch64-apple-darwin/release/autocorrect
	sudo ln -f target/aarch64-apple-darwin/release/autocorrect $(BIN_PATH)
test:
	@cargo test
test\:stdin:
	@echo "hello你好" | cargo run -q -- --stdin
	@echo "hello你好" | cargo run -q -- --lint --format json --stdin
	@echo "hello你好" | cargo run -q -- --lint --stdin
test\:lint:
	@cargo run -q -- --lint tests/fixtures/*.fixed.*
test\:init:
	tests/test_init_config.sh
test\:lint-json:
	tests/test_lint_json.sh
test\:node:
	cd autocorrect-node && yarn && yarn build && yarn test
test\:node\:cli:
	cd autocorrect-node && yarn && yarn build
	cd tests/node-cli-test && yarn upgrade autocorrect-node && yarn autocorrect --lint
test\:python:
	cd autocorrect-py && python3 -m pip install . &&  python3 -m pytest
test\:ruby:
	cd autocorrect-rb && bundle && rake compile && rake test
test\:java:
	cd autocorrect-java && make test
install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
	brew install binaryen
wasm:
	wasm-pack build --release --scope huacnlee -d $(WORKDIR)/pkg --out-name autocorrect autocorrect-wasm
	sed -ie "s/autocorrect\-wasm/autocorrect/" $(WORKDIR)/pkg/package.json
	wasm-opt -Os --signext-lowering -o pkg/autocorrect_bg.wasm pkg/autocorrect_bg.wasm
wasm\:publish:
	make wasm
	@echo "\n\nWill release version: $(LAST_TAG_VERSION)\n\n"
	cd pkg && yarn publish --new-version $(LAST_TAG_VERSION)
crate\:publish:
	cargo release --manifest-path autocorrect/Cargo.toml --config autocorrect/release.toml $(LAST_TAG_VERSION)
version:
	# Use ripgrep
	rg -N $(FROM) -r $(TO) --files-with-matches $(VERSION_FILES) | xargs sed -i '' 's%$(FROM)%$(TO)%g'
