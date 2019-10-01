use hashbrown::HashSet;
use serde::{Serialize};

use readers::prelude::Value;

use crate::lang::{Alignment, Attribute};

#[derive(Serialize, Debug)]
pub struct DataProp<'a> {
  pub alignments: Vec<Alignment>,
  pub predicate_id: usize,
  pub attribute: &'a Attribute,
  pub is_optional: bool,
  #[serde(serialize_with = "super::subject::serialize_set")]
  pub missing_values: HashSet<Value>,
}