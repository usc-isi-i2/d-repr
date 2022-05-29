use readers::prelude::{Index, Value};

use std::default::Default;
use super::PyFunc;
use super::types::ReaderPtr;

// Mock of the pyexecutor to speed up compilation when you don't need to run python code or when
// running the python code won't affect the other parts of the engine but the final results
// (e.g., when python is used to write code to update values on some fields)
#[allow(dead_code)]
pub struct PyExecutor {}

impl PyExecutor {
  #[allow(dead_code)]
  pub fn new(_reader: Vec<ReaderPtr>) -> PyExecutor {
    PyExecutor {}
  }

  #[allow(dead_code)]
  pub fn default() -> PyExecutor {
    PyExecutor {}
  }

  #[allow(dead_code)]
  pub fn compile(&self, _resource_id: usize, _func: &str) -> Result<PyFunc, String> {
    Ok(PyFunc {
      resource_id: 0,
      code: "".to_string(),
      call: "".to_string()
    })
  }

  #[allow(dead_code)]
  pub fn exec<T: Default>(
    &mut self,
    _func: &PyFunc,
    _val: &Value,
    _idx: &[Index],
  ) -> Result<T, String> {
    Ok(T::default())
  }

  #[allow(dead_code)]
  pub fn eval<T>(&mut self, _resource_id: usize, _code: &str) -> Result<T, String> {
    unimplemented!()
  }
}
