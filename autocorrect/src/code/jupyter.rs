// autocorrect: false

use pest::Parser as P;
use pest_derive::Parser;
use serde::Serialize;

use crate::{FormatResult, LineResult, LintResult};

#[derive(Parser)]
#[grammar = "../grammar/jupyter.pest"]
struct JupyterParser;

#[derive(Debug, Clone, Serialize, Default)]
struct NotebookCell<'a> {
    cell_type: &'a str,
    sources: Option<Vec<Source<'a>>>,
}

#[derive(Debug, Default, Clone, Serialize)]
struct Source<'a> {
    input: &'a str,
    start: usize,
    end: usize,
    line_col: (usize, usize),
}

impl<'a> NotebookCell<'a> {
    fn push_source(&mut self, ource: Source<'a>) {
        match self.sources {
            Some(ref mut sources) => sources.push(ource),
            None => {
                self.sources = Some(vec![ource]);
            }
        }
    }
}

pub fn format_jupyter(input: &str) -> FormatResult {
    let mut result = FormatResult::new(input);
    result.out = String::from(input);

    let cells = parse_jupyter(input);
    if let Err(e) = cells {
        result.error = e;
        return result;
    }

    for cell in cells.unwrap() {
        if let Some(sources) = cell.sources {
            if cell.cell_type == "markdown" || cell.cell_type == "md" {
                for source in sources {
                    let sub_result = crate::code::format_markdown(source.input);

                    if sub_result.out != source.input {
                        result.out = result.out.replace(source.input, &sub_result.out);
                    }
                }
            }
        }
    }

    result
}

pub fn lint_jupyter(input: &str) -> LintResult {
    let mut result = LintResult::new(input);

    let cells = parse_jupyter(input);
    if let Err(e) = cells {
        result.error = e;
        return result;
    }

    for cell in cells.unwrap() {
        if let Some(sources) = cell.sources {
            if cell.cell_type == "markdown" || cell.cell_type == "md" {
                for source in sources {
                    let sub_result = crate::code::lint_markdown(source.input);

                    for line in sub_result.lines {
                        result.lines.push(LineResult {
                            line: source.line_col.0 + line.line - 1,
                            col: source.line_col.1 + line.col - 1,
                            new: line.new,
                            old: line.old,
                            severity: line.severity,
                        });
                    }
                }
            }
        }
    }

    result
}

fn parse_jupyter(input: &str) -> Result<Vec<NotebookCell>, String> {
    // Get MarkedSource from pairs for get source when match cell_type is "markdown" in ast
    let mut cells: Vec<NotebookCell> = vec![];

    let ast = JupyterParser::parse(Rule::item, input);
    if let Err(e) = ast {
        return Err(e.to_string());
    }

    let pairs = ast.unwrap();

    let mut iter = pairs.flatten();
    let mut cell = NotebookCell::default();

    while let Some(pair) = iter.next() {
        if let Rule::key = pair.as_rule() {
            match pair.as_str() {
                "cell_type" => {
                    // pair > key > string
                    let text = iter.next().unwrap().as_str();
                    cell.cell_type = text;

                    if !cell.cell_type.is_empty() && cell.sources.is_some() {
                        cells.push(cell);
                        cell = NotebookCell::default()
                    }
                }
                "source" => {
                    // pair > array > [value]
                    let array_pair = iter.next().unwrap();
                    if let Rule::array = array_pair.as_rule() {
                        let sub_iter = array_pair.clone().into_inner().peekable();
                        for sub_pair in sub_iter {
                            let span = sub_pair.as_span();
                            let source = Source {
                                input: span.as_str(),
                                start: span.start(),
                                end: span.end(),
                                line_col: sub_pair.line_col(),
                            };
                            cell.push_source(source);
                        }
                    }

                    if !cell.cell_type.is_empty() && cell.sources.is_some() {
                        cells.push(cell);
                        cell = NotebookCell::default()
                    }
                }
                _ => {}
            }
        }
    }

    Ok(cells)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_jupyter() {
        // inline doc, this includes bad json format
        let raw = indoc! { r###"
        {
          "cells": [
            {
              "cell_type": "markdown",
              "metadata": {},
              "source": [
                "# CHAPTER 1时间序列\n",
                "python有标准包用来表示时间和日期数据datetime, time, calendar这些模块经常被使用。"
              ]
            },
            {
              "cell_type": "code",
              "execution_count": 27,
              "metadata": {
                "collapsed": true
              },
              "outputs": [],
              "source": [
                "# 这里是comment注释",
                "import pandas as pd"
              ]
            },
            {
              "source": [
                "## Hello世界\n",
                "Fixed periods固定的时期,比如2007年的一月，或者2010年整整一年",
              ],
              "cell_type": "md",
            },
            {
              "cell_type": "md",
              "source": [
                "## Hello世界1\n",
                  "比如2007年的一月，或者2010年整整一年",
              ]
            },
          ]
        }
        "### };

        let expected = indoc! { r###"
        [
            {
                "cell_type": "markdown",
                "sources": [
                {
                    "input": "# CHAPTER 1时间序列\\n",
                    "start": 101,
                    "end": 126,
                    "line_col": [7,10]
                },
                {
                    "input": "python有标准包用来表示时间和日期数据datetime, time, calendar这些模块经常被使用。",
                    "start": 138,
                    "end": 243,
                    "line_col": [8,10]
                }
                ]
            },
            {
                "cell_type": "code",
                "sources": [
                {
                    "input": "# 这里是comment注释",
                    "start": 425,
                    "end": 449,
                    "line_col": [19,10]
                },
                {
                    "input": "import pandas as pd",
                    "start": 461,
                    "end": 480,
                    "line_col": [20,10]
                }
                ]
            },
            {
                "cell_type": "md",
                "sources": [
                {
                    "input": "## Hello世界\\n",
                    "start": 530,
                    "end": 546,
                    "line_col": [25,10]
                },
                {
                    "input": "Fixed periods固定的时期,比如2007年的一月，或者2010年整整一年",
                    "start": 558,
                    "end": 637,
                    "line_col": [26,10]
                }
                ]
            },
            {
              "cell_type": "md",
              "sources": [
              {
                  "input": "## Hello世界1\\n",
                  "start": 739,
                  "end": 756,
                  "line_col": [33,10]
              },
              {
                  "input": "比如2007年的一月，或者2010年整整一年",
                  "start": 770,
                  "end": 820,
                  "line_col": [34,12]
              }
              ]
          }
        ]
        "### };

        let cells = parse_jupyter(raw).unwrap();
        let cells_json = serde_json::to_string_pretty(&cells).unwrap();

        assert_json_eq!(expected, cells_json);
    }

    #[test]
    fn test_format() {
        let raw = include_str!("../../tests/fixtures/jupyter.sm.ipynb");

        let expected = indoc! { r###"
        {
          "cells": [
            {
              "cell_type": "markdown",
              "metadata": {},
              "source": [
                "# CHAPTER 1 时间序列\n",
                "python 有标准包用来表示时间和日期数据 datetime, time, calendar 这些模块经常被使用。"
              ]
            },
            {
              "cell_type": "code",
              "execution_count": 27,
              "metadata": {
                "collapsed": true
              },
              "outputs": [],
              "source": [
                "# 这里是comment注释",
                "import pandas as pd"
              ]
            },
            {
              "source": [
                "## Hello 世界\n",
                "Fixed periods 固定的时期，比如 2007 年的一月，或者 2010 年整整一年"
              ],
              "cell_type": "markdown"
            },
            {
              "cell_type": "markdown",
              "source": [
                "## Hello 世界 1\n",
                "比如 2007 年的一月，或者 2010 年整整一年"
              ]
            }
          ]
        }
        "### };

        let result = format_jupyter(raw);
        assert_eq!(expected.trim(), result.out.trim());

        let json = indoc! { r###"
        {
          "filepath": "",
          "lines": [
            {
              "l": 7,
              "c": 12,
              "new": "CHAPTER 1 时间序列\\n",
              "old": "CHAPTER 1时间序列\\n",
              "severity": 1
            },
            {
              "l": 8,
              "c": 10,
              "new": "python 有标准包用来表示时间和日期数据 datetime, time, calendar 这些模块经常被使用。",
              "old": "python有标准包用来表示时间和日期数据datetime, time, calendar这些模块经常被使用。",
              "severity": 1
            },
            {
              "l": 25,
              "c": 13,
              "new": "Hello 世界\\n",
              "old": "Hello世界\\n",
              "severity": 1
            },
            {
              "l": 26,
              "c": 10,
              "new": "Fixed periods 固定的时期，比如 2007 年的一月，或者 2010 年整整一年",
              "old": "Fixed periods固定的时期,比如2007年的一月，或者2010年整整一年",
              "severity": 1
            },
            {
              "l": 33,
              "c": 13,
              "new": "Hello 世界 1\\n",
              "old": "Hello世界1\\n",
              "severity": 1
            },
            {
              "l": 34,
              "c": 10,
              "new": "比如 2007 年的一月，或者 2010 年整整一年",
              "old": "比如2007年的一月，或者2010年整整一年",
              "severity": 1
            }
          ],
          "error": ""
        }
        "### };

        let result = lint_jupyter(raw);
        assert_eq!(json.trim(), result.to_json_pretty().trim());
    }
}
