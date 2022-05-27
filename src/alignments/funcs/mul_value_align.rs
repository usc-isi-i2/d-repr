use readers::prelude::{Index, Value, RAReader, IndexIterator};
use crate::lang::{Attribute};
use fnv::FnvHashMap;
use crate::alignments::MAlignmentFunc;
use crate::alignments::funcs::iters::array_iter::ArrayIndexRefIterator;

#[derive(Debug)]
pub struct MulValueAlignFunc<'a> {
  unbounded_dims: Vec<usize>,
  index: FnvHashMap<&'a Value, Vec<Vec<usize>>>,
}

impl<'a> MulValueAlignFunc<'a> {
  pub fn new(ra_reader: &'a Box<dyn RAReader + 'a>, target: &Attribute) -> MulValueAlignFunc<'a> {
    let mut index: FnvHashMap<&'a Value, Vec<Vec<usize>>> = FnvHashMap::default();
    let unbounded_dims = target.path.get_nary_steps();
    
    let mut iter = ra_reader.iter_index(&target.path);

    if unbounded_dims[0] == 0 {
      loop {
        let value = ra_reader.get_value(&iter.value(), 0);
        match index.get_mut(&value) {
          None => {
            index.insert(
              value,
              vec![MulValueAlignFunc::shorten_index(iter.value(), &unbounded_dims)],
            );
          }
          Some(indices) => {
            indices.push(MulValueAlignFunc::shorten_index(iter.value(), &unbounded_dims));
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
              vec![MulValueAlignFunc::shorten_index(iter.value(), &unbounded_dims)],
            );
          }
          Some(indices) => {
            indices.push(MulValueAlignFunc::shorten_index(iter.value(), &unbounded_dims));
          }
        }

        if !iter.advance() {
          break;
        }
      }
    }

    MulValueAlignFunc { index, unbounded_dims }
  }

  #[inline]
  pub fn shorten_index(index: &[Index], unbounded_dims: &[usize]) -> Vec<usize> {
    unbounded_dims.iter().map(|&d| index[d].as_idx()).collect()
  }
}

impl<'a0> MAlignmentFunc for MulValueAlignFunc<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 mut self, _source: &[Index], source_val: &Value, target: &'a mut [Index]) -> Box<dyn IndexIterator + 'a> {
    let indices = &self.index[source_val];
    let idx = &indices[0];
    for (i, &dim) in self.unbounded_dims.iter().enumerate() {
      target[dim] = Index::Idx(idx[i]);
    }

    return Box::new(ArrayIndexRefIterator::new(target, &self.unbounded_dims, indices));
  }
}