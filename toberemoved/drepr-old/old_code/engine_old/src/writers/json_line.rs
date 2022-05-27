use super::StreamStateWriter;
use crate::readers::Value;
use crate::models::{SemanticModel, GraphNode};
use fnv::FnvHashMap;
use hashbrown::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

struct BufferedObject {
  object: FnvHashMap<String, serde_json::Value>,
  props: Vec<(usize, usize, String)>,
}

pub struct JsonLineWriter<W: Write> {
  current_class_id: usize,
  current_object: FnvHashMap<String, serde_json::Value>,
  // class => pseudo_id => real_id
  idmap: Vec<HashMap<String, String>>,
  write_buffer: BufWriter<W>,
  classes: Vec<String>,
  predicates: Vec<String>,
  buffer: Vec<Vec<BufferedObject>>,
}

impl<W: Write> JsonLineWriter<W> {
  pub fn write2file(fpath: &str) -> JsonLineWriter<File> {
    JsonLineWriter {
      current_class_id: 0,
      idmap: Default::default(),
      write_buffer: BufWriter::new(File::create(fpath).expect("Unable to create file")),
      current_object: FnvHashMap::default(),
      classes: vec![],
      predicates: vec![],
      buffer: vec![],
    }
  }
}

impl<W: Write> StreamStateWriter for JsonLineWriter<W> {
  fn init(&mut self, sm: &SemanticModel) {
    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          self.idmap.push(Default::default());
          self.buffer.push(Default::default());

          if let Some(lbl) = &n.abs_label {
            self.classes.push(lbl.clone());
          } else {
            self.classes.push(n.rel_label.clone());
          }
        }
        _ => {
          break;
        }
      }
    }

    for edge in &sm.edges {
      if let Some(lbl) = &edge.abs_label {
        self.predicates.push(lbl.clone());
      } else {
        self.predicates.push(edge.rel_label.clone());
      }
    }
  }

  fn begin(&mut self) {}
  fn end(&mut self) {
    for records in self.buffer.drain(..) {
      for mut r in records {
        for (cid, pid, opid) in r.props {
          r.object.insert(self.predicates[pid].clone(), serde_json::Value::String(self.idmap[cid][&opid].clone()));
        }

        self
          .write_buffer
          .write(
            serde_json::to_string(&r.object)
              .unwrap()
              .as_bytes(),
          )
          .unwrap();
        self.write_buffer.write("\n".as_bytes()).unwrap();
      }
    }
  }

  fn begin_class(&mut self, class_id: usize) {
    self.current_class_id = class_id;
  }

  fn end_class(&mut self) {}

  fn begin_subject(&mut self, subject_pseudo_id: &str, subject: &str) {
    self.idmap[self.current_class_id]
      .insert(subject_pseudo_id.to_string(), subject.to_string());
    self.current_object.clear();
    self.current_object.insert(
      "@type".to_string(),
      serde_json::Value::String(self.classes[self.current_class_id].clone()),
    );
    self.current_object.insert(
      "@id".to_string(),
      serde_json::Value::String(subject.to_string()),
    );
  }

  fn begin_partial_buffering_subject(&mut self, subject_pseudo_id: &str, subject: &str) {
    self.idmap[self.current_class_id]
      .insert(subject_pseudo_id.to_string(), subject.to_string());
    self.buffer[self.current_class_id].push(BufferedObject {
      object: Default::default(),
      props: vec![]
    });
    self.current_object.clear();
    self.current_object.insert(
      "@type".to_string(),
      serde_json::Value::String(self.classes[self.current_class_id].clone()),
    );
    self.current_object.insert(
      "@id".to_string(),
      serde_json::Value::String(subject.to_string()),
    );
  }

  fn write_data_property(&mut self, _subject: &str, predicate: usize, value: &Value) {
    self
      .current_object
      .insert(self.predicates[predicate].clone(), value.to_serde_json());
  }

  fn write_object_triple(
    &mut self,
    target_cls: usize,
    _subject: &str,
    predicate: usize,
    object_pseudo_id: &str,
  ) {
    self.current_object.insert(
      self.predicates[predicate].to_string(),
      serde_json::Value::String(self.idmap[target_cls][object_pseudo_id].clone()),
    );
  }

  fn end_subject(&mut self) {
    self
      .write_buffer
      .write(
        serde_json::to_string(&self.current_object)
          .unwrap()
          .as_bytes(),
      )
      .unwrap();
    self.write_buffer.write("\n".as_bytes()).unwrap();
  }

  fn end_partial_buffering_subject(&mut self) {
    let r = self.buffer[self.current_class_id].last_mut().unwrap();
    std::mem::swap(&mut r.object, &mut self.current_object);
  }

  fn buffer_object_triple(
    &mut self,
    _target_cls: usize,
    _predicate: usize,
    _object_pseudo_id: String,
  ) {
    unimplemented!()
  }
}
