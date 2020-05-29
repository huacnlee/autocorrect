pub struct Strategery {
    space: bool,
    reverse: bool,
    add_space_re: regex::Regex,
    add_space_reverse_re: regex::Regex,
    remove_space_re: regex::Regex,
    remove_space_reverse_re: regex::Regex,
}

impl Strategery {
    pub fn new(one: &'static str, other: &'static str, space: bool, reverse: bool) -> Self {
        let add_space_str = format!("{}{}{}{}{}", "(", one, ")(", other, ")");
        let add_space_reverse_str = format!("{}{}{}{}{}", "(", other, ")(", one, ")");

        let remove_space_str = format!("{}{}{}{}{}", "(", one, r")\s+(", other, ")");
        let remove_space_reverse_str = format!("{}{}{}{}{}", "(", other, r")\s+(", one, ")");

        return Strategery {
            space: space,
            reverse: reverse,
            add_space_re: regexp!(add_space_str),
            add_space_reverse_re: regexp!(add_space_reverse_str),
            remove_space_re: regexp!(remove_space_str),
            remove_space_reverse_re: regexp!(remove_space_reverse_str),
        };
    }

    pub fn format(&self, text: &str) -> String {
        if self.space {
            return self.add_space(text);
        } else {
            return self.remove_space(text);
        }
    }

    fn add_space(&self, text: &str) -> String {
        let mut out = String::from(text);

        out = (&self.add_space_re.replace_all(&out, "$1 $2")).to_string();

        if self.reverse {
            out = (&self.add_space_reverse_re.replace_all(&out, "$1 $2")).to_string();
        }

        return out;
    }

    fn remove_space(&self, text: &str) -> String {
        let mut out = String::from(text);

        out = (&self.remove_space_re.replace_all(&out, "$1 $2")).to_string();

        if self.reverse {
            out = (&self.remove_space_reverse_re.replace_all(&out, "$1 $2")).to_string();
        }

        return out;
    }
}
