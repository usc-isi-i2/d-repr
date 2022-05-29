use super::stream_class_writer::StreamClassWriter;
use super::WriteMode;
use pyo3::prelude::Py;
use pyo3::types::PyDict;
use readers::into_enum_type_impl;

pub trait StreamWriter {
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
  fn begin_class<'a>(
    &'a mut self,
    class_id: usize,
    write_mode: WriteMode,
  ) -> Box<dyn StreamClassWriter + 'a>;

  /// Tell the writer that we finish writing all records of the ontology class.
  fn end_class(&mut self);
}

pub trait ExtractWriterResult {
  fn extract_result(self: Box<Self>) -> WriteResult;
}

pub trait StreamWriterResult: StreamWriter + ExtractWriterResult {}

pub enum WriteResult {
  None,
  Str1(String),
  Str2(String, String),
  GraphPy(Vec<Vec<Py<PyDict>>>),
}

impl WriteResult {
  into_enum_type_impl!(WriteResult, into_str1, Str1, "String", String);
  into_enum_type_impl!(
    WriteResult,
    into_graphpy,
    GraphPy,
    "Vec<Vec<Py<PyDict>>>",
    Vec<Vec<Py<PyDict>>>
  );
  pub fn into_str2(self) -> (String, String) {
    match self {
      WriteResult::Str2(s0, s1) => (s0, s1),
      _ => panic!("ValueError: cannot convert non-str2 to str2"),
    }
  }
}
