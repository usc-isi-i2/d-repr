use super::{Index, Value};
use crate::iterators::StreamingIndexIterator;
use crate::models::*;
use crate::readers::ra_reader::{reader_iter_data, RAReader};

use std::fs::File;

#[derive(Debug)]
pub struct CSVRAReader {
  pub data: Vec<Value>,
  is_matrix: bool,
}

impl CSVRAReader {
  pub fn from_file(fpath: &str, delimiter: u8) -> CSVRAReader {
    let rdr = csv::ReaderBuilder::new()
      .has_headers(false)
      .delimiter(delimiter)
      .flexible(true)
      .from_reader(File::open(fpath).unwrap());

    let rows: Vec<Value> = rdr
      .into_records()
      .map(|r| {
        Value::Array(
          r.unwrap()
            .into_iter()
            .map(|c| Value::Str(c.to_string()))
            .collect::<Vec<_>>(),
        )
      })
      .collect();

    let mut is_matrix = true;
    if rows.len() > 0 {
      let n_cols = rows[0].len();
      for i in 1..rows.len() {
        if rows[i].as_array().len() != n_cols {
          is_matrix = false;
          break;
        }
      }
    }

    return CSVRAReader {
      data: rows,
      is_matrix,
    };
  }

  pub fn from_str(data: &str, delimiter: u8) -> CSVRAReader {
    let rdr = csv::ReaderBuilder::new()
      .has_headers(false)
      .delimiter(delimiter)
      .from_reader(data.as_bytes());

    let rows: Vec<Value> = rdr
      .into_records()
      .map(|r| {
        Value::Array(
          r.unwrap()
            .into_iter()
            .map(|c| Value::Str(c.to_string()))
            .collect::<Vec<_>>(),
        )
      })
      .collect();

    let mut is_matrix = true;
    if rows.len() > 0 {
      let n_cols = rows[0].len();
      for i in 1..rows.len() {
        if rows[i].as_array().len() != n_cols {
          is_matrix = false;
          break;
        }
      }
    }

    return CSVRAReader {
      data: rows,
      is_matrix,
    };
  }
}

impl RAReader for CSVRAReader {
  fn into_value(self) -> Value {
    Value::Array(self.data)
  }

  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    if index.len() - 1 > start_idx {
      &self.data[index[start_idx].as_idx()].as_array()[index[start_idx + 1].as_idx()]
    } else {
      &self.data[index[start_idx].as_idx()]
    }
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    if index.len() - 1 > start_idx {
      &mut self.data[index[start_idx].as_idx()].as_mut_array()[index[start_idx + 1].as_idx()]
    } else {
      &mut self.data[index[start_idx].as_idx()]
    }
  }

  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    if index.len() - 1 > start_idx {
      self.data[index[start_idx].as_idx()].as_mut_array()[index[start_idx + 1].as_idx()] = val;
    } else {
      self.data[index[start_idx].as_idx()] = val;
    }
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    self.data.remove(index.as_idx());
  }

  fn ground_location(&self, loc: &mut Location, start_idx: usize) {
    if loc.slices[start_idx].is_range() {
      let r = loc.slices[start_idx].as_mut_range();
      match r.end {
        None => {
          r.end = Some(self.data.len() as i64);
        },
        Some(e) => {
          if e < 0 {
            r.end = Some(self.data.len() as i64 + e);
          }
        }
      }
    }

    if self.is_matrix && loc.slices.len() > start_idx + 1 && loc.slices[start_idx + 1].is_range() {
      let r = loc.slices[start_idx+1].as_mut_range();
      match r.end {
        None => {
          r.end = Some(self.data[0].as_array().len() as i64);
        },
        Some(e) => {
          if e < 0 {
            r.end = Some(self.data[0].as_array().len() as i64 + e);
          }
        }
      }
    }
  }

  fn can_change_value_type(&mut self) {
    self.is_matrix = true;
  }

  fn iter_data<'a>(&'a self, loc: &Location) -> Box<dyn StreamingIndexIterator + 'a> {
    reader_iter_data(self, loc)
  }
}
