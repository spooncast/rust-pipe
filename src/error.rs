use std::io::{Error as IoError, ErrorKind};

#[derive(PartialEq, Debug)]
pub enum Error {
    IoError {
        kind: ErrorKind,
    },
    Unknown,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError{ kind: err.kind() }
    }
}
