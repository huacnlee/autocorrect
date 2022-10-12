# AutoCorrect for Ruby

The Native Ruby version of [AutoCorrect](https://github.com/huacnlee/autocorrect).

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)

## Installation

```bash
$ bundle add autocorrect-rb
```

## Usage

```rb
require('autocorrect-rb');

out = AutoCorrect.format('Hello你好.')
puts out
# Hello 你好。

out = AutoCorrect.format_for("title = 'Hello你好。'", 'rb')
puts out
# title = 'Hello 你好。'

result = AutoCorrect.lint_lor("title = 'Hello你好。'", 'rb')
puts result
# {
#   filepath: 'rb',
#   lines: [
#     { l: 1, c: 13, new: "'Hello 你好。'", old: "'Hello你好。'", severity: 1 }
#   ],
#   error: ''
# }
```
