use std::io::{BufWriter, Write};
use std::fmt::Debug;

use hashbrown::{HashSet};

use readers::prelude::Value;
use crate::writers::stream_writer::StreamClassWriter;
use crate::writers::stream_writer::turtle::temp_object_props::TempObjectProps;
use crate::writers::stream_writer::turtle::value_fmt::ValueFmt;


#[allow(dead_code)]
pub struct GenericWriter<'a, W: Write + Debug> {
  pub class_id: usize,
  pub ont_class: &'a str,
  pub channel: &'a mut BufWriter<W>,
  pub predicates: &'a [String],
  pub value_templates: &'a [ValueFmt],
  pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
  pub written_records: &'a mut [HashSet<String>],
  pub always_write_records: &'a [bool],
}

impl<'a, W: Write + Debug> StreamClassWriter for GenericWriter<'a, W> {
  #[inline]
  fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
    self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
  }
  
  fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
    // check if has been inserted before
    if self.written_records[self.class_id].contains(subject) {
      return false;
    }
    
    self.written_records[self.class_id].insert(subject.to_string());
    if is_blank {
      write!(self.channel, "{} a {};\n", subject, self.ont_class).unwrap();
    } else {
      write!(self.channel, "<{}> a {};\n", subject, self.ont_class).unwrap();
    }

    return true;
  }

  fn end_record(&mut self) {
    self.channel.write("\t.\n".as_bytes()).unwrap();
  }

  fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
    // check if has been inserted before
    if self.written_records[self.class_id].contains(subject) {
      return false;
    }
    
    self.buffer_oprops[self.class_id].push(TempObjectProps {
      id: subject.to_string(),
      is_blank,
      props: vec![],
    });
    
    self.written_records[self.class_id].insert(subject.to_string());
    if is_blank {
      write!(self.channel, "{} a {};\n", subject, self.ont_class).unwrap();
    } else {
      write!(self.channel, "<{}> a {};\n", subject, self.ont_class).unwrap();
    }

    return true;
  }

  fn end_partial_buffering_record(&mut self) {
    self.channel.write("\t.\n".as_bytes()).unwrap();
  }

  fn write_data_property(&mut self, _subject: &str, predicate_id: usize, value: &Value) {
    match value {
      Value::Null => {
        // encounter a null value, TTL doesn't have a way to represent a null value, so we should panic
        // because null may mean different things
        panic!("Cannot write null value because RDF doesn't have a way to represent it")
      },
      Value::Str(v) => {
        self.value_templates[predicate_id].write_string_value(&mut self.channel, &v.replace("\"", "\\\""));
      }
      Value::Bool(v) => {
        self.value_templates[predicate_id].write_value(&mut self.channel, &v.to_string());
      }
      Value::I64(v) => {
        self.value_templates[predicate_id].write_value(&mut self.channel, &v.to_string());
      }
      Value::F64(v) => {
        self.value_templates[predicate_id].write_value(&mut self.channel, &v.to_string());
      }
      Value::Array(_) => unimplemented!("TTL writers does not support writing array yet. The input value is: {:?}", value),
      Value::Object(_) => unimplemented!("TTL writers does not support writing array yet. The input value is: {:?}", value),
    }
  }

  fn write_object_property(&mut self, _target_cls: usize, subject: &str, predicate_id: usize, object: &str, is_subject_blank: bool, is_object_blank: bool, is_new_subj: bool) {
    if is_new_subj {
      if is_object_blank {
        write!(self.channel, "\t{} {};\n", self.predicates[predicate_id], object).unwrap();
      } else {
        write!(self.channel, "\t{} <{}>;\n", self.predicates[predicate_id], object).unwrap();
      }
    } else {
      if is_subject_blank {
        write!(self.channel, "{}", subject).unwrap();
      } else {
        write!(self.channel, "<{}>", subject).unwrap();
      }
      
      if is_object_blank {
        write!(self.channel, " {} {}.\n", self.predicates[predicate_id], object).unwrap();
      } else {
        write!(self.channel, " {} <{}>.\n", self.predicates[predicate_id], object).unwrap();
      }
    }
  }

  fn buffer_object_property(&mut self, _target_cls: usize, predicate_id: usize, object: String, is_object_blank: bool) {
    self.buffer_oprops[self.class_id].last_mut().unwrap()
      .props
      .push((predicate_id, object, is_object_blank));
  }
}