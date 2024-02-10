use std::io;
use pyo3::exceptions::{PyIOError, PyNotImplementedError, PyValueError};
use pyo3::PyErr;
use uesave::Error;
use uesave::error::ParseError;


pub struct PyParseError(ParseError);

impl From<PyParseError> for PyErr {
    fn from(PyParseError(value): PyParseError) -> Self {
        match value.error {
            Error::Io(e) => PyIOError::new_err(e),
            Error::Unimplemented(e) => PyNotImplementedError::new_err(e),
            e@ (
                Error::BadMagic() |
                Error::UnknownPropertyType(_) |
                Error::UnknownPropertyMeta(_) |
                Error::UnknownVecType(_)
            ) => PyValueError::new_err(e.to_string()),
        }
    }
}

impl From<ParseError> for PyParseError {
    fn from(value: ParseError) -> Self {
        PyParseError(value)
    }
}

impl From<io::Error> for PyParseError {
    fn from(err: io::Error) -> Self {
        PyParseError(ParseError{error:Error::Io(err), offset: 0})
    }
}