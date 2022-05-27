use serde::{Deserialize, Serialize};
use crate::lang::description::Description;
use crate::lang::alignment::Cardinality;
use hashbrown::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
pub struct AlignedDim {
  #[serde(rename = "source")]
  pub source_dim: usize,
  #[serde(rename = "target")]
  pub target_dim: usize,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RangeAlignment {
  pub source: usize,
  pub target: usize,
  pub aligned_dims: Vec<AlignedDim>,
}

impl RangeAlignment {
  pub fn swap(&self) -> RangeAlignment {
    RangeAlignment {
      source: self.target,
      target: self.source,
      aligned_dims: self
        .aligned_dims
        .iter()
        .map(|x| AlignedDim {
          source_dim: x.target_dim,
          target_dim: x.source_dim,
        })
        .collect(),
    }
  }
  
  /// Compute the cardinality of an alignment
  ///
  /// The cardinality between attribute `x` and attribute `y` are defined as follows:
  ///
  /// 1. one-to-one: one item of `x` can only link to one item of `y` and vice versa.
  /// 2. one-to-many: one item of `x` can link to multiple items of `y`, but one item of `y` can only
  ///    link to one item of `x`.
  /// 3. many-to-one: the reversed case of one-to-many
  /// 4. many-to-many: multiple items of `x` can link to multiple items of `y` and vice versa.
  ///
  /// The cardinality depends on the number of unfixed dimensions of each attribute, if an attribute
  /// has no unfixed steps, it will be one-to-*, otherwise many-to-*
  pub fn compute_cardinality(&self, desc: &Description) -> Cardinality {
    let mut source_nary_steps = HashSet::<usize>::from_iter(desc.attributes[self.source].path.get_nary_steps().into_iter());
    let mut target_nary_steps = HashSet::<usize>::from_iter(desc.attributes[self.target].path.get_nary_steps().into_iter());
    
    for aligned_dim in &self.aligned_dims {
      source_nary_steps.remove(&aligned_dim.source_dim);
      target_nary_steps.remove(&aligned_dim.target_dim);
    }

    if source_nary_steps.len() == 0 {
      if target_nary_steps.len() == 0 {
        Cardinality::O2O
      } else {
        Cardinality::O2M
      }
    } else {
      if target_nary_steps.len() == 0 {
        Cardinality::M2O
      } else {
        Cardinality::M2M
      }
    }
  }
}
