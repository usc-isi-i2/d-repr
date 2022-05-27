use crate::iterators::*;
use crate::models::*;
use super::{ Value, Index };

#[inline]
pub fn reader_iter_data<'a, R: RAReader>(
  reader: &'a R,
  loc: &Location,
) -> Box<dyn StreamingIndexIterator + 'a> {
  let mut index: Vec<Index> = vec![];
  let mut unfrozen_dims: Vec<usize> = vec![];
  let mut unknown_upperbounds: Vec<bool> = vec![];

  let mut lowerbounds: Vec<usize> = vec![];
  let mut upperbounds: Vec<usize> = vec![];
  let mut neg_upperbounds: Vec<usize> = vec![];
  let mut steps: Vec<usize> = vec![];

  let mut has_unknown_dim: bool = false;

  for (i, slice) in loc.slices.iter().enumerate() {
    match slice {
      Slice::Range(s) => {
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
      Slice::Index(s) => {
        index.push(s.idx.clone());
        // this won't be used as it is frozen dim
        lowerbounds.push(0);
        upperbounds.push(0);
        neg_upperbounds.push(0);
        steps.push(0);
        unknown_upperbounds.push(false);
      }
    }
  }

  unfrozen_dims.reverse();
  if has_unknown_dim {
    Box::new(UnknownSizeIter::new(
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
    Box::new(KnownSizeIter::new(
      index,
      unfrozen_dims,
      lowerbounds,
      upperbounds,
      steps,
    ))
  }
}

pub trait RAReader {
  fn into_value(self) -> Value;
  fn get_value(&self, index: &[Index], start_idx: usize) -> &Value;
  fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value;
  fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value);
  fn len(&self) -> usize;
  fn remove(&mut self, index: &Index);

  fn ground_location(&self, loc: &mut Location, start_idx: usize);
  fn can_change_value_type(&mut self);
  fn iter_data<'a>(&'a self, loc: &Location) -> Box<dyn StreamingIndexIterator + 'a>;
}
