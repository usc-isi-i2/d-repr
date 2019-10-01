use crate::iterators::index::unknown_size_iterator::{
  create_tree_ptrs_and_update_unknown_upperbound, update_local_upperbounds,
};
use crate::readers::{Index, RAReader, Value};

use super::StreamingIndexIterator;

#[derive(Debug)]
pub struct UnknownSizeRefIter<'a, R: RAReader> {
  ra_reader: &'a R,
  lowerbounds: &'a Vec<usize>,
  upperbounds: Vec<usize>,
  neg_upperbounds: &'a Vec<usize>,
  steps: &'a Vec<usize>,
  unfrozen_dims: &'a Vec<usize>,
  last_unknown_dim: usize,
  unknown_upperbounds: &'a Vec<bool>,
  index: &'a mut [Index],
  has_more: bool,
  tree_ptrs: Vec<&'a Value>,
}

impl<'a, R: RAReader> UnknownSizeRefIter<'a, R> {
  pub fn new(
    ra_reader: &'a R,
    index: &'a mut [Index],
    unfrozen_dims: &'a Vec<usize>,
    unknown_upperbounds: &'a Vec<bool>,
    last_unknown_dim: usize,
    lowerbounds: &'a Vec<usize>,
    mut upperbounds: Vec<usize>,
    neg_upperbounds: &'a Vec<usize>,
    steps: &'a Vec<usize>,
  ) -> UnknownSizeRefIter<'a, R> {
    let tree_ptrs = create_tree_ptrs_and_update_unknown_upperbound(
      ra_reader,
      &index,
      last_unknown_dim,
      &unknown_upperbounds,
      &neg_upperbounds,
      &mut upperbounds,
    );

    UnknownSizeRefIter {
      ra_reader,
      index,
      unfrozen_dims,
      unknown_upperbounds,
      last_unknown_dim,
      lowerbounds,
      upperbounds,
      neg_upperbounds,
      steps,
      has_more: true,
      tree_ptrs,
    }
  }
}

impl<'a, R: RAReader> StreamingIndexIterator for UnknownSizeRefIter<'a, R> {
  fn value(&self) -> &[Index] {
    return self.index;
  }

  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    return self.index;
  }

  fn advance(&mut self) -> bool {
    if self.has_more {
      for &dim_pivot in self.unfrozen_dims.iter() {
        match &mut self.index[dim_pivot] {
          Index::Idx(idx) => {
            *idx += self.steps[dim_pivot];
            if *idx >= self.upperbounds[dim_pivot] {
              *idx = self.lowerbounds[dim_pivot] as usize;
            } else {
              // successfully advance, the upperbounds only change if the current dimension
              // is before the last unknown dimensions
              if dim_pivot < self.last_unknown_dim {
                update_local_upperbounds(
                  self.ra_reader,
                  &mut self.tree_ptrs,
                  &self.index,
                  self.last_unknown_dim,
                  &self.unknown_upperbounds,
                  &self.neg_upperbounds,
                  &mut self.upperbounds,
                  dim_pivot,
                );
              }

              return true;
            }
          }
          _ => panic!("[BUG]"),
        }
      }

      self.has_more = false;
    }

    return false;
  }

  fn freeze_last_index(&mut self) {
    // We haven't seen use case of freezing last index of a reference index
    // current, if we support that, it requires alignment functions to have mutable reference
    // which may break the usage of other modules
    panic!("[BUG] Reference iterator does not support freeze last index");
  }
}
