use super::severity::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref PAIR_RE: regex::Regex = regex::Regex::new(r"\s*=\s*").unwrap();
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SpellcheckConfig {
    #[serde(default)]
    /// DEPRACTED: use `rules.spellcheck` instead
    pub mode: Option<SeverityMode>,
    #[serde(default)]
    pub words: Vec<SpellcheckWord>,
    #[serde(skip)]
    pub dict: HashMap<String, String>,
    #[serde(skip)]
    pub dict_re: HashMap<String, Regex>,
}

impl SpellcheckConfig {
    pub fn prepare(&mut self) {
        if !self.words.is_empty() {
            let mut lines = self.words.clone();

            // Sort: Longger first, then alphabetically.
            lines.sort_by(|a, b| {
                let mut a = a.as_str();
                let mut b = b.as_str();

                let pair_a = PAIR_RE.split(a).collect::<Vec<_>>();
                if pair_a.len() == 2 {
                    a = pair_a[0];
                }
                let pair_b = PAIR_RE.split(b).collect::<Vec<_>>();
                if pair_b.len() == 2 {
                    b = pair_b[0];
                }
                a = a.trim();
                b = b.trim();

                b.len().cmp(&a.len()).then(a.cmp(b))
            });

            for l in lines.iter().filter(|l| !l.trim().is_empty()) {
                let mut left_str = l.as_str();
                let mut right_str = l.as_str();

                let pair = PAIR_RE.split(l).collect::<Vec<_>>();
                if pair.len() == 2 {
                    left_str = pair[0];
                    right_str = pair[1];
                }

                left_str = left_str.trim();
                right_str = right_str.trim();

                self.dict
                    .insert(left_str.to_string(), right_str.to_string());
                self.dict_re.insert(
                    left_str.to_string(),
                    crate::rule::spellcheck::word_regexp(left_str),
                );
            }
        }
    }
}

type SpellcheckWord = String;
