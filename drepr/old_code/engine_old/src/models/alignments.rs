use std::cmp::max;

use serde::Deserialize;

use crate::alignments::*;
use crate::models::location::Slice;
use crate::models::variable::Variable;
use crate::readers::RAReader;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BasedAlignedDim {
  #[serde(rename = "source")]
  pub source_dim: usize,
  #[serde(rename = "target")]
  pub target_dim: usize,
}

#[derive(Debug, Clone)]
pub struct DimAlignFactory {
  pub source: usize,
  pub target: usize,
  pub aligned_dims: Vec<BasedAlignedDim>,
}

#[derive(Debug, Clone)]
pub struct ValueAlignFactory {
  pub source: usize,
  pub target: usize,
}

#[derive(Debug, Clone)]
pub struct ChainAlignFactory {
  pub immediate_target: usize,
  pub align0: Box<AlignmentFactory>,
  pub align1: Box<AlignmentFactory>
}

#[derive(Debug, Clone)]
pub enum AlignmentFactory {
  DimAlign(DimAlignFactory),
  ValueAlign(ValueAlignFactory),
  ChainAlign(ChainAlignFactory),
  IdenticalAlignment,
}

impl AlignmentFactory {
  pub fn as_dim_align(&self) -> &DimAlignFactory {
    match self {
      AlignmentFactory::DimAlign(x) => x,
      _ => panic!("Cannot convert non dimensional alignment into dimensional alignment"),
    }
  }

  pub fn is_dim_align(&self) -> bool {
    match self {
      AlignmentFactory::DimAlign(_) => true,
      _ => false,
    }
  }

  pub fn is_value_align(&self) -> bool {
    match self {
      AlignmentFactory::ValueAlign(_) => true,
      _ => false
    }
  }

  pub fn swap(&self) -> Option<AlignmentFactory> {
    match self {
      AlignmentFactory::DimAlign(x) => Some(AlignmentFactory::DimAlign(x.swap())),
      AlignmentFactory::ValueAlign(x) => Some(AlignmentFactory::ValueAlign(x.swap())),
      AlignmentFactory::IdenticalAlignment => Some(AlignmentFactory::IdenticalAlignment),
      AlignmentFactory::ChainAlign(_) => None
    }
  }

  /// whether this  alignment function is single value alignment
  pub fn is_single(&self, _source: &Variable, target: &Variable) -> bool {
    match self {
      AlignmentFactory::DimAlign(da) => {
        let mut target_dims = vec![false; target.location.slices.len()];
        for d in &da.aligned_dims {
          target_dims[d.target_dim] = true;
        }

        let mut is_single = true;

        for (d, s) in target.location.slices.iter().enumerate() {
          match s {
            Slice::Range(_) => {
              if !target_dims[d] {
                // unaligned dimension, so it must be multiple value alignments
                is_single = false;
              }
            }
            _ => {}
          }
        }

        return is_single;
      }
      AlignmentFactory::ValueAlign(_) => {
        return false;
      }
      AlignmentFactory::IdenticalAlignment => {
        return true;
      }
      AlignmentFactory::ChainAlign(_) => {
        return false;
      }
    }
  }

  /// convert the input alignment function into a real alignment function
  pub fn to_alignment_func<'a, R: RAReader>(
    &self,
    ra_reader: &'a R,
    variables: &[Variable],
  ) -> AlignmentFunc<'a> {
    match self {
      AlignmentFactory::IdenticalAlignment => {
        AlignmentFunc::Single(Box::new(IdenticalAlignment {}))
      }
      AlignmentFactory::DimAlign(da) => {
        let mut aligned_dims = vec![];
        let source = &variables[da.source];
        let target = &variables[da.target];
        for d in &da.aligned_dims {
          let sr = source.location.slices[d.source_dim].as_range();
          let tr = target.location.slices[d.target_dim].as_range();

          aligned_dims.push(AlignedDimension {
            source_dim: d.source_dim,
            source_start: sr.start,
            source_step: sr.step,
            target_dim: d.target_dim,
            target_start: tr.start,
            target_step: tr.step,
          });
        }

        let n_unbounded_dims =
          target
            .location
            .slices
            .iter()
            .fold(0, |a, s| if s.is_range() { a + 1 } else { a });

        if n_unbounded_dims > aligned_dims.len() {
          AlignmentFunc::Multiple(Box::new(MDimAlignFunc::new(
            ra_reader,
            source.location.slices.len(),
            target.location.slices.len(),
            aligned_dims,
            vec![],
          )))
        } else {
          AlignmentFunc::Single(Box::new(SDimAlignFunc::new(
            max(source.location.slices.len(), target.location.slices.len()),
            aligned_dims,
          )))
        }
      }
      AlignmentFactory::ValueAlign(va) => {
        AlignmentFunc::Multiple(Box::new(ValueAlignFunc::new(ra_reader, &variables[va.target])))
      },
      AlignmentFactory::ChainAlign(ca) => {
        AlignmentFunc::Multiple(Box::new(M2SChainAlignFunc::new(
          variables[ca.immediate_target].get_first_index(),
          ca.align0.to_alignment_func(ra_reader, variables).into_multiple(),
          ca.align1.to_alignment_func(ra_reader, variables).into_single()
        )))
      }
    }
  }
}

impl DimAlignFactory {
  pub fn swap(&self) -> DimAlignFactory {
    DimAlignFactory {
      source: self.target,
      target: self.source,
      aligned_dims: self
        .aligned_dims
        .iter()
        .map(|x| BasedAlignedDim {
          source_dim: x.target_dim,
          target_dim: x.source_dim,
        })
        .collect(),
    }
  }
}

impl ValueAlignFactory {
  pub fn swap(&self) -> ValueAlignFactory {
    ValueAlignFactory {
      source: self.target,
      target: self.source,
    }
  }
}
