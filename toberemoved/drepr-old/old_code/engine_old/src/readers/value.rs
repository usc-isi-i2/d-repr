use super::Index;
use hashbrown::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Null,
  Bool(bool),
  I64(i64),
  F64(f64),
  Str(String),
  Array(Vec<Value>),
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
    let end_idx = index.len() - 1;

    for i in start_idx..end_idx {
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

    match ptr {
      Value::Array(values) => &values[index[end_idx].as_idx()],
      Value::Object(map) => &map[index[end_idx].as_str()],
      _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
    }
  }

  #[inline]
  pub fn get_mut_value(&mut self, index: &[Index], start_idx: usize) -> &mut Value {
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
      Value::Array(values) => &mut values[index[end_idx].as_idx()],
      Value::Object(map) => map.get_mut(index[end_idx].as_str()).unwrap(),
      _ => panic!("ValueError: invalid index! you have reached leaf nodes of the tree"),
    }
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

  #[inline]
  pub fn as_bool(&self) -> bool {
    match self {
      Value::Bool(v) => *v,
      _ => panic!("TODO: add meaningful error message"),
    }
  }

  #[inline]
  pub fn as_i64(&self) -> i64 {
    match self {
      Value::I64(v) => *v,
      _ => panic!("TODO: add meaningful error message"),
    }
  }

  #[inline]
  pub fn as_f64(&self) -> f64 {
    match self {
      Value::F64(v) => *v,
      _ => panic!("TODO: add meaningful error message"),
    }
  }

  #[inline]
  pub fn as_str(&self) -> &str {
    match self {
      Value::Str(v) => v,
      _ => panic!("TODO: add meaningful error message"),
    }
  }

  #[inline]
  pub fn as_array(&self) -> &Vec<Value> {
    match self {
      Value::Array(children) => children,
      _ => panic!("ValueError: cannot convert non-array node into array"),
    }
  }

  #[inline]
  pub fn as_mut_array(&mut self) -> &mut Vec<Value> {
    match self {
      Value::Array(children) => children,
      _ => panic!("ValueError: cannot convert non-array node into array"),
    }
  }

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

  pub fn write2file(&self, fpath: &str) {
    let content = serde_json::to_string_pretty(&self.to_serde_json()).unwrap();
    std::fs::write(fpath, content).expect("Write to file");
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
