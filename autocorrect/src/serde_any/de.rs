use super::Error;
use serde::Deserialize;

/// Serialization or deserialization formats
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Yaml,
}

static SUPPORT_FORMATS: &[Format] = &[Format::Yaml, Format::Json];

fn from_str<T>(s: &str, format: Format) -> Result<T, Error>
where
    T: std::default::Default + for<'de> Deserialize<'de>,
{
    match format {
        Format::Yaml => Ok(serde_yaml::from_str::<T>(s)?),
        Format::Json => Ok(serde_json::from_str::<T>(s)?),
    }
}

/// Parse a string (YAML, JSON) into a value of type `T`
pub fn from_str_any<T>(s: &str) -> Result<T, Error>
where
    T: std::default::Default + for<'de> Deserialize<'de>,
{
    let mut errors = Vec::new();

    for format in SUPPORT_FORMATS {
        match from_str(s, *format) {
            Ok(t) => return Ok(t),
            Err(err) => errors.push((*format, err)),
        }
    }

    Ok(T::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Default)]
    struct Test {
        a: String,
        b: String,
    }

    #[test]
    fn test_from_str_any() {
        let json = r#"
        {
            "a": "Hello",
            "b": "World"
        }
        "#;

        let yaml = r#"
        a: Hello
        # This is comment
        b: World
        "#;

        let t = from_str_any::<Test>(json).unwrap();
        assert_eq!(t.a, "Hello");
        assert_eq!(t.b, "World");

        let t = from_str_any::<Test>(yaml).unwrap();
        assert_eq!(t.a, "Hello");
        assert_eq!(t.b, "World");
    }
}
