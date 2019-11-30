use crate::prelude::{Index, Value, PathExpr, IndexIterator};
use crate::ra_reader::{RAReader, default_iter_index};

use calamine::{open_workbook_auto, DataType, Reader};
use hashbrown::HashMap;


#[derive(Debug)]
pub struct SpreadsheetRAReader {
  data: Value,
}

impl SpreadsheetRAReader {
  pub fn from_file(fpath: &str) -> SpreadsheetRAReader {
    let mut workbook = open_workbook_auto(fpath).expect("Cannot open the resource file");
    let mut data = HashMap::default();
    let sheet_names = workbook.sheet_names().to_vec();
    for sheet_name in sheet_names {
      if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
        let rows = range
          .rows()
          .map(|r| {
            let row = r
              .iter()
              .map(|c| match c {
                DataType::Bool(v) => Value::Bool(*v),
                DataType::String(v) => Value::Str(v.clone()),
                DataType::Int(v) => Value::I64(*v),
                DataType::Float(v) => Value::F64(*v),
                DataType::Empty => Value::Str(String::new()),
                DataType::Error(_) => Value::Null,
              })
              .collect::<Vec<_>>();

            Value::Array(row)
          })
          .collect::<Vec<_>>();

        data.insert(sheet_name, Value::Array(rows));
      }
    }

    return SpreadsheetRAReader { data: Value::Object(data) };
  }
}

impl RAReader for SpreadsheetRAReader {
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    self.data.set_value(index, start_idx, val)
  }

  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    self.data.get_value(index, start_idx)
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    self.data.get_mut_value(index, start_idx)
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    self.data.remove(index)
  }

  fn ground_path(&self, loc: &mut PathExpr, start_idx: usize) {
    unimplemented!()
  }

  fn iter_index<'a>(&'a self, loc: &PathExpr) -> Box<dyn IndexIterator + 'a> {
    default_iter_index(self, loc)
  }
}