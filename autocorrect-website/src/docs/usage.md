# Usage

## Installation

```bash
$ brew install autocorrect
```

Or you can just install it via this:

```bash
$ curl -sSL https://git.io/JcGER | sh
```

After that, you will get `autocorrect` command.

```bash
$ autocorrect -V
AutoCorrect 2.4.0
```

Or install NPM:

```bash
$ yarn add autocorrect-node
$ yarn autocorrect -V
```

## Upgrade

> Since: 1.9.0

AutoCorrect allows you to upgrade itself by `autocorrect update` command.

```bash
$ autocorrect update
```

> NOTE: This command need you input your password, because it will install bin into `/usr/local/bin` directory.

AutoCorrect provides a command line tool, and also provides a library in many languages such as Rust, Node.js, Python, Ruby, Java, etc.

## Use CLI

```bash
$ autocorrect text.txt
你好 Hello 世界

$ echo "hello世界" | autocorrect --stdin
hello 世界

$ autocorrect --fix text.txt
$ autocorrect --fix zh-CN.yml
$ autocorrect --fix
```

## Lint

```bash
$ autocorrect --lint --format json text.txt

$ autocorrect --lint text.txt
```

Then you will get result:

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

How to lint all changed files in Git:

```bash
$ git diff --name-only | xargs autocorrect --lint
```

## Use in NPM

AutoCorrect has been published in NPM with CLI command support. If you want to use it in Frontend or Node.js project, you can just install `autocorrect-node` package for without install AutoCorrect bin.

```bash
cd your-project
yarn add autocorrect-node
```

Now you can run `yarn autocorrect` command in your project. This command is same as `autocorrect` command.

```bash
$ yarn autocorrect -h
```

## Configuration

> Default config: [.autocorrect.default](https://github.com/huacnlee/autocorrect/blob/main/autocorrect/.autocorrectrc.default)

```bash
$ autocorrect init
AutoCorrect init config: .autocorrectrc
```

> NOTE: If you download fail, try to use `autocorrect init --local` command again.

Now the `.autocorrectrc` file has been created.

> .autocorrectrc is allows use YAML, JSON format.

Config file example:

```yml
# yaml-language-server: $schema=https://huacnlee.github.io/autocorrect/schema.json
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

### Ignore option

> Since: 2.2.0

When you want to config some special words or texts to ignore on format or lint.

The `textRules` config may help you.

For example, we want:

- `Hello世界` - To just give a warning.
- `Hi你好` - To ignore.

Use can config:

```yml
textRules:
  Hello世界: 2
  Hi你好: 0
```

After that, AutoCorrect will follow your `textRules` to process.

### Ignore files

**Use `.autocorrectignore` to ignore files**

Sometimes, you may want to ignore some special files that not want to check.

By default, the file matched `.gitignore` rule will be ignored.

You can also use `.autocorrectignore` to ignore other files, format like `.gitignore`.

### Disable by inline comment

If you just want to disable some special lines in a file, you can write a comment `autocorrect-disable`,
when AutoCorrect matched the comment include that, it will disable temporarily.

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

### Disable some rules

> Since: 2.0

You can use `autocorrect-disable <rule>` in a comment to disable some rules.

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

## Editor Integration

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

## CI/CD

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

### Work with ReviewDog

> Since: 2.8.0

AutoCorrect can work with [reviewdog](https://github.com/reviewdog/reviewdog), so you can use it in CI/CD. ReviewDog will post a comment to your PR with the AutoCorrect change suggestions. Then the PR committer can easy to accept the suggestions.

Use `--format rdjson` option to output the lint results as the [reviewdog](https://github.com/reviewdog/reviewdog) supported format.

```bash
autocorrect --lint --format rdjson | reviewdog -f=rdjson -reporter=github-pr-review
```

Use [huacnlee/autocorrect-action](https://github.com/huacnlee/autocorrect-action) can help you setup GitHub Action.

<img src="https://user-images.githubusercontent.com/5518/257680682-050d6f62-d461-44fc-a22f-2fb581ba0912.png" width="640" />

## Use for programming

AutoCorrect makes for support use in many programming languages.

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)
- Java - [autocorrect-java](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-java)
