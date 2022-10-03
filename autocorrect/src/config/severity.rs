use serde::{Deserialize, Serialize, Serializer};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum SeverityMode {
    Off = 0,
    Error = 1,
    Warning = 2,
}

impl Serialize for SeverityMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SeverityMode::Off => serializer.serialize_u8(0),
            SeverityMode::Error => serializer.serialize_u8(1),
            SeverityMode::Warning => serializer.serialize_u8(2),
        }
    }
}

impl<'a> Deserialize<'a> for SeverityMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'a>,
    {
        struct SeverityModeVisitor;

        impl<'de> serde::de::Visitor<'de> for SeverityModeVisitor {
            type Value = SeverityMode;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or string representing a Foo")
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<SeverityMode, E> {
                Ok(match s {
                    "0" => SeverityMode::Off,
                    "1" => SeverityMode::Error,
                    "2" => SeverityMode::Warning,
                    "off" => SeverityMode::Off,
                    "error" => SeverityMode::Error,
                    "warning" => SeverityMode::Warning,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Str(s), &self)),
                })
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<SeverityMode, E> {
                Ok(match n {
                    0 => SeverityMode::Off,
                    1 => SeverityMode::Error,
                    2 => SeverityMode::Warning,
                    _ => return Err(E::invalid_value(serde::de::Unexpected::Unsigned(n), &self)),
                })
            }
        }

        match deserializer.deserialize_any(SeverityModeVisitor) {
            Ok(value) => Ok(value),
            Err(_) => Ok(SeverityMode::Off),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_severity_mode(s: &str) -> SeverityMode {
        serde_json::from_str::<SeverityMode>(s).unwrap()
    }

    #[test]
    fn test_severity_mode_parse() {
        assert_eq!(SeverityMode::Off, parse_severity_mode("0"));
        assert_eq!(SeverityMode::Off, parse_severity_mode(r#""0""#));
        assert_eq!(SeverityMode::Off, parse_severity_mode(r#""off""#));

        assert_eq!(SeverityMode::Error, parse_severity_mode("1"));
        assert_eq!(SeverityMode::Error, parse_severity_mode(r#""1""#));
        assert_eq!(SeverityMode::Error, parse_severity_mode(r#""error""#));

        assert_eq!(SeverityMode::Warning, parse_severity_mode("2"));
        assert_eq!(SeverityMode::Warning, parse_severity_mode(r#""2""#));
        assert_eq!(SeverityMode::Warning, parse_severity_mode(r#""warning""#));
    }
}
