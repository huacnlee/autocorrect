// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/lua.pest"]
struct LuaParser;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn debug_lua_parsing() {
        use pest::Parser;
        use pest::iterators::Pair;
        
        fn print_pair(pair: Pair<Rule>, depth: usize) {
            let indent = "  ".repeat(depth);
            println!("{}{:?}: {:?}", indent, pair.as_rule(), pair.as_str());
            
            for inner in pair.into_inner() {
                print_pair(inner, depth + 1);
            }
        }
        
        // 测试多行注释
        let example1 = indoc! {r###"
[[
   role上面一些业务自定义 obj 功能测试
   对应service/roleagent/mods/init.lua
]]
        "###};
        
        println!("
=== Debug Lua Full Example ===");
        let full_example = indoc! {r###"
--[[
   role上面一些业务自定义 obj 功能测试
   对应service/roleagent/mods/init.lua
]]

-- 单行注释role上面测试
function test()
    print("hello world")
end
        "###};
        
        match LuaParser::parse(Rule::item, full_example) {
            Ok(pairs) => {
                for pair in pairs {
                    print_pair(pair, 0);
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
        println!("=== End Debug ===");
        
        println!("=== Debug Lua Block Comment ===");
        match LuaParser::parse(Rule::item, example1) {
            Ok(pairs) => {
                for pair in pairs {
                    print_pair(pair, 0);
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
        println!("=== End Debug ===");
        
        // 测试单行注释
        let example2 = indoc! {r###"
-- 单行注释role上面测试
        "###};
        
        println!("\n=== Debug Lua Line Comment ===");
        match LuaParser::parse(Rule::item, example2) {
            Ok(pairs) => {
                for pair in pairs {
                    print_pair(pair, 0);
                }
            }
            Err(e) => {
                eprintln!("Parse error: {}", e);
            }
        }
        println!("=== End Debug ===");
    }

    #[test]
    fn it_format_lua() {
        let example = indoc! {r###"
        -- 第1行注释
        -- 第2行注释
        function hello(a)
          re = string.find("hello你好")
          
          a = "hello世界"
          b = '你好hello世界'
          c = [[多行
          字符串]]
        end
        "###};

        let expect = indoc! {r###"
        -- 第 1 行注释
        -- 第 2 行注释
        function hello(a)
          re = string.find("hello你好")
          
          a = "hello 世界"
          b = '你好 hello 世界'
          c = [[多行
          字符串]]
        end
        "###};

        assert_eq!(expect, format_for(example, "lua").to_string());
    }

    #[test]
    fn it_format_lua_multiline_comment() {
        let example = indoc! {r###"
--[[
   role上面一些业务自定义 obj 功能测试
   对应service/roleagent/mods/init.lua
]]

-- 单行注释role上面测试
function test()
    print("hello world")
end
        "###};

        let expect = indoc! {r###"
--[[
   role 上面一些业务自定义 obj 功能测试
   对应 service/roleagent/mods/init.lua
]]

-- 单行注释 role 上面测试
function test()
    print("hello world")
end
        "###};

        assert_eq!(expect, format_for(example, "lua").to_string());
    }

    #[test]
    fn it_format_lua_nested_multiline_comments() {
        let example = indoc! {r###"
--[[
这是一个Lua多行注释
包含多个中英文混合的内容：
- Hello世界测试
- 123数字测试ABC
]]

function example()
    local str = "[[这是Lua字符串，不是注释]]"
    return str
end
        "###};

        let expect = indoc! {r###"
--[[
这是一个 Lua 多行注释
包含多个中英文混合的内容：
- Hello 世界测试
- 123 数字测试 ABC
]]

function example()
    local str = "[[这是 Lua 字符串，不是注释]]"
    return str
end
        "###};

        assert_eq!(expect, format_for(example, "lua").to_string());
    }

    #[test]
    fn it_format_lua_edge_cases() {
        let example = indoc! {r###"
--[[
普通块注释
包含service/roleagent/mods/init.lua路径
]]

-- 普通单行注释测试
print("hello")
        "###};

        let expect = indoc! {r###"
--[[
普通块注释
包含 service/roleagent/mods/init.lua 路径
]]

-- 普通单行注释测试
print("hello")
        "###};

        assert_eq!(expect, format_for(example, "lua").to_string());
    }
}