// autocorrect: false

#[cfg(test)]
mod tests {
    use crate::code::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_zig() {
        let example = indoc! {r###"
        //! 这是top-level文档注释
        const std = @import("std");

        /// 这是main函数
        pub fn main() !void {
            // 这是comment
            const text = "hello你好";
        }
        "###};

        let expect = indoc! {r###"
        //! 这是 top-level 文档注释
        const std = @import("std");

        /// 这是 main 函数
        pub fn main() !void {
            // 这是 comment
            const text = "hello 你好";
        }
        "###};

        assert_eq!(expect, format_for(example, "zig").to_string())
    }
}
