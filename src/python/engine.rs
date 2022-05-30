use crate::alignments::inference::AlignmentInference;
use crate::execution_plans::ClassMapPlan;
use crate::executors::Executor;
use crate::lang::{AlignedDim, Alignment, Description, GraphNode};
use crate::writers::stream_writer::stream_writer::WriteResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::collections::HashMap;

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

impl IntoPy<PyObject> for WriteResult {
  fn into_py(self, py: Python<'_>) -> PyObject {
    match self {
      WriteResult::None => py.None(),
      WriteResult::Str1(s) => s.into_py(py),
      WriteResult::Str2(s1, s2) => PyTuple::new(py, [s1, s2]).into(),
      WriteResult::GraphPy(vec) => vec.into_py(py),
    }
  }
}
