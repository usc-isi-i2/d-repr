use crate::readers::Value;
use crate::models::SemanticModel;

mod turtle;
//mod json_line;
mod graph_json;

pub use self::turtle::TurtleWriter;
pub use self::graph_json::GraphJSONWriter;
//pub use self::json_line::JsonLineWriter;

/// A trail for writers to stream records to an output channel in specific formats
///
/// # Usage
///
/// After create the writer and before using, the writer has to be initialized by a semantic model (`init` method). The semantic model
/// contains information about the schema/ontology that may allow extra-optimization such as generating templates for 
/// different records.
///
/// Then, the `begin` method of the writer will be invoked only once, before any other methods except `init`
/// to tell the writer that we are going to start writing records.
/// 
/// Then, to write records for a specific ontology class, you call `begin_class` method. Note that we cannot
/// mix records of different classes together, you have to write them separatedly. When you finish writing records
/// of the class, you need to indicate that the write process for the class is finish by calling `end_class`.
/// 
/// To write a record, you start with the method `begin_record` or `begin_partial_buffering_record` depends
/// on whether ids of other records that it links to is known or not. If not, we need to buffer the link
/// or the whole subject using `begin_partial_buffering_record`.
///
/// Then, properties of the record is written by order. First, you write its data properties using `write_data_property`.
/// After that, you write its known object properties using `write_object_property` and then unknown object properties
/// using `buffer_object_property` (only if you begin with `begin_partial_buffering_record`).
///
/// You tell the writer that you have finished writing all properties of the record by calling `end_record` or
/// `end_partial_buffering_record` depends on the begin method that you used. Note that, if you are modifying
/// an old record (`begin_record` or `begin_partial_buffering_record` return false), you should not
/// call this method.
///
/// After you finish writing all records, you stop the writing process by calling the `end` method. This allow
/// the writer to flush all pending operations in its buffer (if have).
///
/// # Remark
///
/// When the target of an object property is unknownn at the time we write (using `buffer_object_property`
/// function), if the link is non-optional then the current writing record should also be discard.
///
/// However, as we only buffer object property if there is a cycle in the semantic model. When the
/// buffer link is non-optional, then we have to buffer all classes in the cycle and filter out
/// records with missing links. For example, we have two classes: A & B, and two edges (A) <-> (B).
/// Assume that B is generated first, the the link B -> A is buffered, as B -> A is non-optional,
/// we have to buffer all B records. When we write A records, we need to check if the B records that
/// A records are linked to are presented, which may require cascaded check (as B records may also
/// link to A records which haven't generated yet). So that we need to buffer A records as well.
///
/// The case above prohibite streaming records, which makes implementing stream writer harder than
/// necessary. Therefore, this writer does not support writing non-optional cyclic semantic model,
/// and implementors can assume that every record/object (including the data & object properties) that
/// they receive is always need to be stream to the channel, except the buffered object properties.
///
/// ```
/// # use drepr::writers::{TurtleWriter, StreamStateWriter};
/// # use drepr::readers::Value;
/// let mut writer = TurtleWriter::write2str();
/// writer.init(sm);
///
/// writer.begin();
///
/// writer.begin_class(0);
/// let new_record = writer.begin_partial_buffering_record("_:abc", "http://abc.org");
///
/// if new_record {
///     writer.write_data_property("http://abc.org", 5, &Value::Str("John Wick".to_string()));
/// }
///
/// writer.write_object_property(1, "http://abc", 6, "_:def", true);
/// writer.buffer_object_property(1, 7, "_:ghk".to_string());
///
/// if new_record {
///     writer.end_partial_buffering_record();
/// }
/// writer.end_class();
/// 
/// writer.end();
/// ```
pub trait StreamStateWriter {
  /// Initialize the writer with information from a semantic model. This allow extra-optimization
  /// such as generating templates for different classes.
  ///
  /// This is a mandatory step
  fn init(&mut self, sm: &SemanticModel);

  /// Test if a record has been written
  fn has_written_record(&self, class_id: usize, pseudo_id: &str) -> bool;

  /// Tell the writer that we start to write records. This method must be invoked only once, and
  /// after `init` method
  fn begin(&mut self);

  /// Tell the writer that we finished writing all records. This method must be invoked only once,
  /// and after every other methods
  fn end(&mut self);

  /// Tell the writer that we are going to write records of a class.
  ///
  /// # Arguments
  ///
  /// * `class_id` - the id of the class (`node_id` of the class node in the semantic model)
  fn begin_class(&mut self, class_id: usize);

  /// Tell the writer that we finish writing all records of the ontology class.
  fn end_class(&mut self);

  /// Tell the writer that we are going to write a record, and we already have all information of the
  /// record to write.
  /// The function return `true` if this is a new record, otherwise `false`.
  ///
  /// # Arguments
  ///
  /// * `subject_pseudo_id` - pseudo id of the record, which is determined by the index of the
  ///                         record in the dataset
  /// * `subject` - real id of the record, which is determined by the value of `drepr:uri` property
  ///                         of the record
  fn begin_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool;

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
  /// * `subject_pseudo_id` - pseudo id of the record, which is determined by the index of the
  ///                         record in the dataset
  /// * `subject` - real id of the record, which is determined by the value of `drepr:uri` property
  ///                         of the record
  fn begin_partial_buffering_record(&mut self, subject_pseudo_id: &str, subject: &str) -> bool;

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
  /// * `predicate` - id of the data property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `value` - value of the property
  fn write_data_property(&mut self, subject: &str, predicate: usize, value: &Value);

  /// Write value of a object property of the current record.
  ///
  /// # Arguments
  ///
  /// * `target_cls`: id of the class of the target record that the current record is linked to,
  ///                 which is the `node_id` of the class node in the semantic model
  /// * `subject`: real id of the current record
  /// * `predicate` - id of the object property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `object_pseudo_id` - pseudo id of the target record, which is determined by the index of the
  ///                        target record in the dataset
  /// * `new_subject` - whether the current record is a new record or an existing record (obtained
  ///                   from the `begin_record` or `begin_partial_buffering_record` function
  fn write_object_property(&mut self, target_cls: usize, subject: &str, predicate: usize, object_pseudo_id: &str, new_subject: bool);

  /// Write value of a object property of the current record into buffer because we haven't generated
  /// the target object, so we don't know its real id.
  ///
  /// # Arguments
  ///
  /// * `target_cls`: id of the class of the target record that the current record is linked to,
  ///                 which is the `node_id` of the class node in the semantic model
  /// * `subject`: real id of the current record
  /// * `predicate` - id of the object property that we are writing to (`edge_id` of the edge in the
  ///                 semantic model)
  /// * `object_pseudo_id` - pseudo id of the target record, which is determined by the index of the
  ///                        target record in the dataset
  fn buffer_object_property(&mut self, target_cls: usize, predicate: usize, object_pseudo_id: String);
}