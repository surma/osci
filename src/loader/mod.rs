//! Loaders for osci memory images.
pub mod rawloader;
pub mod hexloader;

use std::{error, fmt, io, num, result};
use std::string::String;

#[derive(Debug)]
/// Error type for all loaders.
///
/// `LoadError` is just a union-type over a couple of built-in errors with the `From` trait implemented for each of them. This makes implementing loaders more convenient due to the `try!` macro and the `?` operator.
pub enum LoadError {
    None,
    IoErr(io::Error),
    ParseIntErr(num::ParseIntError),
    FormatErr(fmt::Error),
    Message(String),
}

// TODO: Remove None?!!

impl LoadError {
    pub fn new() -> LoadError {
        LoadError::None
    }

    pub fn from_message(msg: String) -> LoadError {
        LoadError::Message(msg)
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Loading failed")
    }
}

impl error::Error for LoadError {
    fn description(&self) -> &str {
        match self {
            &LoadError::None => &"Loading failed",
            &LoadError::Message(ref str) => str,
            &LoadError::IoErr(ref err) => err.description(),
            &LoadError::ParseIntErr(ref err) => err.description(),
            &LoadError::FormatErr(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &LoadError::IoErr(ref err) => Some(err),
            &LoadError::ParseIntErr(ref err) => Some(err),
            &LoadError::FormatErr(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        LoadError::IoErr(err)
    }
}

impl From<num::ParseIntError> for LoadError {
    fn from(err: num::ParseIntError) -> Self {
        LoadError::ParseIntErr(err)
    }
}

impl From<fmt::Error> for LoadError {
    fn from(err: fmt::Error) -> Self {
        LoadError::FormatErr(err)
    }
}

pub type Result<T> = result::Result<T, LoadError>;
