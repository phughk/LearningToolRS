use crate::error::Error::{IOError, XMLReaderError};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
  IOError(std::io::Error),
  XMLReaderError(xml::reader::Error),
  SerdeError(serde_xml_rs::Error),
  StateError(String),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::IOError(cause) => {
        write!(f, "Issue with IO: {}", cause)
      }
      Error::XMLReaderError(cause) => {
        write!(f, "Issue with IO in XML: {}", cause)
      }
      Error::SerdeError(cause) => Debug::fmt(cause, f),
      Error::StateError(cause) => {
        write!(f, "Issue with terminal state: {}", cause)
      }
    }
  }
}

impl From<std::io::Error> for Error {
  fn from(err: std::io::Error) -> Self {
    return IOError(err);
  }
}

impl From<xml::reader::Error> for Error {
  fn from(err: xml::reader::Error) -> Self {
    return XMLReaderError(err);
  }
}
