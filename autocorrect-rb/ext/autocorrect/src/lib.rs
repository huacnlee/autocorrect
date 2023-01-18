use magnus::{define_class, function, method, Error, Module, Object};

#[derive(Debug, Clone)]
pub struct LineResult {
    line: usize,
    col: usize,
    new: String,
    old: String,
    severity: usize,
}

impl LineResult {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn get_new(&self) -> String {
        self.new.clone()
    }

    pub fn old(&self) -> String {
        self.old.clone()
    }

    pub fn severity(&self) -> usize {
        self.severity
    }

    pub fn inspect(&self) -> String {
        format!("{self:?}")
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, Error> {
        let hash = magnus::RHash::new();
        hash.aset("line", self.line())?;
        hash.aset("col", self.col())?;
        hash.aset("new", self.get_new())?;
        hash.aset("old", self.old())?;
        hash.aset("severity", self.severity())?;
        Ok(hash)
    }
}

#[derive(Debug, Clone)]
pub struct LintResult {
    pub filepath: String,
    pub lines: Vec<LineResult>,
    pub error: String,
}

impl LintResult {
    pub fn filepath(&self) -> String {
        self.filepath.clone()
    }

    pub fn lines(&self) -> Vec<LineResult> {
        self.lines.clone()
    }

    pub fn error(&self) -> String {
        self.error.clone()
    }

    pub fn inspect(&self) -> String {
        format!("{self:?}")
    }

    pub fn to_hash(&self) -> Result<magnus::RHash, Error> {
        let hash = magnus::RHash::new();
        hash.aset("filepath", self.filepath())?;
        hash.aset(
            "lines",
            self.lines()
                .iter()
                .map(|l| l.to_hash().unwrap())
                .collect::<Vec<magnus::RHash>>(),
        )?;
        hash.aset("error", self.error())?;
        Ok(hash)
    }
}

#[magnus::wrap(class = "AutoCorrect::Ignorer")]
pub struct Ignorer {
    core: autocorrect::ignorer::Ignorer,
}

impl Ignorer {
    pub fn new(work_dir: String) -> Self {
        Ignorer {
            core: autocorrect::ignorer::Ignorer::new(&work_dir),
        }
    }

    fn is_ignored(&self, path: String) -> bool {
        self.core.is_ignored(&path)
    }
}

pub fn format(input: String) -> String {
    autocorrect::format(&input)
}

pub fn format_for(input: String, filename_or_ext: String) -> String {
    autocorrect::format_for(&input, &filename_or_ext).out
}

pub fn lint_for(input: String, filename_or_ext: String) -> magnus::RHash {
    let result = autocorrect::lint_for(&input, &filename_or_ext);

    LintResult {
        filepath: filename_or_ext,
        lines: result
            .lines
            .iter()
            .map(|l| LineResult {
                line: l.line,
                col: l.col,
                new: l.new.clone(),
                old: l.old.clone(),
                severity: l.severity as usize,
            })
            .collect::<_>(),
        error: result.error,
    }
    .to_hash()
    .unwrap()
}

pub fn load_config(config_str: String) {
    autocorrect::config::load(&config_str).unwrap();
}

#[magnus::init(name = "autocorrect")]
fn init() -> Result<(), Error> {
    let class = define_class("AutoCorrect", Default::default())?;
    class.define_singleton_method("format", function!(format, 1))?;
    class.define_singleton_method("format_for", function!(format_for, 2))?;
    class.define_singleton_method("lint_for", function!(lint_for, 2))?;
    class.define_singleton_method("load_config", function!(load_config, 1))?;

    let ignorer_class = class.define_class("Ignorer", Default::default())?;
    ignorer_class.define_singleton_method("new", function!(Ignorer::new, 1))?;
    ignorer_class.define_method("ignored?", method!(Ignorer::is_ignored, 1))?;

    Ok(())
}
