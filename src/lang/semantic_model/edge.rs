use serde::{Deserialize, Serialize};

use crate::lang::semantic_model::node::GraphNode;
use crate::lang::semantic_model::SemanticModel;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Edge {
  pub edge_id: usize,
  pub source: usize,
  pub target: usize,
  pub rel_label: String,
  pub abs_label: String,
  pub is_subject: bool,
}

impl Edge {
  #[inline]
  pub fn get_target<'a>(&self, sm: &'a SemanticModel) -> &'a GraphNode {
    &sm.nodes[self.target]
  }
  
  #[inline]
  pub fn get_source<'a>(&self, sm: &'a SemanticModel) -> &'a GraphNode {
    &sm.nodes[self.source]
  }
}