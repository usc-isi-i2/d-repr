use crate::prelude::Index;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::Borrowed;

impl<'py> IntoPyObject<'py> for Index {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      Index::Str(v) => Ok(v.into_pyobject(py)?.into_any()),
      Index::Idx(v) => Ok(v.into_pyobject(py)?.into_any()),
    }
  }
}

impl<'a, 'py> IntoPyObject<'py> for &'a Index {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      Index::Str(v) => Ok(v.as_str().into_pyobject(py)?.into_any()),
      Index::Idx(v) => Ok((*v).into_pyobject(py)?.into_any()),
    }
  }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Index {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    if let Ok(s) = obj.cast::<PyString>() {
      return Ok(Index::Str(s.to_str()?.into()));
    }
    Ok(Index::Idx(obj.extract::<usize>()?))
  }
}
