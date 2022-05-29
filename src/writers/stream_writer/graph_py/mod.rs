use crate::lang::{GraphNode, SemanticModel};
use crate::writers::stream_writer::graph_py::class_writers::track_withurioptional_writer::TrackWithURIOptionalWriter;
use crate::writers::stream_writer::graph_py::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::stream_writer::{
  ExtractWriterResult, StreamWriterResult, WriteResult,
};
use crate::writers::stream_writer::{StreamClassWriter, StreamWriter, WriteMode};
use hashbrown::HashMap;
use pyo3::prelude::*;
use pyo3::types::PyDict;
pub mod class_writers;
mod temp_object_props;

pub struct GraphPyWriter {
  /// Python GIL so we can generate Python objects
  gil: GILGuard,

  /// A map that maps class id (`node_id`) to class label
  classes: Vec<String>,

  /// A map that maps property id (`edge_id`) to its label
  predicates: Vec<String>,

  /// A map that maps class id to list of predicates presented in the list
  class_predicates: Vec<Vec<String>>,

  /// A mapping of class uri to a map of record id and its value
  nodes: HashMap<String, HashMap<String, Py<PyDict>>>,
  /// A mapping of class id to list of records of the class
  /// This property and the `nodes` property share same record pointers.
  class2nodes: Vec<Vec<Py<PyDict>>>,

  /// buffer for storing links to object that has not been generated
  buffer_oprops: Vec<Vec<TempObjectProps>>,
}

impl GraphPyWriter {
  pub fn write2mem(sm: &SemanticModel) -> GraphPyWriter {
    let mut writer = GraphPyWriter {
      gil: Python::acquire_gil(),
      classes: vec![],
      predicates: vec![],
      class_predicates: vec![],
      nodes: Default::default(),
      class2nodes: vec![],
      buffer_oprops: vec![],
    };

    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          writer.buffer_oprops.push(Default::default());
          writer.classes.push(n.rel_label.clone());
          writer.nodes.insert(n.rel_label.clone(), HashMap::new());
          writer.class2nodes.push(Vec::with_capacity(16));

          let mut class_predicate = Vec::with_capacity(sm.outgoing_edges[n.node_id].len());
          for &e in &sm.outgoing_edges[n.node_id] {
            class_predicate.push(sm.edges[e].rel_label.clone());
          }
          writer.class_predicates.push(class_predicate);
        }
        _ => {
          break;
        }
      }
    }

    for edge in &sm.edges {
      writer.predicates.push(edge.rel_label.clone());
    }

    writer
  }
}

impl StreamWriter for GraphPyWriter {
  fn begin(&mut self) {}

  fn end(&mut self) {
    let py = self.gil.python();

    for (cid, records) in self.buffer_oprops.drain(..).enumerate() {
      let subnodes = self.nodes.get_mut(&self.classes[cid]).unwrap();
      for r in records {
        let u = subnodes.get_mut(&r.id).unwrap().as_ref(py);
        for (_tid, pid, opid) in r.props {
          u.set_item(&self.predicates[pid], opid.into_py(py)).unwrap();
        }
      }
    }
  }

  fn begin_class<'a>(
    &'a mut self,
    class_id: usize,
    _write_mode: WriteMode,
  ) -> Box<dyn StreamClassWriter + 'a> {
    Box::new(TrackWithURIOptionalWriter {
      py: self.gil.python(),
      class_id,
      ont_class: &self.classes[class_id],
      all_nodes: &self.nodes as *const _,
      nodes: self.nodes.get_mut(&self.classes[class_id]).unwrap(),
      class_nodes: &mut self.class2nodes[class_id],
      curr_node: PyDict::new(self.gil.python()).into(),
      buffer_oprops: &mut self.buffer_oprops,
      classes: &self.classes,
      predicates: &self.predicates,
      class_predicates: &self.class_predicates[class_id],
    })
  }

  fn end_class(&mut self) {}
}

impl ExtractWriterResult for GraphPyWriter {
  fn extract_result(self: Box<Self>) -> WriteResult {
    WriteResult::GraphPy(self.class2nodes)
  }
}

impl StreamWriterResult for GraphPyWriter {}
