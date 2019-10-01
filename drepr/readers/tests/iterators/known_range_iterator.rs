use readers::prelude::*;
use crate::helpers::*;


#[test]
fn test_known_range_iterator() {
  let mut index = path("0:0");
  let iter = KnownRangeIter::new(
    index.clone(),
    vec![1, 0],
    vec![0, 0],
    vec![1, 2],
    vec![1, 1],
  );
  
  assert_eq!(
    collect_index_iterator(Box::new(iter)),
    vec![
      path("0:0"),
      path("0:1")
    ]
  );
  
  let mut index = path("0:0");
  let iter = KnownRangeIter::new(
    index.clone(),
    vec![0, 1],
    vec![0, 0],
    vec![2, 0],
    vec![1, 0],
  );
  assert_eq!(
    collect_index_iterator(Box::new(iter)),
    vec![
      path("0:0"),
      path("1:0"),
    ]
  );
  
  let mut index = path("name:0:0");
  let iter = KnownRangeIter::new(
    index.clone(),
    vec![2, 1],
    vec![0, 0, 0],
    vec![0, 2, 3],
    vec![1, 1, 1],
  );
  assert_eq!(
    collect_index_iterator(Box::new(iter)),
    vec![
      path("name:0:0"),
      path("name:0:1"),
      path("name:0:2"),
      path("name:1:0"),
      path("name:1:1"),
      path("name:1:2"),
    ]
  );
}
