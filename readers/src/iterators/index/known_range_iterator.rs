use super::IndexIterator;
use crate::index::Index;

/// An iterator that loops through each range step in the path expression and yield the index of
/// each item in the loop (we define a macro that generates implementation of the iterator)
///
/// A range step at position `i` in the path is defined by `lowerbounds[i]` (start),
/// `upperbounds[i]` (end) and `steps[i]` (step). The length of vectors `lowerbounds`, `upperbounds`,
/// `steps` are all equal, and represent the path from the root of the resource tree. In other words,
/// the index that this iterator yields may be the index of a subtree or of a leaf node.
///
/// Position of range steps are keep tracked at the vector `unfrozen_dims` in reverse order (optimize for performance).
/// So that we can loop through each position of each range step and update the index correctly.
///
/// To optimize for the performance, the index that the iterator produces is stored in an external
/// vector `index` outside of the iterator and the iterator modifies the index after each iteration.
///
/// # Examples
///
///
/// # Structure
///
/// Below is the basic structure that the struct need to have in order to use the
///
/// ```
/// use reader::prelude::Index;
///
/// struct KnownRangeIter {
///  // |lowerbounds| = |upperbounds| = |step| <= |index|
///  // |unfrozen_dims| < |steps|
///  lowerbounds: Vec<usize>,
///  upperbounds: Vec<usize>,
///  steps: Vec<usize>,
///  // unfrozen_dims is in reverse order
///  unfrozen_dims: Vec<usize>,
///  index: Vec<Index>,
///  has_more: bool,
/// }
/// ```
macro_rules! generate_range_iter {
  (get_index_type, $a:lifetime) => { &'a mut [Index] };
  (get_index_type,) => { Vec<Index> };
  ($class:ident $( <$a:lifetime>)?) => {
#[derive(Debug)]
pub struct $class$( <$a> )? {
  lowerbounds: $( &$a mut )? Vec<usize>,
  upperbounds: $( &$a mut )? Vec<usize>,
  steps: $( &$a mut )? Vec<usize>,
  // unfrozen_dims is in reverse order
  unfrozen_dims: $( &$a mut )? Vec<usize>,
  index: generate_range_iter!(get_index_type, $( $a )?),
  has_more: bool,
}

impl$( <$a> )? $class$( <$a> )? {
  pub fn new(
    index: generate_range_iter!(get_index_type, $( $a )?),
    unfrozen_dims: $( &$a mut )? Vec<usize>,
    lowerbounds: $( &$a mut )? Vec<usize>,
    upperbounds: $( &$a mut )? Vec<usize>,
    steps: $( &$a mut )? Vec<usize>,
  ) -> $class$( <$a> )? {
    $class {
      index,
      unfrozen_dims,
      lowerbounds,
      upperbounds,
      steps,
      has_more: true,
    }
  }
}

impl$( <$a> )? IndexIterator for $class$( <$a> )? {
  #[inline]
  fn value(&self) -> &[Index] {
    &self.index
  }

  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    &mut self.index
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
              // successfully advance
              return true;
            }
          }
          _ => unreachable!(),
        }
      }

      self.has_more = false;
    }

    return false;
  }

  fn freeze_last_step(&mut self) {
    if self.unfrozen_dims[0] == self.steps.len() - 1 {
      self.unfrozen_dims.drain(..1);
    }
    self.steps.pop();
    self.upperbounds.pop();
    self.lowerbounds.pop();
  }
}
  }
}

generate_range_iter!(KnownRangeIter);
generate_range_iter!(KnownRangeRefIter<'a>);
