use super::JSONValueFmt;
use serde_json;
use readers::prelude::Value;
use std::io::{BufWriter, Write};

pub struct UnspecifiedValueFmt {
}

impl<W: Write> JSONValueFmt<W> for UnspecifiedValueFmt {
  fn get_value(&self, val: &Value) -> String {
    match val {
      Value::I64(v) => v.to_string(),
      Value::Null => "null".to_string(),
      Value::Bool(v) => v.to_string(),
      Value::F64(v) => v.to_string(),
      Value::Str(v) => format!("\"{}\"", v),
      Value::Array(v) => serde_json::to_string(&serde_json::Value::Array(
        v.iter().map(|vv| vv.to_serde_json()).collect()
      )).unwrap(),
      Value::Object(v) => {
        let mut map = serde_json::Map::with_capacity(v.len());
        for (k, vv) in v.iter() {
          map.insert(k.clone(), vv.to_serde_json());
        }
        
        serde_json::to_string(&serde_json::Value::Object(map)).unwrap()
      },
    }
  }
  
  fn write_value(&self, writer: &mut BufWriter<W>, val: &Value) {
    match val {
      Value::I64(v) => writer.write(v.to_string().as_bytes()),
      Value::Null => writer.write("null".as_bytes()),
      Value::Bool(v) => writer.write(v.to_string().as_bytes()),
      Value::F64(v) => writer.write(v.to_string().as_bytes()),
      Value::Str(v) => {
        writer.write(&[b'"']).unwrap();
        writer.write(v.as_bytes()).unwrap();
        writer.write(&[b'"'])
      },
      Value::Array(v) => {
        writer.write(&serde_json::to_vec(&serde_json::Value::Array(
          v.iter().map(|vv| vv.to_serde_json()).collect()
        )).unwrap())
      },
      Value::Object(v) => {
        let mut map = serde_json::Map::with_capacity(v.len());
        for (k, vv) in v.iter() {
          map.insert(k.clone(), vv.to_serde_json());
        }
        
        writer.write(&serde_json::to_vec(&serde_json::Value::Object(map)).unwrap())
      },
    }.unwrap();
  }
}