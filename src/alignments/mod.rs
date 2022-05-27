use std::fmt::{Debug, Error, Formatter};

use readers::prelude::{Index, IndexIterator, Value};
use readers::{as_enum_type_impl, into_enum_type_impl};

pub mod inference;
pub mod funcs;
pub mod func_builder;
mod dfs;

pub enum AlignmentFunc<'a> {
  Single(Box<dyn SAlignmentFunc + 'a>),
  Multiple(Box<dyn MAlignmentFunc + 'a>),
}

/// single value alignment
pub trait SAlignmentFunc: Debug {
  /// align target's index to source's index
  fn align<'a>(
    &mut self,
    source_idx: &'a [Index],
    source_val: &Value,
    target_idx: &'a mut [Index],
  ) -> &'a [Index];

  ///similar to align func but ignore dimensions that's before from_idx in both source and target
  fn partial_align<'a>(
    &mut self,
    source_idx: &'a [Index],
    source_val: &Value,
    target_idx: &'a mut [Index],
    from_idx: usize,
  ) -> &'a [Index];
}

/// multiple value alignment
pub trait MAlignmentFunc {
  /// for multiple possible alignments between target's index and source's index
  /// after this function is called, the target_idx is updated immediately because
  /// this is also the default behaviour of the index iterator
  fn iter_alignments<'a0: 'a, 'a>(
    &'a0 mut self,
    source_idx: &[Index],
    source_val: &Value,
    target_idx: &'a mut [Index],
  ) -> Box<dyn IndexIterator + 'a>;
}

impl<'a> Debug for AlignmentFunc<'a> {
  fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
    Ok(())
  }
}

impl<'a> AlignmentFunc<'a> {
  as_enum_type_impl!(AlignmentFunc, as_single, as_mut_single, Single, "single", Box<dyn SAlignmentFunc + 'a>);
  as_enum_type_impl!(AlignmentFunc, as_multiple, as_mut_multiple, Multiple, "multiple", Box<dyn MAlignmentFunc + 'a>);
  
  into_enum_type_impl!(AlignmentFunc, into_single, Single, "single", Box<dyn SAlignmentFunc + 'a>);
  into_enum_type_impl!(AlignmentFunc, into_multiple, Multiple, "multiple", Box<dyn MAlignmentFunc + 'a>);
}