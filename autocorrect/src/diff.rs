use crate::result::LineResult;
use owo_colors::AnsiColors::{Red, Yellow};
use owo_colors::OwoColorize;
use std::fmt::Write;

pub(crate) fn diff_line_result(line: &LineResult) -> String {
    let mut err_color = Red;
    if line.severity.is_warning() {
        err_color = Yellow;
    }

    diff_lines_with_err_color(line.old.trim(), line.new.trim(), err_color)
}

#[allow(dead_code)]
pub(crate) fn diff_lines(old_str: &str, new_str: &str) -> String {
    diff_lines_with_err_color(old_str, new_str, Red)
}

// Screenshot:
// https://raw.githubusercontent.com/johannhof/difference.rs/master/assets/github-style.png
// https://github.com/johannhof/difference.rs/blob/master/examples/github-style.rs
#[allow(unused_must_use)]
pub(crate) fn diff_lines_with_err_color(
    old_str: &str,
    new_str: &str,
    err_color: owo_colors::AnsiColors,
) -> String {
    let diffs = diff::lines(old_str, new_str);

    let mut out = String::new();

    for i in 0..diffs.len() {
        match diffs[i] {
            diff::Result::Both(x, _) => {
                writeln!(out, " {}", x);
            }
            // -
            diff::Result::Left(x) => {
                match diffs.get(i + 1) {
                    Some(diff::Result::Right(y)) => {
                        write!(out, "{}", "-".color(Red));
                        let sub_diffs = diff::chars(y, x);
                        for c in sub_diffs {
                            match c {
                                diff::Result::Both(z, _) => {
                                    write!(out, "{}", format!("{}", z).color(err_color));
                                }
                                diff::Result::Right(z) => {
                                    write!(out, "{}", format!("{}", z).on_color(err_color).black());
                                }
                                _ => (),
                            }
                        }
                        writeln!(out);
                    }
                    _ => {
                        writeln!(out, "{}", format!("-{}", x).color(err_color));
                    }
                };
            }
            // +
            diff::Result::Right(x) => {
                match diffs.get(i - 1) {
                    Some(diff::Result::Left(y)) => {
                        write!(out, "{}", "+".green());
                        let sub_diffs = diff::chars(y, x);

                        for c in sub_diffs {
                            match c {
                                diff::Result::Both(z, _) => {
                                    write!(out, "{}", format!("{}", z).green());
                                }
                                diff::Result::Right(z) => {
                                    write!(out, "{}", format!("{}", z).on_green().black());
                                }
                                _ => (),
                            }
                        }
                        writeln!(out);
                    }
                    _ => {
                        writeln!(out, "{}", format!("+{}", x).green());
                    }
                };
            }
        }
    }

    // leave a blank line between each diff
    writeln!(out);

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use owo_colors::AnsiColors::{Red, Yellow};
    use owo_colors::OwoColorize;

    #[test]
    fn test_color_on_ci() {
        println!("{}", "RED".red());
        println!("{}", "GREEN".green());
        println!("{}", "YELLOW".yellow());

        println!("{}", "RED bright".bright_red());
        println!("{}", "GREEN bright".bright_green());
        println!("{}", "YELLOW bright".bright_yellow());

        println!("{}", "RED BG".on_red());
        println!("{}", "GREEN BG".on_green());
        println!("{}", "YELLOW BG".on_yellow());

        println!("{}", "RED BG, BLACK".on_red().black());
        println!("{}", "GREEN BG, BLACK".on_green().black());
        println!("{}", "YELLOW BG, BLACK".on_yellow().black());

        println!("{}", "RED BG (Ansi)".on_color(Red));
        println!("{}", "GREEN BG (Ansi)".on_color(Yellow));

        println!("{}", "RED BG (Ansi), Black FG".on_color(Red).black());
        println!("{}", "GREEN BG (Ansi), Black FG".on_color(Yellow).black());

        assert!(false)
    }
}
