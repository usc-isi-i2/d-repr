use crate::index::Index;
use hashbrown::HashMap;
use crate::as_enum_type_impl;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeMap;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum Value {
  Null,
  Bool(bool),
  I64(i64),
  F64(f64),
  Str(String),
  Array(Vec<Value>),
  #[serde(deserialize_with = "Value::deserialize_object", serialize_with = "Value::serialize_object")]
  Object(HashMap<String, Value>),
}

impl Value {
  #[inline]
  pub fn len(&self) -> usize {
    match self {
      Value::Array(v) => v.len(),
      Value::Object(v) => v.len(),
      _ => panic!("ValueError: Cannot call len() method at leaf nodes"),
    }
  }

  #[inline]
  pub fn get_child_value(&self, index: &Index) -> &Value {
    match self {
      Value::Array(values) => &values[index.as_idx()],
      Value::Object(map) => &map[index.as_str()],
      _ => panic!("ValueError: cannot get child value at leaf nodes"),
    }
  }

  #[inline]
  pub fn get_mut_child_value(&mut self, index: &Index) -> &mut Value {
    match self {
      Value::Array(values) => &mut values[index.as_idx()],
      Value::Object(map) => map.get_mut(index.as_str()).unwrap(),
      _ => panic!("ValueError: cannot get child value at leaf nodes"),
    }
  }

  #[inline]
  pub fn get_value(&self, index: &[Index], start_idx: usize) -> &Value {
    let mut ptr = self;
    for i in start_idx..index.len() {
      match ptr {
        Value::Array(values) => {
          ptr = &values[index[i].as_idx()];
        }
        Value::Object(map) => {
          ptr = &map[index[i].as_str()];
        }
        _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
      }
    }

    ptr
  }

  #[inline]
  pub fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
    let mut ptr = self;
    for i in start_idx..index.len() {
      match ptr {
        Value::Array(values) => {
          ptr = &mut values[index[i].as_idx()];
        }
        Value::Object(map) => {
          ptr = map.get_mut(index[i].as_str()).unwrap();
        }
        _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
      }
    }

    return ptr;
  }

  #[inline]
  pub fn set_value(&mut self, index: &[Index], start_idx: usize, val: Value) {
    let mut ptr = self;
    let end_idx = index.len() - 1;

    for i in start_idx..end_idx {
      match ptr {
        Value::Array(values) => {
          ptr = &mut values[index[i].as_idx()];
        }
        Value::Object(map) => {
          ptr = map.get_mut(index[i].as_str()).unwrap();
        }
        _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
      }
    }

    match ptr {
      Value::Array(values) => {
        values[index[end_idx].as_idx()] = val;
      }
      Value::Object(map) => {
        map.insert(index[end_idx].as_str().to_string(), val);
      }
      _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
    }
  }

  #[inline]
  pub fn remove(&mut self, idx: &Index) {
    match self {
      Value::Array(values) => {
        values.remove(idx.as_idx());
      }
      Value::Object(map) => {
        map.remove(idx.as_str());
      }
      _ => panic!("ValueError: cannot remove child data at leaf nodes"),
    }
  }

  as_enum_type_impl!(Value, as_bool, as_mut_bool, Bool, "bool", bool);
  as_enum_type_impl!(Value, as_i64, as_mut_i64, I64, "i64", i64);
  as_enum_type_impl!(Value, as_f64, as_mut_f64, F64, "f64", f64);
  as_enum_type_impl!(Value, as_str, as_mut_str, Str, "str", str);
  as_enum_type_impl!(Value, as_array, as_mut_array, Array, "array", Vec<Value>);
  as_enum_type_impl!(Value, as_object, as_mut_object, Object, "object", HashMap<String, Value>);

  #[inline]
  pub fn is_primitive(&self) -> bool {
    match self {
      Value::Null => true,
      Value::Bool(_) => true,
      Value::I64(_) => true,
      Value::F64(_) => true,
      Value::Str(_) => true,
      Value::Array(_) => false,
      Value::Object(_) => false,
    }
  }

  #[inline]
  pub fn is_hashable(&self) -> bool {
    match self {
      Value::Null => true,
      Value::Bool(_) => true,
      Value::I64(_) => true,
      Value::F64(_) => false,
      Value::Str(_) => true,
      Value::Array(_) => false,
      Value::Object(_) => false,
    }
  }

  pub fn to_serde_json(&self) -> serde_json::Value {
    match self {
      Value::Null => serde_json::Value::Null,
      Value::Bool(b) => serde_json::Value::Bool(*b),
      Value::I64(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
      Value::F64(v) => serde_json::Value::Number(serde_json::Number::from_f64(*v).unwrap()),
      Value::Str(v) => serde_json::Value::String(v.clone()),
      Value::Array(v) => serde_json::Value::Array(v.iter().map(|vv| vv.to_serde_json()).collect()),
      Value::Object(v) => {
        let mut map = serde_json::Map::with_capacity(v.len());
        for (k, vv) in v.iter() {
          map.insert(k.clone(), vv.to_serde_json());
        }

        serde_json::Value::Object(map)
      }
    }
  }
  
  fn deserialize_object<'de, D>(deserializer: D) -> Result<HashMap<String, Value>, D::Error>
  where D: Deserializer<'de> {
    Ok(std::collections::HashMap::<String, Value>::deserialize(deserializer)?
        .into_iter().collect::<HashMap<_, _>>())
  }
  
  fn serialize_object<S>(obj: &HashMap<String, Value>, s: S) -> Result<S::Ok, S::Error>
  where S: Serializer {
    let mut map = s.serialize_map(Some(obj.len()))?;
    for (k, v) in obj {
      map.serialize_entry(k, v)?;
    }
    map.end()
  }
}

impl Eq for Value {}
impl std::hash::Hash for Value {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match self {
      Value::Null => Option::<bool>::None.hash(state),
      Value::Bool(v) => v.hash(state),
      Value::I64(v) => v.hash(state),
      Value::F64(_) => panic!("You should not call hashing function for float values"),
      Value::Str(v) => v.hash(state),
      Value::Array(_) => panic!("You should not call hashing function for array values"),
      Value::Object(_) => panic!("You should not call hashing function for object values"),
    }
  }
}

impl std::default::Default for Value {
  fn default() -> Self {
    return Value::Str("".to_string());
  }
}