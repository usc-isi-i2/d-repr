use crate::prelude::{Index, RAReader, Value};

use super::IndexIterator;

/// An iterator that loops through each range step in the path expression and yield the index of
/// each item in the loop.
///
/// A range step at position `i` in the path is defined by `lowerbounds[i]` (start),
/// (`upperbounds[i]` & `neg_upperbounds[i]`) (end), and `steps[i]` (step). The length of vectors `lowerbounds`, `upperbounds`,
/// `steps` are all equal, and represent the path from the root of the resource tree. In other words,
/// the index that this iterator yields may be the index of a subtree or of a leaf node. The range
/// step is said to be unknown if its end is None or a negative number, while the start is always present
/// and non-negative (see the supported path expression). Since, the upperbounds is always a positive
/// number, we can tell the range step is unknown if `unknown_upperbounds[i] = true`. The upperbound
/// item is changed dynamically depends on the data item, and is computed by: `upperbounds[i] - neg_upperbounds[i]`.
///
/// We keep track of each node at each step in the path in the `tree_ptrs` vector. The length of the
/// vector is equal to the position of the last unknown step in `last_unknown_dim`.
///
/// Position of range steps are keep tracked at the vector `unfrozen_dims` in reverse order (optimize for performance).
/// So that we can loop through each position of each range step and update the index correctly.
///
/// For example, for this path expression: `[.., authors, 2.., phone, ext]`, then:
///   * `last_unknown_dim = 2`
///   * `unknown_upperbounds = [true, false, true, false, false]`
///
/// If the current index is `[2, authors, 5, phone, ext]`, then:
///   * `tree_ptrs = [value(2), value("2:authors")]`
///
///
/// To optimize for the performance, the index that the iterator produces is stored in an external
/// vector `index` outside of the iterator and the iterator modifies the index after each iteration.
///
/// # Examples
///
/// # Structured
///
/// ```
/// use readers::prelude::{RAReader, Index, Value};
/// pub struct UnknownRangeIter<'a> {
///   ra_reader: &'a Box<dyn RAReader>,
///   // |lowerbounds| = |upperbounds| = |step| = |neg_upperbounds| <= |index|
///   // |unfrozen_dims| < |steps|
///   lowerbounds: Vec<usize>,
///   upperbounds: Vec<usize>,
///   neg_upperbounds: Vec<usize>,
///   steps: Vec<usize>,
///   // in reverse order
///   unfrozen_dims: Vec<usize>,
///
///   last_unknown_dim: usize,
///   // |unknown_upperbounds| = |steps|
///   unknown_upperbounds: Vec<bool>,
///   index: I,
///   has_more: bool,
///   tree_ptrs: Vec<&'a Value>,
/// }
/// ```
macro_rules! generate_unknown_range_iter {
  (get_index_type, $mut_kw:ident) => { &'a mut [Index] };
  (get_index_type,) => { Vec<Index> };
// use the `mut` keyword to identify if we use reference or not
  ($class:ident $(, $mut_kw:ident )?) => {
#[derive(Debug)]
pub struct $class<'a> {
  ra_reader: &'a dyn RAReader,
  // |lowerbounds| = |upperbounds| = |step| = |neg_upperbounds| <= |index|
  // |unfrozen_dims| < |steps|
  lowerbounds: $(&'a $mut_kw)? Vec<usize>,
  // the upperbound is changing, therefore, we should own it instead
  upperbounds: Vec<usize>,
  neg_upperbounds: $(&'a $mut_kw)? Vec<usize>,
  steps: $(&'a $mut_kw)? Vec<usize>,
  // in reverse order
  unfrozen_dims: $(&'a $mut_kw)? Vec<usize>,

  last_unknown_dim: usize,
  // |unknown_upperbounds| = |steps|
  unknown_upperbounds: $(&'a $mut_kw)? Vec<bool>,
  index: generate_unknown_range_iter!(get_index_type, $($mut_kw)?),
  has_more: bool,
  tree_ptrs: Vec<&'a Value>,
}

impl<'a> $class<'a> {
  pub fn new(
    ra_reader: &'a dyn RAReader,
    index: generate_unknown_range_iter!(get_index_type, $($mut_kw)?),
    unfrozen_dims: $(&'a $mut_kw)? Vec<usize>,
    unknown_upperbounds: $(&'a $mut_kw)? Vec<bool>,
    lowerbounds: $(&'a $mut_kw)? Vec<usize>,
    mut upperbounds: Vec<usize>,
    neg_upperbounds: $(&'a $mut_kw)? Vec<usize>,
    steps: $(&'a $mut_kw)? Vec<usize>,
  ) -> $class<'a> {
    let mut last_unknown_dim = 0;
    for (i, &is_unknown) in unknown_upperbounds.iter().enumerate().rev() {
      if is_unknown {
        last_unknown_dim = i;
        break;
      }
    }

    let tree_ptrs = create_tree_ptrs_and_update_unknown_upperbound(
      ra_reader,
      index.as_ref(),
      last_unknown_dim,
      &unknown_upperbounds,
      &neg_upperbounds,
      &mut upperbounds,
    );

    $class {
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

impl<'a> IndexIterator for $class<'a>
{
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
              // successfully advance, the upperbounds only change if the current dimension
              // is before the last unknown dimensions
              if dim_pivot < self.last_unknown_dim {
                update_local_upperbounds(
                  self.ra_reader,
                  &mut self.tree_ptrs,
                  self.index.as_ref(),
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
    self.neg_upperbounds.pop();
    self.unknown_upperbounds.pop();
    if self.last_unknown_dim >= self.unknown_upperbounds.len() {
      // is the last dimension, so we have to update the tree pointers
      for i in (0..self.unknown_upperbounds.len()).rev() {
        if self.unknown_upperbounds[i] {
          self.last_unknown_dim = i;
          break;
        }
        self.tree_ptrs.pop();
      }
    }
  }
}
  }
}

generate_unknown_range_iter!(UnknownRangeIter);
generate_unknown_range_iter!(UnknownRangeRefIter, mut);

pub fn create_tree_ptrs_and_update_unknown_upperbound<'a>(
  ra_reader: &'a dyn RAReader,
  index: &[Index],
  last_unknown_dim: usize,
  unknown_upperbounds: &[bool],
  neg_upperbounds: &[usize],
  upperbounds: &mut [usize],
) -> Vec<&'a Value> {
  // upperbounds[0] is always known
  if unknown_upperbounds[0] {
    upperbounds[0] = ra_reader.len() - neg_upperbounds[0];
  }

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
    upperbounds[last_unknown_dim] =
      tree_ptrs[last_unknown_dim - 1].len() - neg_upperbounds[last_unknown_dim];
  }
  tree_ptrs
}

/// recalculate correct upper bounds of the current index. This always start from second dimension
/// i.e., start_idx = 0, means we figure out upper bounds from the second dimensions (the size of
/// first dimension is always known)
pub fn update_local_upperbounds<'a>(
  ra_reader: &'a dyn RAReader,
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
    upperbounds[last_unknown_dim] =
      tree_ptrs[last_unknown_dim - 1].len() - neg_upperbounds[last_unknown_dim];
  }
}
