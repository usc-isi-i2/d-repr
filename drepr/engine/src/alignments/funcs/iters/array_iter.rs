use readers::prelude::{Index, IndexIterator};

#[derive(Debug)]
pub struct ArrayIndexRefIterator<'a> {
  // current moving pivot
  pivot: usize,
  index: &'a mut [Index],
  // list of unbounded dimension of the original index
  unbounded_dims: &'a [usize],
  // list of indice that we are going to
  indices: &'a [Vec<usize>],
}

impl<'a> ArrayIndexRefIterator<'a> {
  pub fn new(index: &'a mut [Index], unbounded_dims: &'a [usize], indices: &'a [Vec<usize>]) -> ArrayIndexRefIterator<'a> {
    ArrayIndexRefIterator {
      pivot: 0,
      index,
      unbounded_dims,
      indices
    }
  }
}

impl<'a> IndexIterator for ArrayIndexRefIterator<'a> {
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
    if self.pivot < self.indices.len() - 1 {
      self.pivot += 1;
      let idx = &self.indices[self.pivot];

      for (i, &dim) in self.unbounded_dims.iter().enumerate() {
        self.index[dim] = Index::Idx(idx[i]);
      }

      return true;
    }

    return false;
  }

  fn freeze_last_step(&mut self) {
    unreachable!()
  }
}