use super::context::Context;
use super::pyfunc::PyFunc;
use super::types::ReaderPtr;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::ffi::CString;

use readers::prelude::{Index, Value};

pub struct PyFuncRunner<'a> {
  py: Python<'a>,
  locals: Vec<Bound<'a, PyDict>>,
  counter: usize,
}

impl<'a> PyFuncRunner<'a> {
  pub fn new(py: Python<'a>, readers: Vec<ReaderPtr>) -> PyFuncRunner<'a> {
    let mut locals = Vec::with_capacity(readers.len());

    for reader in readers {
      let local = PyDict::new(py);
      let context = Context::new(reader);
      local.set_item("context", context.into_pyobject(py).unwrap()).unwrap();
      locals.push(local);
    }

    PyFuncRunner {
      py,
      locals,
      counter: 0,
    }
  }

  pub fn default(py: Python<'a>) -> PyFuncRunner<'a> {
    let locals = vec![];

    PyFuncRunner {
      py,
      locals,
      counter: 0,
    }
  }

  /// compile a python function
  pub fn compile(&mut self, resource_id: usize, func: &str) -> Result<PyFunc, PyErr> {
    self.counter += 1;

    let pyfunc = PyFunc::from(resource_id, format!("func_{}", self.counter), func);
    let code_c = CString::new(pyfunc.code.as_str())?;

    self
      .py
      .eval(&code_c, None, Some(&self.locals[resource_id]))?;
    Ok(pyfunc)
  }

  /// Execute a python function
  /// Note: use this function with caution
  pub fn exec<T: for<'b, 'py> FromPyObject<'b, 'py, Error = PyErr>>(
    &mut self,
    func: &PyFunc,
    val: &Value,
    idx: &[Index],
  ) -> Result<T, PyErr> {
    let locals = &self.locals[func.resource_id];
    locals.set_item("value", val)?;
    locals.set_item("index", idx)?;
    let call_c = CString::new(func.call.as_str())?;
    let res = self.py.eval(&call_c, None, Some(locals))?;

    res.extract::<T>()
  }

  /// Evaluate a python expression one time
  ///
  /// Note: use this function with caution
  pub fn eval<T: for<'b, 'py> FromPyObject<'b, 'py, Error = PyErr>>(&mut self, resource_id: usize, code: &str) -> Result<T, PyErr> {
    let code_c = CString::new(code)?;
    let res = self.py.eval(&code_c, None, Some(&self.locals[resource_id]))?;
    res.extract::<T>()
  }
}
