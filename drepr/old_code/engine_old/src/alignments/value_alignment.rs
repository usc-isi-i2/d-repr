use crate::readers::{Index, Value, RAReader};
use crate::models::{Variable};
use fnv::FnvHashMap;
use super::MAlignmentFunc;
use crate::iterators::StreamingIndexIterator;

#[derive(Debug)]
pub struct ValueAlignFunc<'a> {
  unbounded_dims: Vec<usize>,
  index: FnvHashMap<&'a Value, Vec<Vec<usize>>>,
}

impl<'a> ValueAlignFunc<'a> {
  pub fn new<R: RAReader>(ra_reader: &'a R, target: &Variable) -> ValueAlignFunc<'a> {
    let mut index: FnvHashMap<&'a Value, Vec<Vec<usize>>> = FnvHashMap::default();
    let unbounded_dims = target.location.get_unbounded_dims();

    let mut iter = ra_reader.iter_data(&target.location);

    if unbounded_dims[0] == 0 {
      loop {
        let value = ra_reader.get_value(&iter.value(), 0);
        match index.get_mut(&value) {
          None => {
            index.insert(
              value,
              vec![ValueAlignFunc::shorten_index(iter.value(), &unbounded_dims)],
            );
          }
          Some(indices) => {
            indices.push(ValueAlignFunc::shorten_index(iter.value(), &unbounded_dims));
          }
        }

        if !iter.advance() {
          break;
        }
      }
    } else {
      // get the tree to eliminate some cost of accessing data
      let node = ra_reader.get_value(&iter.value()[..unbounded_dims[0]], 0);

      loop {
        let value = node.get_value(&iter.value()[unbounded_dims[0]..], 0);
        match index.get_mut(&value) {
          None => {
            index.insert(
              value,
              vec![ValueAlignFunc::shorten_index(iter.value(), &unbounded_dims)],
            );
          }
          Some(indices) => {
            indices.push(ValueAlignFunc::shorten_index(iter.value(), &unbounded_dims));
          }
        }

        if !iter.advance() {
          break;
        }
      }
    }

    ValueAlignFunc { index, unbounded_dims }
  }

  #[inline]
  fn shorten_index(index: &[Index], unbounded_dims: &[usize]) -> Vec<usize> {
    unbounded_dims.iter().map(|&d| index[d].as_idx()).collect()
  }
}

pub struct ArrayIndexRefIterator<'a> {
  // current moving pivot
  pivot: usize,
  index: &'a mut [Index],
  // list of unbounded dimension of the original index
  unbounded_dims: &'a [usize],
  // list of indice that we are going to
  indices: &'a [Vec<usize>],
}

impl<'a> StreamingIndexIterator for ArrayIndexRefIterator<'a> {
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
    if self.pivot < self.indices.len() - 1 {
      self.pivot += 1;
      let idx = &self.indices[self.pivot];

      for (i, &dim) in self.unbounded_dims.iter().enumerate() {
        self.index[dim] = Index::Idx(idx[i]);
      }

      return true;
    }

    return false;
  }

  fn freeze_last_index(&mut self) {
    // We haven't seen use case of freezing last index of a reference index
    // current, if we support that, it requires alignment functions to have mutable reference
    // which may break the usage of other modules
    panic!("[BUG] Reference iterator does not support freeze last index");
  }
}


impl<'a0> MAlignmentFunc for ValueAlignFunc<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 self, _source: &[Index], source_val: &Value, target: &'a mut [Index]) -> Box<dyn StreamingIndexIterator + 'a> {
    let indices = &self.index[source_val];
    let idx = &indices[0];
    for (i, &dim) in self.unbounded_dims.iter().enumerate() {
      target[dim] = Index::Idx(idx[i]);
    }

    return Box::new(ArrayIndexRefIterator {
      pivot: 0,
      index: target,
      unbounded_dims: &self.unbounded_dims,
      indices
    });
  }
}