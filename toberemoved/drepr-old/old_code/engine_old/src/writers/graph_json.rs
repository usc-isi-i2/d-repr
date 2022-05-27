use std::fs::File;
use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashMap};
use super::StreamStateWriter;
use crate::readers::Value;
use crate::models::*;


/// A temporary object holds links of the records to other records
#[derive(Default)]
struct TempObjectProps {
  /// id of the record in the graph
  id: usize,
  /// list of links of the record to other records
  ///
  /// * .0 - id of the class of the target record that the record is linked to (`node_id` of the
  ///        class node in the semantic model)
  /// * .1 - id of the object property (predicate), which is `edge_id` of the edge in the semantic
  ///        model)
  /// * .2 - pseudo_id of the target record
  props: Vec<(usize, usize, String)>,
}

/// A writer that write records with custom graph_json format
///
/// The format use two channels to stream output:
///
/// 1. `node_channel`: for writing nodes and their data properties only. Each line is one node (json
///   format): `{"id": <number>, "data": {["@id": <uri of node if have>], "@type": <node_type>}}`.
///   Every node is identified uniquely by a number (`id` not `@id`) in the graph
/// 2. `edge_channel`: for writing edges. Each line is one edge (tab-separated): `<source_id>\t<target_id>\t<label>`
pub struct GraphJSONWriter<W: Write + Debug> {
  /// The writer is going to write nodes into this channel
  node_channel: BufWriter<W>,

  /// The writer is going to write edges into this channel
  edge_channel: BufWriter<W>,

  /// A number for auto-increment id
  auto_increment_id: usize,

  /// id of the current class that the writing records belong to (`node_id` of the class node in
  /// the semantic model)
  current_class_id: usize,

  /// id of the current writing record
  current_node_id: usize,

  /// Mappings (one per class) that map the pseudo ids of records to their ids in the graph.
  /// As the ids of classes are contiguous (start from 0), instead of using a hashmap, we use
  /// a vector instead.
  ///
  /// Usage: `let real_id = idmap[class_id][pseudo_id];` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  idmap: Vec<HashMap<String, usize>>,

  /// Mappings (one per class) that map ids of records, which have been sent to the writer to write
  /// to their ids in the graph. As the ids of classes are contiguous (start from 0),
  /// instead of using a hashmap, we use a vector instead.
  ///
  /// Usage: `written_records[class_id].contains(real_id)` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  written_records: Vec<HashMap<String, usize>>,

  /// Lists (one per class) of buffered object links (get from `buffer_object_property`)
  buffer_oprops: Vec<Vec<TempObjectProps>>,

  /// A map that tells if a class has URI or not.
  has_uri: Vec<bool>,

  /// A map that maps class id (`node_id`) to class label
  classes: Vec<String>,

  /// A map that maps property id (`edge_id`) to its label
  predicates: Vec<String>,
}

impl<W: Write + Debug> GraphJSONWriter<W> {
  pub fn write2file(node_fpath: &str, edge_fpath: &str) -> GraphJSONWriter<File> {
    GraphJSONWriter {
      node_channel: BufWriter::new(File::create(node_fpath).expect("Unable to create file")),
      edge_channel: BufWriter::new(File::create(edge_fpath).expect("Unable to create file")),
      auto_increment_id: 0,
      current_class_id: 0,
      current_node_id: 0,
      idmap: vec![],
      written_records: vec![],
      buffer_oprops: vec![],
      has_uri: vec![],
      classes: vec![],
      predicates: vec![],
    }
  }

  pub fn write2str() -> GraphJSONWriter<Vec<u8>> {
    GraphJSONWriter {
      node_channel: BufWriter::new(Vec::new()),
      edge_channel: BufWriter::new(Vec::new()),
      auto_increment_id: 0,
      current_class_id: 0,
      current_node_id: 0,
      idmap: vec![],
      written_records: vec![],
      buffer_oprops: vec![],
      has_uri: vec![],
      classes: vec![],
      predicates: vec![],
    }
  }

  pub fn into_inner(self) -> (W, W) {
    (self.node_channel.into_inner().unwrap(), self.edge_channel.into_inner().unwrap())
  }

  #[inline]
  fn get_next_id(&mut self) -> usize {
    let id = self.auto_increment_id;
    self.auto_increment_id += 1;
    return id;
  }
}


impl<W: Write + Debug> StreamStateWriter for GraphJSONWriter<W> {
  fn init(&mut self, sm: &SemanticModel) {
    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          self.idmap.push(Default::default());
          self.written_records.push(Default::default());
          self.buffer_oprops.push(Default::default());
          self.classes.push(format!("{}", serde_json::Value::String(n.rel_label.clone())));

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
      self.predicates.push(format!("{}", serde_json::Value::String(edge.rel_label.clone())));
    }
  }

  fn has_written_record(&self, class_id: usize, pseudo_id: &str) -> bool {
    self.idmap[class_id].contains_key(pseudo_id)
  }

  fn begin(&mut self) {}

  fn end(&mut self) {
    for records in self.buffer_oprops.drain(..) {
      for r in records {
        for (cid, pid, opid) in r.props {
          write!(self.edge_channel, "{}\t{}\t{}\n", r.id, self.idmap[cid][&opid], self.predicates[pid]).unwrap();
        }
      }
    }
  }

  fn begin_class(&mut self, class_id: usize) {
    self.current_class_id = class_id;
  }

  fn end_class(&mut self) {
  }

  fn begin_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool {
    if self.has_uri[self.current_class_id] {
      if self.written_records[self.current_class_id].contains_key(subject) {
        self.idmap[self.current_class_id]
            .insert(subject_pseudo_id.to_string(), self.written_records[self.current_class_id][subject]);
        return false;
      }

      self.current_node_id = self.get_next_id();
      self.written_records[self.current_class_id].insert(subject.to_string(), self.current_node_id);
      self.idmap[self.current_class_id].insert(subject_pseudo_id.to_string(), self.current_node_id);

      write!(self.node_channel, r#"{{"id":{},"data":{{"@id":"{}","@type":{}"#,
             self.current_node_id, subject, self.classes[self.current_class_id])
        .unwrap();
    } else {
      self.current_node_id = self.get_next_id();
      self.idmap[self.current_class_id].insert(subject_pseudo_id.to_string(), self.current_node_id);

      write!(self.node_channel, r#"{{"id":{},"data":{{"@type":{}"#,
             self.auto_increment_id, self.classes[self.current_class_id])
        .unwrap();
    }

    return true;
  }

  fn end_record(&mut self) {
    write!(self.node_channel, "}}}}\n").unwrap();
  }

  fn begin_partial_buffering_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool {
    if self.has_uri[self.current_class_id] {
      if self.written_records[self.current_class_id].contains_key(subject) {
        self.idmap[self.current_class_id]
            .insert(subject_pseudo_id.to_string(), self.written_records[self.current_class_id][subject]);
        return false;
      }

      self.current_node_id = self.get_next_id();
      self.written_records[self.current_class_id].insert(subject.to_string(), self.current_node_id);
      write!(self.node_channel, r#"{{"id":{},"data":{{"@id":"{}","@type":{}"#,
             self.current_node_id, subject, self.classes[self.current_class_id])
        .unwrap();
    } else {
      self.current_node_id = self.get_next_id();
      write!(self.node_channel, r#"{{"id":{},"data":{{"@type":{}"#,
             self.auto_increment_id, self.classes[self.current_class_id])
        .unwrap();
    }

    self.idmap[self.current_class_id].insert(subject_pseudo_id.to_string(), self.current_node_id);
    self.buffer_oprops[self.current_class_id].push(TempObjectProps {
      id: self.current_node_id,
      props: vec![]
    });

    return true;
  }

  fn end_partial_buffering_record(&mut self) {
    write!(self.node_channel, "}}}}\n").unwrap();
  }

  fn write_data_property(&mut self, _subject: &str, predicate: usize, value: &Value) {
    write!(self.node_channel, ",{}:{}", self.predicates[predicate], value.to_serde_json())
      .unwrap();
  }

  fn write_object_property(&mut self, target_cls: usize, _subject: &str, predicate: usize, object_pseudo_id: &str, _new_subject: bool) {
    write!(self.edge_channel, "{}\t{}\t{}\n",
           self.current_node_id,
           self.idmap[target_cls][object_pseudo_id],
           self.predicates[predicate]).unwrap();
  }

  fn buffer_object_property(&mut self, target_cls: usize, predicate: usize, object_pseudo_id: String) {
    self.buffer_oprops[self.current_class_id].last_mut().unwrap()
        .props
        .push((target_cls, predicate, object_pseudo_id));
  }
}