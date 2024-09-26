#[derive(Debug)]
pub enum Error {
    NoValidHomeDirectory,
    UnknownReminderState,
    Sqlite(rusqlite::Error),
    WhenParse(WhenParseError),
}

#[derive(Debug)]
pub enum WhenParseError {
    NoCaptures(String),
    NoNumber(String),
    NoUnit(String),
    ParseInt(std::num::ParseIntError),
}

impl std::error::Error for Error {}
impl std::error::Error for WhenParseError {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Error::NoValidHomeDirectory => write!(f, "No valid home directory"),
            Error::UnknownReminderState => write!(f, "Unknown reminder state"),
            Error::Sqlite(e) => write!(f, "Sqlite error: {}", e),
            Error::WhenParse(e) => write!(f, "Couldn't parse when: {}", e),
        }
    }
}

impl std::fmt::Display for WhenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            WhenParseError::NoCaptures(s) => 
                write!(f, "Couldn't determine when from \"{}\"", s),
            WhenParseError::NoNumber(s) => 
                write!(f, "Couldn't determine a number from \"{}\"", s),
            WhenParseError::NoUnit(s) =>
                write!(f, "Couldn't determine a time unit form \"{}\"", s),
            WhenParseError::ParseInt(e) =>
                write!(f, "Couldn't parse an integer: {}", e),
        }
    }
}

macro_rules! from {
    ($err:ty, $ty:ty, $variant:ident) => {
        impl From<$ty> for $err {
            fn from(e: $ty) -> $err {
                <$err>::$variant(e.into())
            }
        }
    }
}

from!(Error, rusqlite::Error, Sqlite);
from!(Error, WhenParseError, WhenParse);
from!(WhenParseError, std::num::ParseIntError, ParseInt);
