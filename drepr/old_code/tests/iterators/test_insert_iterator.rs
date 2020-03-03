use drepr::readers::{CSVRAReader, RAReader, Index, Value};
use drepr::models::Location;
use drepr::iterators::InsertIterator;
use crate::helpers::*;

#[test]
pub fn test_insert_iterator() {
  let data = r#"
,2004-2006,,2005-2007,,2006-2008,
,2008-2010,,2011-2013,,2014-2016,
  "#.trim();
  let reader = CSVRAReader::from_str(data, b',');
  let mut vloc = Location {
    slices: slice("[0, \"1..\"]")
  };
  let mut mloc = Location {
    slices: slice("[\"..\", \"1..\"]")
  };
  reader.ground_location(&mut vloc, 0);
  reader.ground_location(&mut mloc, 0);
  let reader_ptr = &reader as *const CSVRAReader;
  let mapfn = |val: &Value, idx: &[Index]| {
    if val.as_str().len() > 0 {
      Value::Str(val.as_str().split("-").collect::<Vec<_>>()[0].to_string())
    } else {
      let prev_val = unsafe {
        let tmp = (*reader_ptr).get_value(&idx[..1], 0);
        &tmp.as_array()[idx.last().unwrap().as_idx() - 1]
      };

      Value::Str(prev_val.as_str().split("-").collect::<Vec<_>>()[1].to_string())
    }
  };

  assert_eq!(collect_index(&reader, &vloc), (1..7)
    .map(|i| sindex(&format!("0:{}", i)))
    .collect::<Vec<_>>());
  assert_eq!(collect_value(&reader, &vloc), vec![
    "2004-2006", "", "2005-2007", "", "2006-2008", "",
  ].into_iter().map(|s| Value::Str(s.to_string())).collect::<Vec<_>>());
  assert_eq!(apply_map_fn(&reader, &vloc, &mapfn), Value::Array(
      vec!["2004", "2006", "2005", "2007", "2006", "2008"]
        .into_iter()
        .map(|s| Value::Str(s.to_string()))
        .collect::<Vec<_>>())
  );

  assert_eq!(collect_index(&reader, &mloc), (0..2).into_iter()
    .flat_map(|i| (1..7).into_iter().map(|j| (i, j)).collect::<Vec<_>>())
    .map(|(i, j)| sindex(&format!("{}:{}", i, j)))
    .collect::<Vec<_>>());
  assert_eq!(collect_value(&reader, &mloc), vec![
    "2004-2006", "", "2005-2007", "", "2006-2008", "",
    "2008-2010", "", "2011-2013", "", "2014-2016", "",
  ].into_iter().map(|s| Value::Str(s.to_string())).collect::<Vec<_>>());
  assert_eq!(
    apply_map_fn(&reader, &mloc, &mapfn),
    Value::Array(vec![
      Value::Array(
        vec!["2004", "2006", "2005", "2007", "2006", "2008"]
          .into_iter()
          .map(|s| Value::Str(s.to_string()))
          .collect::<Vec<_>>()),
      Value::Array(
        vec!["2008", "2010", "2011", "2013", "2014", "2016"]
          .into_iter()
          .map(|s| Value::Str(s.to_string()))
          .collect::<Vec<_>>())
    ])
  );
}


fn collect_index<R: RAReader>(reader: &R, loc: &Location) -> Vec<Vec<Index>> {
  let mut iter = InsertIterator::new(reader, &loc);
  let mut results = vec![];
  loop {
    results.push(iter.index().iter().map(|x| x.clone()).collect());
    if !iter.advance() {
      break;
    }
  }
  results
}

fn collect_value<R: RAReader>(reader: &R, loc: &Location) -> Vec<Value> {
  let mut iter = InsertIterator::new(reader, &loc);
  let mut results = vec![];
  loop {
    results.push(iter.value().clone());
    if !iter.advance() {
      break;
    }
  }
  results
}

fn apply_map_fn<R: RAReader>(reader: &R, loc: &Location, f: &dyn Fn(&Value, &[Index]) -> Value) -> Value {
  let mut iter = InsertIterator::new(reader, &loc);
  loop {
    let val = f(iter.value(), iter.index());
    iter.push(val);
    if !iter.advance() {
      break;
    }
  }
  iter.get_data()
}