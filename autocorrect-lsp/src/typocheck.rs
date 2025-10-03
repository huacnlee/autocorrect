use std::sync::LazyLock;

use ropey::Rope;
use tower_lsp::lsp_types::{self, Diagnostic};
use typos::Status;

use crate::DIAGNOSTIC_SOURCE_SPELLCHECK;

static POLICY: LazyLock<typos_cli::policy::Policy> = LazyLock::new(|| {
    let policy = typos_cli::policy::Policy::new();
    policy
});

// hallo worl你好
pub(crate) fn check_typos(text: &str) -> Vec<Diagnostic> {
    let rope = Rope::from_str(text);
    let results = typos::check_str(text, &POLICY.tokenizer, POLICY.dict);

    let mut diagnostics = Vec::new();
    for typo in results {
        let offset = typo.byte_offset;
        let line = rope.byte_to_line(offset);
        let character = offset - rope.line_to_byte(line);

        let start_pos = lsp_types::Position {
            line: line as u32,
            character: character as u32,
        };
        let end_pos = lsp_types::Position {
            line: line as u32,
            character: (character + typo.typo.len()) as u32,
        };
        let range = lsp_types::Range {
            start: start_pos,
            end: end_pos,
        };

        match typo.corrections {
            Status::Corrections(corrections) => {
                let data = corrections
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();

                let diagnostic = Diagnostic {
                    range,
                    severity: Some(lsp_types::DiagnosticSeverity::WARNING),
                    code: None,
                    code_description: None,
                    source: Some(DIAGNOSTIC_SOURCE_SPELLCHECK.to_string()),
                    message: format!(
                        "Possible typo: '{}', suggestions: {}",
                        typo.typo,
                        corrections.join(", ")
                    ),
                    related_information: None,
                    tags: None,
                    data: Some(serde_json::json!(data)),
                };

                diagnostics.push(diagnostic);
            }
            _ => {}
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::Position;

    use super::*;

    #[test]
    fn test_check_typos() {
        let text = "This is 你好 a smaple text with a typo.\nAnother line without typos.\nThis line has a eror.";
        let diagnostics = check_typos(text);
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(
            diagnostics[0].message,
            "Possible typo: 'smaple', suggestions: sample"
        );
        assert_eq!(diagnostics[0].range.start, Position::new(0, 17));
        assert_eq!(
            diagnostics[1].message,
            "Possible typo: 'eror', suggestions: error"
        );
        assert_eq!(diagnostics[1].range.start, Position::new(2, 16));
    }
}
