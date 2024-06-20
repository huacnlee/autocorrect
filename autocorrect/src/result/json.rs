//! AutoCorrect Lint JSON
use crate::LintResult;

#[doc(hidden)]
pub fn to_lint_results_json(lint_results: Vec<LintResult>) -> String {
    format!(
        r#"{{"count": {},"messages": [{}]}}"#,
        lint_results.len(),
        lint_results
            .iter()
            .map(|r| r.to_json())
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
pub(crate) fn crate_test_lint_results() -> Vec<LintResult> {
    use crate::result::{LineResult, Results, Severity};

    let mut lint_result = LintResult::new("hello你好.\n这是第2行");
    lint_result.line = 10;
    lint_result.col = 12;
    lint_result.filepath = "./test/foo/bar.rs".to_string();
    lint_result.push(LineResult {
        line: 1,
        col: 1,
        new: "hello 你好。".to_owned(),
        old: "hello你好.".to_owned(),
        severity: Severity::Error,
    });
    lint_result.push(LineResult {
        line: 2,
        col: 1,
        new: "这是第 2 行".to_owned(),
        old: "这是第2行".to_owned(),
        severity: Severity::Error,
    });

    vec![lint_result]
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_lint_results_json() {
        let json = super::to_lint_results_json(crate::result::json::crate_test_lint_results());

        let expected = r#"{"count": 1,"messages": [{"filepath":"./test/foo/bar.rs","lines":[{"l":1,"c":1,"new":"hello 你好。","old":"hello你好.","severity":1},{"l":2,"c":1,"new":"这是第 2 行","old":"这是第2行","severity":1}],"error":""}]}"#;
        if expected != json {
            println!("--------------- json:\n{}", json);
        }
        assert_json_eq!(expected, json);
    }
}
