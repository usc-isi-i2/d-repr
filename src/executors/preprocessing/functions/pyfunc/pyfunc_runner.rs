use super::context::Context;
use super::pyfunc::PyFunc;
use super::types::ReaderPtr;
use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use readers::prelude::{Index, Value};

pub struct PyFuncRunner<'a> {
  py: Python<'a>,
  locals: Vec<&'a PyDict>,
  counter: usize,
}

impl<'a> PyFuncRunner<'a> {
  pub fn new(py: Python<'a>, readers: Vec<ReaderPtr>) -> PyFuncRunner<'a> {
    let mut locals = Vec::with_capacity(readers.len());

    for reader in readers {
      let local = PyDict::new(py);
      let context = Context::new(reader);
      local.set_item("context", context.into_py(py)).unwrap();
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

    self
      .py
      .eval(&pyfunc.code, None, Some(self.locals[resource_id]))?;
    Ok(pyfunc)
  }

  /// Execute a python function
  /// Note: use this function with caution
  pub fn exec<T: FromPyObject<'a>>(
    &mut self,
    func: &PyFunc,
    val: &Value,
    idx: &[Index],
  ) -> PyResult<T> {
    let locals = self.locals[func.resource_id];
    locals.set_item("value", val)?;
    locals.set_item("index", idx)?;
    let res = self.py.eval(&func.call, None, Some(locals))?;

    return Ok(res.extract::<T>()?);
  }

  /// Evaluate a python expression one time
  ///
  /// Note: use this function with caution
  pub fn eval<T: FromPyObject<'a>>(&mut self, resource_id: usize, code: &str) -> PyResult<T> {
    let res = self.py.eval(&code, None, Some(&self.locals[resource_id]))?;
    return Ok(res.extract::<T>()?);
  }
}
