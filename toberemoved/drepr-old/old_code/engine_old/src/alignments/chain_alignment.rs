use crate::alignments::{SAlignmentFunc, MAlignmentFunc};
use crate::readers::{Index, Value};
use crate::iterators::*;

pub struct M2SChainAlignFunc<'a> {
  func0_target_index: Vec<Index>,
  func0: Box<dyn MAlignmentFunc + 'a>,
  func1: Box<dyn SAlignmentFunc>,
}

pub struct SFuncStreamIndexIterator<'a> {
  index: &'a mut [Index],
  null_value: Value,
  stream0: Box<dyn StreamingIndexIterator + 'a>,
  func: &'a Box<dyn SAlignmentFunc>,
}

impl<'a> M2SChainAlignFunc<'a> {
  pub fn new(immediate_target_index: Vec<Index>, func0: Box<dyn MAlignmentFunc + 'a>, func1: Box<dyn SAlignmentFunc>) -> M2SChainAlignFunc<'a> {
    return M2SChainAlignFunc {
      func0_target_index: immediate_target_index,
      func0, func1
    }
  }
}

impl<'a> StreamingIndexIterator for SFuncStreamIndexIterator<'a> {
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
    if !self.stream0.advance() {
      return false;
    }

    self.func.align(self.stream0.value(), &self.null_value, self.index);
    return true;
  }

  fn freeze_last_index(&mut self) {
    // We haven't seen use case of freezing last index of a reference index
    // current, if we support that, it requires alignment functions to have mutable reference
    // which may break the usage of other modules
    panic!("[BUG] Reference iterator does not support freeze last index");
  }
}

impl<'a0> MAlignmentFunc for M2SChainAlignFunc<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 self, source: &[Index], source_val: &Value, target: &'a mut [Index]) -> Box<StreamingIndexIterator + 'a> {
    let immediate_target = unsafe {
      (*(&self.func0_target_index as *const Vec<Index> as *mut Vec<Index>)).as_mut_slice()
    };
    let stream0 = self.func0.iter_alignments(source, source_val, immediate_target);
    self.func1.align(stream0.value(), &Value::Null, target);
    return Box::new(SFuncStreamIndexIterator {
      index: target,
      null_value: Value::Null,
      stream0,
      func: &self.func1,
    });
  }
}


