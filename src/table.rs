
use std::slice::Iter;

#[derive(Default)]
pub struct Table(Vec<Vec<u8>>);

impl Table {
  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn rows(&self) -> Iter<Vec<u8>> {
    self.0.iter()
  }

  pub fn insert_row(&mut self, row: Vec<u8>) {
    self.0.push(row)
  }
}
