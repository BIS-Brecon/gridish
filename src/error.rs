use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error(transparent)]
    ParseError(#[from] ParseError),
    #[error("Provided Eastings and/or Northings are out of bounds.")]
    OutOfBounds,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("{0} is not a valid grid square.")]
    InvalidSquare(char),
    #[error("{0} is not a valid 500km grid square.")]
    Invalid500kSquare(char),
    #[error("{0} is not a valid number of digits. Supported values: 0, 2, 4, 6, 8, 10.")]
    InvalidPrecision(u32),
    #[error("String can not be empty.")]
    EmptyString,
}
