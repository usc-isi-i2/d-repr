use drepr::iterators::*;
use drepr::readers::*;
use crate::helpers::*;
use super::*;
use hashbrown::HashMap;
use drepr::models::*;

#[test]
fn test_unknown_sized_iterator() {
  let data = r#"[
{
  "a": [2, 3, 5, 10, 11, 23],
  "b": [
    { "b1": [3, 4, 11, 12] },
    { "b1": [15, 2, 12] }
  ]
},
{
  "a": [5, 3, 2],
  "b": [{
    "b1": [0]
   }]
}]"#;
  let reader = JSONRAReader::from_str(data);
  let iter = UnknownSizeIter::new(
    &reader,
    vec![Index::Idx(0), Index::Str(String::from("a")), Index::Idx(0)],
    vec![2, 0],
    vec![false, false, true],
    vec![0, 0, 0],
    vec![2, 0, 0],
    vec![0, 0, 0],
    vec![1, 1, 1],
  );

  let mut gold_results = vec![];
  gold_results.append(
    &mut (0..6)
      .map(|i| sindex(&format!("0:a:{}", i)))
      .collect::<Vec<Vec<Index>>>(),
  );
  gold_results.append(
    &mut (0..3)
      .map(|i| sindex(&format!("1:a:{}", i)))
      .collect::<Vec<Vec<Index>>>(),
  );
  assert_eq!(collect_stream_iterators(Box::new(iter)), gold_results);

  let iter = UnknownSizeIter::new(
    &reader,
    sindex("0:b:0:b1:0"),
    vec![4, 2, 0],
    vec![false, false, true, false, true],
    vec![0, 0, 0, 0, 0],
    vec![2, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0],
    vec![1, 1, 1, 1, 1],
  );

  let mut gold_results = vec![];
  gold_results.append(
    &mut (0..4)
      .map(|i| sindex(&format!("0:b:0:b1:{}", i)))
      .collect::<Vec<Vec<Index>>>(),
  );
  gold_results.append(
    &mut (0..3)
      .map(|i| sindex(&format!("0:b:1:b1:{}", i)))
      .collect::<Vec<Vec<Index>>>(),
  );
  gold_results.append(
    &mut (0..1)
      .map(|i| sindex(&format!("1:b:0:b1:{}", i)))
      .collect::<Vec<Vec<Index>>>(),
  );
  assert_eq!(collect_stream_iterators(Box::new(iter)), gold_results);

  let mut freeze_iter = UnknownSizeIter::new(
    &reader,
    sindex("0:b:0:b1:0"),
    vec![4, 2, 0],
    vec![false, false, true, false, true],
    vec![0, 0, 0, 0, 0],
    vec![2, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0],
    vec![1, 1, 1, 1, 1],
  );
  freeze_iter.freeze_last_index();

  let gold_results = vec![
    sindex("0:b:0:b1:0"),
    sindex("0:b:1:b1:0"),
    sindex("1:b:0:b1:0"),
  ];
  assert_eq!(collect_stream_iterators(Box::new(freeze_iter)), gold_results);
}

#[test]
fn test_unknown_size_iter2() {
  let data = r#"
,2004-2006,,2005-2007,,2006-2008,
,Male,Female,Male,Female,Male,Female
Newport,76.7,80.7,77.1,80.9,77.0,81.5
Cardiff,78.7,83.3,78.6,83.7,78.7,83.4
Monmouthshire,76.6,81.3,76.5,81.5,76.6,81.7
Merthyr Tydfil,75.5,79.1,75.5,79.4,74.9,79.6
  "#.trim();

  let mut maps = HashMap::new();
  maps.insert(
    "default".to_string(),
    CSVRAReader::from_str(data, b',').into_value(),
  );
  let reader = MultipleRAReader::new(maps);
  let mut loc = Location {
    slices: slice(r#"["default", "2..", "1.."]"#)
  };
  reader.ground_location(&mut loc, 0);

  let iter: Box<dyn StreamingIndexIterator> = reader.iter_data(&loc);
  assert_eq!(
    collect_stream_iterators(iter),
    (2..6)
      .flat_map(|i| (1..7).map(|j| (i, j)).collect::<Vec<_>>())
      .map(|(i, j)| sindex(&format!("default:{}:{}", i, j)))
      .collect::<Vec<_>>()
  );
  assert_eq!(reader.get_value(&sindex("default:2:1"), 0), &Value::Str("76.7".to_string()));

  let mut iter: Box<dyn StreamingIndexIterator> = reader.iter_data(&loc);
  iter.freeze_last_index();
  assert_eq!(
    collect_stream_iterators(iter),
    (2..6)
      .map(|i| sindex(&format!("default:{}:1", i)))
      .collect::<Vec<_>>()
  );
}