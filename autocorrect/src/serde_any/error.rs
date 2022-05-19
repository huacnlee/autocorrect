use crate::serde_any::Format;

/// The common error type
#[derive(Debug)]
pub enum Error {
    /// Error serializing or deserializing with JSON
    Json(serde_json::Error),

    /// Error serializing or deserializing with YAML
    Yaml(serde_yaml::Error),

    /// The specified format is not supported
    UnsupportedFormat(Format),

    NoSuccessfulParse(Vec<(Format, Error)>),
}

macro_rules! impl_error_from {
    ($error_type:ty => $variant:expr) => {
        impl From<$error_type> for Error {
            fn from(e: $error_type) -> Error {
                $variant(e)
            }
        }
    };
}

impl_error_from!(serde_json::Error => Error::Json);
impl_error_from!(serde_yaml::Error => Error::Yaml);
