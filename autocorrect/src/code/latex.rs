// autocorrect: false
use super::*;

use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/latex.pest"]
struct LaTeXParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_format_latex() {
        crate::config::setup_test();

        let example = r###"
        \documentclass[]{article}
        % Title Page
        \title{Rust程序设计语言}
        \centerline{\sc Stupid Stuff I Wish Someone Had Told Me Four Years Ago}
        \centerline{\it (Read the .tex file along with this or it won't 
                    make much sense)}
        \author{Steve Klabnik和Carol Nichols}
        
        \begin{document}
        \maketitle
        
        \section{入门指南}
          让我们开始Rust之旅!有很多内容需要学习,但每次旅程总有起点.在本章中,我们会讨论:
          
          1. 在Linux、macOS和Windows上安装Rust
          2. 编写一个打印Hello, world!的程序
          3. 使用Rust的包管理器和构建系统cargo
        \section{常见编程概念}
          \subsection{变量和可变性}
          正如第二章中“使用变量储存值” 部分提到的那样，变量默认是不可改变的（immutable）。这是Rust提供给你的众多优势之一，让你得以充分利用Rust提供的安全性和简单并发性来编写代码。不过，你仍然可以使用可变变量。让我们探讨一下Rust为何及如何鼓励你利用不可变性,以及何时你会选择不使用不可变性。
          
          当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值.为了对此进行说明，使用cargo new variables命令在projects目录生成一个叫做variables的新项目。
        
          接着,在新建的variables目录，打开src/main.rs并将代码替换为如下代码,这些代码还不能编译,我们会首次检查到不可变错误（immutability error）。
        
          \subsection{数据类型}
          在Rust中,每一个值都属于某一个数据类型（data type），这告诉Rust它被指定为何种数据,以便明确数据处理方式.
          
        具体来说,我们将会学习变量、基本类型、函数、注释和控制流。
        \end{document}"###;

        let expected = r###"
        \documentclass[]{article}
        % Title Page
        \title{Rust 程序设计语言}
        \centerline{\sc Stupid Stuff I Wish Someone Had Told Me Four Years Ago}
        \centerline{\it (Read the .tex file along with this or it won't 
                    make much sense)}
        \author{Steve Klabnik 和 Carol Nichols}
        
        \begin{document}
        \maketitle
        
        \section{入门指南}
          让我们开始 Rust 之旅！有很多内容需要学习，但每次旅程总有起点。在本章中，我们会讨论：
          
          1. 在 Linux、macOS 和 Windows 上安装 Rust
          2. 编写一个打印 Hello, world! 的程序
          3. 使用 Rust 的包管理器和构建系统 cargo
        \section{常见编程概念}
          \subsection{变量和可变性}
          正如第二章中“使用变量储存值”部分提到的那样，变量默认是不可改变的（immutable）。这是 Rust 提供给你的众多优势之一，让你得以充分利用 Rust 提供的安全性和简单并发性来编写代码。不过，你仍然可以使用可变变量。让我们探讨一下 Rust 为何及如何鼓励你利用不可变性，以及何时你会选择不使用不可变性。
          
          当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值。为了对此进行说明，使用 cargo new variables 命令在 projects 目录生成一个叫做 variables 的新项目。
        
          接着，在新建的 variables 目录，打开 src/main.rs 并将代码替换为如下代码，这些代码还不能编译，我们会首次检查到不可变错误（immutability error）。
        
          \subsection{数据类型}
          在 Rust 中，每一个值都属于某一个数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。
          
        具体来说，我们将会学习变量、基本类型、函数、注释和控制流。
        \end{document}"###;

        assert_eq!(expected, format_for(example, "latex").to_string());
    }
}
