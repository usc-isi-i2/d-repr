use serde::{Deserialize, Serialize};
use crate::lang::alignment::Cardinality;
use crate::lang::description::Description;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ValueAlignment {
  pub source: usize,
  pub target: usize,
}

impl ValueAlignment {
  pub fn swap(&self) -> ValueAlignment {
    ValueAlignment {
      source: self.target,
      target: self.source
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
  /// The cardinality of the join will be one-to-* or *-to-one if values of source & target are unique.
  pub fn compute_cardinality(&self, desc: &Description) -> Cardinality {
    if desc.attributes[self.source].unique {
      if desc.attributes[self.target].unique {
        Cardinality::O2O
      } else {
        Cardinality::O2M
      }
    } else {
      if desc.attributes[self.target].unique {
        Cardinality::M2O
      } else {
        Cardinality::M2M
      }
    }
  }
}