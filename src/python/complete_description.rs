use crate::alignments::inference::AlignmentInference;
use crate::execution_plans::ClassMapPlan;
use crate::executors::Executor;
use crate::lang::{AlignedDim, Alignment, Description, GraphNode};
use crate::writers::stream_writer::stream_writer::WriteResult;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use std::collections::HashMap;

/// Inferring missing information in the description such as alignments and subjects
#[pyfunction]
pub fn complete_description(py: Python<'_>, args: &[u8]) -> PyResult<PyObject> {
  let desc = match serde_json::from_slice::<Description>(args) {
    Ok(desc) => desc,
    Err(e) => return Err(PyValueError::new_err(format!("{}", e))),
  };
  let inference = AlignmentInference::new(&desc);
  let n_class_nodes = desc.semantic_model.get_n_class_nodes();

  // compute subjects
  let mut class2subj: Vec<i64> = Vec::with_capacity(n_class_nodes);
  for class_id in 0..n_class_nodes {
    // TODO: temporary solution to handle the case where there is no data nodes (only literal nodes)
    let subj = if desc.semantic_model.outgoing_edges[class_id]
      .iter()
      .all(|&eid| desc.semantic_model.get_target(eid).is_literal_node())
    {
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
  dict.set_item("class2subj", class2subj).unwrap();
  dict.set_item("aligned_funcs", aligned_funcs).unwrap();
  Ok(dict.into())
}

impl ToPyObject for Alignment {
  fn to_object(&self, py: Python) -> PyObject {
    let obj = PyDict::new(py);
    match self {
      Alignment::IdenticalAlign => {
        obj.set_item("type", "identical").unwrap();
      }
      Alignment::RangeAlign(align) => {
        obj.set_item("type", "range").unwrap();
        obj.set_item("source", align.source).unwrap();
        obj.set_item("target", align.target).unwrap();
        obj.set_item("aligned_dims", &align.aligned_dims).unwrap();
      }
      Alignment::ValueAlign(align) => {
        obj.set_item("type", "value").unwrap();
        obj.set_item("source", align.source).unwrap();
        obj.set_item("target", align.target).unwrap();
      }
    }
    obj.into()
  }
}

impl ToPyObject for AlignedDim {
  fn to_object(&self, py: Python) -> PyObject {
    let obj = PyDict::new(py);
    obj.set_item("source_idx", self.source_dim).unwrap();
    obj.set_item("target_idx", self.target_dim).unwrap();
    obj.into()
  }
}
