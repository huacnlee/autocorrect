# AutoCorrrect for Rust

[![Go](https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg)](https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI) [![Documentation](https://docs.rs/autocorrect/badge.svg)](https://docs.rs/autocorrect) [![Crates.io](https://img.shields.io/crates/v/autocorrect)](https://crates.io/crates/autocorrect)

The CLI tool based on Rust is used to automatically correct, for add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters, numerical digits and symbols).

It also supports programming source code correcting, based on Parser, can recognize the file name, and can find the strings and the comment part to correct.

基于 Rust 编写的 CLI 工具，用于自动纠正文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，同时尝试以安全的方式自动纠正标点符号等等。

除了纯文本的自动纠正以外，AutoCorrect 基于 Parser 的方式对各种类型源代码文件支持，能自动识别文件名，并准确找到字符串、注释做自动纠正。

> 此方案最早于 [2013 年](https://github.com/huacnlee/auto-correct/commit/688b7f492623baead3477b4cf0baa706777864d6) 出现于 Ruby China 的项目，并逐步完善规则细节，当前准确率较高（级少数异常情况），你可以放心用来辅助你完整自动纠正动作。

## VS Code Extension

https://marketplace.visualstudio.com/items?itemName=huacnlee.auto-correct

## Features

- Auto add spacings between CJK (Chinese, Japanese, Korean) and English words.
- Multiple file content support (HTML, YAML, Rust, Go, SQL, Ruby, Python, Objective-C, Swift, Java, Kotlin, Dart, JavaScript, CSharp ...).
- Fullwidth -> halfwidth (only for [a-zA-Z0-9], and `：` in time).
- Correct punctuations into Fullwidth near the CJK.

## Other implements for programming

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [auto-correct](https://github.com/huacnlee/auto-correct)
- Go - [go-auto-correct](https://github.com/huacnlee/go-auto-correct)

## Install

```bash
$ curl -sSL https://git.io/JcGER | bash
```

after that, you will get `/usr/local/bin/autocorrect` command.

```bash
$ autocorrect -h
AutoCorrect 0.5.1
Jason Lee <huacnlee@gmail.com
Automatically add whitespace between CJK (Chinese, Japanese, Korean) and half-width characters (alphabetical letters,
		numerical digits and symbols).

USAGE:
autocorrect [FLAGS] [file]...

FLAGS:
--fix        Automatically fix problems and rewrite file.
-h, --help       Prints help information
-V, --version    Prints version information

ARGS:
<file>...    Target filepath or dir for format
```

## Usage

```bash
$ autocorrect text.txt
你好 Hello 世界

$ autocorrect --fix text.txt
$ autocorrect --fix zh-CN.yml
$ autocorrect --fix ./
```

## Ignore for file

If you want ignore AutoCorrect for a file, you can put `autocorrect: false`.

```js
// autocorrect: false
function hello() {
  console.log("这整个文件不会被autocorrect修改");
}
```

## Usage in Rust

In your Cargo.toml

```toml
[dependencies]
autocorrect = "0.5.0"
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
	// => "包装日期为2013年3月10日"

	println!("{}", autocorrect::format("全世界已有数百家公司在生产环境中使用Rust，以达到快速、跨平台、低资源占用的目的。"));
	// => "全世界已有数百家公司在生产环境中使用 Rust，以达到快速、跨平台、低资源占用的目的。"

	println!("{}", autocorrect::format("既に、世界中の数百という企業がRustを採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"));
	// => "既に、世界中の数百という企業が Rust を採用し、高速で低リソースのクロスプラットフォームソリューションを実現しています。"

	println!("{}", autocorrect::format("전 세계 수백 개의 회사가 프로덕션 환경에서 Rust를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."));
	// => "전 세계 수백 개의 회사가 프로덕션 환경에서 Rust 를 사용하여 빠르고, 크로스 플랫폼 및 낮은 리소스 사용량을 달성했습니다."

	println!("{}", autocorrect::format("需要符号?自动转换全角字符、数字:我们将在１６：３２分出发去ＣＢＤ中心.")
			// => "需要符号？自动转换全角字符、数字：我们将在 16:32 分出发去 CBD 中心。"
			}
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

### Format

Use `cargo bench` to run benchmark tests.

```bash
test tests::bench_format_100 ... bench:      19,410 ns/iter (+/- 1,571)
test tests::bench_format_400 ... bench:      45,957 ns/iter (+/- 3,444)
test tests::bench_format_50  ... bench:      14,538 ns/iter (+/- 1,555)
```

| Total chars | Duration |
| ----------- | -------- |
| 50          | 0.014 ms |
| 100         | 0.019 ms |
| 400         | 0.045 ms |

### FormatHTML

TODO

## TODO

- [x] Lint
- [x] Lint for HTML, Markdown
- [ ] Lint for Plain text by each line
- [ ] Vim plugin
- [ ] Git Commit message format hook
- [ ] Ignore next line

## License

This project under MIT license.
