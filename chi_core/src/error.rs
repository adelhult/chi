use std::fmt;

// TODO: Replace the error newtype with an enum that also holds the source position of the error
#[derive(Debug)]
pub struct Error(pub(crate) String);

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
