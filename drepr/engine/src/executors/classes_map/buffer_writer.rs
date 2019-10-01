use readers::prelude::Value;

pub struct BufferWriter<'a> {
  /// holding an array of `predicate_id` and `value` of data properties
  pub data_props: Vec<(usize, &'a Value)>,
  /// holding an array of `target_cls`, `predicate_id`, `object` and `is_object_blank` for object properties
  pub borrow_object_props: Vec<(usize, usize, &'a str, bool)>,
  pub object_props: Vec<(usize, usize, String, bool)>
}

impl<'a> BufferWriter<'a> {
  pub fn with_capacity(n_data_props: usize, n_object_props: usize) -> BufferWriter<'a> {
    BufferWriter {
      data_props: Vec::with_capacity(n_data_props),
      object_props: Vec::with_capacity(n_object_props),
      borrow_object_props: Vec::with_capacity(n_object_props)
    }
  }
  
  pub fn clear(&mut self) {
    self.data_props.clear();
    self.borrow_object_props.clear();
    self.object_props.clear();
  }
  
  pub fn write_data_property(&mut self, predicate_id: usize, value: &'a Value) {
    self.data_props.push((predicate_id, value));
  }
  
  pub fn write_borrow_object_property(&mut self, target_cls: usize, predicate_id: usize, object: &'a str, is_object_blank: bool) {
    self.borrow_object_props.push((target_cls, predicate_id, object, is_object_blank));
  }
  
  pub fn write_object_property(&mut self, target_cls: usize, predicate_id: usize, object: String, is_object_blank: bool) {
    self.object_props.push((target_cls, predicate_id, object, is_object_blank));
  }
}