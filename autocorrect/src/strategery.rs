// autocorrect: false
enum SpaceMode {
    Add,
    Remove,
}

pub struct Strategery {
    space_mode: SpaceMode,
    reverse: bool,
    add_space_re: regex::Regex,
    add_space_reverse_re: regex::Regex,
    remove_space_re: regex::Regex,
    remove_space_reverse_re: regex::Regex,
}

impl Strategery {
    /// Create a new strategery object.
    pub fn new(one: &'static str, other: &'static str) -> Self {
        return Strategery {
            space_mode: SpaceMode::Add,
            reverse: false,
            add_space_re: regexp!("({})({})", one, other),
            add_space_reverse_re: regexp!("({})({})", other, one),
            remove_space_re: regexp!("({})[ ]({})", one, other),
            remove_space_reverse_re: regexp!("({})[ ]({})", other, one),
        };
    }

    // Set Strategery for remove space.
    pub fn with_remove_space(mut self) -> Self {
        self.space_mode = SpaceMode::Remove;
        return self;
    }

    // Set Strategery for format by reverse again.
    pub fn with_reverse(mut self) -> Self {
        self.reverse = true;
        return self;
    }

    pub fn format(&self, text: &str) -> String {
        match self.space_mode {
            SpaceMode::Add => self.add_space(text),
            SpaceMode::Remove => self.remove_space(text),
        }
    }

    fn add_space(&self, text: &str) -> String {
        let mut out = String::from(text);

        out = self.add_space_re.replace_all(&out, "$1 $2").to_string();

        if self.reverse {
            out = self
                .add_space_reverse_re
                .replace_all(&out, "$1 $2")
                .to_string();
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
