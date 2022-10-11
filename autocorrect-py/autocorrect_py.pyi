from typing import List


class Severity:
  Pass = 0
  Error = 1
  Warning = 2


class LineResult:
  line: int
  col: int
  new: str
  old: str
  severity: Severity


class LintResult:
    raw: str
    filepath: str
    lines: List[LineResult]
    enable: bool

# Automatically add spaces between Chinese and English words.
#
# This method only work for plain text.
def format(text: str) -> str: ...

# Format a file content with filetype.
def format_for(raw: str, filename_or_ext: str) -> str: ...

# Lint a file content with filetype.
def lint_for(raw: str, filename_or_ext: str) -> LintResult: ...
