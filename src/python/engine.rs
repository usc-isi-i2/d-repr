use crate::executors::Executor;
use crate::writers::stream_writer::stream_writer::WriteResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass]
pub struct Engine {
  executor: Executor,
}

#[pymethods]
impl Engine {
  #[new]
  fn from_str(s: &[u8]) -> PyResult<Self> {
    match serde_json::from_slice::<Executor>(s) {
      Ok(executor) => Ok(Self { executor }),
      Err(e) => Err(PyValueError::new_err(format!("{}", e))),
    }
  }

  fn get_exec_plan(&self) -> String {
    serde_json::to_string_pretty(&self.executor.get_exec_plan()).unwrap()
  }

  fn run(&self) -> WriteResult {
    self.executor.exec()
  }
}

impl<'py> IntoPyObject<'py> for WriteResult {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      WriteResult::None => Ok(py.None().into_pyobject(py)?.into_any()),
      WriteResult::Str1(s) => Ok(s.into_pyobject(py)?.into_any()),
      WriteResult::Str2(s1, s2) => Ok(PyTuple::new(py, [s1, s2])?.into_any()),
      WriteResult::GraphPy(vec) => Ok(vec.into_pyobject(py)?.into_any()),
    }
  }
}
