use super::{Index, RAReader, Value};
use super::ra_reader::reader_iter_data;
use crate::iterators::StreamingIndexIterator;
use crate::models::*;
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
  fn into_value(self) -> Value {
    self.data
  }

  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    self.data.get_value(index, start_idx)
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    self.data.get_mut_value(index, start_idx)
  }

  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    self.data.set_value(index, start_idx, val)
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    self.data.remove(index)
  }

  fn ground_location(&self, loc: &mut Location, start_idx: usize) {
    // we can only ground the first range slice
    let mut ptr = &self.data;
    for s in &mut loc.slices[start_idx..] {
      match s {
        Slice::Range(r) => {
          match r.end {
            None => {
              r.end = Some(ptr.len() as i64);
            },
            Some(e) => {
              if e < 0 {
                r.end = Some(ptr.len() as i64 + e);
              }
            }
          }
          break;
        }
        Slice::Index(i) => {
          ptr = ptr.get_child_value(&i.idx);
        }
      }
    }
  }

  fn can_change_value_type(&mut self) {
  }

  fn iter_data<'a>(&'a self, loc: &Location) -> Box<StreamingIndexIterator + 'a> {
    reader_iter_data(self, loc)
  }
}