use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    NotDirectory(String),
    DirectoryDoesNotExists(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for self::Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Error::NotDirectory(dir) => {
                write!(f, "{} is not a directory", dir)
            }

            Error::DirectoryDoesNotExists(dir) => {
                write!(f, "{} directory does not exists", dir)
            }
        }
    }
}

impl StdError for self::Error {}

