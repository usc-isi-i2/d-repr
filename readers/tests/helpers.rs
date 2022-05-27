use readers::prelude::*;
use readers::path_expr::{StepExpr, IndexExpr, RangeExpr};

/// Collect values of an index iterator
pub fn collect_index_iterator<'a>(
  mut iter: Box<dyn IndexIterator + 'a>,
) -> Vec<Vec<Index>> {
  let mut results: Vec<Vec<Index>> = vec![];
  loop {
    results.push(iter.value().iter().map(|x| x.clone()).collect());
    if !iter.advance() {
      break;
    }
  }
  
  return results;
}

/// Create a path to an node in the resource tree
pub fn path(index_string: &str) -> Vec<Index> {
  // create index vector from <idx|string>:<idx|string>:...
  index_string.split(":")
    .map(|s| {
      match s.parse::<usize>() {
        Ok(v) => Index::Idx(v),
        Err(_) => Index::Str(s.to_string())
      }
    })
    .collect()
}

/// Create a path expression from a list of steps, which step can be "<start>..[<stop>]:step" or <idx> or <string>
pub fn path_expr(steps: &[&str]) -> PathExpr {
  let step_exprs = steps.iter()
    .map(|step| {
      if step.find("..").is_some() {
        let temp0 = step.split("..").collect::<Vec<&str>>();
        let start = if temp0[0] == "" {
          0
        } else {
          temp0[0].parse::<usize>().unwrap()
        };
        let temp1 = temp0[1].split(":").collect::<Vec<_>>();
        let end = if temp1[0] == "" {
          None
        } else {
          Some(temp1[0].parse::<i64>().unwrap())
        };
        let step = if temp1.len() > 0 {
          1
        } else {
          temp1[1].parse::<usize>().unwrap()
        };
        
        StepExpr::Range(RangeExpr { start, end, step })
      } else {
        StepExpr::Index(IndexExpr {
          val: match step.parse::<usize>() {
            Ok(v) => Index::Idx(v),
            Err(_) => Index::Str(step.to_string())
          }
        })
      }
    })
    .collect::<Vec<_>>();
  
  PathExpr { steps: step_exprs }
}