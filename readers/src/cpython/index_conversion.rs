use crate::prelude::Index;
use pyo3::prelude::*;
use pyo3::types::PyString;

impl IntoPy<PyObject> for Index {
  fn into_py(self, py: Python) -> PyObject {
    match self {
      Index::Str(v) => v.into_py(py),
      Index::Idx(v) => v.into_py(py),
    }
  }
}

impl ToPyObject for Index {
  fn to_object(&self, py: Python) -> PyObject {
    match self {
      Index::Str(v) => v.to_object(py),
      Index::Idx(v) => v.to_object(py),
    }
  }
}

impl<'s> FromPyObject<'s> for Index {
  fn extract(obj: &'s PyAny) -> PyResult<Self> {
    Ok(match obj.downcast::<PyString>() {
      Ok(s) => Index::Str(s.to_str()?.into()),
      Err(_) => Index::Idx(obj.extract::<usize>()?),
    })
  }
}
