use crate::alignments::SAlignmentFunc;
use readers::prelude::{Index, Value};
use readers::ra_reader::RAReader;
use crate::lang::Attribute;

#[derive(Debug)]
pub struct SglChainAlign<'a> {
  // |readers| = |funcs| - 1, we get the value from index that the function yield from the corresponding reader
  readers: Vec<&'a Box<dyn RAReader + 'a>>,
  ys: Vec<Vec<Index>>,
  funcs: Vec<Box<dyn SAlignmentFunc + 'a>>,
}

impl<'a> SglChainAlign<'a> {
  pub fn new(readers: &'a [Box<dyn RAReader + 'a>], attrs: Vec<&Attribute>, funcs: Vec<Box<dyn SAlignmentFunc + 'a>>) -> SglChainAlign<'a> {
    SglChainAlign {
      readers: attrs.iter().map(|a| &readers[a.resource_id]).collect::<Vec<_>>(),
      ys: attrs.iter()
        .map(|a| a.path.get_initial_step(readers[a.resource_id].as_ref())).collect::<Vec<_>>(),
      funcs
    }
  }
}

impl<'a0> SAlignmentFunc for SglChainAlign<'a0> {
  fn align<'a>(&mut self, source_idx: &'a [Index], source_val: &Value, target_idx: &'a mut [Index]) -> &'a [Index] {
    self.funcs[0].align(source_idx, source_val, &mut self.ys[0]);
    let mut val = self.readers[0].get_value(&self.ys[0], 0);
    for i in 1..self.ys.len() {
      self.funcs[i].align(unsafe { &(*(&self.ys[i-1] as *const Vec<Index>)) }, val, &mut self.ys[i]);
      val = self.readers[i].get_value(&self.ys[i], 0);
    }
    self.funcs[self.ys.len()].align(&self.ys[self.ys.len() - 1], val, target_idx);
    target_idx
  }

  fn partial_align<'a>(&mut self, source_idx: &'a [Index], source_val: &Value, target_idx: &'a mut [Index], from_idx: usize) -> &'a [Index] {
    self.funcs[0].partial_align(source_idx, source_val, &mut self.ys[0], from_idx);
    let mut val = self.readers[0].get_value(&self.ys[0], 0);
    for i in 1..self.ys.len() {
      self.funcs[i].partial_align(unsafe { &(*(&self.ys[i-1] as *const Vec<Index>)) }, val, &mut self.ys[i], from_idx);
      val = self.readers[i].get_value(&self.ys[i], 0);
    }
    self.funcs[self.ys.len()].partial_align(&self.ys[self.ys.len() - 1], val, target_idx, from_idx);
    target_idx
  }
}