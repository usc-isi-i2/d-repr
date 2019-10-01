use fnv::FnvHashMap;
use readers::prelude::{Value, RAReader, Index};
use crate::lang::Attribute;
use crate::alignments::funcs::mul_value_align::MulValueAlignFunc;
use crate::alignments::SAlignmentFunc;

#[derive(Debug)]
pub struct SglValueAlignFunc<'a> {
  unbounded_dims: Vec<usize>,
  index: FnvHashMap<&'a Value, Vec<usize>>
}

impl<'a> SglValueAlignFunc<'a> {
  pub fn new(reader: &'a Box<dyn RAReader + 'a>, target: &Attribute) -> SglValueAlignFunc<'a> {
    let mut index: FnvHashMap<&'a Value, Vec<usize>> = FnvHashMap::default();
    let mut iter = reader.iter_index(&target.path);
    let unbounded_dims = target.path.get_nary_steps();
    
    loop {
      let value = reader.get_value(&iter.value(), 0);
      index.insert(value, MulValueAlignFunc::shorten_index(iter.value(), &unbounded_dims));
      if !iter.advance() {
        break;
      }
    }
    
    SglValueAlignFunc {
      unbounded_dims,
      index
    }
  }
}

impl<'a0> SAlignmentFunc for SglValueAlignFunc<'a0> {
  fn align<'a>(&mut self, _source_idx: &'a [Index], source_val: &Value, target_idx: &'a mut [Index]) -> &'a [Index] {
    let index = &self.index[source_val];
    for (i, &dim) in self.unbounded_dims.iter().enumerate() {
      target_idx[dim] = Index::Idx(index[i]);
    }
    
    target_idx
  }
  
  fn partial_align<'a>(&mut self, _source_idx: &'a [Index], source_val: &Value, target_idx: &'a mut [Index], _from_idx: usize) -> &'a [Index] {
    let index = &self.index[source_val];
    for (i, &dim) in self.unbounded_dims.iter().enumerate() {
      target_idx[dim] = Index::Idx(index[i]);
    }
    
    target_idx
  }
}