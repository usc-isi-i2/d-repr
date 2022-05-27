use cpython::*;
use cpython::exc::TypeError;

use crate::value::Value;

impl ToPyObject for Value {
  type ObjectType = PyObject;

  fn to_py_object(&self, py: Python) -> PyObject {
    match self {
      Value::Str(v) => v.to_py_object(py).into_object(),
      Value::Bool(v) => v.to_py_object(py).into_object(),
      Value::I64(v) => v.to_py_object(py).into_object(),
      Value::F64(v) => v.to_py_object(py).into_object(),
      Value::Array(v) => v.to_py_object(py).into_object(),
      Value::Object(v) => {
        let dict = PyDict::new(py);
        for (k, v) in v {
          dict.set_item(py, k, v).unwrap();
        }
        dict.into_object()
      }
      Value::Null => py.None(),
    }
  }
}

impl<'s> FromPyObject<'s> for Value {
  fn extract(py: Python, obj: &'s PyObject) -> Result<Self, PyErr> {
    if let Ok(s) = obj.cast_as::<PyString>(py) {
      return Ok(Value::Str(s.to_string(py)?.into()));
    }

    if let Ok(_) = obj.cast_as::<PyInt>(py) {
      return Ok(Value::I64(obj.extract(py)?));
    }

    if let Ok(_) = obj.cast_as::<PyFloat>(py) {
      return Ok(Value::F64(obj.extract(py)?));
    }

    if let Ok(v) = obj.cast_as::<PySequence>(py) {
      return Ok(Value::Array(
        v.iter(py)?
          .map(|vv| vv.unwrap().extract(py).unwrap())
          .collect(),
      ));
    }

    if let Ok(v) = obj.cast_as::<PyDict>(py) {
      return Ok(Value::Object(
        v.items(py)
          .iter()
          .map(|(k, v)| (k.extract(py).unwrap(), v.extract(py).unwrap()))
          .collect(),
      ));
    }

    if obj == &py.None() {
      return Ok(Value::Null);
    }

    Err(PyErr::new::<TypeError, String>(
      py,
      format!(
        "TypeError: function returns a value that we don't understand. We get: {:?}",
        obj
      ),
    ))
  }
}
