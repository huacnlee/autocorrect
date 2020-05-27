#[macro_export]
macro_rules! regexp {
  ($arg:tt) => {{
    let rule_str = String::from($arg).replace(
      r"\p{CJK}",
      r"\p{Han}|\p{Hangul}|\p{Hanunoo}|\p{Katakana}|\p{Hiragana}|\p{Bopomofo}",
    );
    // println!("{}", rule_str);
    let res = regex::Regex::new(&rule_str).unwrap();
    res
  }};
}
