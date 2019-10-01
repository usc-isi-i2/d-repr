use super::ra_reader::reader_iter_data;
use super::{Index, RAReader, Value};
use crate::iterators::StreamingIndexIterator;
use crate::models::{Location, Slice};
use hashbrown::HashMap;

#[derive(Debug)]
pub struct MultipleRAReader {
  pub data: HashMap<String, Value>,
}

impl MultipleRAReader {
  pub fn new(data: HashMap<String, Value>) -> MultipleRAReader {
    MultipleRAReader { data }
  }
}

impl RAReader for MultipleRAReader {
  fn into_value(self) -> Value {
    Value::Object(self.data)
  }

  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    if index.len() - 1 == start_idx {
      return &self.data[index[start_idx].as_str()];
    }
    self.data[index[start_idx].as_str()].get_value(&index, start_idx + 1)
  }

  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    self
      .data
      .get_mut(index[start_idx].as_str())
      .unwrap()
      .get_mut_value(&index, start_idx + 1)
  }

  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    if index.len() - 1 == start_idx {
      self.data.insert(index[start_idx].as_str().to_string(), val);
      return;
    }

    self
      .data
      .get_mut(index[start_idx].as_str())
      .unwrap()
      .set_value(&index, start_idx + 1, val)
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    self.data.remove(index.as_str());
  }

  fn ground_location(&self, loc: &mut Location, start_idx: usize) {
    // we can only ground the first range slice
    let mut ptr = &self.data[loc.slices[start_idx].as_index().idx.as_str()];
    for s in &mut loc.slices[start_idx + 1..] {
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

  fn can_change_value_type(&mut self) {}

  fn iter_data<'a>(&'a self, loc: &Location) -> Box<dyn StreamingIndexIterator + 'a> {
    reader_iter_data(self, loc)
  }
}
