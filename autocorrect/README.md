# AutoCorrrect for Rust

[![Go](https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg)](https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/huacnlee/autocorrect?label=Version)](https://github.com/huacnlee/autocorrect/releases)
[![Docker Image Version (latest semver)](https://img.shields.io/docker/v/huacnlee/autocorrect?label=Docker%20Image)](https://hub.docker.com/r/huacnlee/autocorrect)
[![Crates.io](https://img.shields.io/crates/v/autocorrect)](https://crates.io/crates/autocorrect)
[![NPM](https://badge.fury.io/js/%40huacnlee%2Fautocorrect.svg)](https://badge.fury.io/js/%40huacnlee%2Fautocorrect)
[![Documentation](https://docs.rs/autocorrect/badge.svg)](https://docs.rs/autocorrect)

A linter and formatter for help you improve copywriting, to correct spaces, punctuations between CJK (Chinese, Japanese, Korean).

Like Eslint, Rubocop, Gofmt ..., AutoCorrect allows us to checking soure code, and output as colorized diff with corrected suggest. You can intergrating to CI (GitLab CI, GitHub Action, Travis CI....) for use to checking the contents in source code. Recognize the file name, and find out the strings and the comment part.

基于 Rust 编写的 CLI 工具，用于「自动纠正」或「检查并建议」文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，同时尝试以安全的方式自动纠正标点符号等等。

类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI 环境，它提供 Lint 功能能便捷的检测出项目中有问题的文案，起到统一规范的作用。

支持各种类型源代码文件支持，能自动识别文件名，并准确找到字符串、注释做自动纠正。

> 此方案最早于 [2013 年](https://github.com/huacnlee/auto-correct/commit/688b7f492623baead3477b4cf0baa706777864d6) 出现于 Ruby China 的项目，并逐步完善规则细节，当前准确率较高（级少数异常情况），你可以放心用来辅助你完整自动纠正动作。

## Features

- Auto add spacings between CJK (Chinese, Japanese, Korean) and English words.
- Multiple file content support (HTML, YAML, Rust, Go, SQL, Ruby, Python, Objective-C, Swift, Java, Kotlin, Dart, JavaScript, CSharp ...).
- Fullwidth -> halfwidth (only for [a-zA-Z0-9], and `：` in time).
- Correct punctuations into Fullwidth near the CJK.
- Lint checking and output diff or JSON result, so you can integrating to everwhere (GitLab CI, GitHub Action, VS Code, Vim, Emacs...)
- Allows to use `.gitignore` or `.autocorrectignore` to ignore files that you wants ignore.

## Usage

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

Use `autocorrect::format_for` to format html content.

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

## License

This project under MIT license.
