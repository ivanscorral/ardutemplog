use std::{fmt, io};

pub enum CSVError {
    NoHeaders,
    IOError(io::Error),
    InvalidDelimiter(char),
}

impl From<io::Error> for CSVError {
    fn from(err: io::Error) -> CSVError {
        CSVError::IOError(err)
    }
}

impl fmt::Display for CSVError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CSVError::NoHeaders => write!(f, "No headers found in CSV data/file"),
            CSVError::IOError(err) =>  write!(f, "IO error: {}", err),
            CSVError::InvalidDelimiter(delimiter) => write!(f, "Invalid delimiter: {}", delimiter),
        }
        
    }
}
