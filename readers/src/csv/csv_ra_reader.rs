use crate::prelude::{Index, Value, PathExpr, IndexIterator};
use crate::ra_reader::{RAReader, default_iter_index};

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
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    if index.len() - 1 > start_idx {
      self.data[index[start_idx].as_idx()].set_value(index, start_idx + 1, val);
    } else {
      self.data[index[start_idx].as_idx()] = val;
    }
  }
  
  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    if index.len() - 1 > start_idx {
      self.data[index[start_idx].as_idx()].get_value(index, start_idx + 1)
    } else {
      &self.data[index[start_idx].as_idx()]
    }
  }
  
  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    if index.len() - 1 > start_idx {
      self.data[index[start_idx].as_idx()].get_mut_value(index, start_idx + 1)
    } else {
      &mut self.data[index[start_idx].as_idx()]
    }
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    self.data.remove(index.as_idx());
  }

  fn ground_path(&self, _path: &mut PathExpr, _start_idx: usize) {
    unimplemented!()
  }
  
  fn iter_index<'a>(&'a self, path: &PathExpr) -> Box<dyn IndexIterator + 'a> {
    default_iter_index(self, path)
  }
}
