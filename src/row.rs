use crate::error::SqliteError;
use std::fmt::{Display, Error, Formatter};
use std::io::Write;

const COLUMN_USERNAME_SIZE: usize = 32;
const COLUMN_EMAIL_SIZE: usize = 255;

const ID_SIZE: usize = std::mem::size_of::<i32>();
const USERNAME_SIZE: usize = COLUMN_USERNAME_SIZE;
const EMAIL_SIZE: usize = COLUMN_EMAIL_SIZE;
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Row {
  pub id:       i32,
  pub username: String,
  pub email:    String,
}

impl Row {
  pub fn serialize(&self) -> Result<Vec<u8>, SqliteError> {
    let mut encoded: Vec<u8> = Vec::with_capacity(ROW_SIZE);
    encoded.write_all(&self.id.to_le_bytes())?;
    encoded.write_all(self.username.as_bytes())?;
    encoded.write_all(self.email.as_bytes())?;

    println!("encoded size is {}", encoded.len());
    Ok(encoded)
  }

  pub fn deserialize(source: &Vec<u8>) -> Result<Row, SqliteError> {
    let mut id_bytes: [u8; ID_SIZE] = Default::default();
    id_bytes.copy_from_slice(&source[ID_OFFSET..(ID_OFFSET + ID_SIZE)]);
    let id = i32::from_le_bytes(id_bytes);

    let username = String::from_utf8_lossy(&source[USERNAME_OFFSET..(USERNAME_OFFSET + USERNAME_SIZE)]).to_string();
    let email = String::from_utf8_lossy(&source[EMAIL_OFFSET..(EMAIL_OFFSET + EMAIL_SIZE)]).to_string();

    Ok(Row { id, username, email })
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
