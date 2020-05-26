use regex::Regex;

pub struct Strategery {
  pub one: &'static str,
  pub other: &'static str,
  pub space: bool,
  pub reverse: bool,
}

impl Strategery {
  pub fn new(one: &'static str, other: &'static str, space: bool, reverse: bool) -> Self {
    return Strategery {
      one: one,
      other: other,
      space: space,
      reverse: reverse,
    };
  }

  pub fn format(&self, text: &str) -> &str {
    if self.space {
      return self.add_space(text);
    } else {
      return self.remove_space(text);
    }
  }

  fn add_space(&self, text: &str) -> &str {
    let out = text;
    let re_str = format!("{}{}{}{}{}", "(", self.one, ")(", self.other, ")");
    let re = Regex::new(&re_str).unwrap();
    out = &re.replace_all(&out, "$1 $2");

    if self.reverse {
      let re_str = format!("{}{}{}{}{}", "(", self.other, ")(", self.one, ")");
      let re = Regex::new(&re_str).unwrap();
      out = &re.replace_all(&out, "$1 $2");
    }

    out;
  }

  fn remove_space(&self, text: &str) -> &str {
    let mut out = text;
    let re_str = format!("{}{}{}{}{}", "(", self.one, r")\s+(", self.other, ")");
    let re = Regex::new(&re_str).unwrap();

    out = &re.replace_all(&out, "$1 $2");

    if self.reverse {
      let re_str = format!("{}{}{}{}{}", "(", self.other, r")\s+(", self.one, ")");
      let re = Regex::new(&re_str).unwrap();
      out = &re.replace_all(&out, "$1 $2");
    }

    out;
  }
}
