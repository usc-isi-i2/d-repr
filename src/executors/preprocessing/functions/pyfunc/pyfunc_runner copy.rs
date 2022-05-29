use super::context::Context;
use super::pyfunc::PyFunc;
use super::types::ReaderPtr;
use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;

use readers::prelude::{Index, Value};

pub struct PyFuncRunner {
  gil: GILGuard,
  locals: Vec<Py<PyDict>>,
  counter: usize,
}

impl PyFuncRunner {
  pub fn new(readers: Vec<ReaderPtr>) -> PyFuncRunner {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut locals = Vec::with_capacity(readers.len());

    for reader in readers {
      let local = PyDict::new(py);
      let context = Context::new(reader);
      local.set_item("context", context.into_py(py)).unwrap();
      locals.push(local.into());
    }

    PyFuncRunner {
      gil,
      locals,
      counter: 0,
    }
  }

  pub fn default() -> PyFuncRunner {
    let gil = Python::acquire_gil();
    gil.python();
    let locals = vec![];

    PyFuncRunner {
      gil,
      locals,
      counter: 0,
    }
  }

  /// compile a python function
  pub fn compile(&mut self, resource_id: usize, func: &str) -> Result<PyFunc, PyErr> {
    self.counter += 1;

    let py = self.gil.python();
    let pyfunc = PyFunc::from(resource_id, format!("func_{}", self.counter), func);

    py.eval(
      &pyfunc.code,
      None,
      Some(self.locals[resource_id].as_ref(py)),
    )?;
    Ok(pyfunc)
  }

  /// Execute a python function
  /// Note: use this function with caution
  pub fn exec<'s, T: FromPyObject<'s>>(
    &mut self,
    func: &PyFunc,
    val: &Value,
    idx: &[Index],
  ) -> PyResult<T> {
    let py = self.gil.python();
    let locals = self.locals[func.resource_id].as_ref(py);
    locals.set_item("value", val)?;
    locals.set_item("index", idx)?;
    let res = py.eval(&func.call, None, Some(locals))?;

    return Ok(res.extract::<T>()?);

    // extremely unsafe!
    // we do this because Rust complains that res is dropped when the function is terminated
    // this is likely due to the life-time constraint 's of T.
    // We convert to a raw pointer and convert back because extract function should copy
    // the data instead of passing reference
    // unsafe { (*(&res as *const PyObject)).extract(py) }
  }

  // /// Evaluate a python expression one time
  // ///
  // /// Note: use this function with caution
  // pub fn eval<'s, T: FromPyObject<'s>>(&mut self, resource_id: usize, code: &str) -> PyResult<T> {
  //   let py = self.gil.python();
  //   let res = py.eval(&code, None, Some(&self.locals[resource_id]))?;

  //   // extremely unsafe, see: PyFuncRunner::exec function.
  //   unsafe { (*(&res as *const PyObject)).extract(py) }
  // }
}
