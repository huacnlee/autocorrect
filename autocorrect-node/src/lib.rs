#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi(object)]
pub struct LineResult {
    pub l: u32,
    pub c: u32,
    pub new: String,
    pub old: String,
    pub severity: u32,
}

#[napi(object)]
pub struct LintResult {
    pub filepath: String,
    pub lines: Vec<LineResult>,
    pub error: String,
}

#[napi]
pub struct Ignorer {
    core: autocorrect::ignorer::Ignorer,
}

#[napi]
impl Ignorer {
    #[napi(constructor)]
    pub fn new(work_dir: String) -> Self {
        Ignorer {
            core: autocorrect::ignorer::Ignorer::new(&work_dir),
        }
    }

    #[napi]
    pub fn is_ignored(&self, path: String) -> bool {
        self.core.is_ignored(&path)
    }
}

#[napi]
pub fn format(text: String) -> String {
    autocorrect::format(&text)
}

#[napi]
pub fn format_for(text: String, filepath: String) -> String {
    let result = autocorrect::format_for(&text, &filepath);
    result.out
}

#[napi]
pub fn lint_for(text: String, filepath: String) -> LintResult {
    let result = autocorrect::lint_for(&text, &filepath);
    LintResult {
        filepath,
        error: result.error,
        lines: result
            .lines
            .iter()
            .map(|l| LineResult {
                l: l.line as u32,
                c: l.col as u32,
                new: l.new.clone(),
                old: l.old.clone(),
                severity: l.severity as u32,
            })
            .collect::<_>(),
    }
}

#[napi]
pub fn load_config(config_str: String) {
    autocorrect::config::load(&config_str).unwrap();
}

#[napi]
pub async fn run(args: Vec<String>) {
    // skip 2 args:
    // 1. node
    // 2. node_modules/.bin/autocorrect
    let args = args.iter().skip(1).collect::<Vec<_>>();
    autocorrect_cli::run(args).await;
}
