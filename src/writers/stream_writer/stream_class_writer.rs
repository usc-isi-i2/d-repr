use readers::prelude::Value;

pub trait StreamClassWriter {
  /// Test if a record of a given class `class_id` has been written
  fn has_written_record(&self, class_id: usize, subject: &str) -> bool;
  
  /// Tell the writer that we are going to write a record, and we already have all information of the
  /// record to write.
  /// The function return `true` if this is a new record, otherwise `false`.
  ///
  /// # Arguments
  ///
  /// * `subject` - real id of the record, which is determined by the value of `drepr:uri` property
  ///                         of the record
  /// * `is_blank` - whether this subject is a blank node or not
  fn begin_record(&mut self, subject: &str, is_blank: bool) -> bool;

  /// Tell the writer that we finish writing all information of the record. This method cannot be
  /// called before `begin_record` method.
  ///
  /// Note: this method should not be called if the function `begin_record` return `false`
  fn end_record(&mut self);

  /// Tell the writer that we are going to write a record, and we don't have some information about
  /// the links between the records and other records as the other records haven't been generated
  /// yet.
  /// The function return `true` if this is a new record, otherwise `false`.
  ///
  /// # Arguments
  ///
  /// * `subject` - real id of the record, which is determined by the value of `drepr:uri` property
  ///                         of the record
  /// * `is_blank` - whether this subject is a blank node or not
  fn begin_partial_buffering_record(&mut self, subject: &str, is_blank: bool) -> bool;

  /// Tell the writer that we finish writing this record. This method cannot be called before
  /// `begin_partial_buffering_record` method
  ///
  /// Note: this method should not be called if the function `begin_partial_buffering_record` return
  /// `false`
  fn end_partial_buffering_record(&mut self);

  /// Write value of a data property of the current record.
  ///
  /// # Arguments
  ///
  /// * `subject` - real id of the current record
  /// * `predicate_id` - id of the data property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `value` - value of the property
  fn write_data_property(&mut self, subject: &str, predicate_id: usize, value: &Value);

  /// Write value of a object property of the current record.
  ///
  /// # Arguments
  ///
  /// * `target_cls`: id of the class of the target record that the current record is linked to,
  ///                 which is the `node_id` of the class node in the semantic model
  /// * `subject`: real id of the current record
  /// * `predicate_id` - id of the object property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `object` - id of the target record
  /// * `is_subject_blank` - whether the subject id is blank
  /// * `is_object_blank` - whether the object id is blank
  /// * `is_new_subj` - whether the current record is a new record or an existing record (obtained
  ///                   from the `begin_record` or `begin_partial_buffering_record` function
  fn write_object_property(&mut self, target_cls: usize, subject: &str, predicate_id: usize, object: &str, is_subject_blank: bool, is_object_blank: bool, is_new_subj: bool);

  /// Write value of a object property of the current record into buffer because we haven't generated
  /// the target object, it may be missed later.
  ///
  /// # Arguments
  ///
  /// * `target_cls`: id of the class of the target record that the current record is linked to,
  ///                 which is the `node_id` of the class node in the semantic model
  /// * `predicate_id` - id of the object property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `object` - id of the target record
  /// * `is_object_blank` - whether the object is blank node or not
  fn buffer_object_property(&mut self, target_cls: usize, predicate_id: usize, object: String, is_object_blank: bool);
}
