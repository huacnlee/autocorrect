#!/bin/bash
# brew install jq
diff_lint_json() {
  cargo run -q -- \
    --lint \
    --format json tests/fixtures/$1.raw.$2 \
    | jq | git --no-pager diff --ignore-all-space --no-index -- tests/fixtures/$1.expect.json -
}

diff_lint_json "go" "go"
diff_lint_json "javascript" "js"
diff_lint_json "vue" "vue"
diff_lint_json "html" "html"

# this ignore work with direct file
cargo run -q -- --lint tests/fixtures/this-file-will-ignore.rs
