use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};

use hashbrown::{HashMap};

use crate::lang::{GraphNode, SemanticModel};
use crate::writers::stream_writer::{StreamClassWriter, WriteMode};

use super::StreamWriter;

use self::temp_object_props::TempObjectProps;
use self::class_writers::track_withurioptional_writer::TrackWithURIOptionalWriter;
use crate::writers::stream_writer::stream_writer::{ExtractWriterResult, WriteResult, StreamWriterResult};
use crate::writers::stream_writer::graph_json::json_value_fmt::JSONValueFmt;

mod temp_object_props;
mod class_writers;
pub mod json_value_fmt;

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

  /// Mappings (one per class) that map ids of records, which have been sent to the writer to write
  /// to their ids in the graph. As the ids of classes are contiguous (start from 0),
  /// instead of using a hashmap, we use a vector instead.
  ///
  /// Usage: `written_records[class_id].contains(real_id)` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  written_records: Vec<HashMap<String, usize>>,

  /// Lists (one per class) of buffered object links (get from `buffer_object_property`)
  buffer_oprops: Vec<Vec<TempObjectProps>>,

  /// A map that maps class id (`node_id`) to class label
  classes: Vec<String>,

  /// A map that maps property id (`edge_id`) to its label
  predicates: Vec<String>,
  
  /// A map from property id (`edge_id`) to its formatter for writing value
  value_fmts: Vec<Box<dyn JSONValueFmt<W>>>,
}

impl<W: Write + Debug> GraphJSONWriter<W> {
  pub fn write2channel(node_channel: BufWriter<W>, edge_channel: BufWriter<W>, sm: &SemanticModel) -> GraphJSONWriter<W> {
    let mut writer = GraphJSONWriter {
      node_channel,
      edge_channel,
      auto_increment_id: 0,
      written_records: vec![],
      buffer_oprops: vec![],
      classes: vec![],
      predicates: vec![],
      value_fmts: vec![],
    };

    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          writer.written_records.push(Default::default());
          writer.buffer_oprops.push(Default::default());
          writer.classes.push(format!("{}", serde_json::Value::String(n.rel_label.clone())));
        }
        _ => {
          break;
        }
      }
    }

    for edge in &sm.edges {
      writer.predicates.push(format!("{}", serde_json::Value::String(edge.rel_label.clone())));
      let dt: Option<&str> = match edge.get_target(&sm) {
        GraphNode::DataNode(n) => match &n.data_type {
          None => None,
          Some(dt) => Some(dt.as_str())
        },
        GraphNode::LiteralNode(n) => match &n.data_type {
          None => None,
          Some(dt) => Some(dt.as_str())
        },
        _ => None
      };

      let fmt: Box<dyn JSONValueFmt<W>> = match dt {
        Some("xsd:string") => Box::new(json_value_fmt::StrValueFmt {}),
        Some("xsd:int") => Box::new(json_value_fmt::IntValueFmt {}),
        Some("xsd:decimal") => Box::new(json_value_fmt::FloatValueFmt {}),
        _ => Box::new(json_value_fmt::UnspecifiedValueFmt {}),
      };

      writer.value_fmts.push(fmt);
    }

    writer
  }
}

impl GraphJSONWriter<File> {
  pub fn write2file(node_fpath: &str, edge_fpath: &str, sm: &SemanticModel) -> GraphJSONWriter<File> {
    GraphJSONWriter::write2channel(
      BufWriter::new(File::create(node_fpath).expect("Unable to create file")),
      BufWriter::new(File::create(edge_fpath).expect("Unable to create file")),
      sm
    )
  }
}

impl GraphJSONWriter<Vec<u8>> {
  pub fn write2str(sm: &SemanticModel) -> GraphJSONWriter<Vec<u8>> {
    GraphJSONWriter::write2channel(
      BufWriter::new(Vec::new()),
      BufWriter::new(Vec::new()),
      sm
    )
  }
}

impl<W: Write + Debug> StreamWriter for GraphJSONWriter<W> {
  fn begin(&mut self) {}

  fn end(&mut self) {
    for records in self.buffer_oprops.drain(..) {
      for r in records {
        for (cid, pid, opid) in r.props {
          write!(self.edge_channel, "{}\t{}\t{}\n", r.id, self.written_records[cid][&opid], self.predicates[pid]).unwrap();
        }
      }
    }
  }

  fn begin_class<'a>(&'a mut self, class_id: usize, _write_mode: WriteMode) -> Box<dyn StreamClassWriter + 'a> {
    Box::new(TrackWithURIOptionalWriter {
      class_id,
      ont_class: &self.classes[class_id],
      node_channel: &mut self.node_channel,
      edge_channel: &mut self.edge_channel,
      predicates: &self.predicates,
      value_fmts: &self.value_fmts,
      buffer_oprops: &mut self.buffer_oprops,
      written_records: &mut self.written_records,
      curr_node_id: 0,
      auto_increment_id: &mut self.auto_increment_id
    })
  }

  fn end_class(&mut self) {}
}

impl ExtractWriterResult for GraphJSONWriter<File> {
  fn extract_result(self: Box<Self>) -> WriteResult {
    WriteResult::None
  }
}

impl ExtractWriterResult for GraphJSONWriter<Vec<u8>> {
  fn extract_result(self: Box<Self>) -> WriteResult {
    WriteResult::Str2(
      unsafe { String::from_utf8_unchecked(self.node_channel.into_inner().unwrap()) },
      unsafe { String::from_utf8_unchecked(self.edge_channel.into_inner().unwrap()) }
    )
  }
}

impl StreamWriterResult for GraphJSONWriter<File> {}
impl StreamWriterResult for GraphJSONWriter<Vec<u8>> {}