use std::fmt::{Display, Formatter};
use crate::error::Error::{IOError, XMLReaderError};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    XMLReaderError(xml::reader::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(cause) => {
                write!(
                    f,
                    "Issue with IO: {}",
                    cause
                )
            }
            Error::XMLReaderError(cause) => {
                write!(
                    f,
                    "Issue with XML: {}",
                    cause
                )
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        return IOError(err)
    }
}

impl From<xml::reader::Error> for Error {
    fn from(err: xml::reader::Error) -> Self {
        return XMLReaderError(err)
    }
}