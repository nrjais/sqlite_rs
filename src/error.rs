use core::fmt;
use scan_fmt::parse::ScanError;
use std::fmt::{Display, Formatter};
use std::io::Error;
use std::string::FromUtf8Error;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum SqliteError {
  UnknownStatementType,
  UnknownParserError(String),
  SerializationFailedError,
}

impl From<ScanError> for SqliteError {
  fn from(e: ScanError) -> Self {
    SqliteError::UnknownParserError(e.0)
  }
}

impl From<Error> for SqliteError {
  fn from(_e: Error) -> Self {
    SqliteError::SerializationFailedError
  }
}

impl From<FromUtf8Error> for SqliteError {
  fn from(_e: FromUtf8Error) -> Self {
    SqliteError::SerializationFailedError
  }
}

impl std::error::Error for SqliteError {}

impl Display for SqliteError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
