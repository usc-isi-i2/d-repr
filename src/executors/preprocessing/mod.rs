mod functions;

use self::functions::pyfunc::{PyFuncRunner, ReaderPtr};
use self::functions::{dict2items, FilterFunc, MapFunc, SplitFunc};
use crate::lang::preprocessing::{BuiltinRustMapFunc, PreprocessingFunc};
use pyo3::prelude::*;
use readers::prelude::{Index, RAReader, Value};
use std::ptr::NonNull;

pub fn exec_preprocessing(
  readers: &mut [Box<dyn RAReader>],
  preprocessing_funcs: &[PreprocessingFunc],
) {
  Python::with_gil(|py| {
    let mut py_executor = PyFuncRunner::new(
      py,
      readers
        .iter_mut()
        .map(|reader| ReaderPtr(NonNull::new(reader.as_mut() as *mut dyn RAReader).unwrap()))
        .collect::<Vec<_>>(),
    );
    // execute preprocessing functions
    for preprocessing_func in preprocessing_funcs.iter() {
      match preprocessing_func {
        PreprocessingFunc::PyMap(pm) => {
          let pyfunc = py_executor.compile(pm.resource_id, &pm.code).unwrap();
          let mut func = MapFunc {
            path: &pm.path,
            func: |val: &mut Value, idx: &[Index]| py_executor.exec(&pyfunc, val, idx).unwrap(),
          };
          func.exec(readers[pm.resource_id].as_mut());
        }
        PreprocessingFunc::PyFilter(pf) => {
          let pyfunc = py_executor.compile(pf.resource_id, &pf.code).unwrap();
          let mut func = FilterFunc {
            path: &pf.path,
            func: |val: &Value, idx: &[Index]| py_executor.exec(&pyfunc, val, idx).unwrap(),
          };
          func.exec(readers[pf.resource_id].as_mut());
        }
        PreprocessingFunc::PySplit(ps) => {
          let pyfunc = py_executor.compile(ps.resource_id, &ps.code).unwrap();
          let mut func = SplitFunc {
            path: &ps.path,
            func: |val: &Value, idx: &[Index]| py_executor.exec(&pyfunc, val, idx).unwrap(),
          };
          func.exec(readers[ps.resource_id].as_mut());
        }
        PreprocessingFunc::RuMap(rm) => {
          let mut func = MapFunc {
            path: &rm.path,
            func: match rm.func_id {
              BuiltinRustMapFunc::Dict2Items => &dict2items,
            },
          };
          func.exec(readers[rm.resource_id].as_mut());
        }
      }
    }
  })
}
