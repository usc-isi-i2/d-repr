use std::fs::File;
use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashMap, HashSet};

use crate::models::{GraphNode, SemanticModel};
use crate::readers::Value;

use super::StreamStateWriter;

/// A temporary object holds links of the records to other records
#[derive(Default)]
struct TempObjectProps {
  /// real id of the record
  id: String,
  /// list of links of the record to other records
  ///
  /// * .0 - id of the class of the target record that the record is linked to (`node_id` of the
  ///        class node in the semantic model)
  /// * .1 - id of the object property (predicate), which is `edge_id` of the edge in the semantic
  ///        model)
  /// * .2 - pseudo_id of the target record
  props: Vec<(usize, usize, String)>,
}

/// A writer that write records with Turtle format.
pub struct TurtleWriter<W: Write + Debug> {
  /// The channel that this writer is going to write into
  channel: BufWriter<W>,

  /// id of the current class that the writing records belong to (`node_id` of the class node in
  /// the semantic model)
  current_class_id: usize,

  /// Sets of record ids (one per class) that has been sent to the writer to write.
  /// As the ids of classes are contiguous (start from 0), instead of using a hashmap, we use
  /// a vector instead.
  ///
  /// Usage: `written_records[class_id].contains(real_id)` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  written_records: Vec<HashSet<String>>,

  /// Mappings (one per class) that map the pseudo ids of records to their real ids.
  /// As the ids of classes are contiguous (start from 0), instead of using a hashmap, we use
  /// a vector instead.
  ///
  /// Usage: `let real_id = idmap[class_id][pseudo_id];` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  idmap: Vec<HashMap<String, String>>,

  /// Lists (one per class) of buffered object links (get from `buffer_object_property`)
  buffer_oprops: Vec<Vec<TempObjectProps>>,

  /// A map that tells if a class has URI or not.
  has_uri: Vec<bool>,

  /// A map that maps class id (`node_id`) to class label
  classes: Vec<String>,

  /// A map that maps property id (`edge_id`) to its label
  predicates: Vec<String>,

  /// A map from property id (`edge_id`) to its template for writing value
  value_templates: Vec<ValueFmt>,

  /// Serialized ontology prefixes
  prefixes: String,
}

impl<W: Write + Debug> TurtleWriter<W> {
  pub fn write2file(fpath: &str) -> TurtleWriter<File> {
    TurtleWriter {
      channel: BufWriter::new(File::create(fpath).expect("Unable to create file")),
      current_class_id: 0,
      written_records: Vec::default(),
      buffer_oprops: Vec::default(),
      idmap: Vec::default(),
      has_uri: Vec::default(),
      classes: vec![],
      predicates: vec![],
      value_templates: vec![],
      prefixes: String::new(),
    }
  }

  pub fn write2str() -> TurtleWriter<Vec<u8>> {
    TurtleWriter {
      current_class_id: 0,
      idmap: Vec::default(),
      has_uri: Vec::default(),
      written_records: Vec::default(),
      channel: BufWriter::new(Vec::new()),
      buffer_oprops: Vec::default(),
      classes: vec![],
      predicates: vec![],
      value_templates: vec![],
      prefixes: String::new(),
    }
  }

  pub fn into_inner(self) -> W {
    self.channel.into_inner().unwrap()
  }
}

impl<W: Write + Debug> StreamStateWriter for TurtleWriter<W> {
  fn init(&mut self, sm: &SemanticModel) {
    for (k, v) in sm.prefixes.iter() {
      self.prefixes.push_str("@prefix ");
      self.prefixes.push_str(k);
      self.prefixes.push_str(": <");
      self.prefixes.push_str(v);
      self.prefixes.push_str("> .\n");
    }

    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          self.idmap.push(Default::default());
          self.written_records.push(Default::default());
          self.buffer_oprops.push(Default::default());

          if n.abs_label.is_some() {
            self.classes.push(n.rel_label.clone());
          } else {
            self.classes.push(format!("<{}>", n.rel_label));
          }

          let mut has_uri = false;
          for &e in &sm.outgoing_edges[n.node_id] {
            if sm.edges[e].rel_label == "drepr:uri" {
              has_uri = true;
              break;
            }
          }

          self.has_uri.push(has_uri);
        }
        _ => {
          break;
        }
      }
    }

    for edge in &sm.edges {
      if edge.abs_label.is_some() {
        self.predicates.push(edge.rel_label.clone());
      } else {
        self.predicates.push(format!("<{}>", edge.rel_label));
      }

      match &sm.nodes[edge.target] {
        GraphNode::ClassNode(n) => {
          if n.is_blank_node(sm) {
            self.value_templates.push(ValueFmt::new(
              format!("\t{} ", self.predicates[edge.edge_id]),
              ";\n".to_string()));
          } else {
            self.value_templates.push(ValueFmt::new(
              format!("\t{} <", self.predicates[edge.edge_id]),
              "> ;\n".to_string()));
          }
        }
        GraphNode::DataNode(n) => {
          match &n.data_type {
            None => {
              self.value_templates.push(ValueFmt::new_no_type(
                format!("\t{} ", self.predicates[edge.edge_id]),
                ";\n".to_string()));
            }
            Some(dt) => {
              if dt == "xsd:string" {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} \"", self.predicates[edge.edge_id]),
                  "\";\n".to_string()));
              } else if dt == "xsd:anyURI" {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} <", self.predicates[edge.edge_id]),
                  ">;\n".to_string()));
              } else {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} \"", self.predicates[edge.edge_id]),
                  format!("\"^^{};\n", dt)));
              }
            }
          }
        }
        GraphNode::LiteralNode(n) => {
          match &n.data_type {
            None => {
              self.value_templates.push(ValueFmt::new_no_type(
                format!("\t{} ", self.predicates[edge.edge_id]),
                ";\n".to_string()));
            }
            Some(dt) => {
              if dt == "xsd:string" {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} \"", self.predicates[edge.edge_id]),
                  "\";\n".to_string()));
              } else if dt == "xsd:anyURI" {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} <", self.predicates[edge.edge_id]),
                  ">;\n".to_string()));
              } else {
                self.value_templates.push(ValueFmt::new(
                  format!("\t{} \"", self.predicates[edge.edge_id]),
                  format!("\"^^{};\n", dt)));
              }
            }
          }
        }
      }
    }
  }

  fn has_written_record(&self, class_id: usize, pseudo_id: &str) -> bool {
    self.idmap[class_id].contains_key(pseudo_id)
  }

  fn begin(&mut self) {
    write!(
      self.channel, "{}\n{}\n{}\n{}\n{}\n\n",
      "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .",
      "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .",
      "@prefix xml: <http://www.w3.org/XML/1998/namespace> .",
      "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .",
      self.prefixes
    ).unwrap();
  }

  fn end(&mut self) {
    for records in self.buffer_oprops.drain(..) {
      for r in records {
        for (cid, pid, opid) in r.props {
          write!(self.channel, "{} {}", r.id, self.value_templates[pid].get_value(&self.idmap[cid][&opid])).unwrap();
        }
      }
    }
  }

  fn begin_class(&mut self, class_id: usize) {
    self.current_class_id = class_id;
  }

  fn end_class(&mut self) {}

  fn begin_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool {
    self.idmap[self.current_class_id]
      .insert(subject_pseudo_id.to_string(), subject.to_string());

    if self.has_uri[self.current_class_id] {
      if self.written_records[self.current_class_id].contains(subject) {
        return false;
      }
      self.written_records[self.current_class_id].insert(subject.to_string());

      write!(self.channel, "<{}> a {};\n", subject, self.classes[self.current_class_id])
      .unwrap();
    } else {
      write!(self.channel, "{} a {};\n", subject, self.classes[self.current_class_id])
      .unwrap();
    }

    return true;
  }

  fn end_record(&mut self) {
    self.channel.write("\t.\n".as_bytes()).unwrap();
  }

  fn begin_partial_buffering_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool {
    self.idmap[self.current_class_id]
      .insert(subject_pseudo_id.to_string(), subject.to_string());

    self.buffer_oprops[self.current_class_id].push(TempObjectProps {
      id: subject.to_string(),
      props: vec![],
    });

    if self.has_uri[self.current_class_id] {
      if self.written_records[self.current_class_id].contains(subject) {
        return false;
      }
      self.written_records[self.current_class_id].insert(subject.to_string());

      write!(self.channel, "<{}> a {};\n", subject, self.classes[self.current_class_id])
      .unwrap();
    } else {
      write!(self.channel, "{} a {};\n", subject, self.classes[self.current_class_id])
        .unwrap();
    }

    return true;
  }

  fn end_partial_buffering_record(&mut self) {
    self.channel.write("\t.\n".as_bytes()).unwrap();
  }

  fn write_data_property(&mut self, _subj: &str, pred: usize, dval: &Value) {
    match dval {
      Value::Null => unreachable!(),
      Value::Str(v) => {
        self.channel.write(self.value_templates[pred].get_value_as_str(&v.replace("\"", "\\\"")).as_bytes()).unwrap();
      }
      Value::Bool(v) => {
        self.channel.write(self.value_templates[pred].get_value(&v.to_string()).as_bytes()).unwrap();
      }
      Value::I64(v) => {
        self.channel.write(self.value_templates[pred].get_value(&v.to_string()).as_bytes()).unwrap();
      }
      Value::F64(v) => {
        self.channel.write(self.value_templates[pred].get_value(&v.to_string()).as_bytes()).unwrap();
      }
      Value::Array(_) => unimplemented!("[ttl] write array {:?}", dval),
      Value::Object(_) => unimplemented!("[ttl] write array {:?}", dval),
    }
  }

  fn write_object_property(
    &mut self,
    target_cls: usize,
    subj: &str,
    pred: usize,
    object: &str,
    new_subject: bool
  ) {
    if new_subject {
      self.channel.write(self.value_templates[pred].get_value(&self.idmap[target_cls][object]).as_bytes()).unwrap();
    } else {
      // we know that the only way for this is not a new subject is that it is uri
      write!(self.channel, "<{}> ", subj).unwrap();
      self.channel.write(self.value_templates[pred].get_value(&self.idmap[target_cls][object]).as_bytes()).unwrap();
      self.channel.write(&[b'.']).unwrap();
    }
  }

  fn buffer_object_property(
    &mut self,
    target_cls: usize,
    predicate: usize,
    object_pseudo_id: String,
  ) {
    self.buffer_oprops[self.current_class_id].last_mut().unwrap()
      .props
      .push((target_cls, predicate, object_pseudo_id));
  }
}

struct ValueFmt {
  left: String,
  right: String,
  left_as_str: String,
  right_as_str: String,
}

impl ValueFmt {
  pub fn new(left: String, right: String) -> ValueFmt {
    ValueFmt {
      left_as_str: left.clone(),
      right_as_str: right.clone(),
      left,
      right,
    }
  }

  pub fn new_no_type(left: String, right: String) -> ValueFmt {
    ValueFmt {
      left_as_str: format!("{}\"", left),
      right_as_str: format!("\"{}", right),
      left,
      right,
    }
  }

  #[inline]
  pub fn get_value_as_str(&mut self, val: &str) -> String {
    self.left_as_str.clone() + val + &self.right_as_str
  }

  #[inline]
  pub fn get_value(&mut self, val: &str) -> String {
    self.left.clone() + val + &self.right
  }
}