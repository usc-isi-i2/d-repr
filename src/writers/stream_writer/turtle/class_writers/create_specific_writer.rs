macro_rules! create_writer {
  (
    $writer_class:ident,
    // subj_type is either: b1: always blank, b2: always uri, b3: can either blank or uri
    subj_type=$subj_type:ident,
    // obj_type is either: b1: always blank, b2: always uri, b3: can either blank or uri
    obj_type=$obj_type:ident
    $(, no_duplicated_subj=$no_duplicated_subj:literal)?
    // we need to keep track subject when the subject is not optional
    $(, keep_track_subj=$keep_track_subj:literal)?
  ) => {

#[allow(non_camel_case_types)]
pub struct $writer_class<'a, W: Write + Debug> {
  pub class_id: usize,
  pub ont_class: &'a str,
  pub channel: &'a mut BufWriter<W>,
  pub predicates: &'a [String],
  pub value_templates: &'a [ValueFmt],
  pub buffer_oprops: &'a mut [Vec<TempObjectProps>],
  pub written_records: &'a mut [HashSet<String>],
  pub always_write_records: &'a [bool],
}

impl<'a, W: Write + Debug> StreamClassWriter for $writer_class<'a, W> {
  #[inline]
  fn has_written_record(&self, class_id: usize, subject: &str) -> bool {
    self.always_write_records[class_id] || self.written_records[class_id].contains(subject)
  }
  
  #[allow(unused_variables)]
  fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool {
    // if there is no duplication, we don't need to perform the following block
    block_discard!($( $no_duplicated_subj ;)? {
      // if the subject has been written, don't write it and return false
      if self.written_records[self.class_id].contains(subject) {
        return false;
      }
    });
    
    block_discard!($( $keep_track_subj ;)? {
      // we have to keep track of whether this has been written (this have little thing to do
      // with subject uniqueness)
      self.written_records[self.class_id].insert(subject.to_string());
    });
    
    // write subject
    exclusive_if!($subj_type ; is_blank ; {
      write!(self.channel, "{} a {};\n", subject, self.ont_class).unwrap();
    } else {
      write!(self.channel, "<{}> a {};\n", subject, self.ont_class).unwrap();
    });

    return true;
  }

  fn end_record(&mut self) {
    self.channel.write("\t.\n".as_bytes()).unwrap();
  }

  fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool {
    // if there is no duplication, we don't need to perform the following block
    block_discard!($( $no_duplicated_subj ;)? {
      // if the subject has been written, don't write it and return false
      if self.written_records[self.class_id].contains(subject) {
        return false;
      }
    });
    
    block_discard!($( $keep_track_subj ;)? {
      // we have to keep track of whether this has been written (this have little thing to do
      // with subject uniqueness)
      self.written_records[self.class_id].insert(subject.to_string());
    });
    
    self.buffer_oprops[self.class_id].push(TempObjectProps {
      id: subject.to_string(),
      is_blank,
      props: vec![],
    });
    
    // write subject
    exclusive_if!($subj_type ; is_blank ; {
      write!(self.channel, "{} a {};\n", subject, self.ont_class).unwrap();
    } else {
      write!(self.channel, "<{}> a {};\n", subject, self.ont_class).unwrap();
    });

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
  
  #[allow(unused_variables)]
  fn write_object_property(&mut self, _target_cls: usize, subject: &str, predicate_id: usize, object: &str, is_subject_blank: bool, is_object_blank: bool, is_new_subj: bool) {
    // if there is no duplication, then `is_new_subj` is always true, only keep the true branch, otherwise
    // we keep the whole if
    mif!(is_new_subj; $($no_duplicated_subj exec_true_branch ; )? {
      exclusive_if!($obj_type ; is_object_blank ; {
        write!(self.channel, "\t{} {};\n", self.predicates[predicate_id], object).unwrap();
      } else {
        write!(self.channel, "\t{} <{}>;\n", self.predicates[predicate_id], object).unwrap();
      });
    } else {
      exclusive_if!($subj_type ; is_subject_blank ; {
        write!(self.channel, "{} a {};\n", subject, self.ont_class).unwrap();
      } else {
        write!(self.channel, "<{}> a {};\n", subject, self.ont_class).unwrap();
      });
      
      exclusive_if!($obj_type ; is_object_blank ; {
        write!(self.channel, " {} {}.\n", self.predicates[predicate_id], object).unwrap();
      } else {
        write!(self.channel, " {} <{}>.\n", self.predicates[predicate_id], object).unwrap();
      });
    });
  }

  fn buffer_object_property(&mut self, _target_cls: usize, predicate_id: usize, object: String, is_object_blank: bool) {
    self.buffer_oprops[self.class_id].last_mut().unwrap()
      .props
      .push((predicate_id, object, is_object_blank));
  }
}
  }
}
