use crate::alignments::SAlignmentFunc;
use readers::prelude::{Index, Value};

#[derive(Debug)]
pub struct IdenticalAlignment {}

impl SAlignmentFunc for IdenticalAlignment {
  fn align<'a>(
    &mut self,
    source: &'a [Index],
    _source_val: &Value,
    _target: &'a mut [Index],
  ) -> &'a [Index] {
    source
  }

  fn partial_align<'a>(
    &mut self,
    source: &'a [Index],
    _source_val: &Value,
    _target: &'a mut [Index],
    _from_idx: usize,
  ) -> &'a [Index] {
    source
  }
}
