use super::StreamingIndexIterator;
use crate::readers::Index;

#[derive(Debug)]
pub struct KnownSizeIter {
  // |lowerbounds| = |upperbounds| = |step| = |index|
  // |unfrozen_dims| < |index|
  // unfrozen_dims is in reverse order
  lowerbounds: Vec<usize>,
  upperbounds: Vec<usize>,
  steps: Vec<usize>,
  unfrozen_dims: Vec<usize>,
  index: Vec<Index>,
  has_more: bool,
}

impl KnownSizeIter {
  pub fn new(
    index: Vec<Index>,
    unfrozen_dims: Vec<usize>,
    lowerbounds: Vec<usize>,
    upperbounds: Vec<usize>,
    steps: Vec<usize>,
  ) -> KnownSizeIter {
    KnownSizeIter {
      index,
      unfrozen_dims,
      lowerbounds,
      upperbounds,
      steps,
      has_more: true,
    }
  }
}

impl StreamingIndexIterator for KnownSizeIter {
  #[inline]
  fn value(&self) -> &[Index] {
    return &self.index;
  }

  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    return &mut self.index;
  }

  #[inline]
  fn advance(&mut self) -> bool {
    if self.has_more {
      for &dim_pivot in &self.unfrozen_dims {
        match &mut self.index[dim_pivot] {
          Index::Idx(idx) => {
            *idx += self.steps[dim_pivot];
            if *idx >= self.upperbounds[dim_pivot] {
              *idx = self.lowerbounds[dim_pivot] as usize;
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
    if self.unfrozen_dims[0] == self.index.len() - 1 {
      self.unfrozen_dims.drain(..1);
    }
    self.steps.pop();
    self.upperbounds.pop();
    self.lowerbounds.pop();
  }
}
