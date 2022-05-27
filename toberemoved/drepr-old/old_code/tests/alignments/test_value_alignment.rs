use drepr::alignments::{ValueAlignFunc, MAlignmentFunc};
use drepr::readers::*;
use drepr::models::{Variable, Location, VariableSorted, ValueType};
use crate::helpers::*;
use crate::iterators::collect_stream_iterators;

#[test]
fn test_value_alignment() {
  let data = r#"{
  "alphabet": [
    { "code": "a", "desc": "First letter" },
    { "code": "b", "desc": "Second letter" },
    { "code": "c", "desc": "Third letter" }
  ],
  "tokens": ["b", "a", "c", "a"]
}"#;

  let reader = JSONRAReader::from_str(data);
  let source = Variable {
    name: "tokens".to_string(),
    location: Location {
      slices: slice(r#"["tokens", ".."]"#)
    },
    unique: false,
    sorted: VariableSorted::Null,
    value_type: ValueType::Unspecified
  };

  let target = Variable {
    name: "alphabet".to_string(),
    location: Location {
      slices: slice(r#"["alphabet", "..", "code"]"#)
    },
    unique: false,
    sorted: VariableSorted::Null,
    value_type: ValueType::Unspecified
  };

  let align = ValueAlignFunc::new(&reader, &target);

  let sidx = source.get_first_index();
  let mut tidx = target.get_first_index();

  assert_eq!(
    collect_stream_iterators(align.iter_alignments(&sidx, &Value::Str("b".to_string()), &mut tidx)),
    vec![
      sindex("alphabet:1:code")
    ]
  );
  assert_eq!(
    collect_stream_iterators(align.iter_alignments(&sidx, &Value::Str("a".to_string()), &mut tidx)),
    vec![
      sindex("alphabet:0:code")
    ]
  );
  assert_eq!(
    collect_stream_iterators(align.iter_alignments(&sidx, &Value::Str("c".to_string()), &mut tidx)),
    vec![
      sindex("alphabet:2:code")
    ]
  );
}