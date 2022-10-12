<p align="center">
  <img src="https://user-images.githubusercontent.com/5518/194691346-13856309-266b-4bf6-b505-5a8b15d0c02e.png"
    alt="AutoCorrect Icon" width="128" height="128" />
  <h1 align="center">AutoCorrect</h1>
  <p align="center"><a href="https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI"><img
        src="https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg" alt="Go"></a>
    <a href="https://github.com/huacnlee/autocorrect/releases"><img
        src="https://img.shields.io/github/v/release/huacnlee/autocorrect?label=Version"
        alt="GitHub release (latest by date)"></a>
    <a href="https://hub.docker.com/r/huacnlee/autocorrect"><img
        src="https://img.shields.io/docker/v/huacnlee/autocorrect?label=Docker%20Image"
        alt="Docker Image Version (latest server)"></a>
    <a href="https://crates.io/crates/autocorrect"><img src="https://img.shields.io/crates/v/autocorrect"
        alt="Crates.io"></a>
    <a href="https://www.npmjs.com/package/@huacnlee/autocorrect"><img
        src="https://badge.fury.io/js/%40huacnlee%2Fautocorrect.svg" alt="NPM"></a>
    <a href="https://pypi.org/project/autocorrect-py/"><img src="https://badge.fury.io/py/autocorrect-py.svg" alt="PyPI version"></a>
  </p>
</p>

> 🎯 AutoCorrect 的愿景是提供一套标准化的文案较正方案。以便于在各类场景（例如：撰写书籍、文档、内容发布、项目源代码...）里面应用，让使用者轻松实现标准化、专业化的文案输出 / 校正。

AutoCorrect is a linter and formatter to help you to improve copywriting, correct spaces, words, punctuations between CJK (Chinese, Japanese, Korean).

Like Eslint, Rubocop, Gofmt ..., AutoCorrect allows us to check source code, and output as colorized diff with corrected suggest. You can integrate to CI (GitLab CI, GitHub Action, Travis CI....) for use to checking the contents in source code. Recognize the file name, and find out the strings and the comment part.

基于 Rust 编写的 CLI 工具，用于「自动纠正」或「检查并建议」文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，纠正单词，同时尝试以安全的方式自动纠正标点符号等等。

类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI 环境，它提供 Lint 功能，能便捷的检测出项目中有问题的文案，起到统一规范的作用。

支持各种类型源代码文件，能自动识别文件名，并准确找到字符串、注释做自动纠正。

> 此方案最早于 [2013 年](https://github.com/huacnlee/auto-correct/commit/688b7f492623baead3477b4cf0baa706777864d6) 出现于 Ruby China 的项目，并逐步完善规则细节，当前准确率较高（极少数异常情况），你可以放心用来辅助你完成自动纠正动作。

## Features

- Auto add spacing between CJK (Chinese, Japanese, Korean) and English words.
- Support more than 27 programming languages (Markdown, JSON, YAML, JavaScript, HTML ...), use AST parser to only check for strings, comments.
- Correct punctuations into Fullwidth near the CJK.
- Correct punctuations into Halfwidth, and remove duplication spaces in english contents.
- Spellcheck (experimental) and correct words by your own dictionary.
- Lint checking and output diff or JSON result, so you can integrate to everywhere (GitLab CI, GitHub Action, VS Code, Vim, Emacs...)
- Allows using `.gitignore` or `.autocorrectignore` to ignore files that you want to ignore.
- [Desktop app](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-tauri) for macOS, (Windows, Linux WIP).

<img width="920" alt="autocorrect lint output" src="https://user-images.githubusercontent.com/5518/192738752-89a9e4f5-75cb-40af-b84d-04889d22e834.png">

## Installation

```bash
$ brew install autocorrect
```

Or you can just install via this:

```bash
$ curl -sSL https://git.io/JcGER | bash
```

After that, you will get `autocorrect` command.

```bash
AutoCorrect 1.11.0
Jason Lee <huacnlee@gmail.com
A linter and formatter to help you to improve copywriting, correct spaces, words, punctuations between CJK (Chinese, Japanese, Korean).

USAGE:
    autocorrect [FLAGS] [OPTIONS] [file]... [SUBCOMMAND]

FLAGS:
        --debug      Print debug message.
        --type       Directly use set file type.
        --fix        Automatically fix problems and rewrite file.
    -h, --help       Prints help information
        --lint       Lint and output problems.
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>       Special config file. [default: .autocorrectrc]
        --format <formatter>    Choose an output formatter. [default: diff]  [possible values: json, diff]
        --threads <threads>     Number of threads, 0 - use number of CPU. [default: 0]

ARGS:
    <file>...    Target filepath or dir for format. [default: .]

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    init       Init AutoCorrect config file.
    update     Update AutoCorrect to latest version.
```

## Upgrade

> After: 1.9.0

AutoCorrect allows you to upgrade itself by `autocorrect update` command.

```bash
$ autocorrect update
```

> NOTE: This command need you input your password, because it will install bin into `/usr/local/bin` directory.

## Usage

- [Using CLI](#using-cli)
- [Configuration](#configuration)
- [VS Code Extension](#vs-code-extension)
- [Use for JavaScript](#use-for-javascript)
- [Use for Node.js](#use-for-nodejs)
- [Use for Rust](#use-for-rust)
- [GitHub Action](#github-action)
- [GitLab CI](#gitlab-ci)

### Using CLI

```bash
$ autocorrect text.txt
你好 Hello 世界

$ echo "hello世界" | autocorrect --stdin
hello 世界

$ autocorrect --fix text.txt
$ autocorrect --fix zh-CN.yml
$ autocorrect --fix
```

#### Lint

```bash
$ autocorrect --lint --format json text.txt

$ autocorrect --lint text.txt
```

```diff
Error: 1, Warning: 0

text.txt:1:3
-你好Hello世界
+你好 Hello 世界
```

You also can lint multiple files:

```bash
$ autocorrect --lint
```

### Configuration

```bash
$ autocorrect init
Fetching https://github.com/huacnlee/autocorrect/raw/main/.autocorrectrc.template
AutoCorrect init config: .autocorrectrc
```

> NOTE: If you download fail, try to use `autocorrect init --local` command again.

Now the `.autocorrectrc` file has created.

> .autocorrectrc is allows use YAML, JSON format.

Config file example:

```yml
# Config rules
rules:
  # Auto add spacing between CJK (Chinese, Japanese, Korean) and English words.
  # 0 - off, 1 - error, 2 - warning
  space-word: 1
  # Add space between some punctuations.
  space-punctuation: 1
  # Convert to fullwidth.
  fullwidth: 1
  # Convert to halfwidth.
  halfwidth: 1
  # To remove space near the fullwidth.
  no-space-fullwidth: 1
  # Spellcheck
  spellcheck: 2
spellcheck:
  # Correct Words (Case insensitive) for by Spellcheck
  words:
    - GitHub
    - App Store
    # This means "appstore" into "App Store"
    - AppStore = App Store
    - Git
    - Node.js
    - nodejs = Node.js
    - VIM
    - DNS
    - HTTP
    - SSL
```

#### Ignore option

**Use `.autocorrectignore` to ignore files**

Sometimes, you may want to ignore some special files that not wants to check.

By default, the file matched `.gitignore` rule will be ignored.

You can also use `.autocorrectignore` to ignore other files, format like `.gitignore`.

#### Disable with inline comment

If you just want to disable some special lines in file, you can write a comment `autocorrect-disable`,
when AutoCorrect matched comment include that, it will disable temporary.

And then, you can use `autocorrect-enable` to reopen it again.

For example, in JavaScript:

```js
function hello() {
  // autocorrect-disable
  console.log('现在这行开始autocorrect会暂时禁用');
  console.log('这行也是disable的状态');
  // autocorrect-enable
  let a = '现在起autocorrect回到了启用的状态';
}
```

The output will:

```js
function hello() {
  // autocorrect-disable
  console.log('现在这行开始autocorrect会暂时禁用');
  console.log('这行也是disable的状态');
  // autocorrect-enable
  let a = '现在起 autocorrect 回到了启用的状态';
}
```

#### Disable some rules

> Since: 2.0

You can use `autocorrect-disable <rule>` in comment to disable some rules.

> Rule names please see: [Configuration](#configuration)

```js
function hello() {
  // autocorrect-disable space-word
  console.log('现在这行开始autocorrect会暂时禁用.');
  // autocorrect-disable fullwidth
  console.log('这行也是disable的状态.');
  // autocorrect-enable
  let a = '现在起autocorrect回到了启用的状态.';
}
```

Will get:

```js
function hello() {
  // autocorrect-disable space-word
  console.log('现在这行开始autocorrect会暂时禁用。');
  // autocorrect-disable fullwidth, space-word
  console.log('这行也是disable的状态.');
  // autocorrect-enable
  let a = '现在起 autocorrect 回到了启用的状态。';
}
```

### VS Code Extension

[![Install Extension](https://img.shields.io/badge/Install%20Extension-VS%20Code-brightgreen)](https://marketplace.visualstudio.com/items?itemName=huacnlee.auto-correct)

https://marketplace.visualstudio.com/items?itemName=huacnlee.auto-correct

内置 Visual Studio Code 插件，安装后会将 AutoCorrect 和 Visual Studio Code 完整集成，可以达到「保存自动格式化」或「纠正提示」。

如下图：

<img width="901" alt="huacnlee.autocorrect" src="https://user-images.githubusercontent.com/5518/191890126-4e0c99dc-91ce-4262-a774-3813a636eea1.png">

### GitHub Action

https://github.com/marketplace/actions/huacnlee-autocorrect

Add to your `.github/workflows/ci.yml`

```yml
steps:
  - name: Check source code
    uses: actions/checkout@main

  - name: AutoCorrect
    uses: huacnlee/autocorrect-action@main
```

### GitLab CI

Add to your `.gitlab-ci.yml`, to use [huacnlee/autocorrect](https://hub.docker.com/r/huacnlee/autocorrect) Docker image to check.

```yml
autocorrect:
  stage: build
  image: huacnlee/autocorrect:latest
  script:
    - autocorrect --lint
  # Enable allow_failure if you wants.
  # allow_failure: true
```

### Use for JavaScript

AutoCorrect also provide a JavaScript module via WebAssembly, you can use it in your JavaScript project.

```bash
yarn add @huacnlee/autocorrect
```

And then:

```js
const autocorrect = import('@huacnlee/autocorrect');

const raw = `<p>你好Hello世界</p>`;
autocorrect.then((autocorrect) => {
  const out = autocorrect.formatHTML(raw);
  // "<p>你好 Hello 世界</p>"
});
```

### Use for Node.js

When you want use AutoCorrect in Node.js, you must install `autocorrect-node`.

```bash
yarn add autocorrect-node
```

And then:

```js
const autocorrect = require('autocorrect-node');

autocorrect.format('你好Hello世界');
// "你好 Hello 世界"
```

### Use for Rust

In your `Cargo.toml`

```toml
[dependencies]
autocorrect = "1.0.0"
```

Use `autocorrect::format` to format plain text.

````rust
extern crate autocorrect;

fn main() {
	println!("{}", autocorrect::format("长桥LongBridge App下载"));
	// => "长桥 LongBridge App 下载"

	println!("{}", autocorrect::format("Ruby 2.7版本第1次发布"));
	// => "Ruby 2.7 版本第 1 次发布"

	println!("{}", autocorrect::format("于3月10日开始"));
	// => "于 3 月 10 日开始"

	println!("{}", autocorrect::format("包装日期为2013年3月10日"));
	// => "包装日期为 2013 年 3 月 10 日"

	println!("{}", autocorrect::format("全世界已有数百家公司在生产环境中使用Rust，以达到快速、跨平台、低资源占用的目的。"));
	// => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"

	println!("{}", autocorrect::format("既に、世界中の数百という企業がRustを採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
	// => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"

	println!("{}", autocorrect::format("전 세계 수백 개의 회사가 프로덕션 환경에서 Rust를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."));
	// => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."

	println!("{}", autocorrect::format("需要符号?自动转换全角字符、数字:我们将在１６：３２分出发去ＣＢＤ中心.")
	// => "需要符号？自动转换全角字符、数字：我们将在 16:32 分出发去 CBD 中心。"
```

Use `autocorrect::format_for` to format HTML content.

```rust
extern crate autocorrect;

fn main() {
	let html = r#"
	<article>
	<h1>这是Heading标题</h1>
	<div class="content">
	<p>你好Rust世界<strong>Bold文本</strong></p>
	<p>这是第二行p标签</p>
	</div>
	</article>
	"#;

	println!("{}", autocorrect::format_for(html, "html"));
	// <article>
	// <h1>这是 Heading 标题</h1>
	// <div class="content">
	//     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
	//     <p>这是第二行 p 标签</p>
	// </div>
	// </article>
}
````

## Other implements for programming

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)

## Benchmark

> MacBook Pro (13-inch, M1, 2020)

Use `make bench` to run benchmark tests.

See [autocorrect/src/benches/example.rs](https://github.com/huacnlee/autocorrect/blob/main/autocorrect/src/benches/example.rs) for details.

```bash
test bench_format_050                 ... bench:       9,390 ns/iter (+/- 74)
test bench_format_100                 ... bench:      17,299 ns/iter (+/- 461)
test bench_format_400                 ... bench:      57,316 ns/iter (+/- 2,845)
test bench_format_html                ... bench:     174,572 ns/iter (+/- 7,530)
test bench_format_javascript          ... bench:      80,637 ns/iter (+/- 3,315)
test bench_format_json                ... bench:      34,212 ns/iter (+/- 189)
test bench_format_json_with_2k_lines  ... bench:   5,339,445 ns/iter (+/- 110,482)
test bench_halfwidth_full_english_100 ... bench:      12,584 ns/iter (+/- 2,244)
test bench_markdown                   ... bench:     790,159 ns/iter (+/- 88,039)
test bench_spellcheck_100             ... bench:      59,318 ns/iter (+/- 17,063)
test bench_spellcheck_400             ... bench:     200,657 ns/iter (+/- 21,931)
test bench_spellcheck_50              ... bench:      35,396 ns/iter (+/- 755)
```

| Type       | Total chars | Duration |
| ---------- | ----------- | -------- |
| format     | 50          | 0.010 ms |
| format     | 100         | 0.017 ms |
| format     | 400         | 0.057 ms |
| format     | HTML        | 0.174 ms |
| format     | JavaScript  | 0.086 ms |
| format     | JSON        | 0.034 ms |
| format     | Large JSON  | 3.829 ms |
| halfwidth  | 100         | 0.012 ms |
| format     | Markdown    | 0.749 ms |
| spellcheck | 50          | 0.037 ms |
| spellcheck | 100         | 0.057 ms |
| spellcheck | 400         | 0.195 ms |

### Real world benchmark

With [MDN Translated Content](https://github.com/mdn/translated-content) project, it's has about 30K files.

```bash
~/work/translated-content $ autocorrect --fix
AutoCorrect spend time: 8402.538ms
```

## License

This project under MIT license.
