use std::{collections::HashMap, fs, io, path::Path, sync::Mutex};

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_repr::*;

pub fn merge(config_file: &str) -> Result<Config, io::Error> {
    let config_str = fs::read_to_string(Path::new(config_file))?;

    let config: Config = serde_json::from_str(&config_str)?;

    let new_config: Config = CONFIG.lock().unwrap().merge(&config)?;

    Ok(new_config)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub spellcheck: SpellcheckConfig,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum SpellcheckMode {
    Disabled = 0,
    Enabled = 1,
    LintOnly = 2,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SpellcheckConfig {
    #[serde(default)]
    pub mode: Option<SpellcheckMode>,
    #[serde(default)]
    pub words: Vec<SpellcheckWord>,
    #[serde(skip)]
    pub dict: HashMap<String, String>,
    #[serde(skip)]
    pub dict_re: HashMap<String, Regex>,
}

type SpellcheckWord = String;

include!(concat!(env!("OUT_DIR"), "/default_config.rs"));

lazy_static! {
    pub static ref PAIR_RE: regex::Regex = regex::Regex::new(r"\s*=\s*").unwrap();
}

impl Default for Config {
    fn default() -> Self {
        Config {
            spellcheck: SpellcheckConfig {
                mode: None,
                words: vec![],
                dict: HashMap::new(),
                dict_re: HashMap::new(),
            },
        }
    }
}

impl Config {
    pub fn from_str(s: &str) -> Result<Self, serde_any::Error> {
        let mut config: Config = serde_any::from_str_any(s)?;

        config.prepare();

        Ok(config)
    }

    pub fn prepare(&mut self) {
        if let Some(spellcheck_mode) = self.spellcheck.mode.clone() {
            if spellcheck_mode != SpellcheckMode::Disabled && !self.spellcheck.words.is_empty() {
                let mut lines = self.spellcheck.words.clone();

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

                    self.spellcheck
                        .dict
                        .insert(left_str.to_string(), right_str.to_string());
                    self.spellcheck.dict_re.insert(
                        left_str.to_string(),
                        regexp!(
                            r"(?im)(\s|^)+({})(\s|$)+",
                            left_str.replace('-', r"\-").replace('.', r"\.")
                        ),
                    );
                }
            }
        }
    }

    pub fn merge(&mut self, config: &Config) -> Result<Config, serde_json::Error> {
        if let Some(mode) = config.spellcheck.mode.clone() {
            self.spellcheck.mode = Some(mode);
        }

        self.spellcheck.words = self
            .spellcheck
            .words
            .iter()
            .chain(config.spellcheck.words.iter())
            .cloned()
            .collect();

        self.prepare();

        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = &CONFIG.lock().unwrap();

        assert_eq!(config.spellcheck.mode, Some(SpellcheckMode::Enabled));
        assert_eq!(false, config.spellcheck.words.is_empty());
        assert_eq!(false, config.spellcheck.dict.is_empty());
    }
}
