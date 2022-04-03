use crate::alignments::AlignmentFunc;
use readers::iterators::IndexIterator;
use readers::prelude::{Index, RAReader, Value};

#[derive(Debug)]
pub struct AlignsIter<'a0: 'a, 'a> {
  pub readers: &'a [&'a0 Box<dyn RAReader + 'a0>],
  // final target
  pub target_index: &'a mut [Index],
  // list of align functions: [f0, f1, ..., fn] in the chain |align_funcs| = n+1
  pub align_funcs: &'a mut [AlignmentFunc<'a0>],
  // list of iterators that iterate through values generated from m-alignment function (set-func)
  pub sub_iters: Vec<Box<dyn IndexIterator + 'a>>,
  // list of index of m-alignment function in the align_funcs list
  pub sub_iter_index: Vec<usize>,
  // intermediate indices that are generated from align funcs |ys| = n
  // use box because this is used to reference from sub_iters
  pub ys: &'a mut [Box<Vec<Index>>],
  pub has_more: bool,
}

impl<'a0: 'a, 'a> IndexIterator for AlignsIter<'a0, 'a> {
  #[inline]
  fn value(&self) -> &[Index] {
    self.target_index
  }
  #[inline]
  fn mut_value(&mut self) -> &mut [Index] {
    self.target_index
  }
  fn advance(&mut self) -> bool {
    if self.has_more {
      for (sub_iter, &sub_iter_idx) in self.sub_iters.iter_mut().zip(self.sub_iter_index.iter()) {
        // advance the correspondence sub_iter, in case we already advanced, it means we done on this dimension
        if sub_iter.advance() {
          // successfully advance, we may need to update the sub_iters
          // if the current dimension is not the last dimension
          // all of the intermediate and target index is updated in the sub_iters, or manually via
          // align function is self.update_sub_iters
          if sub_iter_idx < self.align_funcs.len() - 1 {
            self.update_sub_iters(sub_iter_idx);
          }
          return true;
        }
      }
      self.has_more = false;
    }
    return false;
  }
  fn freeze_last_step(&mut self) {
    unreachable!()
  }
}

impl<'a0: 'a, 'a> AlignsIter<'a0, 'a> {
  ///
  pub fn new(
    readers: &'a [&'a0 Box<dyn RAReader + 'a0>],
    align_funcs: &'a mut [AlignmentFunc<'a0>],
    ys: &'a mut [Box<Vec<Index>>],
    source: &[Index],
    source_val: &Value,
    target: &'a mut [Index],
  ) -> AlignsIter<'a0, 'a> {
    let mut sub_iters = vec![];
    let mut sub_iter_index = vec![];
    // gen for the source attribute
    match unsafe { &mut (*(&mut align_funcs[0] as *mut AlignmentFunc)) } {
      AlignmentFunc::Single(sfunc) => {
        sfunc.align(source, source_val, &mut ys[0]);
      }
      AlignmentFunc::Multiple(mfunc) => {
        sub_iter_index.push(0);
        sub_iters.push(mfunc.iter_alignments(source, source_val, unsafe {
          &mut (*(ys[0].as_mut() as *mut Vec<Index>))
        }));
      }
    }
    // gen for the intermediate attributes
    let mut val = readers[0].get_value(unsafe { &(*(ys[0].as_ref() as *const Vec<Index>)) }, 0);
    for i in 1..ys.len() {
      match unsafe { &mut (*(&mut align_funcs[i] as *mut AlignmentFunc)) } {
        AlignmentFunc::Single(sfunc) => {
          sfunc.align(
            unsafe { &(*(ys[i - 1].as_ref() as *const Vec<Index>)) },
            val,
            &mut ys[i],
          );
        }
        AlignmentFunc::Multiple(mfunc) => {
          sub_iter_index.push(i);
          sub_iters.push(mfunc.iter_alignments(
            unsafe { &(*(ys[i - 1].as_ref() as *const Vec<Index>)) },
            val,
            unsafe { &mut (*(ys[i].as_mut() as *mut Vec<Index>)) },
          ))
        }
      }
      val = readers[i - 1].get_value(&ys[i], 0);
    }
    // target attribute
    match unsafe { &mut (*(&mut align_funcs[ys.len()] as *mut AlignmentFunc)) } {
      AlignmentFunc::Single(sfunc) => {
        sfunc.align(
          unsafe { &(*(ys[ys.len() - 1].as_ref() as *const Vec<Index>)) },
          val,
          target,
        );
      }
      AlignmentFunc::Multiple(mfunc) => {
        sub_iter_index.push(ys.len());
        sub_iters.push(mfunc.iter_alignments(
          unsafe { &(*(ys[ys.len() - 1].as_ref() as *const Vec<Index>)) },
          val,
          unsafe { &mut (*(target as *mut [Index])) },
        ))
      }
    }
    AlignsIter {
      readers,
      target_index: target,
      align_funcs,
      sub_iters,
      sub_iter_index,
      ys,
      has_more: false,
    }
  }
  /// Update the sub-iterators
  ///
  fn update_sub_iters(&mut self, from_idx: usize) {
    for i in from_idx..self.align_funcs.len() {
      // update the list of intermediate indices
      match unsafe { &mut (*(&mut self.align_funcs[i] as *mut AlignmentFunc)) } {
        AlignmentFunc::Single(sfunc) => {
          sfunc.align(
            unsafe { &(*(self.ys[i - 1].as_ref() as *const Vec<Index>)) },
            self.readers[i].get_value(&self.ys[i], 0),
            self.ys[i].as_mut(),
          );
        }
        AlignmentFunc::Multiple(mfunc) => {
          self.sub_iters[i] = mfunc.iter_alignments(
            unsafe { &(*(self.ys[i - 1].as_ref() as *const Vec<Index>)) },
            self.readers[i].get_value(&self.ys[i], 0),
            unsafe { &mut (*(self.ys[i].as_mut() as *mut Vec<Index>)) },
          );
        }
      }
    }
  }
}
