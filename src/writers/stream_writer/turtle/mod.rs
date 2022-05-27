use std::fs::File;
use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashSet};

use crate::lang::{GraphNode, SemanticModel};

use super::StreamWriter;
use self::value_fmt::ValueFmt;
use self::temp_object_props::TempObjectProps;
use itertools::Itertools;
use crate::writers::stream_writer::{StreamClassWriter, WriteMode};

mod value_fmt;
mod temp_object_props;
mod class_writers;

use self::class_writers::*;
use crate::writers::stream_writer::stream_writer::{WriteResult, ExtractWriterResult, StreamWriterResult};

pub struct TTLStreamWriter<W: Write + Debug> {
  /// The channel that this writer is going to write into
  channel: BufWriter<W>,
  
  /// Sets of record ids (one per class) that has been sent to the writer to write.
  /// As the ids of classes are contiguous (start from 0), instead of using a hashmap, we use
  /// a vector instead.
  ///
  /// Usage: `written_records[class_id].contains(real_id)` where `class_id` is `node_id` of the class
  /// node in the semantic model.
  written_records: Vec<HashSet<String>>,
  
  /// A map from a class id to a boolean value, telling if a class has all records written (no missing
  /// records). This is useful to use with `written_records` to test if a record has been sent to the
  /// writer to write. For example: `always_write_records[class_id] || written_records[class_id].contains(<..>)`
  ///
  /// The reason for this variable to exist is that in some write mode, we do not track the record id
  always_write_records: Vec<bool>,
  
  /// Lists (one per class) of buffered object links (get from `buffer_object_property`)
  buffer_oprops: Vec<Vec<TempObjectProps>>,
  
  /// A map that maps class id (`node_id`) to class label
  classes: Vec<String>,
  
  /// A map that maps property id (`edge_id`) to its label
  predicates: Vec<String>,
  
  /// A map from property id (`edge_id`) to its template for writing value
  value_templates: Vec<ValueFmt>,
  
  /// Serialized ontology prefixes
  prefixes: String,
}

impl<W: Write + Debug> TTLStreamWriter<W> {
  pub fn write2channel(channel: BufWriter<W>, sm: &SemanticModel) -> TTLStreamWriter<W> {
    let prefixes = sm.prefixes.iter()
      .map(|(k, v)| format!("@prefix {}: <{}> .\n", k, v))
      .join("");
    
    let mut written_records = vec![];
    let mut buffer_oprops = vec![];
    let mut classes = vec![];
    let mut predicates = vec![];
    let mut value_templates = vec![];
    
    for node in &sm.nodes {
      match node {
        GraphNode::ClassNode(n) => {
          written_records.push(Default::default());
          buffer_oprops.push(Default::default());
          classes.push(n.rel_label.clone());
        }
        _ => {
          break;
        }
      }
    }
    
    for edge in &sm.edges {
      predicates.push(edge.rel_label.clone());
      match &sm.nodes[edge.target] {
        GraphNode::ClassNode(n) => {
          if n.is_blank_node(sm) {
            value_templates.push(ValueFmt::specified_type(
              format!("\t{} ", predicates[edge.edge_id]),
              ";\n".to_string()));
          } else {
            value_templates.push(ValueFmt::specified_type(
              format!("\t{} <", predicates[edge.edge_id]),
              "> ;\n".to_string()));
          }
        }
        GraphNode::DataNode(n) => {
          match &n.data_type {
            None => {
              value_templates.push(ValueFmt::unspecified_type(
                format!("\t{} ", predicates[edge.edge_id]),
                ";\n".to_string()));
            }
            Some(dt) => {
              if dt == "xsd:string" {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} \"", predicates[edge.edge_id]),
                  "\";\n".to_string()));
              } else if dt == "xsd:anyURI" {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} <", predicates[edge.edge_id]),
                  ">;\n".to_string()));
              } else {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} \"", predicates[edge.edge_id]),
                  format!("\"^^{};\n", dt)));
              }
            }
          }
        }
        GraphNode::LiteralNode(n) => {
          match &n.data_type {
            None => {
              value_templates.push(ValueFmt::unspecified_type(
                format!("\t{} ", predicates[edge.edge_id]),
                ";\n".to_string()));
            }
            Some(dt) => {
              if dt == "xsd:string" {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} \"", predicates[edge.edge_id]),
                  "\";\n".to_string()));
              } else if dt == "xsd:anyURI" {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} <", predicates[edge.edge_id]),
                  ">;\n".to_string()));
              } else {
                value_templates.push(ValueFmt::specified_type(
                  format!("\t{} \"", predicates[edge.edge_id]),
                  format!("\"^^{};\n", dt)));
              }
            }
          }
        }
      }
    }
    
    TTLStreamWriter {
      channel,
      // we init this as false, but later when a class writer, is obtained, we can set its value
      // correctly based on the write mode
      always_write_records: vec![false; written_records.len()],
      written_records,
      buffer_oprops,
      classes,
      predicates,
      value_templates,
      prefixes,
    }
  }
}

impl TTLStreamWriter<File> {
  pub fn write2file(fpath: &str, sm: &SemanticModel) -> TTLStreamWriter<File> {
    TTLStreamWriter::<File>::write2channel(
      BufWriter::new(File::create(fpath).expect("Unable to create file")),
      sm,
    )
  }
}

impl TTLStreamWriter<Vec<u8>> {
  pub fn write2str(sm: &SemanticModel) -> TTLStreamWriter<Vec<u8>> {
    TTLStreamWriter::<Vec<u8>>::write2channel(
      BufWriter::new(Vec::new()),
      sm,
    )
  }
}

impl<W: Write + Debug> StreamWriter for TTLStreamWriter<W> {
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
        if r.is_blank {
          for (pid, oid, is_obj_blank) in r.props {
            if is_obj_blank {
              write!(self.channel, "{} {} {}.\n", r.id, self.predicates[pid], oid).unwrap();
            } else {
              write!(self.channel, "{} {} <{}>.\n", r.id, self.predicates[pid], oid).unwrap();
            }
          }
        } else {
          for (pid, oid, is_obj_blank) in r.props {
            if is_obj_blank {
              write!(self.channel, "<{}> {} {}.\n", r.id, self.predicates[pid], oid).unwrap();
            } else {
              write!(self.channel, "<{}> {} <{}>.\n", r.id, self.predicates[pid], oid).unwrap();
            }
          }
        }
      }
    }
  }
  
  fn begin_class<'a>(&'a mut self, class_id: usize, write_mode: WriteMode) -> Box<dyn StreamClassWriter + 'a> {
    match write_mode {
      WriteMode::Tt_Ut_Sb_Ob => {
        Box::new(Tt_Ut_Sb_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Sb_Ou => {
        Box::new(Tt_Ut_Sb_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Sb_On => {
        Box::new(Tt_Ut_Sb_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Su_Ob => {
        Box::new(Tt_Ut_Su_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Su_Ou => {
        Box::new(Tt_Ut_Su_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Su_On => {
        Box::new(Tt_Ut_Su_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Sn_Ob => {
        Box::new(Tt_Ut_Sn_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Sn_Ou => {
        Box::new(Tt_Ut_Sn_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Ut_Sn_On => {
        Box::new(Tt_Ut_Sn_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Su_Ob => {
        Box::new(Tt_Uf_Su_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Su_Ou => {
        Box::new(Tt_Uf_Su_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Su_On => {
        Box::new(Tt_Uf_Su_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Sn_Ob => {
        Box::new(Tt_Uf_Sn_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Sn_Ou => {
        Box::new(Tt_Uf_Sn_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tt_Uf_Sn_On => {
        Box::new(Tt_Uf_Sn_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sb_Ob => {
        Box::new(Tf_Ut_Sb_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sb_Ou => {
        Box::new(Tf_Ut_Sb_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sb_On => {
        Box::new(Tf_Ut_Sb_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Su_Ob => {
        Box::new(Tf_Ut_Su_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Su_Ou => {
        Box::new(Tf_Ut_Su_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Su_On => {
        Box::new(Tf_Ut_Su_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sn_Ob => {
        Box::new(Tf_Ut_Sn_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sn_Ou => {
        Box::new(Tf_Ut_Sn_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Ut_Sn_On => {
        Box::new(Tf_Ut_Sn_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Su_Ob => {
        Box::new(Tf_Uf_Su_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Su_Ou => {
        Box::new(Tf_Uf_Su_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Su_On => {
        Box::new(Tf_Uf_Su_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Sn_Ob => {
        Box::new(Tf_Uf_Sn_Ob_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Sn_Ou => {
        Box::new(Tf_Uf_Sn_Ou_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
      WriteMode::Tf_Uf_Sn_On => {
        Box::new(Tf_Uf_Sn_On_Writer {
          class_id,
          ont_class: &self.classes[class_id],
          channel: &mut self.channel,
          predicates: &self.predicates,
          value_templates: &self.value_templates,
          buffer_oprops: &mut self.buffer_oprops,
          written_records: &mut self.written_records,
          always_write_records: &self.always_write_records,
        })
      },
    }
  }
  
  fn end_class(&mut self) {}
}

impl ExtractWriterResult for TTLStreamWriter<File> {
  fn extract_result(self: Box<Self>) -> WriteResult {
    WriteResult::None
  }
}

impl ExtractWriterResult for TTLStreamWriter<Vec<u8>> {
  fn extract_result(self: Box<Self>) -> WriteResult {
    WriteResult::Str1(unsafe { String::from_utf8_unchecked(self.channel.into_inner().unwrap()) })
  }
}

impl StreamWriterResult for TTLStreamWriter<File> {}
impl StreamWriterResult for TTLStreamWriter<Vec<u8>> {}