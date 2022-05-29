use itertools::Itertools;
use pyo3::prelude::*;
use pyo3::types::PyDict;

pub struct PyFuncExecutor {
  gil: GILGuard,
  locals: Vec<Py<PyDict>>,
  counter: usize,
}

impl PyFuncExecutor {
  pub fn new(readers: Vec<ReaderPtr>) -> PyFuncExecutor {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let mut locals = Vec::with_capacity(readers.len());

    for reader in readers {
      let local = PyDict::new(py);
      match Context::create_instance(py, reader) {
        Ok(context) => {
          local.set_item(py, "context", context).unwrap();
        }
        Err(e) => panic!(e),
      };
      locals.push(local);
    }

    PyExecutor {
      gil,
      locals,
      counter: 0,
    }
  }

  pub fn default() -> PyExecutor {
    let gil = Python::acquire_gil();
    gil.python();
    let locals = vec![];

    PyExecutor {
      gil,
      locals,
      counter: 0,
    }
  }

  /// compile a python function
  pub fn compile(&mut self, resource_id: usize, func: &str) -> Result<PyFunc, PyErr> {
    self.counter += 1;

    let py = self.gil.python();
    let pyfunc = PyFunc::from(resource_id, format!("func_{}", self.counter), func);

    py.eval(&pyfunc.code, None, Some(&self.locals[resource_id]))?;
    Ok(pyfunc)
  }

  /// Execute a python function
  /// Note: use this function with caution
  pub fn exec<'s, T: FromPyObject<'s>>(
    &mut self,
    func: &PyFunc,
    val: &Value,
    idx: &[Index],
  ) -> PyResult<T> {
    let py = self.gil.python();

    self.locals[func.resource_id].set_item(py, "value", val)?;
    self.locals[func.resource_id].set_item(py, "index", idx)?;
    let res = py.eval(&func.call, None, Some(&self.locals[func.resource_id]))?;

    // extremely unsafe!
    // we do this because Rust complains that res is dropped when the function is terminated
    // this is likely due to the life-time constraint 's of T.
    // We convert to a raw pointer and convert back because extract function should copy
    // the data instead of passing reference
    unsafe { (*(&res as *const PyObject)).extract(py) }
  }

  /// Evaluate a python expression one time
  ///
  /// Note: use this function with caution
  pub fn eval<'s, T: FromPyObject<'s>>(&mut self, resource_id: usize, code: &str) -> PyResult<T> {
    let py = self.gil.python();
    let res = py.eval(&code, None, Some(&self.locals[resource_id]))?;

    // extremely unsafe, see: PyExecutor::exec function.
    unsafe { (*(&res as *const PyObject)).extract(py) }
  }
}

#[derive(Debug)]
pub struct PyFunc {
  pub resource_id: usize,
  pub code: String,
  pub call: String,
}

impl PyFunc {
  pub fn from(resource_id: usize, name: String, code: &str) -> PyFunc {
    let code = code.trim();
    let indent = PyFunc::detect_indent(code);

    let code = format!(
      "exec('''def {}(value, index, context):\n{}''')",
      name,
      code
        .split("\n")
        .map(|l| format!("{}{}", indent, l))
        .join("\n")
    );

    PyFunc {
      resource_id,
      call: format!("{}(value, index, context)", name),
      code,
    }
  }

  fn detect_indent(code: &str) -> String {
    let mut indent: String = "\t".to_string();

    for line in code.split("\n") {
      if line.starts_with("\t") {
        break;
      }

      if line.starts_with(" ") {
        let mut n = 0;
        for c in line.chars() {
          if c != ' ' {
            indent = " ".repeat(n);
            break;
          }
          n += 1;
        }
        break;
      }
    }

    indent
  }
}
