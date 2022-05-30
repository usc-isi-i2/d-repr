use hashbrown::HashMap;

use readers::value::Value;

use crate::writers::stream_writer::graph_py::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::StreamClassWriter;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub struct TrackWithURIOptionalWriter<'a> {
  pub py: Python<'a>,

  /// id of the class
  pub class_id: usize,

  /// uri of the class
  pub ont_class: &'a str,

  /// All inserted nodes
  pub all_nodes: *const HashMap<String, HashMap<String, Py<PyDict>>>,
  /// A mapping of node id to its data
  pub nodes: &'a mut HashMap<String, Py<PyDict>>,
  /// A mapping of class id to list of nodes
  /// This share same record pointers with the `nodes` property
  pub class_nodes: &'a mut Vec<Py<PyDict>>,

  /// current node that we are writing information into
  pub curr_node: Py<PyDict>,

  /// buffer for storing links to object that has not been generated
  pub buffer_oprops: &'a mut [Vec<TempObjectProps>],

  /// a mapping of class id to its label
  pub classes: &'a [String],

  /// a mapping of property id (`edge_id`) to its label
  pub predicates: &'a [String],

  /// a list of predicates that this class has
  pub class_predicates: &'a [String],
}

impl<'a> StreamClassWriter for TrackWithURIOptionalWriter<'a> {
  fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
    unsafe { (&*self.all_nodes)[&self.classes[class_id]].contains_key(subject) }
  }

  fn begin_record(&mut self, subject: &str, _is_blank: bool) -> bool {
    let is_new = !self.nodes.contains_key(subject);
    if is_new {
      let node = PyDict::new(self.py);
      node.set_item("@id", subject).unwrap();
      for p in self.class_predicates {
        node.set_item(p, PyList::empty(self.py)).unwrap();
      }

      self.class_nodes.push(node.into());
      self.nodes.insert(subject.to_string(), node.into());
      self.curr_node = node.into();
    } else {
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);

      let curr_node = self.curr_node.as_ref(self.py);
      for p in self.class_predicates {
        if curr_node.contains(p).unwrap() {
          curr_node.set_item(p, PyList::empty(self.py)).unwrap();
        }
      }
    }

    is_new
  }

  fn end_record(&mut self) {}

  fn begin_partial_buffering_record(&mut self, subject: &str, _is_blank: bool) -> bool {
    let is_new = !self.nodes.contains_key(subject);
    if is_new {
      let node = PyDict::new(self.py);
      node.set_item("@id", subject).unwrap();
      for p in self.class_predicates {
        node.set_item(p, PyList::empty(self.py)).unwrap();
      }

      self.class_nodes.push(node.into());
      self.nodes.insert(subject.to_string(), node.into());
      self.curr_node = node.into();
      self.buffer_oprops[self.class_id].push(TempObjectProps {
        id: subject.to_string(),
        props: vec![],
      });
    } else {
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);
      let curr_node = self.curr_node.as_ref(self.py);
      for p in self.class_predicates {
        if curr_node.contains(p).unwrap() {
          curr_node.set_item(p, PyList::empty(self.py)).unwrap();
        }
      }
    }
    is_new
  }

  fn end_partial_buffering_record(&mut self) {}

  fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
    let v = self
      .curr_node
      .as_ref(self.py)
      .get_item(&self.predicates[predicate_id])
      .unwrap();
    let lst = v.downcast::<PyList>().unwrap();
    lst.append(value).unwrap();
  }

  fn write_object_property(
    &mut self,
    _target_cls: usize,
    _subject: &str,
    predicate_id: usize,
    object: &str,
    _is_subject_blank: bool,
    _is_object_blank: bool,
    _is_new_subj: bool,
  ) {
    let v = self
      .curr_node
      .as_ref(self.py)
      .get_item(&self.predicates[predicate_id])
      .unwrap();
    let lst = v.downcast::<PyList>().unwrap();
    lst.append(object).unwrap();
  }

  fn buffer_object_property(
    &mut self,
    target_cls: usize,
    predicate_id: usize,
    object: String,
    _is_object_blank: bool,
  ) {
    self.buffer_oprops[self.class_id]
      .last_mut()
      .unwrap()
      .props
      .push((target_cls, predicate_id, object));
  }
}
