use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use std::ffi::OsString;

#[derive(Debug)]
pub enum Error {
    Utf8Error,
    BadString(String),
    FailToConvertFileName(OsString),
    NotFile(String),
    FileDoesNotExist(String),
    NotDirectory(String),
    DirectoryDoesNotExist(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for self::Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::Utf8Error => {
                write!(f, "error while converting string, not valid utf8")
            }

            Error::BadString(string) => {
                write!(f, "{} is not valid!", string)
            }

            Error::FailToConvertFileName(file) => {
                write!(f, "{:?} couldn\'t be converted", file)
            }

            Error::NotFile(file) => {
                write!(f, "{} is not a file", file)
            }

            Error::FileDoesNotExist(file) => {
                write!(f, "{} file does not exist", file)
            }

            Error::NotDirectory(dir) => {
                write!(f, "{} is not a directory", dir)
            }

            Error::DirectoryDoesNotExist(dir) => {
                write!(f, "{} directory does not exist", dir)
            }
        }
    }
}

impl StdError for self::Error {}

