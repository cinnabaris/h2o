use std::error::Error as StdError;
use std::{env, error, fmt, io, num, path, result, string, time};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]

pub enum Error {
    Io(io::Error),
    EnvVar(env::VarError),
    NumParseInt(num::ParseIntError),
    PathStripPrefixError(path::StripPrefixError),
    StringFromUtf8(string::FromUtf8Error),
    SystemTime(time::SystemTimeError),
    WithDescription(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::EnvVar(ref err) => err.fmt(f),
            Error::NumParseInt(ref err) => err.fmt(f),
            Error::PathStripPrefixError(ref err) => err.fmt(f),
            Error::StringFromUtf8(ref err) => err.fmt(f),
            Error::SystemTime(ref err) => err.fmt(f),
            Error::WithDescription(ref desc) => write!(f, "{}", desc),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::EnvVar(ref err) => err.description(),
            Error::NumParseInt(ref err) => err.description(),
            Error::PathStripPrefixError(ref err) => err.description(),
            Error::StringFromUtf8(ref err) => err.description(),
            Error::SystemTime(ref err) => err.description(),
            Error::WithDescription(ref desc) => &desc,
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::EnvVar(ref err) => Some(err),
            Error::NumParseInt(ref err) => Some(err),
            Error::PathStripPrefixError(ref err) => Some(err),
            Error::StringFromUtf8(ref err) => Some(err),
            Error::SystemTime(ref err) => Some(err),
            Error::WithDescription(_) => None,
        }
    }
}
