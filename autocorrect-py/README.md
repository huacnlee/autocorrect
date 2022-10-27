# AutoCorrect for Python

The Python version of [AutoCorrect](https://github.com/huacnlee/autocorrect).

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)
- Java - [autocorrect-java](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-java)

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

result = autocorrect.lint_for("<h1>这是 Heading标题</h1>", "html")
print result
# => LintResult(filepath='html', lines=[LineResult { line: 1, col: 5, new: "这是 Heading 标题", old: "这是 Heading标题", severity: Error }], enable=true)

# Load config
autocorrect.load_config('{ textRules: { "你好hello": 0 } }')
autocorrect.format("Hello你好.")
# => "Hello 你好。"
autocorrect.format("你好hello.")
# => "你好hello."

# Ignorer, if /path/to/workdir contains .autocorrectignore or .gitignore
ignorer = autocorrect.Ignorer('/path/to/workdir');
ignorer.is_ignored('README.md');
```
