# AutoCorrect for Node.js

The Native Node.js version of [AutoCorrect](https://github.com/huacnlee/autocorrect) built on [NAPI.RS](https://napi.rs).

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)
- Java - [autocorrect-java](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-java)

## Installation

```bash
$ yarn add autocorrect-node
```

## Use CLI

```bash
$ yarn autocorrect -h
```

Lint you files

```bash
$ yarn autocorrect --lint .
```

## Work with Lint-Staged

If you are use `lint-staged`, you can add this config to your `package.json` for run AutoCorrect lint before commit for changed files.

```json
{
  "lint-staged": {
    "*": [
      "autocorrect --lint"
    ]
  }
}
```

## Usage

```js
const autocorrect = require('autocorrect-node');

const out = autocorrect.format('Hello你好.');
console.log(out);
// Hello 你好。

out = autocorrect.formatFor("let title = 'Hello你好。'", 'js');
// let title = 'Hello 你好。'

const result = autocorrect.lintFor("let title = 'Hello你好。'", 'js');
console.log(result);
// {
//   filepath: 'js',
//   lines: [
//     { l: 1, c: 13, new: "'Hello 你好。'", old: "'Hello你好。'", severity: 1 }
//   ],
//   error: ''
// }

// Load Config
const configStr = JSON.stringify({ textRules: { 你好hello: 0 } });
autocorrect.loadConfig(configStr);
out = autocorrect.format('Hello你好.');
console.log(out);
// Hello 你好。
out = autocorrect.format('你好hello');
console.log(out);
// 你好hello

// Ignorer, if /path/to/workdir contains .autocorrectignore or .gitignore
const ignorer = new autocorrect.Ignorer('/path/to/workdir');
ignorer.isIgnored('README.md');
```
