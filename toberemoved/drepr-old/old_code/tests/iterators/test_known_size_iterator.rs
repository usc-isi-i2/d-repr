use drepr::iterators::*;
use drepr::readers::*;
use crate::helpers::sindex;
use super::*;


#[test]
fn test_known_sized_iterator() {
  let iter = KnownSizeIter::new(
    vec![Index::Idx(0), Index::Idx(0)],
    vec![1, 0],
    vec![0, 0],
    vec![1, 2],
    vec![1, 1],
  );
  assert_eq!(
    collect_stream_iterators(Box::new(iter)),
    vec![
      sindex("0:0"),
      sindex("0:1")
    ]
  );

  let iter = KnownSizeIter::new(
    vec![Index::Idx(0), Index::Idx(0)],
    vec![0, 1],
    vec![0, 0],
    vec![2, 0],
    vec![1, 0],
  );
  assert_eq!(
    collect_stream_iterators(Box::new(iter)),
    vec![
      sindex("0:0"),
      sindex("1:0"),
    ]
  );

  let iter = KnownSizeIter::new(
    sindex("name:0:0"),
    vec![2, 1],
    vec![0, 0, 0],
    vec![0, 2, 3],
    vec![1, 1, 1],
  );
  assert_eq!(
    collect_stream_iterators(Box::new(iter)),
    vec![
      sindex("name:0:0"),
      sindex("name:0:1"),
      sindex("name:0:2"),
      sindex("name:1:0"),
      sindex("name:1:1"),
      sindex("name:1:2"),
    ]
  );
}
