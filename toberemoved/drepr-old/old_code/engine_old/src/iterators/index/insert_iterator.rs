use crate::readers::{Index, Value, RAReader};
use crate::models::{Slice, Location};
use std::borrow::BorrowMut;

///
/// Iterate through index of a location, and insert value to the index
///
///
pub struct InsertIterator<'a, R: RAReader> {
  /// The random access reader that contains data
  reader: &'a R,
  has_more: bool,

  lowerbounds: Vec<usize>,
  upperbounds: Vec<usize>,
  neg_upperbounds: Vec<usize>,
  unknown_upperbounds: Vec<bool>,
  steps: Vec<usize>,
  unfrozen_dims: Vec<usize>,

  last_dim: usize,
  // = index.len() - 1
  index: Vec<Index>,

  data_pointers: Vec<&'a Value>,
  new_data: Box<Vec<Value>>,
  new_data_pointers: Vec<*mut Vec<Value>>,
}

impl<'a, R: RAReader> InsertIterator<'a, R> {
  pub fn new(
    reader: &'a R,
    loc: &Location,
  ) -> InsertIterator<'a, R> {
    let mut index: Vec<Index> = vec![];
    let mut unfrozen_dims: Vec<usize> = vec![];
    let mut unknown_upperbounds: Vec<bool> = vec![];

    let mut lowerbounds: Vec<usize> = vec![];
    let mut upperbounds: Vec<usize> = vec![];
    let mut neg_upperbounds: Vec<usize> = vec![];
    let mut steps: Vec<usize> = vec![];

    for (i, slice) in loc.slices.iter().enumerate() {
      match slice {
        Slice::Range(s) => {
          unfrozen_dims.push(i);
          index.push(Index::Idx(s.start as usize));

          lowerbounds.push(s.start as usize);
          match s.end {
            Some(v) => {
              if v > 0 {
                upperbounds.push(v as usize);
                unknown_upperbounds.push(false);
                neg_upperbounds.push(0);
              } else {
                upperbounds.push(0);
                neg_upperbounds.push(-v as usize);
                unknown_upperbounds.push(true);
              }
            }
            None => {
              upperbounds.push(0); // random number as it doesn't matter
              neg_upperbounds.push(0);
              unknown_upperbounds.push(true);
            }
          }
          steps.push(s.step);
        }
        Slice::Index(s) => {
          index.push(s.idx.clone());
          // this won't be used as it is frozen dim
          lowerbounds.push(0);
          upperbounds.push(0);
          steps.push(0);
          unknown_upperbounds.push(false);
          neg_upperbounds.push(0);
        }
      }
    }

    let n_roots = upperbounds[unfrozen_dims[0]] - lowerbounds[unfrozen_dims[0]];
    unfrozen_dims.reverse();

    let mut iter = InsertIterator {
      reader,
      has_more: true,

      lowerbounds,
      upperbounds,
      neg_upperbounds,
      unknown_upperbounds,
      steps,
      unfrozen_dims,
      last_dim: index.len() - 1,
      index,
      data_pointers: Vec::with_capacity(n_roots),
      new_data: Box::new(Vec::with_capacity(n_roots)),
      new_data_pointers: Vec::with_capacity(n_roots)
    };

    iter.init_data_ptrs();
    iter
  }

  pub fn get_data(self) -> Value {
    Value::Array(*self.new_data)
  }

  pub fn index(&self) -> &[Index] {
    &self.index
  }

  pub fn value(&self) -> &Value {
    self.data_pointers.last().unwrap().get_child_value(self.index.last().unwrap())
  }

  pub fn push(&mut self, val: Value) {
    unsafe {
      (*(*self.new_data_pointers.last().unwrap())).push(val);
    };
  }

  pub fn advance(&mut self) -> bool {
    if self.has_more {
      for &dim_pivot in &self.unfrozen_dims {
        let idx = self.index[dim_pivot].as_mut_idx();
        *idx += self.steps[dim_pivot];
        if *idx >= self.upperbounds[dim_pivot] {
          *idx = self.lowerbounds[dim_pivot] as usize;
        } else {
          // successfully advance, now we only need to update the pointers if they are not leaf nodes
          if dim_pivot < self.last_dim {
            self.update_data_ptrs(dim_pivot);
          }
          return true;
        }
      }
      self.has_more = false;
    }
    return false;
  }

  fn update_data_ptrs(&mut self, mut start_idx: usize) {
    if start_idx == 0 {
      self.data_pointers[0] = self.reader.get_value(&self.index[..1], 0);
      start_idx += 1;
    }

    for i in start_idx..self.last_dim {
      if self.unknown_upperbounds[i] {
        self.upperbounds[i] = self.data_pointers[i - 1].len() - self.neg_upperbounds[i];
      }
      self.data_pointers[i] = self.data_pointers[i - 1].get_child_value(&self.index[i]);
    }

    if self.unknown_upperbounds[self.last_dim] {
      self.upperbounds[self.last_dim] = self.data_pointers[self.last_dim - 1].len() - self.neg_upperbounds[self.last_dim];
    }

    // update data pointers
    for (i, &di) in self.unfrozen_dims.iter().skip(1).rev().enumerate() {
      unsafe {
        (*self.new_data_pointers[i]).push(Value::Array(Vec::with_capacity(
          self.upperbounds[di] - self.lowerbounds[di]
        )));
      }

      let ptr = unsafe {
        (*self.new_data_pointers[i]).last_mut().unwrap().as_mut_array() as *mut Vec<Value>
      };
      self.new_data_pointers[i+1] = ptr;
    }
  }

  fn init_data_ptrs(&mut self) {
    // upperbounds.0 is always known
    self.data_pointers.push(self.reader.get_value(&self.index[..1], 0));

    for i in 1..self.last_dim {
      if self.unknown_upperbounds[i] {
        self.upperbounds[i] = self.data_pointers[i - 1].len() - self.neg_upperbounds[i];
      }

      self.data_pointers.push(self.data_pointers[i - 1].get_child_value(&self.index[i]));
    }

    if self.unknown_upperbounds[self.last_dim] {
      self.upperbounds[self.last_dim] = self.data_pointers[self.last_dim - 1].len() - self.neg_upperbounds[self.last_dim];
    }

    // init new data pointers
    self.new_data_pointers.push(self.new_data.borrow_mut() as *mut Vec<Value>);
    for (i, &di) in self.unfrozen_dims.iter().skip(1).rev().enumerate() {
      unsafe {
        (*self.new_data_pointers[i]).push(Value::Array(Vec::with_capacity(
          self.upperbounds[di] - self.lowerbounds[di]
        )));
      }

      let ptr = unsafe {
        (*self.new_data_pointers[i]).last_mut().unwrap().as_mut_array() as *mut Vec<Value>
      };
      self.new_data_pointers.push(ptr);
    }
  }
}