#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ParseError(String),
    InvalidPrecision(String),
    OutOfBounds,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error parsing string")
    }
}

impl std::error::Error for Error {}
