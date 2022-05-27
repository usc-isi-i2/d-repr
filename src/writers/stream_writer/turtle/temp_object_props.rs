/// A temporary object holds links of the records to other records
#[derive(Default)]
pub struct TempObjectProps {
  /// real id of the record
  pub id: String,
  
  /// whether this record is blank node
  pub is_blank: bool,
  
  /// list of links of the record to other records
  ///
  /// * .0 - id of the object property (predicate), which is `edge_id` of the edge in the semantic
  ///        model)
  /// * .1 - id of the target record
  /// * .2 - bool value telling if this object is blank node
  pub props: Vec<(usize, String, bool)>,
}