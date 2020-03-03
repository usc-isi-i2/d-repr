use drepr::models::{Slice, RangeSlice};
use drepr::readers::Index;
use drepr::inputs::InputSlice;
use drepr::python::PyExecutor;

pub fn sindex(index_string: &str) -> Vec<Index> {
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

pub fn slice(slice: &str) -> Vec<Slice> {
  let mut pyexec = PyExecutor::default();
  let slices: Vec<InputSlice> = serde_json::from_str(slice).unwrap();

  slices.into_iter()
    .map(|s| s.into_slice(&mut pyexec))
    .collect::<Vec<_>>()
}

pub fn range(s: &str) -> RangeSlice {
  slice(&format!("[\"{}\"]", s)).pop().unwrap().into_range()
}