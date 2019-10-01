use crate::readers::Index;
use super::StreamingIndexIterator;

#[derive(Debug)]
pub struct KnownSizeRefIter<'a> {
  // |lowerbounds| = |upperbounds| = |step| = |index|
  // |unfrozen_dims| < |index|
  // unfrozen_dims is in reverse order
  lowerbounds: &'a Vec<usize>,
  upperbounds: &'a Vec<usize>,
  steps: &'a Vec<usize>,
  unfrozen_dims: &'a Vec<usize>,
  index: &'a mut [Index],
  has_more: bool,
}

impl<'a> KnownSizeRefIter<'a> {
  pub fn new(
    index: &'a mut [Index],
    unfrozen_dims: &'a Vec<usize>,
    lowerbounds: &'a Vec<usize>,
    upperbounds: &'a Vec<usize>,
    steps: &'a Vec<usize>,
  ) -> KnownSizeRefIter<'a> {

    KnownSizeRefIter {
      index,
      unfrozen_dims,
      lowerbounds,
      upperbounds,
      steps,
      has_more: true,
    }
  }
}

impl<'a> StreamingIndexIterator for KnownSizeRefIter<'a> {
  #[inline]
  fn value(&self) -> &[Index] {
    self.index
  }

  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    self.index
  }

  #[inline]
  fn advance(&mut self) -> bool {
    if self.has_more {
      for &dim_pivot in self.unfrozen_dims.iter() {
        match &mut self.index[dim_pivot] {
          Index::Idx(idx) => {
            *idx += self.steps[dim_pivot];
            if *idx >= self.upperbounds[dim_pivot] {
              *idx = self.lowerbounds[dim_pivot];
            } else {
              // successfully advance
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
