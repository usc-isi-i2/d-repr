use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PySequence;
use pyo3::types::{PyDict, PyFloat, PyInt, PyString};

use crate::value::Value;

impl IntoPy<PyObject> for Value {
  fn into_py(self, py: Python) -> PyObject {
    match self {
      Value::Str(v) => v.into_py(py),
      Value::Bool(v) => v.into_py(py),
      Value::I64(v) => v.into_py(py),
      Value::F64(v) => v.into_py(py),
      Value::Array(v) => v.into_py(py),
      Value::Object(v) => {
        let dict = PyDict::new(py);
        for (k, v) in v {
          dict.set_item(k, v.into_py(py)).unwrap();
        }
        dict.into()
      }
      Value::Null => py.None(),
    }
  }
}

impl ToPyObject for Value {
  fn to_object(&self, py: Python) -> PyObject {
    match self {
      Value::Str(v) => v.to_object(py),
      Value::Bool(v) => v.to_object(py),
      Value::I64(v) => v.to_object(py),
      Value::F64(v) => v.to_object(py),
      Value::Array(v) => v.to_object(py),
      Value::Object(v) => {
        let dict = PyDict::new(py);
        for (k, v) in v {
          dict.set_item(k, v.to_object(py)).unwrap();
        }
        dict.into()
      }
      Value::Null => py.None(),
    }
  }
}

impl<'s> FromPyObject<'s> for Value {
  fn extract(obj: &'s PyAny) -> PyResult<Self> {
    if let Ok(s) = obj.downcast::<PyString>() {
      return Ok(Value::Str(s.to_str()?.into()));
    }

    if let Ok(_) = obj.downcast::<PyInt>() {
      return Ok(Value::I64(obj.extract()?));
    }

    if let Ok(_) = obj.downcast::<PyFloat>() {
      return Ok(Value::F64(obj.extract()?));
    }

    if let Ok(v) = obj.downcast::<PySequence>() {
      return Ok(Value::Array(
        v.iter()?
          .map(|vv| vv.unwrap().extract::<Value>().unwrap())
          .collect(),
      ));
    }

    if let Ok(v) = obj.downcast::<PyDict>() {
      return Ok(Value::Object(
        v.iter()
          .map(|(k, v)| (k.extract().unwrap(), v.extract().unwrap()))
          .collect(),
      ));
    }

    if obj.is_none() {
      return Ok(Value::Null);
    }

    Err(PyTypeError::new_err(format!(
      "TypeError: function returns a value that we don't understand. We get: {:?}",
      obj
    )))
  }
}
