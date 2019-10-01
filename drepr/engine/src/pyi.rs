use cpython::*;

use crate::executors::{Executor, PhysicalOutput};
use crate::writers::stream_writer::OutputFormat;

py_module_initializer!(drepr_engine, initdrepr_engine, PyInit_drepr_engine, |py, m | {
  m.add(py, "__doc__", "Rust D-REPR")?;
  m.add(py, "__version__", env!("CARGO_PKG_VERSION"))?;
  m.add(py, "create_executor", py_fn!(py, create_executor(args: String)))?;
  m.add(py, "destroy_executor", py_fn!(py, destroy_executor(ptr: usize)))?;
  m.add(py, "get_exec_plan", py_fn!(py, get_exec_plan(ptr: usize)))?;
  m.add(py, "run_executor", py_fn!(py, run_executor(ptr: usize, output: String)))?;
  Ok(())
});

macro_rules! wtry {
  ($e:expr, $py:ident) => {
    match $e {
      Ok(v) => v,
      Err(v) => {
        return Err(v.into_pyerr($py));
      }
    }
  }
}

fn create_executor(py: Python, args: String) -> PyResult<usize> {
  let executor = Box::new(wtry!(serde_json::from_str::<Executor>(&args), py));
  Ok(Box::into_raw(executor) as *const _ as usize)
}

fn destroy_executor(_py: Python, ptr: usize) -> PyResult<bool> {
  unsafe {
    drop(Box::from_raw(ptr as *mut Executor));
  }
  Ok(true)
}

fn get_exec_plan(py: Python, ptr: usize) -> PyResult<String> {
  let executor = unsafe { &*(ptr as *const Executor) };
  Ok(wtry!(serde_json::to_string_pretty(&executor.get_exec_plan()), py))
}

fn run_executor(py: Python, ptr: usize, output: String) -> PyResult<PyDict> {
  let mut executor = unsafe { &mut *(ptr as *mut Executor) };
  executor.output = wtry!(serde_json::from_str::<PhysicalOutput>(&output), py);
  let result = executor.exec();
  
  let dict = PyDict::new(py);
  dict.set_item(py, "type", format!("{:?}", executor.output.get_format()).to_lowercase())?;
  
  match &executor.output {
    PhysicalOutput::File { fpath: _, format: _ } => {}
    PhysicalOutput::String { format } => {
      match format {
        OutputFormat::TTL => {
          dict.set_item(py, "value", result.into_str1())?;
        }
        OutputFormat::GraphJSON => {
          let (nodes, edges) = result.into_str2();
          dict.set_item(py, "nodes", nodes)?;
          dict.set_item(py, "edges", edges)?;
        }
      }
    }
  }
  
  Ok(dict)
}

trait ToPyError {
  fn into_pyerr(self, py: Python) -> PyErr;
}

impl ToPyError for serde_json::Error {
  fn into_pyerr(self, py: Python) -> PyErr {
    PyErr::new::<exc::ValueError, _>(py, format!("(parsing json error) {}", self.to_string()))
  }
}

impl ToPyError for String {
  fn into_pyerr(self, py: Python) -> PyErr {
    PyErr::new::<exc::ValueError, _>(py, self)
  }
}



