//! Rdjson format for reviewdog
//! https://github.com/reviewdog/reviewdog/tree/master/proto/rdf
use super::LintResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct RdfJson {
    source: RdfSource,
    severity: String,
    diagnostics: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfSource {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfDiagnostic {
    message: String,
    severity: String,
    code: RdfCode,
    location: RdfLocation,
    suggestions: Vec<RdfSuggetion>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfLocation {
    path: String,
    range: RdfRange,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfRange {
    start: Option<RdfLineColumn>,
    end: Option<RdfLineColumn>,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfLineColumn {
    line: usize,
    column: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfSuggetion {
    text: String,
    range: RdfRange,
}

#[derive(Serialize, Deserialize, Clone)]
struct RdfCode {
    value: Option<String>,
    url: String,
}

fn to_severity_str(severity: super::Severity) -> String {
    match severity {
        super::Severity::Error => "ERROR".to_owned(),
        super::Severity::Warning => "WARNING".to_owned(),
        super::Severity::Pass => "PASS".to_owned(),
    }
}

/// RDF JSONSchema
/// https://github.com/reviewdog/reviewdog/blob/master/proto/rdf/jsonschema/Diagnostic.jsonschema
#[doc(hidden)]
pub(crate) fn to_rdjson_diagnostic(lint_result: &LintResult, pretty: bool) -> String {
    let range = RdfRange {
        start: Some(RdfLineColumn {
            line: lint_result.line,
            column: lint_result.col,
        }),
        end: None,
    };

    let mut rdf_diagnostic: RdfDiagnostic = RdfDiagnostic {
        message: "".to_owned(),
        location: RdfLocation {
            path: lint_result.filepath.clone(),
            range: range.clone(),
        },
        severity: "UNKNOWN_SEVERITY".to_owned(),
        code: RdfCode {
            value: Some("AutoCorrect".to_owned()),
            url: "https://github.com/huacnlee/autocorrect".to_owned(),
        },
        suggestions: vec![],
    };

    lint_result.lines.iter().for_each(|line_result| {
        if rdf_diagnostic.severity == "UNKNOWN_SEVERITY" {
            rdf_diagnostic.severity = to_severity_str(line_result.severity);
        }

        let suggestion = RdfSuggetion {
            text: line_result.new.clone(),
            range: RdfRange {
                start: Some(RdfLineColumn {
                    line: line_result.line,
                    column: line_result.col,
                }),
                end: Some(RdfLineColumn {
                    line: line_result.line + line_result.old.split("\n").count() - 1,
                    column: line_result.col
                        + line_result
                            .old
                            .split("\n")
                            .last()
                            .unwrap_or("")
                            .chars()
                            .count(),
                }),
            },
        };

        rdf_diagnostic.suggestions.push(suggestion);
    });

    if pretty {
        serde_json::to_string_pretty(&rdf_diagnostic).unwrap()
    } else {
        serde_json::to_string(&rdf_diagnostic).unwrap()
    }
}

#[doc(hidden)]
pub fn to_lint_results_rdjson(lint_results: Vec<LintResult>) -> String {
    let diagnostics = lint_results
        .iter()
        .map(|r| to_rdjson_diagnostic(r, false))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        r#"{{"source":{{"name":"AutoCorrect Lint","url": "https://github.com/huacnlee/autocorrect"}},"diagnostics": [{diagnostics}]}}"#,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_lint_results_rdjson() {
        let rdjson = super::to_lint_results_rdjson(crate::result::json::crate_test_lint_results());

        let expected = r#"{"source":{"name":"AutoCorrect Lint","url": "https://github.com/huacnlee/autocorrect"},"diagnostics": [{"message":"","severity":"ERROR","code":{"value":"AutoCorrect","url":"https://github.com/huacnlee/autocorrect"},"location":{"path":"test/foo/bar.rs","range":{"start":{"line":1,"column":1},"end":null}},"suggestions":[{"text":"hello 你好。","range":{"start":{"line":1,"column":1},"end":{"line":1,"column":9}}},{"text":"这是第 2 行","range":{"start":{"line":2,"column":1},"end":{"line":2,"column":6}}}]}]}"#;
        if expected != rdjson {
            println!("--------------- rdjson:\n{}", rdjson);
        }
        assert_json_eq!(expected, rdjson);
    }
}
