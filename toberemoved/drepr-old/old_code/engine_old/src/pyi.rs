use crate::executors::exec_mapping;
use crate::inputs::*;
use crate::writers::*;
use cpython::*;
use std::fs::File;

#[derive(Default, Debug)]
pub struct PyDREPR {
  pub repr: InputRepresentation,
  pub resources: Vec<InputPhysicalResource>,
}

py_module_initializer!(drepr, initdrepr, PyInit_drepr, |py, m| {
  m.add(py, "__doc__", "Rust D-REPR")?;
  m.add(py, "__version__", env!("CARGO_PKG_VERSION"))?;
  m.add(py, "create_repr", py_fn!(py, create_repr()))?;
  m.add(py, "print_repr", py_fn!(py, print_repr(ptr: usize)))?;
  m.add(py, "destroy_repr", py_fn!(py, destroy_repr(ptr: usize)))?;
  m.add(
    py,
    "add_resource",
    py_fn!(
      py,
      add_resource(ptr: usize, rid: String, rpath: String, rser: String)
    ),
  )?;
  m.add(
    py,
    "add_preprocess",
    py_fn!(py, add_preprocess(ptr: usize, pser: String)),
  )?;
  m.add(
    py,
    "add_variable",
    py_fn!(py, add_variable(ptr: usize, vid: String, vser: String)),
  )?;
  m.add(
    py,
    "add_alignment",
    py_fn!(py, add_alignment(ptr: usize, aser: String)),
  )?;
  m.add(py, "add_sm", py_fn!(py, add_sm(ptr: usize, sm: String)))?;
  m.add(
    py,
    "exec2str",
    py_fn!(py, exec2str(ptr: usize, format: String)),
  )?;
  m.add(
    py,
    "exec2turtle_file",
    py_fn!(py, exec2turtle_file(ptr: usize, fpath: String)),
  )?;
  Ok(())
});

fn create_repr(_py: Python) -> PyResult<usize> {
  let pyi = Box::new(PyDREPR::default());
  Ok(Box::into_raw(pyi) as *const _ as usize)
}

fn destroy_repr(_py: Python, ptr: usize) -> PyResult<bool> {
  unsafe {
    drop(Box::from_raw(ptr as *mut PyDREPR));
  }
  Ok(true)
}

fn print_repr(_py: Python, ptr: usize) -> PyResult<bool> {
  let pyi = unsafe { &*(ptr as *const PyDREPR) };
  println!("repr: {:#?}", pyi);
  Ok(true)
}

fn add_resource(
  py: Python,
  ptr: usize,
  rid: String,
  rpath: String,
  rser: String,
) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  pyi.resources.push(InputPhysicalResource {
    resource_id: rid.clone(),
    resource_file: rpath,
  });
  let resource = match serde_json::from_str(&rser) {
    Ok(r) => r,
    Err(e) => return Err(e.into_pyerr(py)),
  };
  pyi.repr.resources.resources.insert(rid, resource);
  Ok(true)
}

fn add_preprocess(py: Python, ptr: usize, pser: String) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let func = match serde_json::from_str(&pser) {
    Ok(v) => v,
    Err(e) => return Err(e.into_pyerr(py)),
  };
  pyi.repr.preprocessing.push(func);
  Ok(true)
}

fn add_variable(py: Python, ptr: usize, vid: String, vser: String) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let variable = match serde_json::from_str(&vser) {
    Ok(v) => v,
    Err(e) => return Err(e.into_pyerr(py)),
  };
  pyi.repr.variables.insert(vid, variable);
  Ok(true)
}

fn add_alignment(py: Python, ptr: usize, aser: String) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let alignment = match serde_json::from_str(&aser) {
    Ok(v) => v,
    Err(e) => return Err(e.into_pyerr(py)),
  };
  pyi.repr.alignments.push(alignment);
  Ok(true)
}

fn add_sm(py: Python, ptr: usize, sm: String) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let sm = match serde_json::from_str(&sm) {
    Ok(v) => v,
    Err(e) => return Err(e.into_pyerr(py)),
  };
  pyi.repr.semantic_model = Some(sm);
  Ok(true)
}

fn exec2str(py: Python, ptr: usize, format: String) -> PyResult<PyList> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let result = match format.as_str() {
    "ttl" => {
      let mut writer = TurtleWriter::<Vec<u8>>::write2str();
      exec_mapping(pyi.repr.clone(), &pyi.resources, &mut writer);
      let val = unsafe { String::from_utf8_unchecked(writer.into_inner()) };

      PyList::new(py, &[PyString::new(py, &val).into_object()])
    }
    "graph_json" => {
      let mut writer = GraphJSONWriter::<Vec<u8>>::write2str();
      exec_mapping(pyi.repr.clone(), &pyi.resources, &mut writer);

      let (nc, ec) = writer.into_inner();
      PyList::new(
        py,
        &[
          PyString::new(py, &unsafe { String::from_utf8_unchecked(nc) }).into_object(),
          PyString::new(py, &unsafe { String::from_utf8_unchecked(ec) }).into_object(),
        ],
      )
    }
    _ => return Err(format!("Invalid writer format: {}", format).into_pyerr(py)),
  };

  Ok(result)
}

fn exec2turtle_file(_py: Python, ptr: usize, fpath: String) -> PyResult<bool> {
  let pyi = unsafe { &mut *(ptr as *mut PyDREPR) };
  let mut writer = TurtleWriter::<File>::write2file(&fpath);
  exec_mapping(pyi.repr.clone(), &pyi.resources, &mut writer);
  Ok(true)
}

trait ToPyError {
  fn into_pyerr(self, py: Python) -> PyErr;
}

impl ToPyError for serde_json::Error {
  fn into_pyerr(self, py: Python) -> PyErr {
    PyErr::new::<exc::ValueError, _>(py, self.to_string())
  }
}

impl ToPyError for String {
  fn into_pyerr(self, py: Python) -> PyErr {
    PyErr::new::<exc::ValueError, _>(py, self)
  }
}
