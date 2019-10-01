use hashbrown::HashMap;
use serde_json;
use std::fs::File;
use std::io::Read;

use crate::iterators::StreamingIndexIterator;
use crate::models::{Location, Slice};
use crate::readers::ra_reader::{reader_iter_data, RAReader};

use super::{Index, Value};

#[derive(Debug)]
pub struct XMLRAReader {
  pub data: Value,
}

#[inline]
fn serde2value(sval: serde_json::Value) -> Value {
  match sval {
    serde_json::Value::Bool(b) => Value::Bool(b),
    serde_json::Value::Null => Value::Null,
    serde_json::Value::Number(n) => {
      if n.is_f64() {
        Value::F64(n.as_f64().unwrap())
      } else {
        Value::I64(n.as_i64().unwrap())
      }
    }
    serde_json::Value::String(s) => Value::Str(s),
    serde_json::Value::Array(a) => Value::Array(a.into_iter().map(|ai| serde2value(ai)).collect()),
    serde_json::Value::Object(a) => Value::Object(
      a.into_iter()
        .map(|(k, v)| (k, serde2value(v)))
        .collect::<HashMap<_, _>>(),
    ),
  }
}

impl XMLRAReader {
  pub fn from_file(fpath: &str) -> XMLRAReader {
    // somehow read from buffer is slower than read to string first
    let mut content = String::new();
    File::open(fpath)
      .unwrap()
      .read_to_string(&mut content)
      .unwrap();

    let val: serde_json::Value = serde_json::from_str(&content).unwrap();

    XMLRAReader {
      data: serde2value(val),
    }
  }

  pub fn from_str(data: &str) -> XMLRAReader {
    let val: serde_json::Value = serde_json::from_str(data).unwrap();
    XMLRAReader {
      data: serde2value(val)
    }
  }
}

impl RAReader for XMLRAReader {
  fn into_value(self) -> Value {
    self.data
  }

  #[inline]
  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    self.data.get_value(index, start_idx)
  }

  #[inline]
  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    self.data.get_mut_value(index, start_idx)
  }

  #[inline]
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    self.data.set_value(index, start_idx, val)
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn remove(&mut self, index: &Index) {
    match &mut self.data {
      Value::Array(children) => {
        children.remove(index.as_idx());
      }
      Value::Object(map) => {
        map.remove(index.as_str());
      }
      _ => panic!("Cannot remove child at leaf nodes"),
    }
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

  fn can_change_value_type(&mut self) {}

  fn iter_data<'a>(&'a self, loc: &Location) -> Box<dyn StreamingIndexIterator + 'a> {
    reader_iter_data(self, loc)
  }
}
