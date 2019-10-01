use crate::iterators::StreamingIndexIterator;
use crate::readers::{Index, Value};

mod alignment_inference;
mod dimensional_alignment;
mod identical_alignment;
mod value_alignment;
mod chain_alignment;

pub use self::alignment_inference::BasicAlignmentInference;
pub use self::dimensional_alignment::*;
pub use self::identical_alignment::IdenticalAlignment;
pub use self::value_alignment::ValueAlignFunc;
pub use self::chain_alignment::M2SChainAlignFunc;
use std::fmt::{Debug, Error, Formatter};

pub enum AlignmentFunc<'a> {
  Single(Box<dyn SAlignmentFunc>),
  Multiple(Box<dyn MAlignmentFunc + 'a>),
}

impl<'a> AlignmentFunc<'a> {
  #[inline]
  pub fn as_single(&self) -> &dyn SAlignmentFunc {
    match self {
      AlignmentFunc::Single(a) => a.as_ref(),
      _ => panic!("Cannot convert multiple alignment into single alignment"),
    }
  }

  pub fn into_single(self) -> Box<dyn SAlignmentFunc> {
    match self {
      AlignmentFunc::Single(a) => a,
      _ => panic!("Cannot convert multiple alignment into single alignment")
    }
  }

  pub fn into_multiple(self) -> Box<dyn MAlignmentFunc + 'a> {
    match self {
      AlignmentFunc::Multiple(a) => a,
      _ => panic!("Cannot convert single alignment into multiple alignment")
    }
  }
}

/// single value alignment
pub trait SAlignmentFunc {
  /// align target's index to source's index
  fn align<'a>(
    &self,
    source: &'a [Index],
    source_val: &Value,
    target: &'a mut [Index],
  ) -> &'a [Index];

  ///similar to align func but ignore dimensions that's before from_idx in both source and target
  fn partial_align<'a>(
    &self,
    source: &'a [Index],
    source_val: &Value,
    target: &'a mut [Index],
    from_idx: usize,
  ) -> &'a [Index];
}

/// multiple value alignment
pub trait MAlignmentFunc {
  /// for multiple possible alignments between target's index and source's index
  fn iter_alignments<'a0: 'a, 'a>(
    &'a0 self,
    source: &[Index],
    source_val: &Value,
    target: &'a mut [Index],
  ) -> Box<dyn StreamingIndexIterator + 'a>;
}

impl<'a> Debug for AlignmentFunc<'a> {
  fn fmt(&self, _: &mut Formatter) -> Result<(), Error> {
    Ok(())
  }
}
