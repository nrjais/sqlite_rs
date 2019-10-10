use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::mem::size_of;
use std::path::PathBuf;

use crate::error::SqliteError;

type RowNum = i32;
type RowLength = i32;
type TableLength = i32;

#[derive(Default)]
pub struct Table {
  entries: Vec<(RowNum, RowLength)>,
}

impl Table {
  pub fn new() -> Table {
    Default::default()
  }

  pub fn ser_size(&self) -> usize {
    size_of::<TableLength>() + size_of::<(RowNum, RowLength)>()
  }

  pub fn serialize(&self) -> Result<Vec<u8>, SqliteError> {
    let mut encoded: Vec<u8> = Vec::with_capacity(self.ser_size());
    encoded.write_all(&(self.entries.len() as TableLength).to_le_bytes())?;
    for entry in self.entries.iter() {
      encoded.write_all(&entry.0.to_le_bytes())?;
      encoded.write_all(&entry.1.to_le_bytes())?;
    }

    println!("encoded pager size is {} expected {}", encoded.len(), self.ser_size());
    Ok(encoded)
  }

  pub fn deserialize(source: &Vec<u8>) -> Result<Table, SqliteError> {
    let mut len_bytes: [u8; size_of::<TableLength>()] = Default::default();
    len_bytes.copy_from_slice(&source[0..size_of::<TableLength>()]);
    let table_length = TableLength::from_le_bytes(len_bytes);

    let mut entries = Vec::with_capacity(table_length as usize);

    let mut offset = size_of::<TableLength>();

    for entry_index in 0..table_length {
      let mut bytes: [u8; size_of::<RowNum>()] = Default::default();

      bytes.copy_from_slice(&source[offset..size_of::<RowNum>()]);
      let row_num = RowNum::from_le_bytes(bytes);
      offset = offset + size_of::<RowNum>();

      bytes.copy_from_slice(&source[offset..size_of::<RowLength>()]);
      let row_length = RowLength::from_le_bytes(bytes);
      offset = offset + size_of::<RowLength>();

      entries[entry_index as usize] = (row_num, row_length)
    }

    Ok(Table { entries })
  }
}

pub struct Pager {
  table:       Table,
  file:        File,
  file_length: u64,
  current_pos: i64,
}

impl Pager {
  pub fn open(path: &PathBuf) -> Result<Pager, SqliteError> {
    let mut file = OpenOptions::new().write(true).read(true).create(true).open(path)?;
    let file_length = file.stream_len()?;

    let table = if file_length == 0 {
      Default::default()
    } else {
      Table::deserialize(&Default::default())?
    };

    file.seek(SeekFrom::Start(table.ser_size() as u64))?;

    Ok(Pager {
      file,
      file_length,
      current_pos: 0,
      table,
    })
  }
}
