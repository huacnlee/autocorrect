// autocorrect: false
use super::*;
use autocorrect_derive::GrammarParser;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(GrammarParser, Parser)]
#[grammar = "../grammar/c.pest"]
struct CParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_format_c() {
        let example = r###"
/**
 * 第1行注释
 * 第2行注释
 */
#include "tig/tig.h"

static bool
app_git_exec_path(char *path, size_t path_len)
{
  static char exec_path[SIZEOF_STR] = "";
  // 第1行注释
  struct app_external app = {
    { "git", "--exec-path", NULL },
    { "GIT_CONFIG=/dev/null", NULL },
  };

  printf( "Hello你好");

  return true;
}
"###;

        let expect = r###"
/**
 * 第 1 行注释
 * 第 2 行注释
 */
#include "tig/tig.h"

static bool
app_git_exec_path(char *path, size_t path_len)
{
  static char exec_path[SIZEOF_STR] = "";
  // 第 1 行注释
  struct app_external app = {
    { "git", "--exec-path", NULL },
    { "GIT_CONFIG=/dev/null", NULL },
  };

  printf( "Hello 你好");

  return true;
}
"###;

        assert_eq!(expect, format_for(example, "c").to_string());
    }
}
