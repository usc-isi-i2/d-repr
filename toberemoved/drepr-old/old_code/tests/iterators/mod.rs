use drepr::iterators::*;
use drepr::readers::*;

mod test_known_size_iterator;
mod test_unknown_size_iterator;
mod test_insert_iterator;

pub fn collect_stream_iterators<'a>(
  mut iter: Box<dyn StreamingIndexIterator + 'a>,
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