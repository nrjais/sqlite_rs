use serde_derive::{Deserialize, Serialize};

use crate::error::ParseError;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Row {
  pub id:       i32,
  pub username: String,
  pub email:    String,
}

impl Row {
  pub fn serialize(&self) -> Result<Vec<u8>, ParseError> {
    let encoded: Vec<u8> = bincode::serialize(self)?;
    Ok(encoded)
  }

  pub fn deserialize(source: &Vec<u8>) -> Result<Row, ParseError> {
    let row = bincode::deserialize::<Row>(source)?;
    Ok(row)
  }
}

impl Display for Row {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "({}, {}, {})", self.id, self.username, self.email)
  }
}

#[cfg(test)]
mod tests {
  use crate::row::Row;

  #[test]
  fn test() {
    let row = Row {
      id:       30,
      username: "neeraj".into(),
      email:    "nj@gmail.com".into(),
    };
    let en = row.serialize();
    let ar = Row::deserialize(&en.unwrap_or(Vec::new()));

    assert_eq!(ar, Ok(row))
  }
}
