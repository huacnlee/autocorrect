# AutoCorrrect

[![Go](https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg)](https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/huacnlee/autocorrect?label=Version)](https://github.com/huacnlee/autocorrect/releases)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/huacnlee/autocorrect?label=Docker%20Image)](https://hub.docker.com/r/huacnlee/autocorrect)
[![Crates.io](https://img.shields.io/crates/v/autocorrect)](https://crates.io/crates/autocorrect)
[![NPM](https://badge.fury.io/js/%40huacnlee%2Fautocorrect.svg)](https://badge.fury.io/js/%40huacnlee%2Fautocorrect)
[![Documentation](https://docs.rs/autocorrect/badge.svg)](https://docs.rs/autocorrect)

A linter and formatter for help you improve copywriting, to correct spaces, punctuations between CJK (Chinese, Japanese, Korean).

Like Eslint, Rubocop, Gofmt ..., AutoCorrect allow us to checking soure code, and output as colorized diff with corrected suggest. You can intergrating to CI (GitLab CI, GitHub Action, Travis CI....) for use to checking the contents in source code. Recognize the file name, and find out the strings and the comment part.

基于 Rust 编写的 CLI 工具，用于「自动纠正」或「检查并建议」文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，同时尝试以安全的方式自动纠正标点符号等等。

类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI 环境，它提供 Lint 功能能便捷的检测出项目中有问题的文案，起到统一规范的作用。

支持各种类型源代码文件支持，能自动识别文件名，并准确找到字符串、注释做自动纠正。

> 此方案最早于 [2013 年](https://github.com/huacnlee/auto-correct/commit/688b7f492623baead3477b4cf0baa706777864d6) 出现于 Ruby China 的项目，并逐步完善规则细节，当前准确率较高（级少数异常情况），你可以放心用来辅助你完整自动纠正动作。

## VS Code Extension

[![Install Extension](https://img.shields.io/badge/Install%20Extension-VS%20Code-brightgreen)](vscode:extension/huacnlee.auto-correct)

https://marketplace.visualstudio.com/items?itemName=huacnlee.auto-correct

内置 VS Code 插件，安装后会将 AutoCorrect 和 VS Code 完整集成，可以达到「保存自动格式化」或「纠正提示」，如下图。

<img width="901" alt="huacnlee.autocorrect" src="https://user-images.githubusercontent.com/5518/126027685-cee6f91d-1a10-4fcc-b5f4-1a99ac4cd5ae.png">

## Features

- Auto add spacings between CJK (Chinese, Japanese, Korean) and English words.
- Multiple file content support (HTML, YAML, Rust, Go, SQL, Ruby, Python, Objective-C, Swift, Java, Kotlin, Dart, JavaScript, CSharp ...).
- Fullwidth -> halfwidth (only for [a-zA-Z0-9], and `：` in time).
- Correct punctuations into Fullwidth near the CJK.
- Lint checking and output diff or JSON result, so you can integrating to everwhere (GitLab CI, GitHub Action, VS Code, Vim, Emacs...)
- Allows to use `.gitignore` or `.autocorrectignore` to ignore files that you wants ignore.

<img width="920" alt="autocorrect lint output" src="https://user-images.githubusercontent.com/5518/126027750-fce415a2-3141-4489-8863-ad3aae82d6dd.png">

## Install

```bash
$ curl -sSL https://git.io/JcGER | bash
```

after that, you will get `/usr/local/bin/autocorrect` command.

```bash
AutoCorrect 1.0.0
Jason Lee <huacnlee@gmail.com
Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters,
numerical digits and symbols).

USAGE:
    autocorrect [FLAGS] [OPTIONS] [file]...

FLAGS:
        --fix        Automatically fix problems and rewrite file.
    -h, --help       Prints help information
        --lint       Lint and output problems.
    -V, --version    Prints version information

OPTIONS:
        --type <filetype>       Directly use set file type [default: ]
        --format <formatter>    Choose an output formatter. [default: diff]  [possible values: json, diff]

ARGS:
    <file>...    Target filepath or dir for format
```

## Usage

- [Using CLI](#using-cli)
- [Use for JavaScript](#use-for-javascript)
- [Use for Rust](#use-for-rust)
- [Ignore files](#ignore-files)
- [GitHub Action](#github-action)
- [GitLab CI](#gitlab-ci)

### Using CLI

```bash
$ autocorrect text.txt
你好 Hello 世界

$ autocorrect --fix text.txt
$ autocorrect --fix zh-CN.yml
$ autocorrect --fix ./
```

#### Lint

```bash
$ autocorrect --lint --format json text.txt

$ autocorrect --lint text.txt
```

```diff
  --> text.txt:1:3
- 你好Hello世界
+ 你好 Hello 世界
```

You also can lint multiple files:

```bash
$ autocorrect --lint .
```

### Ignore option

**Use `.autocorrectignore` to ignore files**

Some times, you may wants ignore some special files that not wants to check.

By default, the file matched `.gitignore` rule will be ignored.

You can also use `.autocorrectignore` to ignore other files, format like `.gitignore`.

**Disable with inline comment**

If you just want to disable some special lines in file, you can write a comment `autocorrect: false` or `autocorrect-disable`,
when AutoCorrect matched comment include that, it will disable temporary.

And then, you can use `autocorrect: true` or `autocorrect-enable` to reopen it agian.

For example in JavaScript:

```js
function hello() {
  console.log('现在这行开始autocorrect会暂时禁用');
  console.log('这行也是disable的状态');
  // autocorrect: true
  let a = '现在起autocorrect回到了启用的状态';
}
```

The out put will:

```js
function hello() {
  console.log('现在这行开始autocorrect会暂时禁用');
  console.log('这行也是disable的状态');
  // autocorrect: true
  let a = '现在起 autocorrect 回到了启用的状态';
}
```

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
    - autocorrect --lint .
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

### Use for Rust

Other implements for programming:

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [auto-correct](https://github.com/huacnlee/auto-correct)
- Go - [go-auto-correct](https://github.com/huacnlee/go-auto-correct)

In your Cargo.toml

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

Use `autocorrect::format_html` to format html content.

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

	println!("{}", autocorrect::format_html(html));
	// <article>
	// <h1>这是 Heading 标题</h1>
	// <div class="content">
	//     <p>你好 Rust 世界<strong>Bold 文本</strong></p>
	//     <p>这是第二行 p 标签</p>
	// </div>
	// </article>
}
````

## Benchmark

Use `make bench` to run benchmark tests.

See [autocorrect/src/bench.rs](https://github.com/huacnlee/autocorrect/blob/main/autocorrect/src/bench.rs) for details.

```bash
test bench::tests::bench_format_100                ... bench:      16,717 ns/iter (+/- 7,790)
test bench::tests::bench_format_400                ... bench:      59,149 ns/iter (+/- 9,923)
test bench::tests::bench_format_50                 ... bench:       9,508 ns/iter (+/- 6,278)
test bench::tests::bench_format_html               ... bench:     163,533 ns/iter (+/- 129,441)
test bench::tests::bench_format_javascript         ... bench:      85,127 ns/iter (+/- 2,663)
test bench::tests::bench_format_json               ... bench:      29,588 ns/iter (+/- 106)
test bench::tests::bench_format_json_with_2k_lines ... bench:   3,975,075 ns/iter (+/- 753,329)
```

| Total chars | Duration |
| ----------- | -------- |
| 50          | 0.014 ms |
| 100         | 0.019 ms |
| 400         | 0.045 ms |

## TODO

- [x] Lint
- [x] Lint for HTML, Markdown
- [x] Lint for Plain text by each line
- [x] Disable next line

## License

This project under MIT license.
