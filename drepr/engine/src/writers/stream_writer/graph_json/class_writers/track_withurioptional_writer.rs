use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashMap};

use readers::prelude::Value;
use super::super::StreamClassWriter;
use super::super::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::graph_json::json_value_fmt::JSONValueFmt;

pub struct TrackWithURIOptionalWriter<'a, W: Write + Debug> {
  pub class_id: usize,
  pub ont_class: &'a str,
  pub node_channel: &'a mut BufWriter<W>,
  pub edge_channel: &'a mut BufWriter<W>,
  pub predicates: &'a [String],
  pub value_fmts: &'a [Box<dyn JSONValueFmt<W>>],
  pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
  pub written_records: &'a mut [HashMap<String, usize>],
  pub curr_node_id: usize,
  pub auto_increment_id: &'a mut usize,
}

impl<'a, W: Write + Debug> TrackWithURIOptionalWriter<'a, W> {
  #[inline]
  fn get_next_id(&mut self) -> usize {
    let id = *self.auto_increment_id;
    *self.auto_increment_id += 1;
    return id;
  }
}

impl<'a, W: Write + Debug> StreamClassWriter for TrackWithURIOptionalWriter<'a, W> {
  fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
    self.written_records[class_id].contains_key(subject)
  }

  fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
    if self.written_records[self.class_id].contains_key(subject) {
      return false;
    }

    self.curr_node_id = self.get_next_id();
    self.written_records[self.class_id].insert(subject.to_string(), self.curr_node_id);

    if !is_blank {
      write!(self.node_channel, r#"{{"id":{},"data":{{"@id":"{}","@type":{}"#,
             self.curr_node_id, subject, self.ont_class)
        .unwrap();
    } else {
      write!(self.node_channel, r#"{{"id":{},"data":{{"@type":{}"#,
             self.curr_node_id, self.ont_class)
        .unwrap();
    }

    return true;
  }

  fn end_record(&mut self) {
    write!(self.node_channel, "}}}}\n").unwrap();
  }

  fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
    if self.written_records[self.class_id].contains_key(subject) {
      return false;
    }

    self.curr_node_id = self.get_next_id();
    self.written_records[self.class_id].insert(subject.to_string(), self.curr_node_id);
    self.buffer_oprops[self.class_id].push(TempObjectProps {
      id: self.curr_node_id,
      props: vec![]
    });

    if !is_blank {
      write!(self.node_channel, r#"{{"id":{},"data":{{"@id":"{}","@type":{}"#,
             self.curr_node_id, subject, self.ont_class)
        .unwrap();
    } else {
      write!(self.node_channel, r#"{{"id":{},"data":{{"@type":{}"#,
             self.curr_node_id, self.ont_class)
        .unwrap();
    }

    return true;
  }

  fn end_partial_buffering_record(&mut self) {
    write!(self.node_channel, "}}}}\n").unwrap();
  }

  fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
    write!(self.node_channel, ",{}:", self.predicates[predicate_id]).unwrap();
    self.value_fmts[predicate_id].write_value(self.node_channel, value);
  }

  fn write_object_property(&mut self, target_cls: usize, _subject: &str, predicate_id: usize, object: &str, _is_subject_blank: bool, _is_object_blank: bool, _is_new_subj: bool) {
    write!(self.edge_channel, "{}\t{}\t{}\n",
           self.curr_node_id,
           self.written_records[target_cls][object],
           self.predicates[predicate_id]).unwrap();
  }

  fn buffer_object_property(&mut self, target_cls: usize, predicate_id: usize, object: String, _is_object_blank: bool) {
    self.buffer_oprops[self.class_id].last_mut().unwrap()
        .props
        .push((target_cls, predicate_id, object));
  }
}