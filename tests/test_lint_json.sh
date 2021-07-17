#!/bin/bash
diff_lint_json() {
  cargo run -q --features="bin" -- \
    --lint \
    --format json tests/fixtures/$1.raw.$2 \
    | git --no-pager diff --no-index -- tests/fixtures/$1.expect.json -
}

diff_lint_json "go" "go"
diff_lint_json "javascript" "js"