// autocorrect: false
{
  /* <option selected value="html">HTML</option>
<option value="js">JavaScript</option>
<option value="ts">TypeScript</option>
<option value="css">CSS / SCSS</option>
<option value="json">JSON</option>
<option value="yml">YAML</option>
<option value="go">Go</option>
<option value="rs">Rust</option>
<option value="rb">Ruby</option>
<option value="py">Python</option>
<option value="java">Java</option>
<option value="php">PHP</option>
<option value="sql">SQL</option>
<option value="objective_c">Objective-C</option>
<option value="swift">Swift</option>
<option value="scala">Scala</option>
<option value="kt">Kotlin</option>
<option value="dart">Dart</option>
<option value="ex">Elixir</option>
<option value="csharp">C#</option>
<option value="md">Markdown</option>
<option value="plain">Plain Text</option> */
}
export default {
  html: {
    title: 'HTML',
    raw: `<h1>编译Rust为WebAssembly</h1>
<div></div>
<p class="summary">如果你写了一些Rust代码，你可以把它编译成WebAssembly！这份教程将带你编译Rust项目为wasm并在一个现存的web应用中使用它。</p></div>
<h2 id="rust_和_webassembly_用例"><a href="#rust_和_webassembly_用例" title="Permalink to Rust和WebAssembly用例">Rust和WebAssembly用例</a></h2>
<div><p>Rust和WebAssembly有两大主要用例：</p>
<ul>
<li>构建完整应用 —— 整个Web应用都基于Rust开发！</li>
<li>构建应用的组成部分 —— 在现存的JavaScript前端中使用Rust。</li>
</ul>

<p>目前，Rust团队正专注于第二种用例，因此我们也将着重介绍它。对于第一种用例，可以参阅<code><a href="https://github.com/DenisKolodin/yew" class="external" rel=" noopener">yew</a></code>这类项目。</p>

<p>在本教程中，我们将使用Rust的npm包构建工具<code>wasm-pack</code>来构建一个NPM包。这个包只包含WebAssembly和JavaScript代码，以便包的用户无需安装Rust就能使用。他们甚至不需要知道这里包含WebAssembly！</p></div>
`,
  },
  js: {
    title: 'JavaScript',
    raw: `/**
* Hello你好
* 这是第2行
*/
function application() {
  let example = "这是single line单行注释";
  console.log(\`这是string第1行
  这是string第2行
  \`)

  // autocorrect-disable
  const disable_1 = "这行将会disable掉";
  const disable_2 = "这行将也会disable掉";
  // autocorrect-enable

  const c = "这是string第3行";
}
    `,
  },
  css: {
    title: 'CSS / SCSS',
    raw: `
/* 
  这是多行CSS第1行
  这是第2行
*/
.btn {
  .strong { font-weight: bold; }
  padding: 10px; // comment在属性后面
  // 这是comment单行
  font: Helvetica, sans-serif;
}
`,
  },
  md: {
    title: 'Markdown',
    raw: `# 这是Heading 1大标题

**加粗** 
*倾斜*
~~删除线~~
这是**Bold加粗**在1个段落中，这端会correct掉，如果是inline code，例如\`Rust语言\`，也可以应该处理。

> 引用文本：Quote也是可以的。

\`\`\`rust
// Codeblock里面也会处理
let a = "你好hello";
\`\`\`

- ![img图片](https://google.com/a/b/url不处理)
- [link链接](https://google.com/a/b/url不处理)`,
  },
  rust: {
    title: 'Rust',
    raw: `fn main() {
let number_list = vec![34, 50, 25, 100, 65];

let mut largest = number_list[0];

let regexp = %r"包含#regexp测试";

// 1第一行Single line注释
// 2第二行注释
for number in number_list {
    if number > largest {
        largest = number;
    }
}

// autocorrect: false
let disable_1 = "这行将会disable掉";
let disable_2 = "这行将也会disable掉";
// autocorrect: true

let a = r#"
这是第1行
这是第2行
"#;

let b = r##"
这是第 3 行
这是第 4 行
"##;

/**
 * 多行Rust注释
 * 第二行Rust注释
*/
println!("最大的数字number是{}", largest);
}`,
  },
  go: {
    title: 'Go',
    raw: `// WithContext创建基于ctx的db
// 第2行注释
func (d *Dao) WithContext(ctx context.Context) (db *gorm.DB) {
  a := "第1个"
  b := \`
  多行string
  第2行
  \`

  re := regexp.MustCompile(\`regexp不处理\`)
  re1 := regexp.Compile("regexp不处理")
  t := time.Parse("2006年01月02日 15:04", t)

  fmt.Println(a + b + "go语言")
  fmt.Println("%s链接的内容不会空格%d也不处理，保守", "格式", 100)
  db = d.DB.WithContext(ctx)
  return
}`,
  },
  rb: {
    title: 'Ruby',
    raw: `# 第1行注释
# 第2行注释
def hello(a, b: "第1个参数")
  re = /hello你好/
  re1 = %r{hello你好}
  re2 = Regexp.new('hello你好' )
  re3 = Regexp.new( "hello你好")

  a = "hello世界#{a}"
  b = '你好hello世界'
end`,
  },
  py: {
    title: 'Python',
    raw: `'''
这是多行1注释
这是多行2注释
这是多行3注释
'''
def hello(a):
  multi_str = """
  第1行多行字符串
  第2行多行字符串
  """

  re = r'包含#regexp测试'
  re1 = r"""
    包含re0测试
    包含re1测试
  """
  re2 = re.compile( "hello你" + "world好")

  # 第4个注释
  print("你好hello世界")
  print('你好hello世界')`,
  },
  objective_c: {
    title: 'Objective-C',
    raw: `// 第1行注释
// 第2行注释
- (void) helloWorld {
  // 第3行注释
  NSString *geotestUrl = @"第1个字符串string";

  NSRegularExpression* re0 = [NSRegularExpression regularExpressionWithPattern:  @"re正则" options:0 error:&err];
}`,
  },
  java: {
    title: 'Java',
    raw: `// 第1行注释
// 第2行注释
public String helloWorld() {
  // 第3行注释
  String singleLineString = "第1个字符串string"

  Pattern re0 = Pattern.compile("re正则" );
  Pattern.matches( "re1正则" , "foobar你好");

  /**
   * 第4行注释
   * 第5行注释
   */
  String quotation = """
  这是多行string里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """
}`,
  },
  swift: {
    title: 'Swift',
    raw: `// 第1行注释
// 第2行注释
func helloWorld(name: String) -> String {
  // 第3行注释
  let singleLineString = "第1个字符串string"

  let quotation = """
这是多行string里面包含"双引号"
"Begin at the beginning," the King said gravely.
"""

  let re = try! NSRegularExpression(pattern:    "re正则")
}`,
  },
  kt: {
    title: 'Kotlin',
    raw: `/** 
* 第1行注释
* 第2行注释
*/
fun helloWorld(name: String) {
  // 第3行注释
  var singleLineString = "第1个字符串string"

  var quotation = """
  这是多行string里面包含"双引号"
  "Begin at the beginning," the King said gravely.
  """

  var re0 = Regex("re正则" )
  var re1 = "re正则".toRegex()
}`,
  },
  php: {
    title: 'PHP',
    raw: `<div class="container">
<p>目前html tag里的无法处理</p>
<?php
  /** 
   * 第1行注释
   * 第2行注释
   */
  class HelloWorld {
      // 这是第3行注释
      var singleLineString: String = "单行string测试"
      var multilineString: String = "多行string测试
      第2行字符串"

      var re0 = preg_match( "re1正则", singleLineString )
      var re1 = preg_match_all("re2正则" ,  multilineString )
  }
?>
</div>`,
  },
  cs: {
    title: 'C#',
    raw: `/**
* 第1行注释
* 第2行注释
*/
public String helloWorld(stirng name) {
  // 第3行注释
  string singleLineString = "第1个字符串string";
  string stringLiteral = $"这是stringLiteral {name}!";

  string quotation = @"
  这是多行string第1行
  这是多行string第2行
  ";

  Regex rx = new Regex( @"re正则", RegexOptions.Compiled  | RegexOptions.IgnoreCase);
}`,
  },
  ex: {
    title: 'Elixir',
    raw: `defmodule Test do
@moduledoc """
多行注释第1行
multiline comment第2行
"""

def hello do
  # 单行comment注释
  str1 = "hello你好双引号"
  str2 = 'hello你好单引号'
  str3 = ~s(hello你好)
  str4 = ~c(hello你好)

  multiline_str = ~S"""
  多行字符串第1行
  多行string第2行
  """

  pattern1 = ~r/hello正则/
  pattern2 = Regex.compile("hello正则")
end
end`,
  },
  dart: {
    title: 'Dart',
    raw: `/** 
* 第1行注释
* 第2行注释
*/
String helloWorld(String name) {
  // 第3行注释
  var singleLineString = "第1个字符串string";
  var singleLineString = '第2个字符串string';

  var quotation = """
  这是第3行字符串
  这是第4行
  """;

  let quotation = '''
  这是第5行字符串
  这是第6行
  ''';

  let re0 = r"re正则"
  let re1 = r're正则'
}`,
  },
  sql: {
    title: 'SQL',
    raw: `SELECT * FROM "topics" WHERE "id" = ?;
COMMENT ON COLUMN "topics"."user_id" IS 'topic创建者';
-- Comment注释信息
COMMENT ON COLUMN "topics"."status" IS '3屏蔽 1审核中 2已发布';
/* 
  多行，且带有换行
  注释comment信息 
*/
COMMENT ON COLUMN "topics"."kind" IS '0普通 1转发';`,
  },
  yml: {
    title: 'YAML',
    raw: `# this is comment line
foo: 'hello世界'
region:
  cn-north-1
"en":
  name: "你好Hello世界"
  foo: Bar
  dar:
    - foo: 1
    - bar: "数字2"
  "abc字段": abc字段`,
  },
  json: {
    title: 'JSON',
    raw: `{
  "name": "你好hello世界",
  "displayName": "JSON格式测试",
  "publisher": "huacnlee",
  "meta": {
    // 第1行注释
    "title": "第1个meta", 
    /** 
     * 第2行注释
     * 第3行注释
     */
    "description": "第2个meta", 
    "测试key不格式化": false
  }
}`,
  },
  strings: {
    title: 'Strings',
    raw: `/* 
  InfoPlist.strings测试
  Created by某某
*/

"CFBundleDisplayName" = "App名称";//app中文名称
"CFBundleIdentifier" = "huacnlee.autocorrect";

"NSCameraUsageDescription" = "开启Wi-Fi后继续使用";
// 单行comment
"中文key测试" = "开启定位权限";`,
  },
  txt: {
    title: 'Plain Text',
    raw: `苹果「最强促销」开启,最高可省4446元!但这些细节值得注意

在7月16日–9月27日期间,只要你符合教育优惠的条件,便能以低价购买指定的Mac或iPad,并收获一副免费的AirPods 2耳机（官方售价1246元）.

你可以加312元，换成无线充电盒款,也可以加753元，换成AirPods Pro?`,
  },
};
