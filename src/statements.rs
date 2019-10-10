use crate::error::SqliteError;
use crate::row::Row;
use scan_fmt::scan_fmt;

#[derive(Debug)]
pub struct InsertStatement {
  pub row: Row,
}

impl<'a> InsertStatement {
  pub fn parse(args: &'a str) -> Result<InsertStatement, SqliteError> {
    let row = Self::parse_row(args)?;
    Ok(InsertStatement { row })
  }

  fn parse_row(args: &str) -> Result<Row, SqliteError> {
    let (id, username, email) = scan_fmt!(args, "insert {} {} {}", i32, String, String)?;
    Ok(Row { id, username, email })
  }
}

#[derive(Debug)]
pub enum Statement {
  Insert(InsertStatement),
  Select,
}
