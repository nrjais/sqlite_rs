use bincode::ErrorKind;
use core::fmt;
use scan_fmt::parse::ScanError;
use serde::export::fmt::Display;
use std::fmt::Formatter;
use std::io::Error;
use std::string::FromUtf8Error;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ParseError {
  UnknownStatementType,
  UnknownParserError(String),
  SerializationFailedError,
}

impl From<ScanError> for ParseError {
  fn from(e: ScanError) -> Self {
    ParseError::UnknownParserError(e.0)
  }
}

impl From<Error> for ParseError {
  fn from(_e: Error) -> Self {
    ParseError::SerializationFailedError
  }
}

impl From<FromUtf8Error> for ParseError {
  fn from(_e: FromUtf8Error) -> Self {
    ParseError::SerializationFailedError
  }
}

impl From<Box<ErrorKind>> for ParseError {
  fn from(_e: Box<ErrorKind>) -> Self {
    ParseError::SerializationFailedError
  }
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
