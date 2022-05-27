use super::JSONValueFmt;
use readers::prelude::Value;
use std::io::{BufWriter, Write};

pub struct FloatValueFmt {
}

impl<W: Write> JSONValueFmt<W> for FloatValueFmt {
  fn get_value(&self, val: &Value) -> String {
    match val {
      Value::I64(v) => (*v as f64).to_string(),
      Value::Null => "null".to_string(),
      Value::Bool(_) => panic!("Error while writing boolean value as float in GraphJSON"),
      Value::F64(v) => v.to_string(),
      Value::Str(v) => v.clone(), // no check here
      Value::Array(_) => panic!("Error while writing array values as float in GraphJSON"),
      Value::Object(_) => panic!("Error while writing object value as float in GraphJSON"),
    }
  }
  
  fn write_value(&self, writer: &mut BufWriter<W>, val: &Value) {
    match val {
      Value::I64(v) => writer.write((*v as f64).to_string().as_bytes()),
      Value::Null => writer.write("null".as_bytes()),
      Value::Bool(_) => panic!("Error while writing boolean value as float in GraphJSON"),
      Value::F64(v) => writer.write(v.to_string().as_bytes()),
      Value::Str(v) => writer.write(v.as_bytes()), // no check here
      Value::Array(_) => panic!("Error while writing array values as float in GraphJSON"),
      Value::Object(_) => panic!("Error while writing object value as float in GraphJSON"),
    }.unwrap();
  }
}