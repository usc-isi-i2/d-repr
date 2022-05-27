use serde::{Deserialize, Serialize};

use crate::index::Index;

pub use self::index_expr::IndexExpr;
pub use self::range_expr::RangeExpr;
pub use self::set_index_expr::SetIndexExpr;
use crate::ra_reader::RAReader;
use crate::as_enum_type_impl;

mod range_expr;
mod index_expr;
mod set_index_expr;

/// Representing a query path expression that selects elements in the virtual tree
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathExpr {
  pub steps: Vec<StepExpr>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum StepExpr {
  /// access one or more nodes based on their positions
  #[serde(rename = "range")]
  Range(RangeExpr),
  /// access a single node based on its position in or its key in an array/object
  #[serde(rename = "index")]
  Index(IndexExpr),
  /// access a list of nodes based on multiple positions or keys in an array/object
  #[serde(rename = "set_index")]
  SetIndex(SetIndexExpr),
  /// an wildcard that match all nodes, which are values of an object (NOT ARRAY!!)
  #[serde(rename = "wildcard")]
  Wildcard,
}

impl PathExpr {
  /// Return the first steps in the path
  pub fn get_initial_step(&self, _ra_reader: &dyn RAReader) -> Vec<Index> {
    // TODO: implement correctly, we haven't support wildcard yet because its values is unknown
    let mut idx = Vec::with_capacity(self.steps.len());
    for s in &self.steps {
      match s {
        StepExpr::Index(i) => {
          idx.push(i.val.clone());
        }
        StepExpr::Range(r) => {
          idx.push(Index::Idx(r.start));
        }
        StepExpr::SetIndex(si) => {
          idx.push(si.values[0].clone());
        }
        StepExpr::Wildcard => unimplemented!()
      }
    }
    idx
  }

  /// Obtain a list of indices of steps that select more than one elements
  pub fn get_nary_steps(&self) -> Vec<usize> {
    let mut unfixed_dims = vec![];
    for (d, s) in self.steps.iter().enumerate() {
      match s {
        StepExpr::Index(_) => {}
        _ => {
          unfixed_dims.push(d);
        }
      }
    }

    unfixed_dims
  }

  /// Get length of steps that select more than one elements
  pub fn get_no_nary_steps(&self) -> usize {
    let mut n_nary_steps = 0;

    for s in &self.steps {
      match s {
        StepExpr::Index(_) => {},
        _ => {
          n_nary_steps += 1;
        }
      }
    }

    n_nary_steps
  }
}

impl StepExpr {
  as_enum_type_impl!(StepExpr, as_range, as_mut_range, Range, "RangeExpr", RangeExpr);
  as_enum_type_impl!(StepExpr, as_index, as_mut_index, Index, "IndexExpr", IndexExpr);
  as_enum_type_impl!(StepExpr, as_set_index, as_mut_set_index, SetIndex, "SetIndexExpr", SetIndexExpr);
}