use crate::index::Index;
use crate::iterators::IndexIterator;
use crate::path_expr::{PathExpr, StepExpr};
use crate::prelude::RAReader;
use crate::ra_reader::default_iter_index;
use crate::value::Value;
use hashbrown::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct JSONRAReader {
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

impl JSONRAReader {
  pub fn from_file(fpath: &str) -> JSONRAReader {
    let mut content = String::new();
    File::open(fpath)
      .unwrap()
      .read_to_string(&mut content)
      .unwrap();

    let val: serde_json::Value = serde_json::from_str(&content).unwrap();

    JSONRAReader {
      data: serde2value(val),
    }
  }
  pub fn from_str(data: &str) -> JSONRAReader {
    let val: serde_json::Value = serde_json::from_str(data).unwrap();
    JSONRAReader {
      data: serde2value(val),
    }
  }
  pub fn from_json(val: serde_json::Value) -> JSONRAReader {
    JSONRAReader {
      data: serde2value(val),
    }
  }
}

impl RAReader for JSONRAReader {
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
  fn ground_path(&self, path: &mut PathExpr, start_idx: usize) {
    // we can only ground the first range slice
    let mut ptr = &self.data;
    for s in &mut path.steps[start_idx..] {
      match s {
        StepExpr::Range(r) => {
          match r.end {
            None => {
              r.end = Some(ptr.len() as i64);
            }
            Some(e) => {
              if e < 0 {
                r.end = Some(ptr.len() as i64 + e);
              }
            }
          }
          break;
        }
        StepExpr::Index(i) => {
          ptr = ptr.get_child_value(&i.val);
        }
        StepExpr::SetIndex(_) => {
          unimplemented!()
        }
        StepExpr::Wildcard => {
          unimplemented!()
        }
      }
    }
  }
  fn iter_index<'a>(&'a self, path: &PathExpr) -> Box<dyn IndexIterator + 'a> {
    default_iter_index(self, path)
  }
}
