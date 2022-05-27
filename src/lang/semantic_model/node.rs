use serde::{Deserialize, Serialize};

use readers::{as_enum_type_impl, is_enum_type_impl};
use super::SemanticModel;
use crate::lang::DREPR_URI;
use readers::value::Value;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClassNode {
  pub node_id: usize,
  pub rel_label: String,
  pub abs_label: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DataNode {
  pub node_id: usize,
  pub attr_id: usize,
  pub data_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LiteralNode {
  pub node_id: usize,
  pub val: Value,
  pub data_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum GraphNode {
  #[serde(rename = "class_node")]
  ClassNode(ClassNode),
  #[serde(rename = "data_node")]
  DataNode(DataNode),
  #[serde(rename = "literal_node")]
  LiteralNode(LiteralNode),
}

impl GraphNode {
  as_enum_type_impl!(GraphNode, as_class_node, as_mut_class_node, ClassNode, "class node", ClassNode);
  as_enum_type_impl!(GraphNode, as_data_node, as_mut_data_node, DataNode, "data node", DataNode);
  as_enum_type_impl!(GraphNode, as_literal_node, as_mut_literal_node, LiteralNode, "literal node", LiteralNode);
  
  is_enum_type_impl!(GraphNode::is_class_node(ClassNode(_)));
  is_enum_type_impl!(GraphNode::is_data_node(DataNode(_)));
  is_enum_type_impl!(GraphNode::is_literal_node(LiteralNode(_)));
  
  pub fn get_node_id(&self) -> usize {
    match self {
      GraphNode::ClassNode(n) => n.node_id,
      GraphNode::DataNode(n) => n.node_id,
      GraphNode::LiteralNode(n) => n.node_id
    }
  }
}

impl ClassNode {
  /// Compute a pseudo prefix from the label of the class node, so that it complies
  /// with standard turtle blank node
  pub fn get_pseudo_prefix(&self) -> String {
    let short_lbl = self.rel_label
      .rsplitn(2, "/").next().unwrap()
      .rsplitn(2, ":").next().unwrap();
    
    format!("{}{}", short_lbl, self.node_id)
  }

  pub fn is_blank_node(&self, sm: &SemanticModel) -> bool {
    for &e in &sm.outgoing_edges[self.node_id] {
      if sm.edges[e].rel_label == DREPR_URI {
        return false;
      }
    }
    return true;
  }
}
