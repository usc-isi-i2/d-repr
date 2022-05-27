use std::cmp::{max, min};

use readers::prelude::{Index, Value};

use crate::alignments::SAlignmentFunc;
use crate::lang::{Description, RangeAlignment};

#[derive(Debug)]
pub struct AlignedRangeStep {
  pub source_dim: usize,
  pub source_start: usize,
  pub source_step: usize,
  
  pub target_dim: usize,
  pub target_start: usize,
  pub target_step: usize,
}

#[derive(Debug)]
pub struct SRangeAlignFunc {
  // at every value d at possible i: min(sources[d].dim, targets[d].dim) >= i
  // where i is a step index of the longest path of source or target attribute
  aligned_dim_index: Vec<usize>,
  // aligned dims is sorted by (i, j)
  aligned_dims: Vec<AlignedRangeStep>,
}

impl SRangeAlignFunc {
  pub fn from_dim_align(desc: &Description, dalign: &RangeAlignment) -> SRangeAlignFunc {
    let longest_path_len = max(desc.attributes[dalign.source].path.steps.len(), desc.attributes[dalign.target].path.steps.len());
    let mut aligned_dims = dalign.aligned_dims
      .iter()
      .map(|ad| {
        let sr = desc.attributes[dalign.source].path.steps[ad.source_dim].as_range();
        let tr = desc.attributes[dalign.target].path.steps[ad.target_dim].as_range();
        
        AlignedRangeStep {
          source_dim: ad.source_dim,
          source_start: sr.start,
          source_step: sr.step,
          target_dim: ad.target_dim,
          target_start: tr.start,
          target_step: tr.step,
        }
      })
      .collect::<Vec<_>>();
    
    // the max_n_dims should be calculated again
    aligned_dims.sort_by_key(|d| (d.source_dim, d.target_dim));
    
    SRangeAlignFunc {
      aligned_dim_index: create_aligned_range_step_index(
        longest_path_len,
        &aligned_dims),
      aligned_dims,
    }
  }
}

#[inline]
pub fn create_aligned_range_step_index(longest_path_length: usize, sorted_aligned_dims: &[AlignedRangeStep]) -> Vec<usize> {
  let mut aligned_dim_index = vec![0; longest_path_length];
  let mut i = 0;
  
  for (j, d) in sorted_aligned_dims.iter().enumerate() {
    let m = min(d.source_dim, d.target_dim);
    while i < m {
      aligned_dim_index[i] = j;
      i += 1;
    }
  }
  
  return aligned_dim_index;
}

#[inline]
pub fn update_range_step(source: &[Index], target: &mut [Index], dim: &AlignedRangeStep) {
  target[dim.target_dim].set_idx((source[dim.source_dim].as_idx() - dim.source_start) * (dim.target_step / dim.source_step) + dim.target_start)
}


impl SAlignmentFunc for SRangeAlignFunc {
  #[inline]
  fn align<'a>(&mut self, source: &'a [Index], _source_val: &Value, target: &'a mut [Index]) -> &'a [Index] {
    for dim in &self.aligned_dims {
      update_range_step(source, target, dim);
    }
    
    target
  }
  
  #[inline]
  fn partial_align<'a>(&mut self, source: &'a [Index], _source_val: &Value, target: &'a mut [Index], from_idx: usize) -> &'a [Index] {
    for i in self.aligned_dim_index[from_idx]..self.aligned_dims.len() {
      update_range_step(source, target, &self.aligned_dims[i]);
    }
    
    target
  }
}

