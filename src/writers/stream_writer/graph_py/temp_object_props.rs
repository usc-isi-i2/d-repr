/// A temporary object holds links of the records to other records
#[derive(Default)]
pub struct TempObjectProps {
  /// id of the record in the graph
  pub id: String,
  /// list of links of the record to other records
  ///
  /// * .0 - id of the class of the target record that the record is linked to (`node_id` of the
  ///        class node in the semantic model)
  /// * .1 - id of the object property (predicate), which is `edge_id` of the edge in the semantic
  ///        model)
  /// * .2 - id of the target record
  pub props: Vec<(usize, usize, String)>,
}