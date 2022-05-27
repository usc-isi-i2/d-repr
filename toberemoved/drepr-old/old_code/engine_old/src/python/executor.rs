use super::py_func::PyFunc;
use super::types::*;
use crate::readers::{Index, Value};
use cpython::*;

py_class!(pub class Context |py| {
  data reader: ReaderPtr;

  def get_value(&self, index: Vec<Index>) -> PyResult<Value> {
    unsafe {
      let ptr = self.reader(py).0.as_ptr();
      let res = (*ptr).get_value(&index, 0);

      Ok(res.clone())
    }
  }

  def get_left_value(&self, index: Vec<Index>) -> PyResult<Value> {
    let left = Index::Idx(index.last().unwrap().as_idx() - 1);

    if index.len() > 1 {
      unsafe {
        let ptr = self.reader(py).0.as_ptr();
        let res = (*ptr).get_value(&index[..index.len() - 1], 0);

        Ok(res.get_child_value(&left).clone())
      }
    } else {
      unsafe {
        let ptr = self.reader(py).0.as_ptr();
        let res = (*ptr).get_value(&[left], 0);
        Ok(res.clone())
      }
    }
  }
});

pub struct PyExecutor {
  gil: GILGuard,
  locals: PyDict,
  counter: usize,
}

impl PyExecutor {
  pub fn new(reader: ReaderPtr) -> PyExecutor {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);

    let wcontext = Context::create_instance(py, reader);
    match wcontext {
      Ok(context) => {
        locals.set_item(py, "context", context).unwrap();
      }
      Err(e) => panic!(e),
    };

    PyExecutor { gil, locals, counter: 0 }
  }

  pub fn default() -> PyExecutor {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = PyDict::new(py);

    PyExecutor { gil, locals, counter: 0 }
  }

  #[inline]
  pub fn python(&self) -> Python {
    self.gil.python()
  }

  /// compile a python function
  pub fn compile(&mut self, func: &str) -> Result<PyFunc, PyErr> {
    self.counter += 1;

    let py = self.gil.python();
    let pyfunc = PyFunc::from(format!("func_{}", self.counter), func);

    py.eval(&pyfunc.code, None, Some(&self.locals))?;
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

    self.locals.set_item(py, "value", val)?;
    self.locals.set_item(py, "index", idx)?;
    let res = py.eval(&func.call, None, Some(&self.locals))?;

    // extremely unsafe!
    // we do this because Rust complains that res is dropped when the function is terminated
    // this is likely due to the life-time constraint 's of T.
    // We convert to a raw pointer and convert back because extract function should copy
    // the data instead of passing reference
    unsafe { (*(&res as *const PyObject)).extract(py) }
  }

  /// Evaluate a python expression one time
  ///
  /// Note: use this function with caution
  pub fn eval<'s, T: FromPyObject<'s>>(&mut self, code: &str) -> PyResult<T> {
    let py = self.gil.python();
    let res = py.eval(&code, None, Some(&self.locals))?;

    // extremely unsafe, see: PyExecutor::exec function.
    unsafe { (*(&res as *const PyObject)).extract(py) }
  }
}
