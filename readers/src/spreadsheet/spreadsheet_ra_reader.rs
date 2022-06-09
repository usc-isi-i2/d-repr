use crate::prelude::{Index, Value, PathExpr, IndexIterator};
use crate::ra_reader::{RAReader, default_iter_index};

use calamine::{open_workbook_auto, DataType, Reader};
use hashbrown::HashMap;


#[derive(Debug)]
pub struct SpreadsheetRAReader {
  sheets: Vec<Value>,
  sheet_names: Vec<String>,
  name2index: HashMap<String, usize>,
}

impl SpreadsheetRAReader {
  pub fn from_file(fpath: &str) -> SpreadsheetRAReader {
    let mut workbook = open_workbook_auto(fpath).expect("Cannot open the resource file");
    let sheet_names = workbook.sheet_names().to_vec();

    let mut sheets = Vec::with_capacity(sheet_names.len());
    let mut name2index: HashMap<String, usize> = HashMap::default();

    for sheet_name in &sheet_names {
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
                DataType::DateTime(v) => Value::F64(*v),
                DataType::Empty => Value::Str(String::new()),
                DataType::Error(_) => Value::Null,
              })
              .collect::<Vec<_>>();

            Value::Array(row)
          })
          .collect::<Vec<_>>();

        sheets.push(Value::Array(rows));
        name2index.insert(sheet_name.clone(), sheets.len() - 1);
      }
    }

    return SpreadsheetRAReader { sheets, name2index, sheet_names };
  }
}

impl RAReader for SpreadsheetRAReader {
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    match &index[start_idx] {
      Index::Idx(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[*v].set_value(index, start_idx + 1, val);
        } else {
          self.sheets[*v] = val;
        }
      },
      Index::Str(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[self.name2index[v]].set_value(index, start_idx + 1, val);
        } else {
          self.sheets[self.name2index[v]] = val;
        }
      }
    }
  }

  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    match &index[start_idx] {
      Index::Idx(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[*v].get_value(index, start_idx + 1)
        } else {
          &self.sheets[*v]
        }
      },
      Index::Str(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[self.name2index[v]].get_value(index, start_idx + 1)
        } else {
          &self.sheets[self.name2index[v]]
        }
      }
    }
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    match &index[start_idx] {
      Index::Idx(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[*v].get_mut_value(index, start_idx + 1)
        } else {
          &mut self.sheets[*v]
        }
      },
      Index::Str(v) => {
        if start_idx < index.len() - 1 {
          self.sheets[self.name2index[v]].get_mut_value(index, start_idx + 1)
        } else {
          &mut self.sheets[self.name2index[v]]
        }
      }
    }
  }

  fn len(&self) -> usize {
    self.sheets.len()
  }

  fn remove(&mut self, index: &Index) {
    let sheet_index = match index {
      Index::Idx(v) => {
        *v
      },
      Index::Str(v) => {
        self.name2index[v]
      }
    };

    self.sheets.remove(sheet_index);
    self.name2index.remove(&self.sheet_names[sheet_index]);
    self.sheet_names.remove(sheet_index);
    for i in sheet_index..self.sheets.len() {
      *self.name2index.get_mut(&self.sheet_names[i]).unwrap() -= 1;
    }
  }

  fn ground_path(&self, _loc: &mut PathExpr, _start_idx: usize) {
    unimplemented!()
  }

  fn iter_index<'a>(&'a self, loc: &PathExpr) -> Box<dyn IndexIterator + 'a> {
    default_iter_index(self, loc)
  }
}