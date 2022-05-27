use cpython::*;

use crate::prelude::Index;

impl ToPyObject for Index {
  type ObjectType = PyObject;

  fn to_py_object(&self, py: Python) -> Self::ObjectType {
    match self {
      Index::Str(v) => v.to_py_object(py).into_object(),
      Index::Idx(v) => v.to_py_object(py).into_object(),
    }
  }
}

impl<'s> FromPyObject<'s> for Index {
  fn extract(py: Python, obj: &'s PyObject) -> Result<Self, PyErr> {
    Ok(match obj.cast_as::<PyString>(py) {
      Ok(s) => Index::Str(s.to_string(py)?.into()),
      Err(_) => Index::Idx(obj.extract::<usize>(py)?),
    })
  }
}
