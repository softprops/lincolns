use std::{error::Error as StdError, fmt, io, str::Utf8Error};
use yaml_rust::ScanError;

/// Possible errors that may occur while loading content
#[derive(Debug)]
pub enum Error {
    /// Failure to parse content
    Parse(ScanError),
    /// Failure to load data
    Io(io::Error),
    /// Failure to read data as utf8 text
    Utf8(Utf8Error),
}

impl fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> std::result::Result<(), fmt::Error> {
        match self {
            Error::Parse(ref err) => writeln!(f, "{}", err),
            Error::Io(ref err) => writeln!(f, "{}", err),
            Error::Utf8(ref err) => writeln!(f, "{}", err),
        }
    }
}

impl StdError for Error {}

impl From<ScanError> for Error {
    fn from(err: ScanError) -> Error {
        Error::Parse(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Error {
        Error::Utf8(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
