import autocorrect_py as autocorrect
import pytest


@pytest.mark.parametrize(
    "text,expected",
    [
        ("Hello你好.", "Hello 你好。"),
    ],
)
def test_format(text, expected):
    output = autocorrect.format(text)
    assert output == expected


@pytest.mark.parametrize(
    "raw,filename,expected",
    [
        ("<h1>Hello你好.</h1>", "html", "<h1>Hello 你好。</h1>"),
    ],
)
def test_format_for(raw, filename, expected):
    output = autocorrect.format_for(raw, filename)
    assert output == expected


def test_lint_for():
    output = autocorrect.lint_for("<h1>这是 Heading 标题</h1>", "index.html")
    assert not output.lines

    output = autocorrect.lint_for("<h1>这是 Heading标题</h1>", "html")
    assert len(output.lines) == 1
    line = output.lines[0]
    assert line.line == 1
    assert line.col == 5
    assert line.new == "这是 Heading 标题"

def test_load_config():
    autocorrect.load_config('{ textRules: { "你好hello": 0 } }')
    assert autocorrect.format("Hello你好.") == "Hello 你好。"
    assert autocorrect.format("你好hello.") == "你好hello."

def test_ignorer():
    ignorer = autocorrect.Ignorer("../")
    assert ignorer.is_ignored("README.md") == True
    assert ignorer.is_ignored("src/lib.rs") == True
    assert ignorer.is_ignored("target/foo/bar") == True
    assert ignorer.is_ignored("Cagro.toml") == False
    assert ignorer.is_ignored("autocorrect-rb.gemspec") == False
