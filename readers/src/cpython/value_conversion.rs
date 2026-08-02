use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::types::PySequence;
use pyo3::types::{PyDict, PyFloat, PyInt, PyList, PyString};
use pyo3::Borrowed;

use crate::value::Value;

impl<'py> IntoPyObject<'py> for Value {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      Value::Str(v) => Ok(v.into_pyobject(py)?.into_any()),
      Value::Bool(v) => Ok(v.into_pyobject(py)?.to_owned().into_any()),
      Value::I64(v) => Ok(v.into_pyobject(py)?.into_any()),
      Value::F64(v) => Ok(v.into_pyobject(py)?.into_any()),
      Value::Array(v) => {
        let list = PyList::empty(py);
        for item in v {
          list.append(item.into_pyobject(py)?)?;
        }
        Ok(list.into_any())
      }
      Value::Object(v) => {
        let dict = PyDict::new(py);
        for (k, val) in v {
          dict.set_item(k, val.into_pyobject(py)?)?;
        }
        Ok(dict.into_any())
      }
      Value::Null => Ok(py.None().into_pyobject(py)?.into_any()),
    }
  }
}

impl<'a, 'py> IntoPyObject<'py> for &'a Value {
  type Target = PyAny;
  type Output = Bound<'py, PyAny>;
  type Error = PyErr;

  fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
    match self {
      Value::Str(v) => Ok(v.as_str().into_pyobject(py)?.into_any()),
      Value::Bool(v) => Ok((*v).into_pyobject(py)?.to_owned().into_any()),
      Value::I64(v) => Ok((*v).into_pyobject(py)?.into_any()),
      Value::F64(v) => Ok((*v).into_pyobject(py)?.into_any()),
      Value::Array(v) => {
        let list = PyList::empty(py);
        for item in v {
          list.append(item.into_pyobject(py)?)?;
        }
        Ok(list.into_any())
      }
      Value::Object(v) => {
        let dict = PyDict::new(py);
        for (k, val) in v {
          dict.set_item(k, val.into_pyobject(py)?)?;
        }
        Ok(dict.into_any())
      }
      Value::Null => Ok(py.None().into_pyobject(py)?.into_any()),
    }
  }
}

impl<'a, 'py> FromPyObject<'a, 'py> for Value {
  type Error = PyErr;

  fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
    if let Ok(s) = obj.cast::<PyString>() {
      return Ok(Value::Str(s.to_str()?.into()));
    }

    if let Ok(_) = obj.cast::<PyInt>() {
      return Ok(Value::I64(obj.extract()?));
    }

    if let Ok(_) = obj.cast::<PyFloat>() {
      return Ok(Value::F64(obj.extract()?));
    }

    if let Ok(v) = obj.cast::<PySequence>() {
      let mut array = Vec::new();
      for item in v.try_iter()? {
        array.push(item?.extract::<Value>()?);
      }
      return Ok(Value::Array(array));
    }

    if let Ok(v) = obj.cast::<PyDict>() {
      let mut map = hashbrown::HashMap::new();
      for (k, val) in v.iter() {
        map.insert(k.extract()?, val.extract()?);
      }
      return Ok(Value::Object(map));
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
