# AutoCorrrect for Rust

[![Build Status](https://travis-ci.org/huacnlee/auto-correct.rs.svg?branch=master)](https://travis-ci.org/huacnlee/auto-correct.rs)

Automatically add spaces between Chinese and English words.

## Other implements

- Ruby - [auto-correct](https://github.com/huacnlee/auto-correct).
- Go - [go-auto-correct](https://github.com/huacnlee/go-auto-correct).
- Rust - [auto-correct.rs](https://github.com/huacnlee/auto-correct.rs).

## Features

- Auto add spacings between Chinese and English words.
- HTML content support.

## Install

In your Cargo.toml

```toml
[dependencies]
autocorrect = "0.1.1"
```

## Usage

Use `autocorrect::format` to format plain text.

```rust
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
}
```

## Benchmark

### Format

TODO

### FormatHTML

TODO

## License

This project under MIT license.
