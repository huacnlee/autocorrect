use autocorrect::{format_for, lint_for};
use pretty_assertions::assert_eq;

#[test]
fn test_format_jupyter() {
    let raw = include_str!("./fixtures/jupyter.ipynb");
    let expected = include_str!("./fixtures/jupyter.expected.txt");

    let result = format_for(raw, "jupyter.ipynb");
    assert_eq!(false, result.has_error());
    assert_eq!(expected, result.out)
}

#[test]
fn test_lint_jupyter() {
    let raw = include_str!("./fixtures/jupyter.ipynb");
    let expected = include_str!("./fixtures/jupyter.expected.json");

    let result = lint_for(raw, "jupyter.ipynb");
    assert_eq!(expected.trim(), result.to_json_pretty().trim())
}
