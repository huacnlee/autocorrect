# AutoCorrect Desktop

> 🎯 AutoCorrect 的愿景是提供一套标准化的文案较正方案。以便于在各类场景（例如：撰写书籍、文档、内容发布、项目源代码...）里面应用，让使用者轻松实现标准化、专业化的文案输出 / 校正。

A linter and formatter for help you improve copywriting, to correct spaces, words, punctuations between CJK (Chinese, Japanese, Korean).

Like Eslint, Rubocop, Gofmt ..., AutoCorrect allows us to checking soure code, and output as colorized diff with corrected suggest. You can intergrating to CI (GitLab CI, GitHub Action, Travis CI....) for use to checking the contents in source code. Recognize the file name, and find out the strings and the comment part.

基于 Rust 编写的 CLI 工具，用于「自动纠正」或「检查并建议」文案，给 CJK（中文、日语、韩语）与英文混写的场景，补充正确的空格，纠正单词，同时尝试以安全的方式自动纠正标点符号等等。

类似 ESlint、Rubocop、Gofmt 等工具，AutoCorrect 可以用于 CI 环境，它提供 Lint 功能能便捷的检测出项目中有问题的文案，起到统一规范的作用。

支持各种类型源代码文件，能自动识别文件名，并准确找到字符串、注释做自动纠正。

## Download

https://github.com/huacnlee/autocorrect/releases

## Screenshot

![SCR-20220618-v6l](https://user-images.githubusercontent.com/5518/174442991-b8bbda54-78e4-4970-8f72-89cec716c44c.png)

