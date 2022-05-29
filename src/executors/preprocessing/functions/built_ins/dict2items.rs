use readers::prelude::{Index, Value};

/// Convert a hashmap to a list of key and value pairs
pub fn dict2items(val: &mut Value, _idx: &[Index]) -> Value {
  match val {
    Value::Object(map) => {
      Value::Array(map.drain()
        .map(|(k, v)| Value::Array(vec![
          Value::Str(k),
          v
        ]))
        .collect::<Vec<_>>())
    },
    _ => panic!("ValueError: cannot convert non-object node into array of items")
  }
}