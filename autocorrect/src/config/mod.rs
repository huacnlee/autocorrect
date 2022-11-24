mod severity;
mod spellcheck;
pub mod toggle;

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

pub trait ConfigFileTypes {
    fn get_ext(&self, ext: &str) -> Option<&str>;
}

impl ConfigFileTypes for HashMap<String, String> {
    fn get_ext(&self, ext: &str) -> Option<&str> {
        if let Some(value) = self.get(ext) {
            return Some(value);
        }

        if let Some(value) = self.get(&format!("*.{}", ext)) {
            return Some(value);
        }

        if let Some(value) = self.get(&format!(".{}", ext)) {
            return Some(value);
        }

        None
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default)]
    pub spellcheck: SpellcheckConfig,
    #[serde(default)]
    pub rules: HashMap<String, SeverityMode>,
    // Speical text to ignore
    #[serde(default)]
    pub text_rules: HashMap<String, SeverityMode>,
    // Addition file types map, high priority than default
    #[serde(default)]
    pub file_types: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            rules: HashMap::new(),
            text_rules: HashMap::new(),
            spellcheck: SpellcheckConfig {
                mode: None,
                words: vec![],
                dict: HashMap::new(),
                dict_re: HashMap::new(),
            },
            file_types: HashMap::new(),
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

    pub fn get_file_type(&self, ext: &str) -> Option<&str> {
        self.file_types.get_ext(ext)
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

        // DEPRECATED: since 2.0.0, remove this in 3.0.0
        if let Some(mode) = config.spellcheck.mode.clone() {
            println!("DEPRECATED: `spellcheck.mode` use `rules.spellcheck` instead since 2.0.0");
            self.spellcheck.mode = Some(mode.clone());
            self.rules.insert("spellcheck".to_string(), mode);
        }
        config.text_rules.iter().for_each(|(k, v)| {
            self.text_rules.insert(k.to_owned(), v.to_owned());
        });

        config.file_types.iter().for_each(|(k, v)| {
            self.file_types.insert(k.to_owned(), v.to_owned());
        });

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
    use indoc::indoc;

    #[test]
    fn test_parse_json() {
        let json_str = indoc! {r#"
        {
            "rules": {
                "foo": 1,
                "bar": "off",
                "dar": "2"
            },
            "textRules": {
                "hello": 1,
                "word": 2
            },
            "spellcheck": {
                "mode": 0,
                "words": [
                "Foo",
                "Bar"
                ]
            },
            "fileTypes": {
                "md": "markdown",
                "md1": "markdown",
                "*.ascii": "asciidoc",
                "Gemfile": "ruby"
            }
        }
        "#};
        let mut config = Config::from_str(json_str).unwrap();

        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        assert_eq!(Some(SeverityMode::Off), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);
        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        assert_eq!(Some(&SeverityMode::Error), config.text_rules.get("hello"));
        assert_eq!(Some(&SeverityMode::Warning), config.text_rules.get("word"));

        assert_eq!(Some(&"ruby".into()), config.file_types.get("Gemfile"));
        assert_eq!(Some(&"markdown".into()), config.file_types.get("md"));
        assert_eq!(Some(&"markdown".into()), config.file_types.get("md1"));
        assert_eq!(Some(&"asciidoc".into()), config.file_types.get("*.ascii"));
        assert_eq!(None, config.file_types.get("foo"));

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

        let yaml_str = indoc! {r#"
        rules:
          foo: '1'
          bar: off
          dar: warning
        textRules:
          hello: error
          word: '0'
        spellcheck:
          mode: 1
          words:
            - Foo
            - Bar
        fileTypes:
          Foo: foo
        "#};

        config = Config::from_str(yaml_str).unwrap();

        assert_eq!(Some(&SeverityMode::Error), config.rules.get("foo"));
        assert_eq!(Some(&SeverityMode::Off), config.rules.get("bar"));
        assert_eq!(Some(&SeverityMode::Warning), config.rules.get("dar"));

        assert_eq!(Some(&SeverityMode::Error), config.text_rules.get("hello"));
        assert_eq!(Some(&SeverityMode::Off), config.text_rules.get("word"));

        assert_eq!(Some(SeverityMode::Error), config.spellcheck.mode);
        assert_eq!(vec!["Foo", "Bar"], config.spellcheck.words);

        assert_eq!(Some(&"foo".to_owned()), config.file_types.get("Foo"));

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
        assert!(!config.spellcheck.words.is_empty());
        assert!(!config.spellcheck.dict.is_empty());
    }

    #[test]
    fn test_merge_config() {
        let mut config = Config {
            rules: map! {
                "foo".to_owned() => SeverityMode::Error,
            },
            text_rules: map! {
                "a".to_owned() => SeverityMode::Off,
                "hello".to_owned() => SeverityMode::Error
            },
            file_types: map! {
                "a".to_owned() => "A".to_owned(),
                "foo".to_owned() => "Foo".to_owned()
            },
            spellcheck: SpellcheckConfig {
                mode: Some(SeverityMode::Warning),
                words: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                ..Default::default()
            },
        };

        let config1 = Config {
            rules: map! {
                "bar".to_owned() => SeverityMode::Warning,
            },
            text_rules: map! {
                "world".to_owned() => SeverityMode::Off
            },
            file_types: map! {
                "foo".to_owned() => "Foo New".to_owned(),
                "bar".to_owned() => "Bar".to_owned()
            },
            spellcheck: SpellcheckConfig {
                mode: Some(SeverityMode::Off),
                words: vec!["foo1".to_string(), "bar1".to_string()],
                ..Default::default()
            },
        };

        config.merge(&config1).unwrap();

        let new_rules = map! {
            "spellcheck".to_owned() => SeverityMode::Off,
            "foo".to_owned() => SeverityMode::Error,
            "bar".to_owned() => SeverityMode::Warning
        };
        assert_eq!(new_rules, config.rules);

        let new_text_rules = map! {
            "a".to_owned() => SeverityMode::Off,
            "hello".to_owned() => SeverityMode::Error,
            "world".to_owned() => SeverityMode::Off
        };
        assert_eq!(new_text_rules, config.text_rules);

        let new_file_types = map! {
            "a".to_owned() => "A".to_owned(),
            "foo".to_owned() => "Foo New".to_owned(),
            "bar".to_owned() => "Bar".to_owned(),
        };
        assert_eq!(new_file_types, config.file_types);

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

    #[test]
    fn test_file_types_get_ext() {
        let config = Config {
            file_types: map! {
                "*.rs".to_owned() => "rust".to_owned(),
                ".asc".to_owned() => "asciidoc".to_owned(),
                "rb".to_owned() => "ruby".to_owned(),
                "Gemfile".to_owned() => "ruby".to_owned(),
            },
            ..Default::default()
        };

        assert_eq!(Some("rust"), config.file_types.get_ext("rs"));
        assert_eq!(Some("asciidoc"), config.file_types.get_ext("asc"));
        assert_eq!(Some("ruby"), config.file_types.get_ext("rb"));
        assert_eq!(Some("ruby"), config.file_types.get_ext("Gemfile"));
        assert_eq!(None, config.file_types.get_ext("foo"));
    }
}
