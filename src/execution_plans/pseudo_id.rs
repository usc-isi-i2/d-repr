use std::fmt::Write;

use readers::prelude::Index;
use serde::{Deserialize, Serialize};
/// Object to compute pseudo id (unique index of primary key of a class)
///
/// The idea is to only keep the unbounded (varied) dimensions of an attribute
/// that is used as the subject, and we prefix it with class id to return
/// a unique string in case the primary key variable is used in another class
///
/// For example: class Person has primary keys defined as:
///   `0..:organization:employees:0..:name`, then the id is:
///   `person:0:0`, `person:0:1`, ...
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassPseudoID {
  pub prefix: String,
  pub unbounded_dims: Vec<usize>,
}

impl ClassPseudoID {
  #[inline]
  pub fn new(prefix: String, unbounded_dims: Vec<usize>) -> ClassPseudoID {
    ClassPseudoID {
      prefix,
      unbounded_dims,
    }
  }

  #[inline]
  pub fn get_id_string(&self, index: &[Index]) -> String {
    let mut out = self.prefix.clone();
    for &d in &self.unbounded_dims {
      write!(out, "_{}", index[d].as_idx()).unwrap();
    }

    out
  }

  #[inline]
  pub fn get_id_vec(&self, index: &[Index]) -> Vec<usize> {
    self
      .unbounded_dims
      .iter()
      .map(|&i| index[i].as_idx())
      .collect()
  }
}
