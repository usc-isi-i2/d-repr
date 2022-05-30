use std::cmp::max;

use readers::path_expr::RangeExpr;
use readers::prelude::{
  Index, IndexIterator, KnownRangeRefIter, RAReader, UnknownRangeRefIter, Value,
};

use crate::alignments::MAlignmentFunc;

use super::sgl_range_align::{
  create_aligned_range_step_index, update_range_step, AlignedRangeStep,
};
use crate::lang::{Description, RangeAlignment};
use hashbrown::HashSet;

#[derive(Debug)]
pub struct MAlignedDimension {
  pub target_dim: usize,
  pub target_range: RangeExpr,
}

#[derive(Debug)]
pub struct MRangeAlignFunc<'a> {
  ra_reader: &'a Box<dyn RAReader + 'a>,
  saligned_dim_index: Vec<usize>,
  saligned_dims: Vec<AlignedRangeStep>,
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

impl<'a> MRangeAlignFunc<'a> {
  pub fn from_dim_align(
    ra_reader: &'a Box<dyn RAReader + 'a>,
    desc: &Description,
    dalign: &RangeAlignment,
  ) -> MRangeAlignFunc<'a> {
    let mut aligned_dims = vec![];
    let source = &desc.attributes[dalign.source];
    let target = &desc.attributes[dalign.target];
    let mut marked_dims = HashSet::new();
    for d in &dalign.aligned_dims {
      let sr = source.path.steps[d.source_dim].as_range();
      let tr = target.path.steps[d.target_dim].as_range();
      marked_dims.insert(d.target_dim);
      aligned_dims.push(AlignedRangeStep {
        source_dim: d.source_dim,
        source_start: sr.start,
        source_step: sr.step,
        target_dim: d.target_dim,
        target_start: tr.start,
        target_step: tr.step,
      });
    }

    let maligned_dims = target
      .path
      .get_nary_steps()
      .into_iter()
      .filter(|dim| !marked_dims.contains(dim))
      .map(|dim| MAlignedDimension {
        target_dim: dim,
        target_range: target.path.steps[dim].as_range().clone(),
      })
      .collect::<Vec<_>>();
    MRangeAlignFunc::new(
      ra_reader,
      source.path.steps.len(),
      target.path.steps.len(),
      aligned_dims,
      maligned_dims,
    )
  }

  pub fn new(
    ra_reader: &'a Box<dyn RAReader + 'a>,
    source_n_dims: usize,
    target_n_dims: usize,
    mut saligned_dims: Vec<AlignedRangeStep>,
    maligned_dims: Vec<MAlignedDimension>,
  ) -> MRangeAlignFunc<'a> {
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
        }
        None => {
          upperbounds[dim.target_dim] = 0; // random number as it doesn't matter
          unknown_upperbounds[dim.target_dim] = true;
          has_unknown_dim = true;
          last_unknown_dim = dim.target_dim;
        }
      }
    }
    unfrozen_dims.reverse();
    MRangeAlignFunc {
      ra_reader,
      saligned_dim_index: create_aligned_range_step_index(
        max(source_n_dims, target_n_dims),
        &saligned_dims,
      ),
      saligned_dims,
      maligned_dims,
      unfrozen_dims,
      unknown_upperbounds,
      lowerbounds,
      upperbounds,
      neg_upperbounds,
      steps,
      has_unknown_dim,
      last_unknown_dim,
    }
  }
}

impl<'a0> MAlignmentFunc for MRangeAlignFunc<'a0> {
  fn iter_alignments<'a1: 'a, 'a>(
    &'a1 mut self,
    source: &[Index],
    _source_val: &Value,
    target: &'a mut [Index],
  ) -> Box<dyn IndexIterator + 'a> {
    for dim in &self.saligned_dims {
      update_range_step(source, target, dim);
    }
    for dim in &self.maligned_dims {
      target[dim.target_dim].set_idx(dim.target_range.start);
    }
    // TODO: uncomment the code below
    if self.has_unknown_dim {
      Box::new(UnknownRangeRefIter::new(
        self.ra_reader.as_ref(),
        target,
        &mut self.unfrozen_dims,
        &mut self.unknown_upperbounds,
        &mut self.lowerbounds,
        self.upperbounds.clone(),
        &mut self.neg_upperbounds,
        &mut self.steps,
      ))
    } else {
      Box::new(KnownRangeRefIter::new(
        target,
        &mut self.unfrozen_dims,
        &mut self.lowerbounds,
        &mut self.upperbounds,
        &mut self.steps,
      ))
    }
  }
}
