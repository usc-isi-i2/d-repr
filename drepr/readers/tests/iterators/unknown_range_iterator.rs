use readers::prelude::*;
use crate::helpers::*;

#[test]
fn test_unknown_range_iterator() {
  let resource = JSONRAReader::from_str(r#"
[
  {"id": "a", "lbl":  "s01-a"},
  {"id": "b", "lbl":  "s01-b"}
]
  "#);
  
  assert_eq!(
    collect_index_iterator(resource.iter_index(&path_expr(&["..", "id"]))),
    vec![
      path("0:id"),
      path("1:id"),
    ]
  );
}