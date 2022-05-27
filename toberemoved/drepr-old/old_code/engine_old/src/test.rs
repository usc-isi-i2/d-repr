use crate::alignments::*;
use crate::inputs::InputRepresentation;
use crate::models::*;
use crate::functions::*;
use crate::python::*;
use crate::readers::*;

use std::ptr::NonNull;
use std::time::Instant;

pub fn test_read_data(fpath: &str) {
  let start = Instant::now();
  let ext = fpath.rsplitn(1, ".").collect::<Vec<_>>()[0];
  match ext {
    "csv" => {
      CSVRAReader::from_file(fpath, b',');
    }
    "json" => {
      JSONRAReader::from_file(fpath);
    }
    _ => panic!("Cannot create reader for unknown type file: {}", ext),
  }
  println!(">>> [DREPR] read data: {:?}", start.elapsed());
}

pub fn test_value_alignment(fpath: &str, repr: InputRepresentation, var_name: &str) {
  fn exec<R: 'static + RAReader>(mut reader: R, mut repr: InputRepresentation, var_name: &str) {
    let mut py_exec = PyExecutor::new(ReaderPtr(
      NonNull::new(&mut reader as *mut dyn RAReader).unwrap(),
    ));
    let input_var = repr.variables.remove(var_name).unwrap();

    let var = Variable {
      name: var_name.to_string(),
      location: input_var.location.into_location(&mut py_exec),
      unique: input_var.unique,
      sorted: input_var.sorted,
      value_type: input_var.value_type,
    };

    println!("{:?}", var.location);

    let mut result = 0;
    for _i in 1..3 {
      let start = Instant::now();
      let _alignment = ValueAlignFunc::new(&reader, &var);
      result += start.elapsed().as_millis();
    }
    println!(">>> [DREPR] runtime: {}ms", result);
  }

  let start = Instant::now();
  let ext = fpath.rsplitn(1, ".").collect::<Vec<_>>()[0];
  match ext {
    "csv" => {
      exec(CSVRAReader::from_file(fpath, b','), repr, var_name);
    }
    "json" => {
      exec(JSONRAReader::from_file(fpath), repr, var_name);
    }
    _ => panic!("Cannot create reader for unknown type file: {}", ext),
  }
  println!(">>> runtime: {:?}", start.elapsed());
}

pub fn test_preprocessing(fpath: &str) {
  fn exec<R: RAReader>(mut reader: R) {
    let mut func = MapFunc {
      location: &Location {
        slices: vec![
          Slice::Index(IndexSlice {
            idx: Index::Str("companies".to_string()),
          }),
          Slice::Range(RangeSlice {
            start: 0,
            end: None,
            step: 1,
          }),
          Slice::Index(IndexSlice {
            idx: Index::Str("phone".to_string()),
          }),
        ],
      },
      func: |val: &mut Value, _idx: &[Index]| Value::Str(format!("+1 {}", val.as_str())),
    };

    func.exec(&mut reader);
    reader
      .get_value(&vec![Index::Str("companies".to_string())], 0)
      .write2file("/tmp/test.json")
  }

  let start = Instant::now();
  let ext = fpath.rsplitn(1, ".").collect::<Vec<_>>()[0];
  match ext {
    "csv" => {
      exec(CSVRAReader::from_file(fpath, b','));
    }
    "json" => {
      exec(JSONRAReader::from_file(fpath));
    }
    _ => panic!("Cannot create reader for unknown type file: {}", ext),
  }
  println!(">>> runtime: {:?}", start.elapsed());
}
