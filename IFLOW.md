# AutoCorrect 项目 - iFlow CLI 上下文文档

## 项目概述

AutoCorrect 是一个基于 Rust 编写的工具，用于自动纠正或检查并建议文案，专门处理 CJK（中文、日语、韩语）与英文混写的场景。它能够补充正确的空格，纠正单词，并以安全的方式自动纠正标点符号。

**核心功能：**
- 在 CJK 和英文单词之间自动添加空格
- 将标点符号转换为全角（靠近 CJK 时）或半角（在英文内容中）
- 拼写检查和单词纠正（实验性功能）
- Lint 检查和差异输出，支持 CI 集成
- 支持超过 28 种文件类型（Markdown、JSON、YAML、JavaScript、HTML 等）

## 项目架构

这是一个 Rust 多工作区项目，包含多个模块：

### 主要模块
- `autocorrect/` - 核心库，包含主要的格式化逻辑和规则引擎
- `autocorrect-cli/` - 命令行接口
- `autocorrect-derive/` - 派生宏支持
- `autocorrect-lsp/` - 语言服务器协议实现
- `autocorrect-wasm/` - WebAssembly 支持

### 语言绑定
- `autocorrect-node/` - Node.js 绑定
- `autocorrect-py/` - Python 绑定  
- `autocorrect-rb/` - Ruby 绑定
- `autocorrect-java/` - Java 绑定

### 其他
- `autocorrect-website/` - 官方网站
- `tests/` - 测试文件

## 开发环境设置

### 构建和运行

**构建项目：**
```bash
# 构建整个工作区
cargo build

# 构建特定模块
cargo build --manifest-path autocorrect-cli/Cargo.toml --release
```

**运行测试：**
```bash
# 运行所有测试
make test

# 运行特定测试
cargo test

# 运行 Node.js 绑定测试
make test:node

# 运行 Python 绑定测试  
make test:python

# 运行 Ruby 绑定测试
make test:ruby

# 运行 Java 绑定测试
make test:java
```

**基准测试：**
```bash
make bench
```

**运行示例：**
```bash
# 运行 CLI 示例
cargo run -- --lint tests/fixtures/*.fixed.*

# 通过 stdin 测试
echo "hello你好" | cargo run -q -- --stdin
```

## 开发命令

### 常用开发命令
- `make test` - 运行所有测试
- `make bench` - 运行基准测试
- `cargo run -- --lint` - 运行 lint 检查
- `cargo run -- --fix` - 自动修复文件

### WebAssembly 构建
```bash
# 安装依赖
make install

# 构建 WASM
make wasm

# 发布 WASM
make wasm:publish
```

## 项目配置

### 配置文件
- `.autocorrectrc` - 项目配置文件（通过 `autocorrect init` 生成）
- `.autocorrectignore` - 忽略文件配置
- `.autocorrectrc.template` - 配置模板

### 依赖管理
- `Cargo.toml` - Rust 依赖管理
- `package.json` - Node.js 依赖管理
- 使用 workspace 模式管理多 crate 依赖

## 开发约定

### 代码风格
- 遵循 Rust 标准编码规范
- 使用 clippy 进行代码检查
- 使用 rustfmt 进行代码格式化

### 测试策略
- 单元测试位于各模块的 `tests/` 目录
- 集成测试在根目录的 `tests/` 文件夹
- 每个语言绑定都有对应的测试套件

### 发布流程
1. 更新版本号：`FROM= TO= make version`
2. 创建新标签：`git tag vx.x.x`
3. 推送标签触发 GitHub Actions 自动发布

## 项目结构说明

### 核心模块（autocorrect/）
- `src/lib.rs` - 主要库入口点
- `src/format.rs` - 格式化逻辑
- `src/rule/` - 校正规则实现
- `src/code/` - 代码文件解析
- `grammar/` - 各种文件类型的语法定义

### CLI 模块（autocorrect-cli/）
- 提供命令行接口
- 支持多线程处理
- 支持多种输出格式（diff、JSON、rdjson）

## 重要文件

- `README.md` - 项目文档和说明
- `DEVELOPMENT.md` - 开发指南
- `Makefile` - 构建和测试脚本
- `Cargo.toml` - 工作区配置
- `package.json` - Node.js 工作区配置

## 注意事项

- 项目使用 Rust 2021 Edition
- 支持跨平台（Linux、macOS、Windows、WebAssembly）
- 包含多种语言绑定，便于集成到不同环境中
- 性能优化良好，支持大文件处理