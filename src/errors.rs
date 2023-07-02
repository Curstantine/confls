use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Descriptive(String),
    Abort,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Self {
        Self::Descriptive(format!("Join error:\n{:#?}", error))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error:\n{:#?}", error),
            Self::Descriptive(message) => write!(f, "{}", message),
            Self::Abort => write!(f, "Aborted."),
        }
    }
}

/// macro to format a message and return an Error::Descriptive
#[macro_export]
macro_rules! descriptive {
    ($($arg:tt)*) => {
        $crate::errors::Error::Descriptive(format!($($arg)*))
    };
}

pub(crate) use descriptive;
