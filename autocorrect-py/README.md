# AutoCorrect for Python

The Python version of [AutoCorrect](https://github.com/huacnlee/autocorrect).

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)

## Installation

```bash
$ pip install autocorrect-py
```

## Usage

```py
import autocorrect_py as autocorrect

autocorrect.format("Hello你好.")
# => "Hello 你好。"

autocorrect.format_for("let title = 'Hello你好。'", "js")
# => "let title = 'Hello 你好。'"
```
