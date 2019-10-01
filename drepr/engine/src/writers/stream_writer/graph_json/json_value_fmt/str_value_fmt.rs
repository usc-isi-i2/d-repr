use super::JSONValueFmt;
use readers::prelude::Value;
use std::io::{BufWriter, Write};

pub struct StrValueFmt {
}

impl<W: Write> JSONValueFmt<W> for StrValueFmt {
  fn get_value(&self, val: &Value) -> String {
    match val {
      Value::I64(v) => format!("\"{}\"", v),
      Value::Null => "null".to_string(),
      Value::Bool(v) => format!("\"{}\"", v),
      Value::F64(v) => format!("\"{}\"", v),
      Value::Str(v) => format!("\"{}\"", v),
      Value::Array(_) => panic!("Error while writing array values as string in GraphJSON"),
      Value::Object(_) => panic!("Error while writing object value as string in GraphJSON"),
    }
  }
  
  fn write_value(&self, writer: &mut BufWriter<W>, val: &Value) {
    match val {
      Value::I64(v) => {
        writer.write(&[b'"']).unwrap();
        writer.write(v.to_string().as_bytes()).unwrap();
        writer.write(&[b'"'])
      },
      Value::Null => {
        writer.write("null".as_bytes())
      },
      Value::Bool(v) => {
        writer.write(&[b'"']).unwrap();
        writer.write(v.to_string().as_bytes()).unwrap();
        writer.write(&[b'"'])
      },
      Value::F64(v) => {
        writer.write(&[b'"']).unwrap();
        writer.write(v.to_string().as_bytes()).unwrap();
        writer.write(&[b'"'])
      },
      Value::Str(v) => {
        writer.write(&[b'"']).unwrap();
        writer.write(v.as_bytes()).unwrap();
        writer.write(&[b'"'])
      },
      Value::Array(_) => panic!("Error while writing array values as string in GraphJSON"),
      Value::Object(_) => panic!("Error while writing object value as string in GraphJSON"),
    }.unwrap();
  }
}