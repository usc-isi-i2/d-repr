use serde::{Deserialize, Serialize};

pub use self::edge::Edge;
pub use self::node::GraphNode;
use crate::lang::semantic_model::node::DataNode;

pub mod node;
pub mod edge;

pub const DREPR_URI: &str = "drepr:uri";

#[derive(Default, Serialize, Debug, Clone)]
pub struct SemanticModel {
  // all class nodes must be before data nodes and literal nodes in the array
  pub nodes: Vec<GraphNode>,
  pub edges: Vec<Edge>,
  pub incoming_edges: Vec<Vec<usize>>,
  pub outgoing_edges: Vec<Vec<usize>>,
  pub prefixes: Vec<(String, String)>
}

impl SemanticModel {
  /// all class nodes must be before data nodes and literal nodes in the array
  pub fn new(nodes: Vec<GraphNode>, edges: Vec<Edge>, prefixes: Vec<(String, String)>) -> SemanticModel {
    let mut incoming_edges: Vec<Vec<usize>> = vec![vec![]; nodes.len()];
    let mut outgoing_edges: Vec<Vec<usize>> = vec![vec![]; nodes.len()];

    for (eid, edge) in edges.iter().enumerate() {
      assert_eq!(edge.edge_id, eid);
      incoming_edges[edge.target].push(eid);
      outgoing_edges[edge.source].push(eid);
    }
    
    // enforce the rules that all class nodes must be before data nodes and literal nodes in the array
    for i in 0..nodes.len() {
      assert_eq!(nodes[i].get_node_id(), i);
      if !nodes[i].is_class_node() {
        for j in i..nodes.len() {
          assert!(!nodes[j].is_class_node());
        }
        break;
      }
    }
    
    SemanticModel {
      nodes,
      edges,
      incoming_edges,
      outgoing_edges,
      prefixes
    }
  }
  
  #[inline]
  pub fn get_n_class_nodes(&self) -> usize {
    for i in 0..self.nodes.len() {
      if !self.nodes[i].is_class_node() {
        return i;
      }
    }
    
    return self.nodes.len();
  }
  
  /// Get the data node which is associated with `attr_id`
  #[inline]
  pub fn get_data_node_by_attr_id(&self, attr_id: usize) -> &DataNode {
    for n in &self.nodes {
      if n.is_data_node() && n.as_data_node().attr_id == attr_id {
        return n.as_data_node();
      }
    }
    
    unreachable!()
  }
  
  #[inline]
  pub fn get_source(&self, eid: usize) -> &GraphNode {
    &self.nodes[self.edges[eid].source]
  }
  
  #[inline]
  pub fn get_target(&self, eid: usize) -> &GraphNode {
    &self.nodes[self.edges[eid].target]
  }
  
  #[inline]
  pub fn get_edge(&self, source_id: usize, target_id: usize) -> Option<&Edge> {
    for eid in &self.incoming_edges[target_id] {
      if self.edges[*eid].source == source_id {
        return Some(&self.edges[*eid]);
      }
    }
    
    return None;
  }
}

impl<'de> Deserialize<'de> for SemanticModel {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct TmpSM {
      nodes: Vec<GraphNode>,
      edges: Vec<Edge>,
      prefixes: Vec<(String, String)>
    }
    
    let sm = TmpSM::deserialize(deserializer)?;
    Ok(SemanticModel::new(sm.nodes, sm.edges, sm.prefixes))
  }
}