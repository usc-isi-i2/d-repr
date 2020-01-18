use cpython::{PyClone, PyDict, PyList, Python, PythonObject, ToPyObject};
use hashbrown::HashMap;

use readers::value::Value;

use crate::writers::stream_writer::graph_py::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::StreamClassWriter;

pub struct TrackWithURIOptionalWriter<'a> {
  pub py: Python<'a>,

  /// id of the class
  pub class_id: usize,

  /// uri of the class
  pub ont_class: &'a str,

  /// All inserted nodes
  pub all_nodes: *const HashMap<String, HashMap<String, PyDict>>,
  /// A mapping of node id to its data
  pub nodes: &'a mut HashMap<String, PyDict>,
  /// A mapping of class id to list of nodes
  /// This share same record pointers with the `nodes` property
  pub class_nodes: &'a mut Vec<PyDict>,

  /// current node that we are writing information into
  pub curr_node: PyDict,

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
      node.set_item(self.py, "@id", subject).unwrap();
      for p in self.class_predicates {
        node.set_item(self.py, p, PyList::new(self.py, &[])).unwrap();
      }

      self.class_nodes.push(node.clone_ref(self.py));
      self.nodes.insert(subject.to_string(), node);
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);
    } else {
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);
      for p in self.class_predicates {
        if self.curr_node.contains(self.py, p).unwrap() {
          self.curr_node.set_item(self.py, p, PyList::new(self.py, &[])).unwrap();
        }
      }
    }

    is_new
  }

  fn end_record(&mut self) {
  }

  fn begin_partial_buffering_record(&mut self, subject: &str, _is_blank: bool) -> bool {
    let is_new = !self.nodes.contains_key(subject);
    if is_new {
      let node = PyDict::new(self.py);
      node.set_item(self.py, "@id", subject).unwrap();
      for p in self.class_predicates {
        node.set_item(self.py, p, PyList::new(self.py, &[])).unwrap();
      }

      self.class_nodes.push(node.clone_ref(self.py));
      self.nodes.insert(subject.to_string(), node);
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);
      self.buffer_oprops[self.class_id].push(TempObjectProps {
        id: subject.to_string(),
        props: vec![]
      });
    } else {
      self.curr_node = self.nodes.get(subject).unwrap().clone_ref(self.py);
      for p in self.class_predicates {
        if self.curr_node.contains(self.py, p).unwrap() {
          self.curr_node.set_item(self.py, p, PyList::new(self.py, &[])).unwrap();
        }
      }
    }
    is_new
  }

  fn end_partial_buffering_record(&mut self) {
  }

  fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
    let v = self.curr_node.get_item(self.py, &self.predicates[predicate_id]).unwrap();
    let lst = v.cast_as::<PyList>(self.py).unwrap();
    lst.append_item(self.py, value.to_py_object(self.py));
  }

  fn write_object_property(&mut self, _target_cls: usize, _subject: &str, predicate_id: usize, object: &str, _is_subject_blank: bool, _is_object_blank: bool, _is_new_subj: bool) {
    let v = self.curr_node.get_item(self.py, &self.predicates[predicate_id]).unwrap();
    let lst = v.cast_as::<PyList>(self.py).unwrap();
    lst.append_item(self.py, object.to_py_object(self.py).into_object());
  }

  fn buffer_object_property(&mut self, target_cls: usize, predicate_id: usize, object: String, _is_object_blank: bool) {
    self.buffer_oprops[self.class_id].last_mut().unwrap()
      .props
      .push((target_cls, predicate_id, object));
  }
}