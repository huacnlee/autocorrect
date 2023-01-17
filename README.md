<p align="center">
  <img src="https://user-images.githubusercontent.com/5518/194691346-13856309-266b-4bf6-b505-5a8b15d0c02e.png"
    alt="AutoCorrect Icon" width="128" height="128" />
  <h1 align="center">AutoCorrect</h1>
  <p align="center">
    <a href="https://github.com/huacnlee/autocorrect/actions?query=workflow%3ACI"><img src="https://github.com/huacnlee/autocorrect/workflows/CI/badge.svg" alt="Go"></a>
    <a href="https://github.com/huacnlee/autocorrect/releases"><img src="https://img.shields.io/github/v/release/huacnlee/autocorrect?label=CLI&color=blue" alt="GitHub release (latest by date)"></a>
    <a href="https://hub.docker.com/r/huacnlee/autocorrect"><img src="https://img.shields.io/docker/v/huacnlee/autocorrect?label=Docker&color=blue" alt="Docker Image Version (latest server)"></a>
    <a href="https://crates.io/crates/autocorrect"><img src="https://img.shields.io/crates/v/autocorrect?color=1t&label=Crate" alt="Crates.io"></a>
    <a href="https://www.npmjs.com/package/@huacnlee/autocorrect"><img src="https://img.shields.io/npm/v/@huacnlee/autocorrect?color=1t&label=NPM" alt="NPM"></a>
    <a href="https://pypi.org/project/autocorrect-py/"><img src="https://img.shields.io/pypi/v/autocorrect-py?color=1&label=PyPI" alt="PyPI version"></a>
    <a href="https://rubygems.org/gems/autocorrect-rb"><img src="https://img.shields.io/gem/v/autocorrect-rb?color=1&label=Gem" alt="Gem Version"></a>
    <a href="https://repo1.maven.org/maven2/io/github/huacnlee/autocorrect-java/"><img alt="Maven Central" src="https://img.shields.io/maven-central/v/io.github.huacnlee/autocorrect-java?color=1&label=Maven"></a>
  </p>
</p>

> 🎯 AutoCorrect 的愿景是提供一套标准化的文案较正方案。以便于在各类场景（例如：撰写书籍、文档、内容发布、项目源代码...）里面应用，让使用者轻松实现标准化、专业化的文案输出 / 校正。

AutoCorrect is a linter and formatter to help you to improve copywriting, correct spaces, words, punctuations between CJK (Chinese, Japanese, Korean).

Like Eslint, Rubocop, Gofmt ..., AutoCorrect allows us to check source code, and output as colorized diff with corrected suggest. You can integrate to CI (GitLab CI, GitHub Action, Travis CI....) for use to checking the contents in source code. Recognize the file name, and find out the strings and the comment part.

AutoCorrect 是一个基于 Rust 编写的工具，用于「自动纠正」或「检查并建议」文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，纠正单词，同时尝试以安全的方式自动纠正标点符号等等。

类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI 环境，它提供 Lint 功能，能便捷的检测出项目中有问题的文案，起到统一规范的作用。

支持各种类型源代码文件，能自动识别文件名，并准确找到字符串、注释做自动纠正。

> 此方案最早于 [2013 年](https://github.com/huacnlee/auto-correct/commit/47d7b7836f3e5f97dd31b5dc477eb0dbf8176e6b) 出现于 Ruby China 的项目，并逐步完善规则细节，当前准确率较高（极少数异常情况），你可以放心用来辅助你完成自动纠正动作。

<img width="920" alt="autocorrect lint output" src="https://user-images.githubusercontent.com/5518/192738752-89a9e4f5-75cb-40af-b84d-04889d22e834.png">

## Features

- Auto add spacing between CJK (Chinese, Japanese, Korean) and English words.
- Correct punctuations into full-width near the CJK.
- Correct punctuations into half-width in english contents.
- (Experimental) Spellcheck and correct words by your own dictionary.
- Lint checking and output diff or JSON result, so you can integrate to everywhere (GitLab CI, GitHub Action, VS Code, Vim, Emacs...)
- Allows using `.gitignore` or `.autocorrectignore` to ignore files that you want to ignore.
- Support more than [28 file types](https://github.com/huacnlee/autocorrect/tree/main/autocorrect/grammar) (Markdown, JSON, YAML, JavaScript, HTML ...), use AST parser to only check for strings, comments.
- Cross platform for Linux, macOS, Windows, and WebAssembly, and as Native SDK for programming (Node.js, JavaScript Browser, Ruby, Python, Java).

## 典型应用场景

- 撰写书籍、文档，新闻媒体等内容发布，应用于 Markdown、AsciiDoc、HTML 等文档场景，确保文案的标准化、专业化（案例：[MDN 项目](https://github.com/mdn/translated-content/pulls?q=is%3Apr+is%3Aclosed+author%3Ahuacnlee)、[少数派](https://sspai.com/search/post/AutoCorrect)）。
- 集成 GitLab CI、GitHub Action、Travis CI 等 CI 环境，需要对项目进行自动化检查。
- 集成到 Docusaurus、Hexo、Hugo、Jekyll、Gatsby 等静态网站生成器，在生成的时候自动格式化。
- 利用语言支持的 SDK 集成到应用程序，在存储或输出网站内容的时候格式化，提升网站品质（如：[Ruby China](https://ruby-china.org)、[V2EX](https://www.v2ex.com)、[Longbridge](https://longportapp.com/news)）。
- 作为 VS Code、Intellij Platform IDE（已支持）、Vim、Emacs (待实现) 插件，需要对文案进行检查（Linter & Formatter），依靠 LintResult 给出的（Annotator、Diagnostic）提示。
- 基于 [WebAssembly](https://www.npmjs.com/package/@huacnlee/autocorrect) 实现，作为 Chrome、Safari 等浏览器插件，应用于任何网站（待实现）
- 也可以集成到 WYSIWYG Editor 里面，例如（ProseMirror、CKEditor、Slate、Draft.js、Tiptap、Monaco Editor、CodeMirror 等）。

## Installation

```bash
$ brew install autocorrect
```

Or you can just install via this:

```bash
$ curl -sSL https://git.io/JcGER | sh
```

After that, you will get `autocorrect` command.

```bash
$ autocorrect -V
AutoCorrect 2.4.0
```

## Upgrade

> Since: 1.9.0

AutoCorrect allows you to upgrade itself by `autocorrect update` command.

```bash
$ autocorrect update
```

> NOTE: This command need you input your password, because it will install bin into `/usr/local/bin` directory.

## Usage

- [Use in CLI](#use-in-cli)
- [Configuration](#configuration)
- [VS Code Extension](#vs-code-extension)
- [Intellij Platform Plugin](#intellij-platform-plugin)
- [GitHub Action](#github-action)
- [GitLab CI](#gitlab-ci)
- [Use for programming](#use-for-programming)

### Use in CLI

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

> Default config: [.autocorrect.default](https://github.com/huacnlee/autocorrect/blob/main/autocorrect/.autocorrectrc.default)

```bash
$ autocorrect init
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
  # Add space between brackets (), [] when near the CJK.
  space-bracket: 1
  # Convert to fullwidth.
  fullwidth: 1
  # To remove space near the fullwidth.
  no-space-fullwidth: 1
  # Fullwidth alphanumeric characters to halfwidth.
  halfwidth-word: 1
  # Fullwidth punctuations to halfwidth in english.
  halfwidth-punctuation: 1
  # Spellcheck
  spellcheck: 2
textRules:
  # Config special rules for some texts
  # For example, if we wants to let "Hello你好" just warning, and "Hi你好" to ignore
  # "Hello你好": 2
  # "Hi你好": 0
fileTypes:
  # Config the files associations, you config is higher priority than default.
  # "rb": ruby
  # "Rakefile": ruby
  # "*.js": javascript
  # ".mdx": markdown
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

> Since: 2.2.0

When you wants to config some special words or texts to ignore on format or lint.

The `textRules` config may help you.

For example we wants:

- `Hello世界` - To just give warning.
- `Hi你好` - To ignore.

Use can config:

```yml
textRules:
  Hello世界: 2
  Hi你好: 0
```

After that, the AutoCorrect will follow your `textRules` to process.

#### Ignore files

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

[![Install Extension](https://img.shields.io/badge/Install%20Extension-VS%20Code-brightgreen)](https://marketplace.visualstudio.com/items?itemName=huacnlee.autocorrect)

https://marketplace.visualstudio.com/items?itemName=huacnlee.autocorrect

内置 Visual Studio Code 插件，安装后会将 AutoCorrect 和 Visual Studio Code 完整集成，可以达到「保存自动格式化」或「纠正提示」。

如下图：

<img width="900" alt="AutoCorrect for VS Code Extension" src="https://user-images.githubusercontent.com/5518/191890126-4e0c99dc-91ce-4262-a774-3813a636eea1.png">

### Intellij Platform Plugin

[![](https://img.shields.io/badge/Plugin-Intellij%20Platform-brightgreen)](https://plugins.jetbrains.com/plugin/20244-autocorrect)

<img width="900" alt="AutoCorrect for Intellij Platform Plugin" src="https://user-images.githubusercontent.com/5518/198998494-82d017f1-40c1-4622-b13f-f67cfecd284a.png">

https://github.com/huacnlee/autocorrect-idea-plugin

### GitHub Action

https://github.com/huacnlee/autocorrect-action

Add to your `.github/workflows/ci.yml`

```yml
steps:
  - name: Check source code
    uses: actions/checkout@v3

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

### Use for programming

AutoCorrect make for supports use in many programming languages.

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)
- Java - [autocorrect-java](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-java)

## Benchmark

> MacBook Pro (13-inch, M1, 2020)

Use `make bench` to run benchmark tests.

See [autocorrect/src/benches/example.rs](https://github.com/huacnlee/autocorrect/blob/main/autocorrect/src/benches/example.rs) for details.

```bash
format_050              time:   [10.003 µs 10.013 µs 10.024 µs]
format_100              time:   [18.511 µs 18.550 µs 18.597 µs]
format_400              time:   [60.089 µs 60.216 µs 60.461 µs]
format_html             time:   [220.68 µs 220.91 µs 221.18 µs]
halfwidth_english       time:   [3.2353 µs 3.2452 µs 3.2545 µs]
format_json             time:   [55.620 µs 55.658 µs 55.695 µs]
format_javascript       time:   [108.63 µs 108.70 µs 108.79 µs]
format_json_2k          time:   [9.3879 ms 9.4871 ms 9.6541 ms]
format_markdown         time:   [1.0007 ms 1.0123 ms 1.0285 ms]
format_jupyter          time:   [86.660 µs 86.743 µs 86.830 µs]
spellcheck_50           time:   [1.9177 µs 1.9422 µs 1.9766 µs]
spellcheck_100          time:   [3.5868 µs 3.5909 µs 3.5950 µs]
spellcheck_400          time:   [14.873 µs 14.974 µs 15.110 µs]
format_markdown         time:   [1.0010 ms 1.0187 ms 1.0484 ms]
lint_markdown           time:   [1.1195 ms 1.1205 ms 1.1215 ms]
lint_json               time:   [67.764 µs 68.341 µs 69.137 µs]
lint_html               time:   [280.87 µs 281.44 µs 282.20 µs]
lint_javascript         time:   [123.99 µs 125.50 µs 128.38 µs]
lint_yaml               time:   [433.41 µs 441.09 µs 452.74 µs]
lint_to_json            time:   [1.8391 ms 1.8428 ms 1.8476 ms]
lint_to_diff            time:   [1.8632 ms 1.8801 ms 1.9030 ms]
```

### Real world benchmark

With [MDN Translated Content](https://github.com/mdn/translated-content) project, it's has about 30K files.

```bash
~/work/translated-content $ autocorrect --fix
AutoCorrect spend time: 8402.538ms
```

## Other Extensions

The other implementations from the community.

- [prettier-plugin-autocorrect](https://github.com/un-ts/prettier/tree/master/packages/autocorrect)
- [autocorrect-popclip](https://github.com/TomBener/autocorrect-popclip)
- [autocorrect-markdown.vim](https://github.com/aisensiy/dotfiles/blob/master/nvim/after/ftplugin/markdown.vim)

## License

This project under MIT license.
