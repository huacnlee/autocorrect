# AutoCorrect for Ruby

The Native Ruby version of [AutoCorrect](https://github.com/huacnlee/autocorrect).

- Rust - [autocorrect](https://github.com/huacnlee/autocorrect)
- Ruby - [autocorrect-rb](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-rb)
- Go - [autocorrect-go](https://github.com/longbridgeapp/autocorrect)
- Python - [autocorrect-py](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-py)
- Node.js - [autocorrect-node](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-node)
- JavaScript (Browser) - [autocorrect-wasm](https://github.com/huacnlee/autocorrect/tree/main/autocorrect-wasm)

## Installation

```bash
$ bundle add autocorrect-rb
```

## Usage

```rb
require('autocorrect-rb');

out = AutoCorrect.format('Hello你好.')
puts out
# Hello 你好。

out = AutoCorrect.format_for("title = 'Hello你好。'", 'rb')
puts out
# title = 'Hello 你好。'

result = AutoCorrect.lint_lor("title = 'Hello你好。'", 'rb')
puts result
# {
#   filepath: 'rb',
#   lines: [
#     { l: 1, c: 13, new: "'Hello 你好。'", old: "'Hello你好。'", severity: 1 }
#   ],
#   error: ''
# }
```

## Benchmarks

🎊 Rust version is 3x faster than the Ruby (pure) version.

> NOTE: In this case Rust version has more complex rules.

### Rust version

```bash
Warming up --------------------------------------
     format 50 chars    11.348k i/100ms
    format 100 chars     6.033k i/100ms
    format 400 chars     1.772k i/100ms
         format_html   545.000  i/100ms
Calculating -------------------------------------
     format 50 chars    111.904k (± 3.1%) i/s -    567.400k in   5.075674s
    format 100 chars     58.684k (± 2.1%) i/s -    295.617k in   5.039837s
    format 400 chars     17.266k (± 2.9%) i/s -     86.828k in   5.033234s
         format_html      5.448k (± 1.5%) i/s -     27.250k in   5.002853s
```

### Pure [Ruby version](https://rubygems.org/gems/auto-correct/versions/1.0.0) result:

```bash
Warming up --------------------------------------
     format 50 chars     3.167k i/100ms
    format 100 chars     1.588k i/100ms
    format 400 chars   496.000  i/100ms
         format_html   166.000  i/100ms
Calculating -------------------------------------
     format 50 chars     31.589k (± 2.5%) i/s -    158.350k in   5.016131s
    format 100 chars     16.122k (± 1.2%) i/s -     80.988k in   5.024082s
    format 400 chars      4.946k (± 2.6%) i/s -     24.800k in   5.017711s
         format_html      1.659k (± 1.7%) i/s -      8.300k in   5.003164s
```

## Know issues

Bundler install error:

```
Could not find gem 'autocorrect-rb' with platform 'ruby' in rubygems repository https://rubygems.org/ or installed locally.
```

To fix this you can run:

```bash
$ bundle lock --remove-platform ruby
```

Because of autocorrect-rb not release the gem for `platform: ruby`, but your `Gemfile.lock` specialed that. This command will remove `ruby` platform from your `Gemfile.lock`


