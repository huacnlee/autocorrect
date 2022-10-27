use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Severity {
    Pass = 0,
    Error = 1,
    Warning = 2,
}

#[derive(Debug, Clone)]
#[pyclass]
struct LineResult {
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub col: usize,
    #[pyo3(get)]
    pub new: String,
    #[pyo3(get)]
    pub old: String,
    #[pyo3(get)]
    pub severity: Severity,
}

#[pymethods]
impl LineResult {
    fn __repr__(&self) -> String {
        format!(
            "LineResult(line={}, col={}, new='{}', old='{}', severity={:?})",
            self.line, self.col, self.new, self.old, self.severity
        )
    }
}

#[pyclass]
struct LintResult {
    #[pyo3(get)]
    pub raw: String,
    #[pyo3(get)]
    pub filepath: String,
    #[pyo3(get)]
    pub lines: Vec<LineResult>,
    #[pyo3(get)]
    pub enable: bool,
    // #[pyo3(get)]
    // pub toggle: Toggle,
}

#[pymethods]
impl LintResult {
    fn __repr__(&self) -> String {
        format!(
            "LintResult(filepath='{}', lines={:?}, enable={})",
            self.filepath, self.lines, self.enable
        )
    }
}

#[pyclass]
struct Ignorer {
    core: autocorrect::ignorer::Ignorer,
}

#[pymethods]
impl Ignorer {
    #[new]
    fn new(work_dir: &str) -> Self {
        Ignorer {
            core: autocorrect::ignorer::Ignorer::new(work_dir),
        }
    }

    fn is_ignored(&self, path: &str) -> bool {
        self.core.is_ignored(path)
    }
}

/// Automatically add spaces between Chinese and English words.
///
/// This method only work for plain text.
#[pyfunction]
fn format(text: &str) -> String {
    autocorrect::format(text)
}

/// Format a file content with filetype.
#[pyfunction]
fn format_for(raw: &str, filename_or_ext: &str) -> PyResult<String> {
    let result = autocorrect::format_for(raw, filename_or_ext);
    if result.has_error() {
        Err(PyValueError::new_err(result.error))
    } else {
        Ok(result.out)
    }
}

/// Lint a file content with filetype.
#[pyfunction]
fn lint_for(raw: &str, filename_or_ext: &str) -> PyResult<LintResult> {
    let result = autocorrect::lint_for(raw, filename_or_ext);
    if result.has_error() {
        Err(PyValueError::new_err(result.error))
    } else {
        let lines = result
            .lines
            .into_iter()
            .map(|l| LineResult {
                line: l.line,
                col: l.col,
                new: l.new,
                old: l.old,
                severity: match l.severity as u8 {
                    0 => Severity::Pass,
                    1 => Severity::Error,
                    2 => Severity::Warning,
                    _ => unreachable!(),
                },
            })
            .collect();
        let lint_result = LintResult {
            raw: result.raw,
            filepath: result.filepath,
            lines,
            enable: result.enable,
        };
        Ok(lint_result)
    }
}

#[pyfunction]
fn load_config(config_str: &str) {
    autocorrect::config::load(config_str).unwrap();
}

/// Automatically add whitespace between CJK (Chinese, Japanese, Korean)
/// and half-width characters (alphabetical letters, numerical digits and symbols).
#[pymodule]
fn autocorrect_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Severity>()?;
    m.add_class::<LineResult>()?;
    m.add_class::<LintResult>()?;
    m.add_class::<Ignorer>()?;

    m.add_function(wrap_pyfunction!(format, m)?)?;
    m.add_function(wrap_pyfunction!(format_for, m)?)?;
    m.add_function(wrap_pyfunction!(lint_for, m)?)?;
    m.add_function(wrap_pyfunction!(load_config, m)?)?;

    Ok(())
}
