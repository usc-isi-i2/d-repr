use serde::{Deserialize, Serialize};

/// A range step, which matches nodes at a step by their positions from a `start` (default is 0),
/// `end` is either infinity (`none`), positive or negative, and `step`.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RangeExpr {
  pub start: usize,
  pub end: Option<i64>,
  pub step: usize,
}

impl RangeExpr {
  pub fn get_end(&self, n_elements: usize) -> usize {
    match self.end {
      None => n_elements,
      Some(v) => {
        if v < 0 {
          (n_elements as i64 + v) as usize
        } else {
          v as usize
        }
      }
    }
  }
}