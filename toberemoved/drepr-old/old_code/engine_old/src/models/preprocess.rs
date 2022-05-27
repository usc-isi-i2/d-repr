use crate::functions::*;
use crate::python::{PyExecutor, PyFunc};
use crate::readers::{Index, RAReader, Value};

use super::Location;

#[derive(Debug)]
pub enum PreprocessFunc {
  PMap(PMap),
  PFilter(PFilter),
  RMap(RMap),
}

#[derive(Debug)]
pub struct PMap {
  pub input: Location,
  pub output: Option<String>,
  pub code: PyFunc,
}

#[derive(Debug)]
pub struct PFilter {
  pub input: Location,
  pub output: Option<String>,
  pub code: PyFunc,
}

#[derive(Debug)]
pub struct RMap {
  pub func_id: String,
  pub input: Location,
  pub output: Option<String>,
}

impl PreprocessFunc {
  pub fn does_fundamental_structure_change<R: RAReader>(
    &self,
    py_exec: &mut PyExecutor,
    reader: &mut R,
  ) -> bool {
    match self {
      PreprocessFunc::PMap(v) => {
        // execute function and check if the return type is different (list to value, or primitive
        // value to list/dict)
        let idx = v.input.get_first_index();
        let val = reader.get_value(&idx, 0);
        let mval: Value = py_exec.exec(&v.code, val, &idx).unwrap();
        does_fundamental_structure_change(val, &mval)
      }
      PreprocessFunc::PFilter(_) => false,
      PreprocessFunc::RMap(m) => {
        match m.func_id.as_str() {
          "rmap-dict2items" => {
            return true;
          }
          _ => unimplemented!()
        }
      }
    }
  }

  pub fn exec<R: RAReader>(&mut self, py_exec: &mut PyExecutor, reader: &mut R) {
    match self {
      PreprocessFunc::PMap(pmap) => {
        reader.ground_location(&mut pmap.input, 0);

        match &pmap.output {
          Some(name) => {
            let mut map = MapInsertFunc {
              location: &pmap.input,
              output: name.to_string(),
              func: |val: &Value, idx: &[Index]| py_exec.exec(&pmap.code, val, idx).unwrap(),
            };
            let val = map.exec(reader);
            reader.set_value(&[Index::Str(name.to_string())], 0, val);
          }
          None => {
            let mut map = MapFunc {
              location: &pmap.input,
              func: |val: &mut Value, idx: &[Index]| py_exec.exec(&pmap.code, val, idx).unwrap(),
            };
            map.exec(reader);
          }
        }
      }
      PreprocessFunc::PFilter(pfilter) => {
        reader.ground_location(&mut pfilter.input, 0);
        let mut filter = FilterFunc {
          location: &pfilter.input,
          func: |val: &Value, idx: &[Index]| py_exec.exec(&pfilter.code, val, idx).unwrap(),
        };
        filter.exec(reader);
      },
      PreprocessFunc::RMap(rmap) => {
        reader.ground_location(&mut rmap.input, 0);

        match &rmap.output {
          Some(_name) => {
            match rmap.func_id.as_str() {
              "rmap-dict2items" => {
                unimplemented!()
              }
              _ => unimplemented!()
            }
          }
          None => {
            match rmap.func_id.as_str() {
              "rmap-dict2items" => {
                let mut map = MapFunc {
                  location: &rmap.input,
                  func: built_ins::dict2items,
                };
                map.exec(reader);
              },
              _ => unimplemented!()
            }

          }
        }
      }
    }
  }
}

fn does_fundamental_structure_change(val: &Value, mval: &Value) -> bool {
  match val {
    Value::Null => !mval.is_primitive(),
    Value::Bool(_) => !mval.is_primitive(),
    Value::I64(_) => !mval.is_primitive(),
    Value::F64(_) => !mval.is_primitive(),
    Value::Str(_) => !mval.is_primitive(),
    Value::Array(_) => true,
    Value::Object(_) => true,
  }
}
