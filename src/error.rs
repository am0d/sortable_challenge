use std::io;
use serde_json;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum SortableError {
    IOError(io::Error),
    JsonError(serde_json::Error),
}


impl fmt::Display for SortableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SortableError::IOError(ref err) => write!(f, "IO error: {}", err),
            SortableError::JsonError(ref err) => write!(f, "Deserialization error: {}", err),
        }
    }
}

impl error::Error for SortableError {
    fn description(&self) -> &str {
        match *self {
            SortableError::IOError(ref err) => err.description(),
            SortableError::JsonError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SortableError::IOError(ref err) => Some(err),
            SortableError::JsonError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for SortableError {
    fn from(err: io::Error) -> SortableError {
        SortableError::IOError(err)
    }
}

impl From<serde_json::Error> for SortableError {
    fn from(err: serde_json::Error) -> SortableError {
        SortableError::JsonError(err)
    }
}
