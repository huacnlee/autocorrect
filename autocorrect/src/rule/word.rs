use std::borrow::Cow;

// autocorrect: false
use super::{strategery::Strategery, CJK_RE};

lazy_static! {
    // Strategies all rules
    static ref WORD_STRATEGIES: Vec<Strategery> = vec![
        // EnglishLetter, Number
        // Avoid add space when Letter, Number has %, $, \ prefix, eg. %s, %d, $1, $2, \1, \2, \d, \r, \p ... in source code
        Strategery::new(r"\p{CJK}[^%\$\\]", r"[a-zA-Z0-9]"),
        Strategery::new(r"[^%\$\\][a-zA-Z0-9]", r"\p{CJK}"),
        // Number, -100, +100
        Strategery::new(r"\p{CJK}", r"[\-+][\d]+").with_reverse(),
        // Spcial format Letter, Number leading case, because the before Strategery can't cover eg. A开头的case测试
        Strategery::new(r"^[a-zA-Z0-9]", r"\p{CJK}"),
        // 10%中文
        Strategery::new(r"[0-9][%]", r"\p{CJK}"),
        // 300+单词，A+评分，C++中文，C#中文, 100#中文
        // The `#` can'not work, because is related to URL anchor, can't do it.
        Strategery::new(r"[a-zA-Z0-9][+#]+", r"\p{CJK}"),
    ];

    static ref PUNCTUATION_STRATEGIES: Vec<Strategery> = vec![
        // SpecialSymbol
        Strategery::new(r"[\p{CJK_N}”’]", r"[\|+][\p{CJK_N}\s（【「《“‘]"),
        Strategery::new(r"[\p{CJK_N}\s）】」”’》][\|+]", r"[\p{CJK_N}“‘]"),
        Strategery::new(r"[!]", r"\p{CJK}"),
    ];

    static ref BRACKETS_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after brackets [] or () near the CJK
        Strategery::new(r"\p{CJK}", r"[\[\(]"),
        Strategery::new(r"[\]\)]", r"\p{CJK}"),
    ];

    static ref BACKTICKS_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after backtick ` near the CJK
        Strategery::new(r"\p{CJK}", r"`.+`"),
        Strategery::new(r"`.+`", r"\p{CJK}"),
    ];

    static ref DASH_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after dash - near the CJK
        Strategery::new(r"[\p{CJK_N}”’]", r"[\-][\p{CJK_N}\s（【「《“‘]"),
        Strategery::new(r"[\p{CJK_N}\s）】」”’》][\-]", r"[\p{CJK_N}“‘]"),
    ];

    static ref DOLLAR_STRATEGIES: Vec<Strategery> = vec![
        // Add space before and after dollar $ near the CJK
        Strategery::new(r"\p{CJK}", r"\$"),
        Strategery::new(r"\$", r"\p{CJK}"),
    ];

    static ref NO_SPACE_FULLWIDTH_STRATEGIES: Vec<Strategery> = vec![
        // FullwidthPunctuation remove space case, Fullwidth can safe to remove spaces
        Strategery::new(r"\w|\p{CJK}|`", r"[，。、！？：；（）「」《》【】]").with_remove_space().with_reverse(),
    ];

    static ref NO_SPACE_FULLWIDTH_QUOTE_STRATEGIES: Vec<Strategery> = vec![
        // Remove space around fullwidth quotes
        Strategery::new(r"\w|\p{CJK}", r"[“”‘’]").with_remove_space().with_reverse(),
    ];
}

pub fn format_space_word(input: &str) -> Cow<str> {
    WORD_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_space_punctuation(input: &str) -> Cow<str> {
    PUNCTUATION_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_space_bracket(input: &str) -> Cow<str> {
    BRACKETS_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_space_dash(input: &str) -> Cow<str> {
    DASH_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_space_backticks(input: &str) -> Cow<str> {
    BACKTICKS_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_space_dollar(input: &str) -> Cow<str> {
    DOLLAR_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        },
    )
}

pub fn format_no_space_fullwidth(input: &str) -> Cow<str> {
    if !CJK_RE.is_match(input) {
        return Cow::Borrowed(input);
    }

    NO_SPACE_FULLWIDTH_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

pub fn format_no_space_fullwidth_quote(input: &str) -> Cow<str> {
    if !CJK_RE.is_match(input) {
        return Cow::Borrowed(input);
    }

    NO_SPACE_FULLWIDTH_QUOTE_STRATEGIES
        .iter()
        .fold(Cow::Borrowed(input), |text, strategy| match text {
            Cow::Borrowed(s) => strategy.format(s),
            Cow::Owned(s) => Cow::Owned(strategy.format(&s).into_owned()),
        })
}

#[cfg(test)]
mod tests {
    use crate::rule::word::{format_space_backticks, format_space_bracket, format_space_dash, format_space_dollar};

    #[test]
    fn test_format_space_dash() {
        assert_eq!(format_space_dash("你好-世界"), "你好 - 世界");
        assert_eq!(format_space_dash("foo-世界"), "foo-世界");
        assert_eq!(format_space_dash("你好-world"), "你好-world");
        assert_eq!(format_space_dash("hello-world"), "hello-world");
    }

    #[test]
    fn test_format_space_bracket() {
        assert_eq!(format_space_bracket("你好[世界]"), "你好 [世界]");
        assert_eq!(format_space_bracket("你好(世界)"), "你好 (世界)");
        assert_eq!(format_space_bracket("foo[世界"), "foo[世界");
        assert_eq!(format_space_bracket("你好]world"), "你好]world");
        assert_eq!(format_space_bracket("hello]world"), "hello]world");
    }

    #[test]
    fn test_format_space_backticks() {
        assert_eq!(format_space_backticks("代码`code`"), "代码 `code`");
        assert_eq!(format_space_backticks("代码`code`代码"), "代码 `code` 代码");
        assert_eq!(
            format_space_backticks("`code`代码`code`"),
            "`code` 代码 `code`"
        );
        assert_eq!(
            format_space_backticks("`code`hello`code`"),
            "`code`hello`code`"
        );

        assert_eq!(format_space_backticks("```rs"), "```rs");
        assert_eq!(format_space_backticks("``代码第1行"), "``代码第1行");
        assert_eq!(format_space_backticks("`代码第1行"), "`代码第1行");
        assert_eq!(format_space_backticks("代码第2行`"), "代码第2行`");
    }

    #[test]
    fn test_format_space_dollar() {
        assert_eq!(format_space_dollar("你好$世界"), "你好 $ 世界");
        assert_eq!(format_space_dollar("hello$世界"), "hello$ 世界");
        assert_eq!(format_space_dollar("你好$world"), "你好 $world");
        assert_eq!(format_space_dollar("你好$x$世界"), "你好 $x$ 世界");
        assert_eq!(format_space_dollar("变量 $x$ 代表"), "变量 $x$ 代表");
        assert_eq!(format_space_dollar("令$x^2+y^2=z^2$，可得"), "令 $x^2+y^2=z^2$，可得");
        assert_eq!(format_space_dollar("这是一个例子：$E=mc^2$。"), "这是一个例子：$E=mc^2$。");
        assert_eq!(format_space_dollar("$x+y$是方程"), "$x+y$ 是方程");
        assert_eq!(format_space_dollar("方程为$x+y=1$"), "方程为 $x+y=1$");
        assert_eq!(format_space_dollar("若$x>0$且$y<0$"), "若 $x>0$ 且 $y<0$");
        assert_eq!(format_space_dollar("函数$f(x)$的极值"), "函数 $f(x)$ 的极值");
        assert_eq!(format_space_dollar("变数$x$、$y$满足"), "变数 $x$、$y$ 满足");
        assert_eq!(format_space_dollar("公式$$E=mc^2$$证明了"), "公式 $$E=mc^2$$ 证明了");
        assert_eq!(format_space_dollar("矩阵$A=\\begin{bmatrix}1&0\\\\0&1\\end{bmatrix}$满足"), "矩阵 $A=\\begin{bmatrix}1&0\\\\0&1\\end{bmatrix}$ 满足");
        assert_eq!(format_space_dollar("测试$x_1,x_2$以及$x_3$"), "测试 $x_1,x_2$ 以及 $x_3$");
        assert_eq!(format_space_dollar("若$a>b$则有$c>d$"), "若 $a>b$ 则有 $c>d$");
        assert_eq!(format_space_dollar("设$a,b∈\\mathbb{R}$且$a>b$"), "设 $a,b∈\\mathbb{R}$ 且 $a>b$");
        assert_eq!(format_space_dollar("下式成立：$$f(x)=x^2$$"), "下式成立：$$f(x)=x^2$$");
    }
}
