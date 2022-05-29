use super::types::ReaderPtr;
use pyo3::prelude::*;
use readers::prelude::{Index, Value};

/// A python class allowing python preprocessing code to access to read the data stored in Rust
/// such as get value at the specific index, or get value on the left
///
/// Reader is the pointer to RAReader. We have to store the raw pointer instead of
/// reference since it is not feasible to handle life-time checking when the code
/// is executed in python.
#[pyclass]
pub struct Context {
  reader: ReaderPtr,
}

impl Context {
  pub fn new(reader: ReaderPtr) -> Self {
    Context { reader }
  }
}

#[pymethods]
impl Context {
  fn get_value(&self, index: Vec<Index>) -> Value {
    unsafe {
      let ptr = self.reader.0.as_ptr();
      let res = (*ptr).get_value(&index, 0);
      res.clone()
    }
  }

  fn get_left_value(&self, index: Vec<Index>) -> Value {
    let left = Index::Idx(index.last().unwrap().as_idx() - 1);

    if index.len() > 1 {
      unsafe {
        let ptr = self.reader.0.as_ptr();
        let res = (*ptr).get_value(&index[..index.len() - 1], 0);

        res.get_child_value(&left).clone()
      }
    } else {
      unsafe {
        let ptr = self.reader.0.as_ptr();
        let res = (*ptr).get_value(&[left], 0);
        res.clone()
      }
    }
  }
}
