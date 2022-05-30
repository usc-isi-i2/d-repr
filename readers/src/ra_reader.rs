use std::fmt::Debug;

use crate::index::Index;
use crate::iterators::*;
use crate::path_expr::{PathExpr, StepExpr};
use crate::value::Value;

pub trait RAReader: Debug {
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value);
  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value;
  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value;
  fn len(&self) -> usize;
  fn remove(&mut self, index: &Index);
  fn ground_path(&self, path: &mut PathExpr, start_idx: usize);
  fn iter_index<'a>(&'a self, path: &PathExpr) -> Box<dyn IndexIterator + 'a>;
}

#[inline]
pub(super) fn default_iter_index<'a, R: RAReader>(
  reader: &'a R,
  path: &PathExpr,
) -> Box<dyn IndexIterator + 'a> {
  let mut index: Vec<Index> = vec![];
  let mut unfrozen_dims: Vec<usize> = vec![];
  let mut unknown_upperbounds: Vec<bool> = vec![];

  let mut lowerbounds: Vec<usize> = vec![];
  let mut upperbounds: Vec<usize> = vec![];
  let mut neg_upperbounds: Vec<usize> = vec![];
  let mut steps: Vec<usize> = vec![];

  let mut has_unknown_dim: bool = false;

  for (i, slice) in path.steps.iter().enumerate() {
    match slice {
      StepExpr::Range(s) => {
        unfrozen_dims.push(i);
        index.push(Index::Idx(s.start as usize));

        lowerbounds.push(s.start as usize);
        match s.end {
          Some(v) => {
            if v > 0 {
              upperbounds.push(v as usize);
              unknown_upperbounds.push(false);
              neg_upperbounds.push(0);
            } else {
              upperbounds.push(0);
              neg_upperbounds.push(-v as usize);
              unknown_upperbounds.push(true);
              has_unknown_dim = true;
            }
          }
          None => {
            upperbounds.push(0); // random number as it doesn't matter
            neg_upperbounds.push(0);
            unknown_upperbounds.push(true);
            has_unknown_dim = true;
          }
        }
        steps.push(s.step);
      }
      StepExpr::Index(s) => {
        index.push(s.val.clone());
        // this won't be used as it is frozen dim
        lowerbounds.push(0);
        upperbounds.push(0);
        neg_upperbounds.push(0);
        steps.push(0);
        unknown_upperbounds.push(false);
      }
      _ => unimplemented!("{:?}", slice),
    }
  }

  unfrozen_dims.reverse();
  if has_unknown_dim {
    Box::new(UnknownRangeIter::new(
      reader,
      index,
      unfrozen_dims,
      unknown_upperbounds,
      lowerbounds,
      upperbounds,
      neg_upperbounds,
      steps,
    ))
  } else {
    Box::new(KnownRangeIter::new(
      index,
      unfrozen_dims,
      lowerbounds,
      upperbounds,
      steps,
    ))
  }
}
