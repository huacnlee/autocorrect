// autocorrect: false
pub struct Strategery {
    space: bool,
    reverse: bool,
    add_space_re: regex::Regex,
    add_space_reverse_re: regex::Regex,
    remove_space_re: regex::Regex,
    remove_space_reverse_re: regex::Regex,
}

impl Strategery {
    /// Create a new strategery object.
    /// ## Arguments
    ///
    /// - `space` - `true` - add space / `false` - remove space
    /// - `reverse` - false just format `(one + other)`, true will format `(other + one)` and `(other + one)`.
    pub fn new(one: &'static str, other: &'static str, space: bool, reverse: bool) -> Self {
        return Strategery {
            space,
            reverse,
            add_space_re: regexp!("({})({})", one, other),
            add_space_reverse_re: regexp!("({})({})", other, one),
            remove_space_re: regexp!("({})[ ]({})", one, other),
            remove_space_reverse_re: regexp!("({})[ ]({})", other, one),
        };
    }

    pub fn format(&self, text: &str) -> String {
        if self.space {
            self.add_space(text)
        } else {
            self.remove_space(text)
        }
    }

    fn add_space(&self, text: &str) -> String {
        let mut out = String::from(text);

        out = (&self.add_space_re.replace_all(&out, "$1 $2")).to_string();

        if self.reverse {
            out = (&self.add_space_reverse_re.replace_all(&out, "$1 $2")).to_string();
        }

        out
    }

    fn remove_space(&self, text: &str) -> String {
        let mut out = String::from(text);

        out = (&self.remove_space_re.replace_all(&out, "$1 $2")).to_string();

        if self.reverse {
            out = (&self.remove_space_reverse_re.replace_all(&out, "$1 $2")).to_string();
        }

        out
    }
}
