use std::error;
use std::fmt;
use std::string;

#[derive(Debug)]
pub enum CellError {
    Generic(GenericError),
    String(string::FromUtf8Error),
}

impl CellError {
    pub fn generic(message: &str) -> CellError {
        CellError::Generic(GenericError::new(message))
    }
}

impl fmt::Display for CellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            CellError::Generic(ref err) => write!(f, "{}", err),
            CellError::String(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for CellError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CellError::Generic(ref err) => err.description(),
            CellError::String(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            CellError::Generic(ref err) => Some(err),
            CellError::String(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub struct GenericError {
    message: String,
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError { message: String::from(message) }
    }
}

impl<'a> fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Store error: {}", self.message)
    }
}

impl<'a> error::Error for GenericError {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}
