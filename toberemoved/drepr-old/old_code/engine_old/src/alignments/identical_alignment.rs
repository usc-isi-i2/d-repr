use crate::alignments::SAlignmentFunc;
use crate::readers::{Index, Value};

pub struct IdenticalAlignment {}

impl SAlignmentFunc for IdenticalAlignment {
  fn align<'a>(
    &self,
    source: &'a [Index],
    _source_val: &Value,
    _target: &'a mut [Index],
  ) -> &'a [Index] {
    source
  }

  fn partial_align<'a>(
    &self,
    source: &'a [Index],
    _source_val: &Value,
    _target: &'a mut [Index],
    _from_idx: usize,
  ) -> &'a [Index] {
    source
  }
}
