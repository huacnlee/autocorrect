mod severity;
mod spellcheck;

pub use severity::*;
pub use spellcheck::*;

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, RwLock, RwLockReadGuard},
    vec,
};

use crate::serde_any;

include!(concat!(env!("OUT_DIR"), "/default_config.rs"));

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    #[serde(default)]
    pub spellcheck: SpellcheckConfig,
    #[serde(default)]
    pub rules: HashMap<String, SeverityMode>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rules: HashMap::new(),
            spellcheck: SpellcheckConfig {
                mode: None,
                words: vec![],
                dict: HashMap::new(),
                dict_re: HashMap::new(),
            },
        }
    }
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
            message: format!("{:?}", err),
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

impl From<std::string::String> for Error {
    fn from(err: std::string::String) -> Error {
        Error { message: err }
    }
}

impl Config {
    pub fn current() -> Arc<RwLockReadGuard<'static, Config>> {
        Arc::new(CURRENT_CONFIG.read().unwrap())
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, Error> {
        let mut config: Config = match serde_any::from_str_any(s) {
            Ok(config) => config,
            Err(err) => return Err(format!("Config::from_str parse error: {:?}", err).into()),
        };

        config.prepare();

        Ok(config)
    }

    pub fn prepare(&mut self) {
        self.spellcheck.prepare();
    }

    pub fn merge(&mut self, config: &Config) -> Result<Config, Error> {
        for (k, v) in config.rules.clone() {
            self.rules.insert(k, v);
        }

        // DEPRECATED: since 2.0.0, remove this in 2.1.0
        if let Some(mode) = config.spellcheck.mode.clone() {
            println!("DEPRECATED: `spellcheck.mode` use `rules.spellcheck` instead since 2.0.0, remove this in 2.1.0");
            self.spellcheck.mode = Some(mode.clone());
            self.rules.insert("spellcheck".to_string(), mode);
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
        let config_str = include_str!("../../tests/.autocorrectrc.test").to_owned();
        crate::config::load(&config_str).unwrap();
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let mut config =
            Config::from_str(r#"{ "rules": { "foo": 1, "bar": "off", "dar": "2" }, "spellcheck": { "mode": 0, "words": ["Foo", "Bar"] } }"#)
                .unwrap();

        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        assert_eq!(Some(SeverityMode::Off), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);
        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        config = Config::from_str(r#"{ "spellcheck": { } }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": 1 } }"#).unwrap();
        assert_eq!(Some(SeverityMode::Error), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": 2 } }"#).unwrap();
        assert_eq!(Some(SeverityMode::Warning), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "0" } }"#).unwrap();
        assert_eq!(Some(SeverityMode::Off), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "1" } }"#).unwrap();
        assert_eq!(Some(SeverityMode::Error), config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "mode": "2" } }"#).unwrap();
        assert_eq!(Some(SeverityMode::Warning), config.spellcheck.mode);

        config = Config::from_str(r#"{ }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);

        config = Config::from_str(r#"{ "spellcheck": { "words" : ["Hello"] } }"#).unwrap();
        assert_eq!(None, config.spellcheck.mode);
        assert_eq!(vec!["Hello"], config.spellcheck.words);
    }

    #[test]
    fn test_spellcheck_parse_yaml() {
        let mut config = Config::from_str("spellcheck:\n  mode: 0").unwrap();
        assert_eq!(Some(SeverityMode::Off), config.spellcheck.mode);

        config =
            Config::from_str("rules:\n  foo: '1'\n  bar: off\n  dar: warning\nspellcheck:\n  mode: 1\n  words:\n    - Foo\n    - Bar").unwrap();

        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        assert_eq!(Some(SeverityMode::Error), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);

        config = Config::from_str("").unwrap();
        assert_eq!(None, config.spellcheck.mode);
        assert_eq!(Vec::<String>::new(), config.spellcheck.words);
    }

    #[test]
    fn test_current_config_with_default_config_file() {
        let config = Config::current();

        let mut keys: Vec<String> = config.rules.keys().cloned().collect();
        keys.sort();
        let mut rule_names: Vec<String> = crate::rule::default_rule_names();
        rule_names.sort();
        assert_eq!(rule_names, keys);

        for (k, v) in config.rules.clone() {
            if k == "spellcheck" {
                assert_eq!(SeverityMode::Warning, v);
            } else {
                assert_eq!(SeverityMode::Error, v);
            }
        }

        assert_eq!(None, config.spellcheck.mode);
        assert_eq!(false, config.spellcheck.words.is_empty());
        assert_eq!(false, config.spellcheck.dict.is_empty());
    }

    #[test]
    fn test_merge_config() {
        let mut config = Config::default();
        config.spellcheck.mode = Some(SeverityMode::Warning);
        config.spellcheck.words = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];

        let mut config1 = Config::default();
        config1.spellcheck.mode = Some(SeverityMode::Off);
        config1.spellcheck.words = vec!["foo1".to_string(), "bar1".to_string()];
        config.merge(&config1).unwrap();

        assert_eq!(config.spellcheck.mode, Some(SeverityMode::Off));
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
