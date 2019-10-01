use serde::{Deserialize, Serialize};

use readers::prelude::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct LiteralProp {
  pub predicate_id: usize,
  pub value: Value,
}
