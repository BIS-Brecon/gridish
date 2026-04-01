use std::{num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum OutOfBoundsError {
    #[error("Eastings is out of bounds")]
    Eastings,
    #[error("Northings is out of bounds")]
    Northings,
    #[error("Eastings and Northings are out of bounds")]
    Both,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error(transparent)]
    OutOfBounds(#[from] OutOfBoundsError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("Invalid resolution")]
    InvalidResolution,
    #[error("{0} is not a valid grid square")]
    InvalidSquare(char),
    #[error("{0} is not a valid quadrant")]
    InvalidQuadrant(String),
    #[error("{0}")]
    InvalidString(String),
}
