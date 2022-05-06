use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};

include!(concat!(env!("OUT_DIR"), "/default_config.rs"));

lazy_static! {
    pub static ref PAIR_RE: regex::Regex = regex::Regex::new(r"\s*=\s*").unwrap();
}

pub fn load_file(config_file: &str) -> Result<Config, Error> {
    let config_path = Path::new(config_file);
    if !Path::exists(config_path) {
        return Ok(Config::default());
    }

    let config_str = fs::read_to_string(Path::new(config_file))?;

    load(&config_str)
}

pub fn load(config_str: &str) -> Result<Config, Error> {
    let config: Config = Config::from_str(config_str)?;

    let new_config: Config = CURRENT_CONFIG.write().unwrap().merge(&config)?;

    Ok(new_config)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    #[serde(default)]
    pub spellcheck: SpellcheckConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
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

impl SpellcheckConfig {
    pub fn is_disabled(&self) -> bool {
        self.mode == Some(SpellcheckMode::Disabled) || self.mode.is_none()
    }
}

type SpellcheckWord = String;

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
#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Error {
        Error {
            message: err.to_string(),
        }
    }
}

impl From<serde_any::Error> for Error {
    fn from(err: serde_any::Error) -> Error {
        Error {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            message: err.to_string(),
        }
    }
}

impl Config {
    pub fn current() -> Arc<RwLockReadGuard<'static, Config>> {
        Arc::new(CURRENT_CONFIG.read().unwrap())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let mut config: Config = serde_any::from_str_any(s)?;

        config.prepare();

        Ok(config)
    }

    pub fn prepare(&mut self) {
        if !self.spellcheck.words.is_empty() {
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
                    crate::spellcheck::word_regexp(left_str),
                );
            }
        }
    }

    pub fn merge(&mut self, config: &Config) -> Result<Config, Error> {
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

// Setup config for test for load tests/.autocorrectrc.test
static STEUP_ONCE: std::sync::Once = std::sync::Once::new();

#[allow(unused)]
pub(crate) fn setup_test() {
    STEUP_ONCE.call_once(|| {
        let config_str = include_str!("../tests/.autocorrectrc.test").to_owned();
        crate::config::load(&config_str).unwrap();
    })
}

#[derive(PartialEq, Clone, Debug)]
pub enum SpellcheckMode {
    Disabled,
    Enabled,
    LintOnly,
}

impl<'a> Serialize for SpellcheckMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SpellcheckMode::Disabled => serializer.serialize_u8(0),
            SpellcheckMode::Enabled => serializer.serialize_u8(1),
            SpellcheckMode::LintOnly => serializer.serialize_u8(2),
        }
    }
}

impl<'a> Deserialize<'a> for SpellcheckMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        struct SpellcheckModeVisitor;

        impl<'de> serde::de::Visitor<'de> for SpellcheckModeVisitor {
            type Value = SpellcheckMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a Foo")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<SpellcheckMode, E> {
                Ok(match s {
                    "0" => SpellcheckMode::Disabled,
                    "1" => SpellcheckMode::Enabled,
                    "2" => SpellcheckMode::LintOnly,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                })
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<SpellcheckMode, E> {
                Ok(match n {
                    0 => SpellcheckMode::Disabled,
                    1 => SpellcheckMode::Enabled,
                    2 => SpellcheckMode::LintOnly,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Unsigned(n), &self)),
                })
            }
        }

        match deserializer.deserialize_any(SpellcheckModeVisitor) {
            Ok(value) => Ok(value),
            Err(_) => Ok(SpellcheckMode::Disabled),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spellcheck_parse_json() {
        let mut config =
            Config::from_str(r#"{ "spellcheck": { "mode": 0, "words": ["Foo", "Bar"] } }"#)
                .unwrap();
        assert_eq!(Some(SpellcheckMode::Disabled), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);

        config = Config::from_str(r#"{ "spellcheck": { } }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": 1 } }"#).unwrap();
        assert_eq!(Some(SpellcheckMode::Enabled), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": 2 } }"#).unwrap();
        assert_eq!(Some(SpellcheckMode::LintOnly), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "0" } }"#).unwrap();
        assert_eq!(Some(SpellcheckMode::Disabled), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "1" } }"#).unwrap();
        assert_eq!(Some(SpellcheckMode::Enabled), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "2" } }"#).unwrap();
        assert_eq!(Some(SpellcheckMode::LintOnly), config.spellcheck.mode);

        config = Config::from_str(r#"{ }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "words" : ["Hello"] } }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);
        assert_eq!(vec!["Hello"], config.spellcheck.words);
    }

    #[test]
    fn test_spellcheck_parse_yaml() {
        let mut config = Config::from_str("spellcheck:\n  mode: 0").unwrap();
        assert_eq!(Some(SpellcheckMode::Disabled), config.spellcheck.mode);

        config =
            Config::from_str("spellcheck:\n  mode: 1\n  words:\n    - Foo\n    - Bar").unwrap();
        assert_eq!(Some(SpellcheckMode::Enabled), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);

        config = Config::from_str("").unwrap();
        assert_eq!(None, config.spellcheck.mode);
        assert_eq!(Vec::<String>::new(), config.spellcheck.words);
    }

    #[test]
    fn test_spellcheck_is_disabled() {
        let mut config = Config::default();
        assert!(config.spellcheck.is_disabled());
        config.spellcheck.mode = Some(SpellcheckMode::Disabled);
        assert!(config.spellcheck.is_disabled());
        config.spellcheck.mode = Some(SpellcheckMode::Enabled);
        assert!(!config.spellcheck.is_disabled());
        config.spellcheck.mode = Some(SpellcheckMode::LintOnly);
        assert!(!config.spellcheck.is_disabled());
    }

    #[test]
    fn test_default_config() {
        let config = Config::current();

        assert_eq!(Some(SpellcheckMode::Enabled), config.spellcheck.mode);
        assert_eq!(false, config.spellcheck.words.is_empty());
        assert_eq!(false, config.spellcheck.dict.is_empty());
    }

    #[test]
    fn test_merge_config() {
        let mut config = Config::default();
        config.spellcheck.mode = Some(SpellcheckMode::LintOnly);
        config.spellcheck.words = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

        let mut config1 = Config::default();
        config1.spellcheck.mode = Some(SpellcheckMode::Disabled);
        config1.spellcheck.words = vec!["foo1".to_string(), "bar1".to_string()];
        config.merge(&config1).unwrap();

        assert_eq!(config.spellcheck.mode, Some(SpellcheckMode::Disabled));
        assert_eq!(
            config.spellcheck.words,
            vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
                "foo1".to_string(),
                "bar1".to_string()
            ]
        );
    }
}
