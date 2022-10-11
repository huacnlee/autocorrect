import pytest

import autocorrect_py as autocorrect


@pytest.mark.parametrize(
    "text,expected",
    [
        ("长桥 LongBridge App 下载", "长桥 LongBridge App 下载"),
        ("长桥LongBridge App下载", "长桥 LongBridge App 下载"),
    ],
)
def test_format(text, expected):
    output = autocorrect.format(text)
    assert output == expected


@pytest.mark.parametrize(
    "raw,filename,expected",
    [
        ("<h1>这是 Heading 标题</h1>", "html", "<h1>这是 Heading 标题</h1>"),
        ("<h1>这是 Heading标题</h1>", "html", "<h1>这是 Heading 标题</h1>"),
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
