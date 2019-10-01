use super::StreamingIndexIterator;
use crate::readers::{Index, RAReader, Value};

#[derive(Debug)]
pub struct UnknownSizeIter<'a, R: RAReader> {
  ra_reader: &'a R,
  // |lowerbounds| = |upperbounds| = |step| = |unknown_upperbounds| = |index|
  // |unfrozen_dims| < |index| and in reversed order
  lowerbounds: Vec<usize>,
  upperbounds: Vec<usize>,
  // the negative upperbounds are used for the case that the ending is negative
  neg_upperbounds: Vec<usize>,
  steps: Vec<usize>,
  unfrozen_dims: Vec<usize>, // reverse order
  last_unknown_dim: usize,
  unknown_upperbounds: Vec<bool>,
  index: Vec<Index>,
  has_more: bool,
  tree_ptrs: Vec<&'a Value>,
}

pub fn create_tree_ptrs_and_update_unknown_upperbound<'a, R: RAReader>(
  ra_reader: &'a R,
  index: &[Index],
  last_unknown_dim: usize,
  unknown_upperbounds: &[bool],
  neg_upperbounds: &[usize],
  upperbounds: &mut [usize],
) -> Vec<&'a Value> {
  // upperbounds[0] is always known
//  if unknown_upperbounds[0] {
//    upperbounds[0] = ra_reader.len() - neg_upperbounds[0];
//  }
  let mut tree_ptrs = vec![ra_reader.get_value(&index[..1], 0)];
  // only need to compute tree_ptrs until the last unknown dimension (exclusive)
  for i in 1..last_unknown_dim {
    if unknown_upperbounds[i] {
      upperbounds[i] = tree_ptrs[i - 1].len() - neg_upperbounds[i];
    }
    tree_ptrs.push(tree_ptrs[i - 1].get_child_value(&index[i]));
  }

  if last_unknown_dim > 0 {
    // now update the last unknown dimension
    upperbounds[last_unknown_dim] = tree_ptrs[last_unknown_dim - 1].len() - neg_upperbounds[last_unknown_dim];
  }
  tree_ptrs
}

/// recalculate correct upper bounds of the current index. This always start from second dimension
/// i.e., start_idx = 0, means we figure out upper bounds from the second dimensions (the size of
/// first dimension is always known)
pub fn update_local_upperbounds<'a, R: RAReader>(
  ra_reader: &'a R,
  tree_ptrs: &mut [&'a Value],
  index: &[Index],
  last_unknown_dim: usize,
  unknown_upperbounds: &[bool],
  neg_upperbounds: &[usize],
  upperbounds: &mut [usize],
  mut start_idx: usize,
) {
  if start_idx == 0 {
    tree_ptrs[0] = ra_reader.get_value(&index[..1], 0);
    start_idx += 1;
  }

  for i in start_idx..last_unknown_dim {
    // upper bound of dim `i` is unknown
    if unknown_upperbounds[i] {
      upperbounds[i] = tree_ptrs[i - 1].len() - neg_upperbounds[i];
    }
    tree_ptrs[i] = tree_ptrs[i - 1].get_child_value(&index[i]);
  }

  if last_unknown_dim > 0 {
    upperbounds[last_unknown_dim] = tree_ptrs[last_unknown_dim - 1].len() - neg_upperbounds[last_unknown_dim];
  }
}

impl<'a, R: RAReader> UnknownSizeIter<'a, R> {
  pub fn new(
    ra_reader: &R,
    index: Vec<Index>,
    unfrozen_dims: Vec<usize>,
    unknown_upperbounds: Vec<bool>,
    lowerbounds: Vec<usize>,
    mut upperbounds: Vec<usize>,
    neg_upperbounds: Vec<usize>,
    steps: Vec<usize>,
  ) -> UnknownSizeIter<R> {
    // reverse unfrozen_dims to save some minus operators
    let mut last_unknown_dim = 0;
    for (i, &is_unknown) in unknown_upperbounds.iter().enumerate().rev() {
      if is_unknown {
        last_unknown_dim = i;
        break;
      }
    }

    let tree_ptrs = create_tree_ptrs_and_update_unknown_upperbound(
      ra_reader,
      &index,
      last_unknown_dim,
      &unknown_upperbounds,
      &neg_upperbounds,
      &mut upperbounds,
    );

    UnknownSizeIter {
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

impl<'a, R: RAReader> StreamingIndexIterator for UnknownSizeIter<'a, R> {
  #[inline]
  fn value(&self) -> &[Index] {
    return &self.index;
  }

  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    return &mut self.index;
  }

  fn advance(&mut self) -> bool {
    if self.has_more {
      for &dim_pivot in &self.unfrozen_dims {
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
    if self.unfrozen_dims[0] == self.index.len() - 1 {
      self.unfrozen_dims.drain(..1);
    }
    self.steps.pop();
    self.upperbounds.pop();
    self.neg_upperbounds.pop();
    self.lowerbounds.pop();

    if let Some(_) = self.unknown_upperbounds.pop() {
      // update the last unknown dim
      self.last_unknown_dim = 0;
      for (i, &is_unknown) in self.unknown_upperbounds.iter().enumerate().rev() {
        if is_unknown {
          self.last_unknown_dim = i;
          break;
        }
      }

      // as the last unknown dim change, we also need to update tree_ptrs
      self.tree_ptrs.pop();
    }
  }
}
