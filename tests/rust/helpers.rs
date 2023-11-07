use readers::prelude::{Index, IndexIterator};

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