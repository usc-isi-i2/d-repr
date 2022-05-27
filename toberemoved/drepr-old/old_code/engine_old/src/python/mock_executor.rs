use crate::readers::{Index, Value};

use super::PyFunc;
use super::types::ReaderPtr;

// Mock of the pyexecutor to speed up compilation when you don't need to run python code
#[allow(dead_code)]
pub struct PyExecutor {}

impl PyExecutor {
  #[allow(dead_code)]
  pub fn new(_reader: ReaderPtr) -> PyExecutor {
    PyExecutor {}
  }

  #[allow(dead_code)]
  pub fn default() -> PyExecutor {
    PyExecutor {}
  }

  #[allow(dead_code)]
  pub fn compile(&self, _func: &str) -> Result<PyFunc, String> {
    unimplemented!()
  }

  #[allow(dead_code)]
  pub fn exec<T>(
    &mut self,
    _func: &PyFunc,
    _val: &Value,
    _idx: &[Index],
  ) -> Result<T, String> {
    unimplemented!()
  }

  #[allow(dead_code)]
  pub fn eval<T>(&mut self, _code: &str) -> Result<T, String> {
    unimplemented!()
  }
}
