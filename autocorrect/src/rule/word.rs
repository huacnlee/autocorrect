// autocorrect: false
use super::strategery::Strategery;

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
    ];

    static ref PUNCTUATION_STRATEGIES: Vec<Strategery> = vec![
        // SpecialSymbol
        Strategery::new(r"[\p{CJK_N}”’]", r"[\-\|+][\p{CJK_N}\s（【「《“‘]"),
        Strategery::new(r"[\p{CJK_N}\s）】」”’》][\-\|+]", r"[\p{CJK_N}“‘]"),
        Strategery::new(r"\p{CJK}", r"[\[\(]"),
        Strategery::new(r"[\]\)!]", r"\p{CJK}"),
    ];

    static ref NO_SPACE_FULLWIDTH_STRATEGIES: Vec<Strategery> = vec![
        // FullwidthPunctuation remove space case, Fullwidth can safe to remove spaces
        Strategery::new(r"\w|\p{CJK}", r"[，。、！？：；（）「」《》【】“”‘’]").with_remove_space().with_reverse(),
    ];

}

pub fn format_space_word(input: &str) -> String {
    let mut out = String::from(input);
    WORD_STRATEGIES.iter().for_each(|s| out = s.format(&out));
    out
}

pub fn format_space_punctuation(input: &str) -> String {
    let mut out = String::from(input);
    PUNCTUATION_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}

pub fn format_no_space_fullwidth(input: &str) -> String {
    let mut out = String::from(input);
    NO_SPACE_FULLWIDTH_STRATEGIES
        .iter()
        .for_each(|s| out = s.format(&out));
    out
}
