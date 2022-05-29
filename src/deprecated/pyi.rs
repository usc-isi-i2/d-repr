use cpython::*;

use crate::executors::{Executor, PhysicalOutput};
use crate::writers::stream_writer::OutputFormat;
use crate::alignments::inference::AlignmentInference;
use crate::execution_plans::ClassMapPlan;
use crate::lang::{GraphNode, Alignment, AlignedDim, Description};
use std::collections::HashMap;

py_module_initializer!(drepr_engine, initdrepr_engine, PyInit_drepr_engine, |py, m | {
  m.add(py, "__doc__", "Rust D-REPR")?;
  m.add(py, "__version__", env!("CARGO_PKG_VERSION"))?;
  m.add(py, "create_executor", py_fn!(py, create_executor(args: String)))?;
  m.add(py, "destroy_executor", py_fn!(py, destroy_executor(ptr: usize)))?;
  m.add(py, "get_exec_plan", py_fn!(py, get_exec_plan(ptr: usize)))?;
  m.add(py, "complete_description", py_fn!(py, complete_description(args: String)))?;
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
    PhysicalOutput::Memory { format } => {
      match format {
        OutputFormat::TTL => {
          dict.set_item(py, "value", result.into_str1())?;
        }
        OutputFormat::GraphJSON => {
          let (nodes, edges) = result.into_str2();
          dict.set_item(py, "nodes", nodes)?;
          dict.set_item(py, "edges", edges)?;
        }
        OutputFormat::GraphPy => {
          dict.set_item(py, "class2nodes", result.into_graphpy())?;
        }
      }
    }
  }
  
  Ok(dict)
}

/// Inferring missing information in the description such as alignments and subjects
fn complete_description(py: Python, args: String) -> PyResult<PyDict> {
  let desc = wtry!(serde_json::from_str::<Description>(&args), py);
  let inference = AlignmentInference::new(&desc);
  let n_class_nodes = desc.semantic_model.get_n_class_nodes();

  // compute subjects
  let mut class2subj: Vec<i64> = Vec::with_capacity(n_class_nodes);
  for class_id in 0..n_class_nodes {
    // TODO: temporary solution to handle the case where there is no data nodes (only literal nodes)
    let subj = if desc.semantic_model.outgoing_edges[class_id].iter().all(|&eid| desc.semantic_model.get_target(eid).is_literal_node()) {
      -1
    } else {
      ClassMapPlan::find_subject(&desc, class_id, &inference) as i64
    };
    class2subj.push(subj);
  }

  // generate alignments between subject and other data attributes
  let mut aligned_funcs: HashMap<(usize, usize), Vec<Alignment>> = HashMap::new();
  for class_id in 0..n_class_nodes {
    if class2subj[class_id] == -1 {
      continue;
    }
    let class_subj = class2subj[class_id] as usize;

    for &eid in &desc.semantic_model.outgoing_edges[class_id] {
      match desc.semantic_model.get_target(eid) {
        GraphNode::DataNode(n) => {
          if class_subj == n.attr_id {
            continue;
          }

          let lst = inference.get_alignments(class_subj, n.attr_id);
          aligned_funcs.insert((class_subj, n.attr_id), lst);
        }
        GraphNode::LiteralNode(_n) => {
          continue;
        }
        GraphNode::ClassNode(n) => {
          if class2subj[n.node_id] == -1 {
            continue;
          }
          let target_subj = class2subj[n.node_id] as usize;
          let lst = inference.get_alignments(class_subj, target_subj);
          aligned_funcs.insert((class_subj, target_subj), lst);
        }
      }
    }
  }

  let dict = PyDict::new(py);
  dict.set_item(py, "class2subj", class2subj).unwrap();
  dict.set_item(py, "aligned_funcs", aligned_funcs).unwrap();
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

impl ToPyObject for AlignedDim {
  type ObjectType = PyDict;
  fn to_py_object(&self, py: Python) -> Self::ObjectType {
    let obj = PyDict::new(py);
    obj.set_item(py, "source_idx", self.source_dim).unwrap();
    obj.set_item(py, "target_idx", self.target_dim).unwrap();
    obj
  }
}

impl ToPyObject for Alignment {
  type ObjectType = PyDict;

  fn to_py_object(&self, py: Python) -> Self::ObjectType {
    let obj = PyDict::new(py);
    match self {
      Alignment::IdenticalAlign => {
        obj.set_item(py, "type", "identical").unwrap();
      }
      Alignment::RangeAlign(align) => {
        obj.set_item(py, "type", "range").unwrap();
        obj.set_item(py, "source", align.source).unwrap();
        obj.set_item(py, "target", align.target).unwrap();
        obj.set_item(py, "aligned_dims", &align.aligned_dims).unwrap();
      }
      Alignment::ValueAlign(align) => {
        obj.set_item(py, "type", "value").unwrap();
        obj.set_item(py, "source", align.source).unwrap();
        obj.set_item(py, "target", align.target).unwrap();
      }
    }
    obj
  }
}


