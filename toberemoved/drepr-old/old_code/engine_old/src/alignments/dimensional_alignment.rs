use crate::alignments::{SAlignmentFunc, MAlignmentFunc};
use crate::models::*;
use crate::readers::{Index, Value, RAReader};
use crate::iterators::*;
use std::cmp::{min, max};

#[derive(Debug)]
pub struct AlignedDimension {
  pub source_dim: usize,
  pub source_start: usize,
  pub source_step: usize,

  pub target_dim: usize,
  pub target_start: usize,
  pub target_step: usize,
}

#[derive(Debug)]
pub struct SDimAlignFunc {
  // at every value d at possible i: min(sources[d].dim, targets[d].dim) >= i
  aligned_dim_index: Vec<usize>,
  // aligned dims is sorted by (i, j)
  aligned_dims: Vec<AlignedDimension>
}

impl SDimAlignFunc {
  pub fn new(max_n_dims: usize, mut aligned_dims: Vec<AlignedDimension>) -> SDimAlignFunc {
    aligned_dims.sort_by_key(|d| (d.source_dim, d.target_dim));

    SDimAlignFunc {
      aligned_dim_index: create_aligned_dimension_index(max_n_dims, &aligned_dims),
      aligned_dims
    }
  }
}

#[inline]
fn create_aligned_dimension_index(max_n_dims: usize, sorted_aligned_dims: &[AlignedDimension]) -> Vec<usize> {
  let mut aligned_dim_index = vec![0; max_n_dims];
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
fn update_dim_index(source: &[Index], target: &mut [Index], dim: &AlignedDimension) {
  target[dim.target_dim].set_idx((source[dim.source_dim].as_idx() - dim.source_start) * (dim.target_step / dim.source_step) + dim.target_start)
}


impl SAlignmentFunc for SDimAlignFunc {
  #[inline]
  fn align<'a>(&self, source: &'a [Index], _source_val: &Value, target: &'a mut [Index]) -> &'a [Index] {
    for dim in &self.aligned_dims {
      update_dim_index(source, target, dim);
    }

    target
  }

  #[inline]
  fn partial_align<'a>(&self, source: &'a [Index], _source_val: &Value, target: &'a mut [Index], from_idx: usize) -> &'a [Index] {
    for i in self.aligned_dim_index[from_idx]..self.aligned_dims.len() {
      update_dim_index(source, target, &self.aligned_dims[i]);
    }

    target
  }
}

#[derive(Debug)]
pub struct MAlignedDimension {
  pub target_dim: usize,
  pub target_range: RangeSlice
}

#[derive(Debug)]
pub struct MDimAlignFunc<'a, R: RAReader> {
  ra_reader: &'a R,
  saligned_dim_index: Vec<usize>,
  saligned_dims: Vec<AlignedDimension>,
  maligned_dims: Vec<MAlignedDimension>,

  // for stream ref index iterator
  unfrozen_dims: Vec<usize>,
  unknown_upperbounds: Vec<bool>,
  lowerbounds: Vec<usize>,
  upperbounds: Vec<usize>,
  neg_upperbounds: Vec<usize>,
  steps: Vec<usize>,
  has_unknown_dim: bool,
  last_unknown_dim: usize,
}

impl<'a, R: RAReader> MDimAlignFunc<'a, R> {
  pub fn new(ra_reader: &'a R, source_n_dims: usize, target_n_dims: usize, mut saligned_dims: Vec<AlignedDimension>, maligned_dims: Vec<MAlignedDimension>) -> MDimAlignFunc<'a, R> {
    saligned_dims.sort_by_key(|d| (d.source_dim, d.target_dim));

    // create basic components for constructing index iterator
    let mut unfrozen_dims: Vec<usize> = vec![];
    let mut unknown_upperbounds: Vec<bool> = vec![false; target_n_dims];
    let mut lowerbounds: Vec<usize> = vec![0; target_n_dims];
    let mut upperbounds: Vec<usize> = vec![0; target_n_dims];
    let mut neg_upperbounds: Vec<usize> = vec![0; target_n_dims];
    let mut steps: Vec<usize> = vec![0; target_n_dims];

    let mut has_unknown_dim: bool = false;
    let mut last_unknown_dim: usize = 0;

    for dim in &maligned_dims {
      unfrozen_dims.push(dim.target_dim);
      lowerbounds[dim.target_dim] = dim.target_range.start;
      steps[dim.target_dim] = dim.target_range.step;
      match dim.target_range.end {
        Some(v) => {
          if v < 0 {
            upperbounds[dim.target_dim] = 0; // random number as it doesn't matter
            neg_upperbounds[dim.target_dim] = -v as usize;
            unknown_upperbounds[dim.target_dim] = true;
            has_unknown_dim = true;
            last_unknown_dim = dim.target_dim;
          } else {
            upperbounds[dim.target_dim] = v as usize;
          }
        },
        None => {
          upperbounds[dim.target_dim] = 0; // random number as it doesn't matter
          unknown_upperbounds[dim.target_dim] = true;
          has_unknown_dim = true;
          last_unknown_dim = dim.target_dim;
        }
      }
    }
    unfrozen_dims.reverse();

    MDimAlignFunc {
      ra_reader,
      saligned_dim_index: create_aligned_dimension_index(max(source_n_dims, target_n_dims), &saligned_dims),
      saligned_dims,
      maligned_dims,
      unfrozen_dims,
      unknown_upperbounds,
      lowerbounds,
      upperbounds,
      neg_upperbounds,
      steps,
      has_unknown_dim,
      last_unknown_dim
    }
  }
}

impl<'a0, R: RAReader> MAlignmentFunc for MDimAlignFunc<'a0, R> {
  fn iter_alignments<'a1: 'a, 'a>(&'a1 self, source: &[Index], _source_val: &Value, target: &'a mut [Index]) -> Box<dyn StreamingIndexIterator + 'a> {
    for dim in &self.saligned_dims {
      update_dim_index(source, target, dim);
    }

    for dim in &self.maligned_dims {
      target[dim.target_dim].set_idx(dim.target_range.start);
    }

    if self.has_unknown_dim {
      Box::new(UnknownSizeRefIter::new(
        self.ra_reader, target,
        &self.unfrozen_dims, &self.unknown_upperbounds,
        self.last_unknown_dim, &self.lowerbounds, self.upperbounds.clone(),
        &self.neg_upperbounds, &self.steps))
    } else {
      Box::new(KnownSizeRefIter::new(target, &self.unfrozen_dims, &self.lowerbounds, &self.upperbounds, &self.steps))
    }
  }
}
